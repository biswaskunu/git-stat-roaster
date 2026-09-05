mod jokes;

use jokes::Stats;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct GitHubUser {
    login: String,
    public_repos: u32,
    followers: u32,
    following: u32,
    created_at: String,
    bio: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Repo {
    name: String,
    language: Option<String>,
    stargazers_count: u32,
    forks_count: u32,
    fork: bool,
    created_at: String,
}

fn current_year() -> i32 {
    use std::time::{SystemTime, UNIX_EPOCH};

    let secs_since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock is before unix epoch")
        .as_secs();

    // average seconds in a year, accounts for leap years well enough for our purposes
    const SECS_PER_YEAR: u64 = 31_557_600;

    1970 + (secs_since_epoch / SECS_PER_YEAR) as i32
}

fn account_age_years(created_at: &str) -> i32 {
    let created_year: i32 = created_at[0..4]
        .parse()
        .expect("created_at did not start with a 4-digit year");
    current_year() - created_year
}

fn most_used_language(repos: &[Repo]) -> Option<String> {
    let mut counts: HashMap<&str, u32> = HashMap::new();

    for repo in repos.iter().filter(|r| !r.fork) {
        if let Some(lang) = &repo.language {
            *counts.entry(lang.as_str()).or_insert(0) += 1;
        }
    }

    counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(lang, _)| lang.to_string())
}

fn most_starred_repo(repos: &[Repo]) -> Option<&Repo> {
    repos
        .iter()
        .filter(|r| !r.fork)
        .max_by_key(|r| r.stargazers_count)
}

fn most_forked_repo(repos: &[Repo]) -> Option<&Repo> {
    repos
        .iter()
        .filter(|r| !r.fork)
        .max_by_key(|r| r.forks_count)
}

fn oldest_repo(repos: &[Repo]) -> Option<&Repo> {
    // created_at is an ISO 8601 string (e.g. "2019-05-02T12:00:00Z"),
    // so plain string comparison sorts chronologically same as a real date type would.
    repos
        .iter()
        .filter(|r| !r.fork)
        .min_by(|a, b| a.created_at.cmp(&b.created_at))
}

#[tokio::main]
async fn main() {
    let username = match std::env::args().nth(1) {
        Some(u) => u,
        None => {
            println!("usage: git-stat-roaster <github-username>");
            return;
        }
    };

    let client = reqwest::Client::new();

    // fetching user profile
    let user_url = format!("https://api.github.com/users/{}", username);
    let resp = match client
        .get(&user_url)
        .header("User-Agent", "git-stat-roaster")
        .header("Accept", "application/vnd.github+json")
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            println!("network request failed: {}", e);
            return;
        }
    };

    // errors
    if resp.status() == reqwest::StatusCode::NOT_FOUND {
        println!("no such user: {}", username);
        return;
    }

    if !resp.status().is_success() {
        println!("github api error: {}", resp.status());
        return;
    }

    let user: GitHubUser = match resp.json().await {
        Ok(u) => u,
        Err(e) => {
            println!("unexpected response shape from github (couldn't parse user data): {}", e);
            println!("this is often a rate-limit response disguised as a 200 — wait an hour and retry.");
            return;
        }
    };

    // fetch repos, paginated (GitHub caps at 100/page, defaults to 30 without per_page)
    let mut repos: Vec<Repo> = Vec::new();
    let mut page = 1u32;
    const PER_PAGE: u32 = 100;

    loop {
        let repos_url = format!(
            "https://api.github.com/users/{}/repos?per_page={}&page={}",
            username, PER_PAGE, page
        );
        let repos_resp = match client
            .get(&repos_url)
            .header("User-Agent", "git-stat-roaster")
            .header("Accept", "application/vnd.github+json")
            .send()
            .await
        {
            Ok(r) => r,
            Err(e) => {
                println!("network request failed while fetching repos: {}", e);
                return;
            }
        };

        if !repos_resp.status().is_success() {
            println!("github api error fetching repos: {}", repos_resp.status());
            return;
        }

        let page_repos: Vec<Repo> = match repos_resp.json().await {
            Ok(r) => r,
            Err(e) => {
                println!("unexpected response shape from github (couldn't parse repos data): {}", e);
                println!("this is often a rate-limit response disguised as a 200 — wait an hour and retry.");
                return;
            }
        };
        let got = page_repos.len();
        repos.extend(page_repos);

        if got < PER_PAGE as usize {
            break;
        }
        page += 1;
    }

    // derived stats
    let age_years = account_age_years(&user.created_at);
    let non_fork_repos: Vec<&Repo> = repos.iter().filter(|r| !r.fork).collect();
    let language = most_used_language(&repos);
    let top_starred = most_starred_repo(&repos);
    let top_forked = most_forked_repo(&repos);
    let oldest = oldest_repo(&repos);

    println!("--- {} ---", user.login);
    println!("account age: {} years", age_years);
    println!("public_repos (raw, incl. forks): {}", user.public_repos);
    println!("non-fork repos: {}", non_fork_repos.len());
    println!("followers/following: {}/{}", user.followers, user.following);
    println!("bio: {:?}", user.bio);
    println!("most-used language: {:?}", language);
    println!(
        "most-starred repo: {:?}",
        top_starred.map(|r| (&r.name, r.stargazers_count))
    );
    println!(
        "most-forked repo: {:?}",
        top_forked.map(|r| (&r.name, r.forks_count))
    );
    println!(
        "oldest repo: {:?}",
        oldest.map(|r| (&r.name, &r.created_at))
    );

    // roast
    let stats = Stats {
        username: &user.login,
        account_age_years: age_years,
        non_fork_repo_count: non_fork_repos.len() as u32,
        most_used_language: language.as_deref(),
        top_starred_repo: top_starred.map(|r| (r.name.as_str(), r.stargazers_count)),
        top_forked_repo: top_forked.map(|r| (r.name.as_str(), r.forks_count)),
        bio: user.bio.as_deref(),
        followers: user.followers,
        following: user.following,
    };

    println!();
    println!("{}", jokes::generate_roast(&stats));
}
