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
    let created_year: i32 = created_at.get(0..4)
        .expect("too short date argument")
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

#[tokio::main]
async fn main() {
    let username = std::env::args()
        .nth(1)
        .expect("usage: git-stat-roaster <github-username>");

    let client = reqwest::Client::new();

    // fetching user profile
    let user_url = format!("https://api.github.com/users/{}", username);
    let resp = client
        .get(&user_url)
        .header("User-Agent", "git-stat-roaster")
        .header("Accept", "application/vnd.github+json")
        .send()
        .await
        .expect("request failed");

    // errors
    if resp.status() == reqwest::StatusCode::NOT_FOUND {
        println!("no such user: {}", username);
        return;
    }

    if !resp.status().is_success() {
        println!("github api error: {}", resp.status());
        return;
    }

    let user: GitHubUser = resp.json().await.expect("failed to parse user json");

    // fetch repos
    let repos_url = format!("https://api.github.com/users/{}/repos", username);
    let repos_resp = client
        .get(&repos_url)
        .header("User-Agent", "git-stat-roaster")
        .header("Accept", "application/vnd.github+json")
        .send()
        .await
        .expect("request failed");

    if !repos_resp.status().is_success() {
        println!("github api error fetching repos: {}", repos_resp.status());
        return;
    }

    let repos: Vec<Repo> = repos_resp.json().await.expect("failed to parse repos json");

    // derived stats
    let age_years = account_age_years(&user.created_at);
    let non_fork_repos: Vec<&Repo> = repos.iter().filter(|r| !r.fork).collect();
    let language = most_used_language(&repos);
    let top_starred = most_starred_repo(&repos);
    let top_forked = most_forked_repo(&repos);


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
}