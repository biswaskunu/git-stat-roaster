use serde::Deserialize;

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
    println!("{:#?}", user);


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
    println!("{:#?}", repos);
}