#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

lazy_static! {
    static ref RULE1_REGEX: Regex = Regex::new(r"^([aeiou]|xr|yt)").unwrap();
    static ref RULE2_REGEX: Regex =
        Regex::new(r"^((b|c|d|f|g|h|j|k|l|m|n|p|q|r|s|t|v|w|x|y|z)+)(.+)").unwrap();
    static ref RULE3_REGEX: Regex =
        Regex::new(r"^((b|c|d|f|g|h|j|k|l|m|n|p|q|r|s|t|v|w|x|y|z)?qu)(.+)").unwrap();
    static ref RULE4_REGEX: Regex =
        Regex::new(r"^(((b|c|d|f|g|h|j|k|l|m|n|p|q|r|s|t|v|w|x|y|z)+)y)(.+)").unwrap();
}

pub fn translate(input: &str) -> String {
    let mut output: Vec<String> = Vec::new();

    for word in input.split_whitespace() {
        if RULE1_REGEX.is_match(word) {
            let s = word.to_string() + "ay";
            output.push(s);
            continue;
        }
        if let Some(c) = RULE4_REGEX.captures(word) {
            let mut s = "y".to_string();
            s.push_str(&c[4]);
            s.push_str(&c[2]);
            s.push_str("ay");
            output.push(s);
            continue;
        }
        if let Some(c) = RULE3_REGEX.captures(word) {
            let mut s = c[3].to_string();
            s.push_str(&c[1]);
            s.push_str("ay");
            output.push(s);
            continue;
        }
        if let Some(c) = RULE2_REGEX.captures(word) {
            let mut s = c[3].to_string();
            s.push_str(&c[1]);
            s.push_str("ay");
            output.push(s);
            continue;
        }
        output.push(word.to_string());
    }

    output.join(" ")
}
