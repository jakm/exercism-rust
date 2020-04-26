pub fn reply(message: &str) -> &str {
    let s = message.trim();
    if s.is_empty() {
        return "Fine. Be that way!";
    }
    let yelling = s.matches(char::is_alphabetic).count() > 0 && s.to_uppercase() == s;
    let questioning = s.chars().last() == Some('?');
    match (questioning, yelling) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, false) => "Sure.",
        (false, true) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
