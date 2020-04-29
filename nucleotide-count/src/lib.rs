use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    for c in dna.chars() {
        if !valid_nucleotide(c) {
            return Err(c);
        }
    }
    if valid_nucleotide(nucleotide) {
        Ok(dna.matches(nucleotide).count())
    } else {
        Err(nucleotide)
    }
}

fn valid_nucleotide(c: char) -> bool {
    match c {
        'A' | 'C' | 'G' | 'T' => true,
        _ => false,
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut map: HashMap<char, usize> = HashMap::new();
    for nucleotide in &['A', 'C', 'G', 'T'] {
        let count = count(*nucleotide, dna)?;
        map.insert(*nucleotide, count);
    }
    Ok(map)
}
