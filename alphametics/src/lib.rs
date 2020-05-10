use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    if input
        .matches(|c| char::is_alphabetic(c) || c == ' ' || c == '+' || c == '=')
        .count()
        != input.len()
    {
        panic!("Unexpected character");
    }

    let s = input.replace(" ", "");

    let mut sides = s.split("==");

    let terms = sides.next()?.split("+").collect::<Vec<&str>>();
    let goal = sides.next()?;

    let letters = input
        .matches(char::is_alphabetic)
        .flat_map(|s| s.chars())
        .collect::<HashSet<char>>()
        .into_iter()
        .collect::<Vec<char>>();

    let mut leading = terms
        .iter()
        .map(|term| term.chars().next().unwrap())
        .collect::<Vec<char>>();
    leading.push(goal.chars().next().unwrap());
    let leading = leading.iter().copied().collect::<HashSet<char>>();

    attempt(HashMap::new(), &letters, &terms, &goal, &leading)
}

fn attempt(
    map: HashMap<char, u8>,
    letters: &[char],
    terms: &[&str],
    goal: &str,
    leading: &HashSet<char>,
) -> Option<HashMap<char, u8>> {
    if letters.len() == 0 {
        return if evaluate(&map, &terms, &goal) {
            Some(map)
        } else {
            None
        };
    }

    for digit in 0..=9 {
        let letter = letters[0];

        if digit == 0 && leading.contains(&letter) {
            continue;
        }
        if map.values().any(|c| *c == digit) {
            continue;
        }
        let mut map2 = map.clone();
        map2.insert(letter, digit);

        if let Some(res) = attempt(map2, &letters[1..], terms, goal, leading) {
            return Some(res);
        }
    }

    None
}

fn evaluate(map: &HashMap<char, u8>, terms: &[&str], goal: &str) -> bool {
    let left: u64 = terms
        .iter()
        .map(|term| {
            term.chars().fold(0, |acc, c| {
                let num = *map.get(&c).unwrap();
                acc * 10 + num as u64
            })
        })
        .sum();
    let right: u64 = goal.chars().fold(0, |acc, c| {
        let num = *map.get(&c).unwrap();
        acc * 10 + num as u64
    });

    left == right
}
