# git-stat-roaster

Give it a GitHub username, it pulls public stats and generates a short, funny "roast" from them.

## Status: v0.2 in progress

## What works right now

- Fetches a user's public GitHub profile (`GET /users/{username}`)
- Fetches their public repos (`GET /users/{username}/repos`), paginated across all pages (not capped at the default 30)
- Handles the "user doesn't exist" (404) case, other non-success API statuses, network failures, and unexpected/malformed JSON responses without crashing
- Computes derived stats:
  - Account age in years (current year pulled from `std::time`, not hardcoded)
  - Most-used language across non-fork repos (simple repo-count tally, no extra API calls)
  - Repo counts: raw `public_repos` (incl. forks) alongside a non-fork-only count
  - Most-starred repo and most-forked repo (forks excluded from both)
  - Oldest repo (forks excluded)
- Maps those stats to a roast: per-category joke pools (account age, language, stars, forks, bio, follower/following ratio) with 2-4 template variants each, one picked at random via `rand`
- Prints the fetched profile fields, computed stats, and a roast line to the console

## What's not built yet

- Further expansion of joke variety per category (ongoing, self-directed)
- Image card output (v0.3+, optional)
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

- **Unauthenticated rate limits**: GitHub API requests here are unauthenticated, capped at 60 requests/hour per IP. Fine for local/personal use; running it repeatedly in a short window (or letting others hit it) will trip the limit. A PAT would raise this to 5000/hr but isn't wired up in this version — deliberately, to keep this a no-secrets, clone-and-run tool. If you hit the limit, requests will now print a clean error message (rather than panicking) telling you to wait an hour.
- **New accounts / zero repos / private-only profiles**: all derived stats are `Option`-based and the joke pools have dedicated branches for these cases (e.g. brand-new accounts, zero stars, zero forks, empty bio), so these don't produce broken or nonsensical output.

## Next milestone

Move on to v0.3: either a web-facing endpoint (Axum) or an image card generator, per the project spec — pick based on whether you want a web tool or static content to post.
