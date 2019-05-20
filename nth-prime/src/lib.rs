pub fn nth(n: u32) -> u32 {
    let n = n+1;
    let mut count = 0;
    for candidate in 2.. {
        if is_prime(candidate) {
            count += 1;
        }
        if count == n {
            return candidate;
        }
    }

    return 0; // make compiler happy
}

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if (n == 2) || (n == 3) {
        return true;
    }
    if (n % 2 == 0) || (n % 3 == 0) {
        return false;
    }

    let mut i = 5;
    let mut w = 2;
    while i < n {
        if n % i == 0 {
            return false;
        }
        i += w;
        w = 6 - w; // either 4 or 2
    }


    return true;
}
