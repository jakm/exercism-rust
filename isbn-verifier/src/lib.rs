/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    if isbn.len()
        != isbn
            .matches(|c| char::is_digit(c, 10) || c == 'X' || c == '-')
            .count()
    {
        return false;
    }

    let s: String = isbn
        .chars()
        .enumerate()
        .filter(|(i, c)| c.is_digit(10) || (*c == 'X' && *i == isbn.len() - 1))
        .map(|(_, c)| c)
        .collect();

    if s.len() != 10 {
        return false;
    }

    let n = s.chars().enumerate().fold(0, |acc, (i, c)| {
        let d = if c == 'X' {
            10
        } else {
            c.to_digit(10).unwrap()
        };
        acc + d * (10 - i as u32)
    });

    n % 11 == 0
}
