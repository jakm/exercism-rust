/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.len() != code.matches(|c| char::is_digit(c, 10) || c == ' ').count() {
        return false;
    }

    let digits = code
        .matches(|c| char::is_digit(c, 10))
        .flat_map(|s| s.chars())
        .collect::<Vec<char>>();

    if digits.len() <= 1 {
        return false;
    }

    let sum = digits.iter().rev().enumerate().fold(0, |acc, (i, c)| {
        let mut d = c.to_digit(10).unwrap();
        if i % 2 != 0 {
            d *= 2;
            if d > 9 {
                d -= 9;
            }
        }
        acc + d
    });

    sum % 10 == 0
}
