#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

pub struct WordProblem<'a> {
    tokens: Vec<&'a str>,
}

impl<'a> WordProblem<'a> {
    pub fn new(expression: &'a str) -> Self {
        lazy_static! {
            static ref TOKENS_RE: Regex =
                Regex::new(r"(-?[\d]+|plus|minus|multiplied by|divided by|\w+)").unwrap();
        }
        let tokens = TOKENS_RE
            .find_iter(expression)
            .map(|m| m.as_str())
            .collect();

        WordProblem { tokens }
    }

    pub fn evaluate(&self) -> Option<i32> {
        if self.tokens.len() == 0 {
            return None;
        }

        if self.tokens.len() % 2 == 0 {
            return None;
        }

        let mut op: Option<Box<dyn Fn(i32, i32) -> i32>> = None;
        let mut acc = 0;

        for (i, token) in self.tokens.iter().enumerate() {
            // numbers must be at even index, operators at odd index
            match *token {
                "plus" if i % 2 == 1 => op = Some(Box::new(|a, b| a + b)),
                "minus" if i % 2 == 1 => op = Some(Box::new(|a, b| a - b)),
                "multiplied by" if i % 2 == 1 => op = Some(Box::new(|a, b| a * b)),
                "divided by" if i % 2 == 1 => op = Some(Box::new(|a, b| a / b)),
                _ if i % 2 == 0 => {
                    if op.is_none() {
                        acc = token.parse::<i32>().ok()?;
                    } else {
                        let tmp = token.parse::<i32>().ok()?;
                        match op {
                            Some(ref op) => acc = op(acc, tmp),
                            _ => return None,
                        }
                    }
                }
                _ => return None,
            }
        }

        Some(acc)
    }
}

pub fn answer(command: &str) -> Option<i32> {
    if command.len() <= 8 || !command.starts_with("What is") || !command.ends_with("?") {
        return None;
    }

    WordProblem::new(&command[7..command.len() - 1]).evaluate()
}
