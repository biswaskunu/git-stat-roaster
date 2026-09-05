# git-stat-roaster

Give it a GitHub username, it pulls public stats and generates a short, funny "roast" from them.

## Status: v0.1 complete

## What works right now

- Fetches a user's public GitHub profile (`GET /users/{username}`)
- Fetches their public repos (`GET /users/{username}/repos`)
- Handles the "user doesn't exist" (404) case and other non-success API statuses without crashing
- Computes derived stats:
  - Account age in years (current year pulled from `std::time`, not hardcoded)
  - Most-used language across non-fork repos (simple repo-count tally, no extra API calls)
  - Repo counts: raw `public_repos` (incl. forks) alongside a non-fork-only count
  - Most-starred repo and most-forked repo (forks excluded from both)
- Maps those stats to a roast: per-category joke pools (account age, language, stars, forks, bio, follower/following ratio) with 2-4 template variants each, one picked at random via `rand`
- Prints the fetched profile fields, computed stats, and a roast line to the console

## What's not built yet

- Further expansion of joke variety per category (ongoing, self-directed)
- Pagination for the repos endpoint (currently only the first 30 repos are considered)
- Image card output (v0.2+, optional)
- Any web-facing endpoint (v0.3+, optional)

## Tech stack

- Rust
- `reqwest` (with `json` feature) — GitHub API calls
- `serde` / `serde_json` — response deserialization
- `tokio` (`rt-multi-thread`, `macros`) — async runtime
- `rand` — random joke selection

No database, no auth, no stored user data — stateless lookup-and-print for now.

## Running it

```bash
cargo run -- <github-username>
```

Example:

```bash
cargo run -- biswaskunu
```

Prints the fetched profile fields, computed stats, and a roast line for that user.

## Known limitations

- **Unauthenticated rate limits**: GitHub API requests here are unauthenticated, capped at 60 requests/hour per IP. Fine for local/personal use; running it repeatedly in a short window (or letting others hit it) will trip the limit. A PAT would raise this to 5000/hr but isn't wired up in this version — deliberately, to keep this a no-secrets, clone-and-run tool.
- **Panics on unexpected API responses**: error handling covers the two expected cases (404 user-not-found, non-success status codes), but malformed/unexpected JSON shapes (e.g. GitHub changing a field, or a 403 from rate-limiting) will currently cause a panic with a Rust backtrace rather than a clean error message. If you hit this, it's most likely the rate limit above — wait an hour and retry.
- **Only the first page of repos** is fetched (GitHub's default page size), so stats for users with 30+ public repos are computed on a subset.

## Next milestone

Continue expanding joke template variety, then move on to pagination handling and/or v0.2 stat additions (oldest repo, etc.) per the project spec.
