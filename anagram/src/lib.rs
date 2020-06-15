use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut res = HashSet::new();
    let word_lower = word.to_lowercase();
    let wlc = letter_count(&word_lower);

    for candidate in possible_anagrams {
        let candidate_lower = candidate.to_lowercase();
        if &candidate_lower != &word_lower {
            let clc = letter_count(&candidate_lower);
            if clc == wlc {
                res.insert(*candidate);
            }
        }
    }

    res
}

fn letter_count(word: &String) -> HashMap<char, u16> {
    word.chars()
        .flat_map(|c| c.to_lowercase())
        .fold(HashMap::new(), |mut map, c| {
            let cnt = map.entry(c).or_default();
            *cnt += 1;
            map
        })
}
