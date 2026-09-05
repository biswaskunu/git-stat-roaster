use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GitHubUser {
    pub login: String,
    pub public_repos: u32,
    pub followers: u32,
    pub following: u32,
    pub created_at: String,
    pub bio: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Repo {
    pub name: String,
    pub language: Option<String>,
    pub stargazers_count: u32,
    pub forks_count: u32,
    pub fork: bool,
    pub created_at: String,
}

/// Same shape of failures the old CLI handled via println!+return, now
/// carried as a proper error type so the web handler can turn them into
/// a JSON response with the right HTTP status instead of exiting.
#[derive(Debug)]
pub enum FetchError {
    UserNotFound(String),
    ApiError(StatusCode),
    Network(String),
    // Unexpected JSON shape — most often a rate-limit response
    // disguised as a 200, per the CLI's known limitation.
    UnexpectedResponse(String),
}

impl IntoResponse for FetchError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            FetchError::UserNotFound(username) => {
                (StatusCode::NOT_FOUND, format!("no such user: {}", username))
            }
            FetchError::ApiError(s) => (
                StatusCode::BAD_GATEWAY,
                format!("github api error: {}", s),
            ),
            FetchError::Network(e) => (
                StatusCode::BAD_GATEWAY,
                format!("network request to github failed: {}", e),
            ),
            FetchError::UnexpectedResponse(e) => (
                StatusCode::BAD_GATEWAY,
                format!(
                    "unexpected response shape from github ({}). this is often a disguised rate-limit response — wait an hour and retry.",
                    e
                ),
            ),
        };

        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}

pub async fn fetch_user(client: &reqwest::Client, username: &str) -> Result<GitHubUser, FetchError> {
    let url = format!("https://api.github.com/users/{}", username);
    let resp = client
        .get(&url)
        .header("User-Agent", "git-stat-roaster")
        .header("Accept", "application/vnd.github+json")
        .send()
        .await
        .map_err(|e| FetchError::Network(e.to_string()))?;

    if resp.status() == StatusCode::NOT_FOUND {
        return Err(FetchError::UserNotFound(username.to_string()));
    }
    if !resp.status().is_success() {
        return Err(FetchError::ApiError(resp.status()));
    }

    resp.json::<GitHubUser>()
        .await
        .map_err(|e| FetchError::UnexpectedResponse(e.to_string()))
}

/// Paginates through all of a user's public repos (GitHub caps at 100/page),
/// same loop as the CLI version.
pub async fn fetch_all_repos(client: &reqwest::Client, username: &str) -> Result<Vec<Repo>, FetchError> {
    let mut repos: Vec<Repo> = Vec::new();
    let mut page = 1u32;
    const PER_PAGE: u32 = 100;

    loop {
        let url = format!(
            "https://api.github.com/users/{}/repos?per_page={}&page={}",
            username, PER_PAGE, page
        );
        let resp = client
            .get(&url)
            .header("User-Agent", "git-stat-roaster")
            .header("Accept", "application/vnd.github+json")
            .send()
            .await
            .map_err(|e| FetchError::Network(e.to_string()))?;

        if !resp.status().is_success() {
            return Err(FetchError::ApiError(resp.status()));
        }

        let page_repos: Vec<Repo> = resp
            .json()
            .await
            .map_err(|e| FetchError::UnexpectedResponse(e.to_string()))?;

        let got = page_repos.len();
        repos.extend(page_repos);

        if got < PER_PAGE as usize {
            break;
        }
        page += 1;
    }

    Ok(repos)
}
