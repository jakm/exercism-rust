use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let trim: Vec<char> = candidate
        .to_lowercase()
        .matches(char::is_alphabetic)
        .flat_map(|s| s.chars())
        .collect();
    let chars = trim.iter().collect::<HashSet<&char>>();
    trim.len() == chars.len()
}
