use rand::prelude::IndexedRandom;

pub struct Stats<'a> {
    pub username: &'a str,
    pub account_age_years: i32,
    pub non_fork_repo_count: u32,
    pub most_used_language: Option<&'a str>,
    pub top_starred_repo: Option<(&'a str, u32)>, // (name, stars)
    pub top_forked_repo: Option<(&'a str, u32)>,  // (name, forks)
    pub bio: Option<&'a str>,
    pub followers: u32,
    pub following: u32,
}

/// Account age vs. repo count. e.g. "N years on GitHub and only M repos?"
pub fn account_age_jokes(stats: &Stats) -> Vec<String> {
    let age = stats.account_age_years;
    let repos = stats.non_fork_repo_count;

    vec![
        format!(
            "{} years on GitHub and only {} repos to show for it.",
            age, repos
        ),
        // TODO: add 2-3 more variants, maybe branch on age/repo thresholds
    ]
}

/// Most-used language jokes (JS/npm cardio, Python indentation, etc).
pub fn language_jokes(stats: &Stats) -> Vec<String> {
    match stats.most_used_language {
        Some(lang) => vec![
            format!("Mostly {}? Bold choice.", lang),
            // TODO: per-language template variants
        ],
        None => vec![
            "Not even a dominant language. Just vibes, apparently.".to_string(),
        ],
    }
}

/// Star count extremes — including the "zero stars, ever" case.
pub fn star_jokes(stats: &Stats) -> Vec<String> {
    match stats.top_starred_repo {
        Some((name, stars)) if stars == 0 => vec![
            format!("Even '{}' couldn't get a single star.", name),
        ],
        Some((name, stars)) => vec![
            format!("'{}' peaked at {} stars. Peaked.", name, stars),
        ],
        None => vec!["No repos, no stars, no problem I guess.".to_string()],
    }
}

/// Most-forked repo jokes — including the "nobody forked anything" case.
pub fn fork_jokes(stats: &Stats) -> Vec<String> {
    match stats.top_forked_repo {
        Some((name, forks)) if forks == 0 => vec![
            format!("Not even one fork of '{}'. Nobody's copying this homework.", name),
        ],
        Some((name, forks)) => vec![
            format!("'{}' got forked {} times. Guess someone found it useful, unlike you finishing it.", name, forks),
        ],
        None => vec!["No repos to fork, no forks to brag about.".to_string()],
    }
}

/// Bio presence/length/absence jokes.
pub fn bio_jokes(stats: &Stats) -> Vec<String> {
    match stats.bio {
        None => vec!["No bio. Mysterious, or just didn't bother.".to_string()],
        Some(bio) if bio.trim().is_empty() => {
            vec!["Bio exists but says nothing. Impressive, actually.".to_string()]
        }
        Some(_) => vec![
            // TODO: maybe roast bio length/content itself here
            "At least you filled in the bio field.".to_string(),
        ],
    }
}

/// Follower/following ratio jokes.
pub fn follow_ratio_jokes(stats: &Stats) -> Vec<String> {
    let (f, g) = (stats.followers, stats.following);
    if g > f {
        vec![format!(
            "Following {} people but only {} follow back. Rough.",
            g, f
        )]
    } else {
        vec![format!("{} followers, {} following. Balanced enough.", f, g)]
    }
}

/// Pulls one joke from each category pool and picks one line overall at random.
/// TODO: decide if you want one joke per category (multi-line roast) or
/// just one joke total (single punchline) — currently does the latter.
pub fn generate_roast(stats: &Stats) -> String {
    let mut pool: Vec<String> = Vec::new();
    pool.extend(account_age_jokes(stats));
    pool.extend(language_jokes(stats));
    pool.extend(star_jokes(stats));
    pool.extend(fork_jokes(stats));
    pool.extend(bio_jokes(stats));
    pool.extend(follow_ratio_jokes(stats));

    let mut rng = rand::rng();
    pool.choose(&mut rng)
        .cloned()
        .unwrap_or_else(|| format!("{} has no discernible personality traits to roast.", stats.username))
}
