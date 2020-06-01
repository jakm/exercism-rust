use std::iter::FromIterator;

pub fn encode(source: &str) -> String {
    if source.is_empty() {
        return String::new();
    }

    let mut chars = source.chars();
    let mut current = chars.next().unwrap();
    let mut count = 1;
    let mut buf = Vec::new();

    for ch in chars {
        if ch == current {
            count += 1;
        } else {
            if count > 1 {
                buf.push(count.to_string());
            }
            buf.push(current.to_string());
            current = ch;
            count = 1;
        }
    }

    if count > 1 {
        buf.push(count.to_string());
    }
    buf.push(current.to_string());

    String::from_iter(buf)
}

pub fn decode(source: &str) -> String {
    if source.is_empty() {
        return String::new();
    }

    let mut output: Vec<String> = Vec::new();
    let mut digits: Vec<char> = Vec::new();

    for ch in source.chars() {
        if ch.is_digit(10) {
            digits.push(ch);
        } else {
            let n = if digits.is_empty() {
                1
            } else {
                String::from_iter(&digits).parse().unwrap()
            };
            digits.clear();
            output.push(std::iter::repeat(ch).take(n).collect())
        }
    }

    String::from_iter(output)
}
