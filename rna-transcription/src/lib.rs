#[derive(Debug, PartialEq)]
pub struct DNA(String);

#[derive(Debug, PartialEq)]
pub struct RNA(String);

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        for (i, c) in dna.chars().enumerate() {
            match c {
                'A' | 'C' | 'G' | 'T' => (),
                _ => return Err(i),
            }
        }
        Ok(DNA(dna.to_string()))
    }

    pub fn into_rna(self) -> RNA {
        let rna = self
            .0
            .chars()
            .map(|c| match c {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => unreachable!(),
            })
            .collect();
        RNA(rna)
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        for (i, c) in rna.chars().enumerate() {
            match c {
                'C' | 'G' | 'A' | 'U' => (),
                _ => return Err(i),
            }
        }
        Ok(RNA(rna.to_string()))
    }
}
