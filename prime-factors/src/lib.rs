pub fn factors(n: u64) -> Vec<u64> {
    let mut n = n;
    let mut primes = vec![];
    for candidate in 2..=n {
        if n <= 1 {
            break;
        }
        while n % candidate == 0 {
            primes.push(candidate);
            n /= candidate;
        }
    }
    primes
}
