use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Palindrome {
    a: u64,
    b: u64,
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        Self { a, b }
    }

    pub fn value(&self) -> u64 {
        self.a * self.b
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        self.a = a;
        self.b = b;
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut palindromes: BTreeMap<u64, Palindrome> = BTreeMap::new();

    for i in min..=max {
        for j in i..=max {
            let product = i * j;
            if is_palindrome(digits(product)) {
                palindromes
                    .entry(product)
                    .and_modify(|e| e.insert(i, j))
                    .or_insert(Palindrome::new(i, j));
            }
        }
    }

    let min_key = *palindromes.keys().min()?;
    let max_key = *palindromes.keys().max()?;

    Some((palindromes.remove(&min_key)?, palindromes.remove(&max_key)?))
}

fn is_palindrome(digits: Vec<u8>) -> bool {
    for i in 0..digits.len() / 2 {
        if digits[i] != digits[digits.len() - i - 1] {
            return false;
        }
    }
    true
}

fn digits(num: u64) -> Vec<u8> {
    let mut digits = Vec::new();

    // for num == 0
    digits.push((num % 10) as u8);
    let mut n = num / 10;

    while n > 0 {
        digits.push((n % 10) as u8);
        n = n / 10;
    }

    digits
}
