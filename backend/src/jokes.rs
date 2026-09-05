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

/// Account age vs. repo count, with different framing depending on scale.
pub fn account_age_jokes(stats: &Stats) -> Vec<String> {
    let age = stats.account_age_years;
    let repos = stats.non_fork_repo_count;

    // Brand new account — different joke territory than a stale veteran one.
    if age <= 0 {
        return vec![
            "Account so new the paint hasn't dried, and there's already this much to judge.".to_string(),
            "Joined GitHub this year. Bold of you to already have opinions about tabs vs spaces.".to_string(),
        ];
    }

    let mut jokes = vec![
        format!(
            "{} years on GitHub and only {} repos to show for it.",
            age, repos
        ),
        format!(
            "That's {} repos in {} years — roughly one every {:.1} years. Glacial.",
            repos,
            age,
            if repos > 0 { age as f64 / repos as f64 } else { age as f64 }
        ),
    ];

    if repos == 0 {
        jokes.push(format!(
            "{} years and zero repos. Just here for the vibes, apparently.",
            age
        ));
    } else if (repos as i32) > age * 20 {
        jokes.push(format!(
            "{} repos in {} years — either wildly productive or wildly indecisive about naming things.",
            repos, age
        ));
    } else if age >= 10 {
        jokes.push(format!(
            "{} years old and still shipping like it's a side project. Which, let's be honest, it is.",
            age
        ));
    }

    jokes
}

/// Most-used language jokes, including a small set of per-language burns.
pub fn language_jokes(stats: &Stats) -> Vec<String> {
    match stats.most_used_language {
        Some(lang) => {
            let mut jokes = vec![
                format!("Mostly {}? Bold choice.", lang),
                format!("{} is the main language here. Says a lot, and none of it flattering.", lang),
            ];

            match lang.to_lowercase().as_str() {
                "javascript" | "typescript" => jokes.push(
                    "npm install is basically your cardio at this point.".to_string(),
                ),
                "python" => jokes.push(
                    "Indentation-based syntax for someone who clearly struggles with structure.".to_string(),
                ),
                "rust" => jokes.push(
                    "Fighting the borrow checker so you don't have to fight anyone in your personal life.".to_string(),
                ),
                "java" => jokes.push(
                    "AbstractFactoryFactoryBuilder energy detected.".to_string(),
                ),
                "html" | "css" => jokes.push(
                    "Calling that a 'language' is doing a lot of heavy lifting.".to_string(),
                ),
                "go" => jokes.push(
                    "if err != nil energy, in code and in life.".to_string(),
                ),
                _ => {}
            }

            jokes
        }
        None => vec![
            "Not even a dominant language. Just vibes, apparently.".to_string(),
            "No detectable language pattern. A true polyglot, or just noncommittal.".to_string(),
        ],
    }
}

/// Star count extremes — including the "zero stars, ever" case.
pub fn star_jokes(stats: &Stats) -> Vec<String> {
    match stats.top_starred_repo {
        Some((name, 0)) => vec![
            format!("Even '{}' couldn't get a single star.", name),
            format!("'{}' sits at zero stars. Even your mom didn't star it.", name),
        ],
        Some((name, stars)) if stars < 5 => vec![
            format!("'{}' peaked at {} stars. Peaked.", name, stars),
            format!("{} stars on '{}' — technically nonzero, generously called a success.", stars, name),
        ],
        Some((name, stars)) => vec![
            format!("'{}' has {} stars. Alright, that one's actually kind of impressive.", name, stars),
        ],
        None => vec![
            "No repos, no stars, no problem I guess.".to_string(),
        ],
    }
}

/// Most-forked repo jokes — including the "nobody forked anything" case.
pub fn fork_jokes(stats: &Stats) -> Vec<String> {
    match stats.top_forked_repo {
        Some((name, 0)) => vec![
            format!("Not even one fork of '{}'. Nobody's copying this homework.", name),
            format!("Zero forks on '{}'. Original work, unfortunately for its popularity.", name),
        ],
        Some((name, forks)) => vec![
            format!(
                "'{}' got forked {} times. Guess someone found it useful, unlike you finishing it.",
                name, forks
            ),
        ],
        None => vec!["No repos to fork, no forks to brag about.".to_string()],
    }
}

/// Bio presence/length/absence jokes.
pub fn bio_jokes(stats: &Stats) -> Vec<String> {
    match stats.bio {
        None => vec![
            "No bio. Mysterious, or just didn't bother.".to_string(),
            "Bio field left empty. Nothing to say for yourself, huh.".to_string(),
        ],
        Some(bio) if bio.trim().is_empty() => vec![
            "Bio exists but says nothing. Impressive, actually.".to_string(),
        ],
        Some(bio) if bio.trim().len() < 15 => vec![
            format!("Bio: \"{}\". Really put the effort in there.", bio.trim()),
        ],
        Some(_) => vec![
            "At least you filled in the bio field. Effort noted, barely.".to_string(),
            "A whole bio. Someone's proud of themselves.".to_string(),
        ],
    }
}

/// Follower/following ratio jokes.
pub fn follow_ratio_jokes(stats: &Stats) -> Vec<String> {
    let (f, g) = (stats.followers, stats.following);

    if f == 0 && g == 0 {
        return vec![
            "Zero followers, following zero people. A true lone wolf, or just new here.".to_string(),
        ];
    }

    if g > f * 3 && g > 5 {
        vec![
            format!(
                "Following {} people but only {} follow back. That's not networking, that's begging.",
                g, f
            ),
        ]
    } else if f > g * 3 && f > 5 {
        vec![
            format!(
                "{} followers to {} following — practically GitHub royalty, or just followed by bots.",
                f, g
            ),
        ]
    } else {
        vec![
            format!("{} followers, {} following. Balanced enough.", f, g),
        ]
    }
}

/// Pulls candidate jokes from every category and picks one line at random.
/// Currently a single punchline rather than a multi-line roast — revisit
/// if you want a paragraph-style roast instead (v0.2 idea).
pub fn generate_roast(stats: &Stats) -> String {
    let mut pool: Vec<String> = Vec::new();
    pool.extend(account_age_jokes(stats));
    pool.extend(language_jokes(stats));
    pool.extend(star_jokes(stats));
    pool.extend(fork_jokes(stats));
    pool.extend(bio_jokes(stats));
    pool.extend(follow_ratio_jokes(stats));

    let mut rng = rand::rng();
    pool.choose(&mut rng).cloned().unwrap_or_else(|| {
        format!(
            "{} has no discernible personality traits to roast. Truly a blank slate.",
            stats.username
        )
    })
}
