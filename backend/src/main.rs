mod github;
mod jokes;
mod stats;

use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

#[derive(Clone)]
struct AppState {
    client: Arc<reqwest::Client>,
}

#[derive(Serialize)]
struct StarredRepo {
    name: String,
    stars: u32,
}

#[derive(Serialize)]
struct ForkedRepo {
    name: String,
    forks: u32,
}

#[derive(Serialize)]
struct OldestRepo {
    name: String,
    created_at: String,
}

#[derive(Serialize)]
struct RoastResponse {
    username: String,
    account_age_years: i32,
    public_repos: u32,
    non_fork_repo_count: u32,
    followers: u32,
    following: u32,
    bio: Option<String>,
    most_used_language: Option<String>,
    top_starred_repo: Option<StarredRepo>,
    top_forked_repo: Option<ForkedRepo>,
    oldest_repo: Option<OldestRepo>,
    roast: String,
}

async fn roast_handler(
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> Result<Json<RoastResponse>, github::FetchError> {
    let user = github::fetch_user(&state.client, &username).await?;
    let repos = github::fetch_all_repos(&state.client, &username).await?;

    let age_years = stats::account_age_years(&user.created_at);
    let non_fork_count = repos.iter().filter(|r| !r.fork).count() as u32;
    let language = stats::most_used_language(&repos);
    let top_starred = stats::most_starred_repo(&repos);
    let top_forked = stats::most_forked_repo(&repos);
    let oldest = stats::oldest_repo(&repos);

    let roast_stats = jokes::Stats {
        username: &user.login,
        account_age_years: age_years,
        non_fork_repo_count: non_fork_count,
        most_used_language: language.as_deref(),
        top_starred_repo: top_starred.map(|r| (r.name.as_str(), r.stargazers_count)),
        top_forked_repo: top_forked.map(|r| (r.name.as_str(), r.forks_count)),
        bio: user.bio.as_deref(),
        followers: user.followers,
        following: user.following,
    };
    let roast = jokes::generate_roast(&roast_stats);

    Ok(Json(RoastResponse {
        username: user.login,
        account_age_years: age_years,
        public_repos: user.public_repos,
        non_fork_repo_count: non_fork_count,
        followers: user.followers,
        following: user.following,
        bio: user.bio,
        most_used_language: language,
        top_starred_repo: top_starred.map(|r| StarredRepo {
            name: r.name.clone(),
            stars: r.stargazers_count,
        }),
        top_forked_repo: top_forked.map(|r| ForkedRepo {
            name: r.name.clone(),
            forks: r.forks_count,
        }),
        oldest_repo: oldest.map(|r| OldestRepo {
            name: r.name.clone(),
            created_at: r.created_at.clone(),
        }),
        roast,
    }))
}

#[tokio::main]
async fn main() {
    let state = AppState {
        client: Arc::new(reqwest::Client::new()),
    };

    // Permissive CORS so the frontend can also be run standalone (e.g. a
    // local dev server on a different port) instead of only through the
    // static-file serving below.
    let app = Router::new()
        .route("/api/roast/:username", get(roast_handler))
        .with_state(state)
        .layer(CorsLayer::permissive())
        .fallback_service(ServeDir::new("../frontend"));

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    println!("git-stat-roaster listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("failed to bind to address");
    axum::serve(listener, app).await.expect("server error");
}
