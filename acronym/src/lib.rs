#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

lazy_static! {
    static ref TERM_REGEX: Regex = Regex::new(r"[^[:alnum:]']+").expect("error parsing regex");
}

pub fn abbreviate(phrase: &str) -> String {
    TERM_REGEX
        .split(phrase)
        .filter_map(term_abbreviation)
        .flatten()
        .collect()
}

fn term_abbreviation(s: &str) -> Option<Vec<char>> {
    if s.len() == 0 {
        return None;
    }
    let m: Vec<_> = s.matches(char::is_uppercase).collect();
    let v = if m.len() == 0 || m.len() == s.len() {
        s.chars()
            .nth(0)
            .map(|c| c.to_uppercase())
            .unwrap()
            .collect()
    } else {
        m.iter().flat_map(|s| s.chars()).collect()
    };
    Some(v)
}
