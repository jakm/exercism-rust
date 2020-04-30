use std::cell::RefCell;
use std::collections::HashSet;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let multiples = RefCell::new(HashSet::with_capacity(upper_bound as usize));
    (2..=upper_bound)
        .filter(|n| !multiples.borrow().contains(n))
        .map(|n| {
            multiples
                .borrow_mut()
                .extend((n * n..=upper_bound).step_by(n as usize));
            n
        })
        .collect()
}
