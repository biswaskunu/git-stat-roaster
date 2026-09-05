use crate::github::Repo;
use std::collections::HashMap;

pub fn current_year() -> i32 {
    use std::time::{SystemTime, UNIX_EPOCH};

    let secs_since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock is before unix epoch")
        .as_secs();

    // average seconds in a year, accounts for leap years well enough for our purposes
    const SECS_PER_YEAR: u64 = 31_557_600;

    1970 + (secs_since_epoch / SECS_PER_YEAR) as i32
}

pub fn account_age_years(created_at: &str) -> i32 {
    let created_year: i32 = created_at[0..4]
        .parse()
        .expect("created_at did not start with a 4-digit year");
    current_year() - created_year
}

pub fn most_used_language(repos: &[Repo]) -> Option<String> {
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

pub fn most_starred_repo(repos: &[Repo]) -> Option<&Repo> {
    repos
        .iter()
        .filter(|r| !r.fork)
        .max_by_key(|r| r.stargazers_count)
}

pub fn most_forked_repo(repos: &[Repo]) -> Option<&Repo> {
    repos
        .iter()
        .filter(|r| !r.fork)
        .max_by_key(|r| r.forks_count)
}

pub fn oldest_repo(repos: &[Repo]) -> Option<&Repo> {
    // created_at is an ISO 8601 string (e.g. "2019-05-02T12:00:00Z"),
    // so plain string comparison sorts chronologically same as a real date type would.
    repos
        .iter()
        .filter(|r| !r.fork)
        .min_by(|a, b| a.created_at.cmp(&b.created_at))
}
