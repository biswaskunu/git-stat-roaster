use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GitHubUser {
    login: String,
    // TODO: add public_repos, followers, following, created_at, bio
}

#[derive(Debug, Deserialize)]
struct Repo {
    name: String,
    // TODO: add language, stargazers_count, forks_count, fork
}

#[tokio::main]
async fn main() {
    let username = std::env::args()
        .nth(1)
        .expect("usage: git-stat-roaster <github-username>");

    let client = reqwest::Client::new();

    // --- fetch user profile ---
    let user_url = format!("https://api.github.com/users/{}", username);
    let resp = client
        .get(&user_url)
        .header("User-Agent", "git-stat-roaster")
        .header("Accept", "application/vnd.github+json")
        .send()
        .await
        .expect("request failed");

    if resp.status() == reqwest::StatusCode::NOT_FOUND {
        println!("no such user: {}", username);
        return;
    }

    // TODO: handle other non-success statuses (rate limit = 403, etc.) — at least don't panic blindly

    let user: GitHubUser = resp.json().await.expect("failed to parse user json");
    println!("{:#?}", user);

    // TODO: repeat the same pattern for GET /users/{username}/repos
    // note: repos endpoint returns a JSON *array*, so you'll deserialize into Vec<Repo>
}