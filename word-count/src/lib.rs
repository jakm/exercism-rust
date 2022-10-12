use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut acc = HashMap::new();

    words
        .split(|c| {
            !(c == '\''
                || (c >= '0' && c <= '9')
                || (c >= 'A' && c <= 'Z')
                || (c >= 'a' && c <= 'z'))
        })
        .filter(|s| s.len() > 0)
        .map(|s| s.trim_matches('\''))
        .for_each(|term| {
            let counter = acc.entry(term.to_lowercase()).or_insert(0);
            *counter += 1;
        });

    acc
}
