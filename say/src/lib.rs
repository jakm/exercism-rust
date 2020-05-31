extern crate itertools;
use itertools::Itertools;

use std::iter::FromIterator;

pub fn encode(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    let s = n.to_string();
    s.chars()
        // map to triplets in reverse order
        .rev()
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let mut v = chunk.collect::<Vec<_>>();
            v.reverse();
            String::from_iter(v)
        })
        .collect::<Vec<String>>()
        // iterate over triplets and generate a word and magnitude
        .iter()
        .enumerate()
        .filter_map(|(i, digits)| {
            let mut s = number_to_word(digits.as_str());
            if !s.is_empty() {
                s += match i {
                    1 => " thousand",
                    2 => " million",
                    3 => " billion",
                    4 => " trillion",
                    5 => " quadrillion",
                    6 => " quintillion",
                    _ => "",
                };
                Some(s)
            } else {
                None
            }
        })
        // reverse back and add spaces
        .rev()
        .collect::<Vec<String>>()
        .join(" ")
}

fn number_to_word(digits: &str) -> String {
    assert!(digits.len() > 0 && digits.len() <= 3);

    let mut buf: Vec<String> = vec![];

    match digits.chars().nth_back(2) {
        Some(x) if x != '0' => {
            buf.push(digit_to_word(x));
            buf.push(" hundred ".to_string());
        }
        _ => (),
    }

    match (digits.chars().nth_back(1), digits.chars().nth_back(0)) {
        (Some(x), Some(y)) if x == '1' => buf.push(
            match (x, y) {
                ('1', '0') => "ten",
                ('1', '1') => "eleven",
                ('1', '2') => "twelve",
                ('1', '3') => "thirteen",
                ('1', '4') => "fourteen",
                ('1', '5') => "fifteen",
                ('1', '6') => "sixteen",
                ('1', '7') => "seventeen",
                ('1', '8') => "eighteen",
                ('1', '9') => "nineteen",
                _ => "",
            }
            .to_string(),
        ),
        (Some(x), Some(y)) if x != '0' => {
            buf.push(
                match x {
                    '2' => "twenty",
                    '3' => "thirty",
                    '4' => "forty",
                    '5' => "fifty",
                    '6' => "sixty",
                    '7' => "seventy",
                    '8' => "eighty",
                    '9' => "ninety",
                    _ => "",
                }
                .to_string(),
            );
            if y != '0' {
                buf.push("-".to_string());
                buf.push(digit_to_word(y));
            }
        }
        (_, Some(x)) => buf.push(digit_to_word(x)),
        _ => (),
    }

    buf.join("").trim().to_string()
}

fn digit_to_word(n: char) -> String {
    match n {
        '1' => "one",
        '2' => "two",
        '3' => "three",
        '4' => "four",
        '5' => "five",
        '6' => "six",
        '7' => "seven",
        '8' => "eight",
        '9' => "nine",
        _ => "",
    }
    .to_string()
}
