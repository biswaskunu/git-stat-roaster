# git-stat-roaster

Give it a GitHub username, it pulls public stats and generates a short, funny "roast" from them — a small web app.

## Status: v0.3 in progress (web version)

## Structure

```
git-stat-roaster/
├── backend/          Rust + Axum JSON API, also serves the built frontend
│   └── src/
│       ├── main.rs    server + routing
│       ├── github.rs  GitHub API fetch layer, paginated, typed errors
│       ├── stats.rs    derived stats (age, language, stars, forks, oldest repo)
│       └── jokes.rs    joke pools + roast generator (unchanged from v0.2)
└── frontend/          React (Vite), no extra tooling beyond that
    └── src/
        ├── main.jsx   React entry point
        ├── App.jsx    form, fetch call, results rendering
        └── index.css  styling
```

## What works right now

- `GET /api/roast/:username` — fetches the user's profile + all public repos (paginated), computes derived stats, and returns them alongside a roast line as JSON
- Errors (user not found, GitHub API errors, network failures, malformed responses) come back as proper HTTP status codes + a JSON `{ "error": "..." }` body instead of crashing
- React frontend: type a username, hit "Roast me", see the stats and roast rendered on the page
- Backend serves the frontend's built output directly (`ServeDir` fallback against `frontend/dist`), so a single process serves the whole app

## Running it locally

**Option A — production-style, one process:**

```bash
cd frontend
npm install
npm run build

cd ../backend
cargo run
```

Then open `http://localhost:3000`.

**Option B — frontend dev server (hot reload) + backend separately:**

```bash
# terminal 1
cd backend
cargo run

# terminal 2
cd frontend
npm install
npm run dev
```

Vite's dev server proxies `/api/*` to `http://localhost:3000` (see `frontend/vite.config.js`), so you get hot-reloading UI changes without rebuilding the Rust binary or hitting CORS issues.

By default the backend binds to port 3000; set `PORT` to override (useful for deployment platforms that assign their own port).

## Tech stack

- **Backend**: Rust, Axum, `reqwest` (GitHub API calls), `serde`/`serde_json`, `tokio`, `rand` (joke selection), `tower-http` (static file serving + CORS)
- **Frontend**: React + Vite

No database, no auth, no stored user data — stateless lookup-and-respond, same as the CLI version.

## Known limitations

- **Unauthenticated rate limits**: GitHub API requests are unauthenticated, capped at 60 requests/hour per IP. A rate-limited request now surfaces as a clean `502` with an explanatory message rather than crashing the server.
- **New accounts / zero repos / private-only profiles**: all derived stats are optional and the joke pools have dedicated branches for these cases, so they don't produce broken output.
- **CORS is currently wide open** (`CorsLayer::permissive()`) on the backend — needed for the Vite dev-server workflow above, but should be tightened before any real deployment.
- **`frontend/dist` must exist** before running the backend in "Option A" above — the backend doesn't build the frontend for you.

## Next milestone

- Deploy to Railway (or similar) per the v1.0 milestone in the project spec
- Optional: image card output, if you want shareable static content instead of/alongside the web tool
