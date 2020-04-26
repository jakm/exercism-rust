pub fn reply(message: &str) -> &str {
    let s = message.trim();
    if s.is_empty() {
        return "Fine. Be that way!";
    }
    let alpha: Vec<char> = s.chars().filter(|c| c.is_alphabetic()).collect();
    let yelling = !alpha.is_empty() && alpha.iter().all(|c| c.is_uppercase());
    let questioning = s.chars().last() == Some('?');
    match (questioning, yelling) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, false) => "Sure.",
        (false, true) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
