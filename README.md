# git-stat-roaster

Give it a GitHub username, it pulls public stats and (eventually) generates a short, funny "roast" from them.

## Status: v0.1 in progress — fetch & stats done, joke engine not yet built

## What works right now

- Fetches a user's public GitHub profile (`GET /users/{username}`)
- Fetches their public repos (`GET /users/{username}/repos`)
- Handles the "user doesn't exist" (404) case and other non-success API statuses without crashing
- Computes derived stats:
  - Account age in years (current year pulled from `std::time`, not hardcoded)
  - Most-used language across non-fork repos (simple repo-count tally, no extra API calls)
  - Repo counts: raw `public_repos` (incl. forks) alongside a non-fork-only count
  - Most-starred repo and most-forked repo (forks excluded from both)
- Prints all of the above to the console

## What's not built yet

- Stat → joke mapping (the actual roast text)
- Randomized template variants per joke category
- Image card output (v0.2+, optional)
- Any web-facing endpoint (v0.3+, optional)

## Tech stack

- Rust
- `reqwest` (with `json` feature) — GitHub API calls
- `serde` / `serde_json` — response deserialization
- `tokio` (`rt-multi-thread`, `macros`) — async runtime

No database, no auth, no stored user data — stateless lookup-and-print for now.

## Running it

```bash
cargo run -- <github-username>
```

Example:

```bash
cargo run -- biswaskunu
```

Prints the fetched profile fields and computed stats for that user.

Note: unauthenticated GitHub API requests are rate-limited to 60/hr. Fine for local dev; a PAT would be needed for anything public-facing later.

## Next milestone

Build the stat → joke rule table: map thresholds on the computed stats (account age vs. repo count, most-used language, star/fork extremes, bio presence, follower/following ratio) to 3–4 template joke lines each, picked at random so output doesn't feel robotic on repeat runs.