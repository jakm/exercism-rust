#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    match num {
        0 => return None,
        1 => return Some(Classification::Deficient),
        _ => (),
    };

    match aliquot_sum(num) {
        n if n == num => Some(Classification::Perfect),
        n if n > num => Some(Classification::Abundant),
        n if n < num => Some(Classification::Deficient),
        _ => unreachable!(),
    }
}

fn aliquot_sum(num: u64) -> u64 {
    factors(num).iter().sum()
}

fn factors(num: u64) -> Vec<u64> {
    if num <= 1 {
        return vec![1];
    }

    (1..num - 1).filter(|x| num % x == 0).collect()
}
