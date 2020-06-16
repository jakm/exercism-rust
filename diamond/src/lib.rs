const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn get_diamond(c: char) -> Vec<String> {
    let count = ALPHABET.find(c).expect("invalid character") + 1;

    (0..count * 2 - 1)
        .map(|mut i| {
            if i >= count {
                i = count * 2 - i - 2;
            }
            let c = ALPHABET.chars().nth(i).unwrap();
            if i == 0 {
                format!("{0:^1$}", c, count * 2 - 1)
            } else {
                format!("{0:>1$}{2:^3$}{0:<1$}", c, count - i, " ", i * 2 - 1)
            }
        })
        .collect()
}
