pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = vec![];
    for ch in string.chars() {
        match ch {
            '[' | '{' | '(' => stack.push(ch),
            ']' | '}' | ')' => {
                if !check_opening(&mut stack, map_closing_opening(ch)) {
                    return false;
                };
            }
            _ => (),
        }
    }
    stack.len() == 0
}

fn map_closing_opening(ch: char) -> char {
    return match ch {
        ']' => '[',
        '}' => '{',
        ')' => '(',
        _ => panic!("Invalid value"),
    };
}

fn check_opening(stack: &mut Vec<char>, expected: char) -> bool {
    if let Some(ch) = stack.pop() {
        return ch == expected;
    }
    false
}
