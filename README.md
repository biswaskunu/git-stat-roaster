# git-stat-roaster

Give it a GitHub username, it pulls public stats and generates a short, funny "roast" from them — now a small web app instead of just a CLI.

## Status: v0.3 in progress (web version)

## Structure

```
git-stat-roaster/
├── backend/      Rust + Axum JSON API, also serves the frontend
│   └── src/
│       ├── main.rs    server + routing
│       ├── github.rs  GitHub API fetch layer, paginated, typed errors
│       ├── stats.rs    derived stats (age, language, stars, forks, oldest repo)
│       └── jokes.rs    joke pools + roast generator (unchanged from v0.2)
└── frontend/      plain HTML/CSS/JS, no build step
```

## What works right now

- `GET /api/roast/:username` — fetches the user's profile + all public repos (paginated), computes derived stats, and returns them alongside a roast line as JSON
- Errors (user not found, GitHub API errors, network failures, malformed responses) come back as proper HTTP status codes + a JSON `{ "error": "..." }` body instead of crashing
- A minimal frontend: type a username, hit "Roast me", see the stats and roast rendered on the page
- Backend serves the frontend directly (`ServeDir` fallback), so running one process gives you the whole app at `http://localhost:3000`

## Running it locally

```bash
cd backend
cargo run
```

Then open `http://localhost:3000` in a browser.

By default it binds to port 3000; set `PORT` to override (useful for deployment platforms that assign their own port).

## Tech stack

- **Backend**: Rust, Axum, `reqwest` (GitHub API calls), `serde`/`serde_json`, `tokio`, `rand` (joke selection), `tower-http` (static file serving + CORS)
- **Frontend**: plain HTML/CSS/JS — no framework, no build step, kept intentionally simple

No database, no auth, no stored user data — stateless lookup-and-respond, same as the CLI version.

## Known limitations

- **Unauthenticated rate limits**: GitHub API requests are unauthenticated, capped at 60 requests/hour per IP. A rate-limited request now surfaces as a clean `502` with an explanatory message rather than crashing the server.
- **New accounts / zero repos / private-only profiles**: all derived stats are optional and the joke pools have dedicated branches for these cases, so they don't produce broken output.
- **CORS is currently wide open** (`CorsLayer::permissive()`) to make local frontend-only dev servers easy to run against the API. Tighten this before any real deployment.

## Next milestone

- Deploy to Railway (or similar) per the v1.0 milestone in the project spec
- Optional: image card output, if you want shareable static content instead of/alongside the web tool
