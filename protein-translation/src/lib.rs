use std::collections::HashMap;
use std::iter::FromIterator;

pub struct CodonsInfo<'a> {
    map: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.map.get(codon).map(|&name| name)
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        if rna.is_empty() {
            return None;
        }

        rna.chars()
            .collect::<Vec<char>>()
            .chunks(3)
            .map(|chunk| String::from_iter(chunk))
            .map(|codon| self.name_for(codon.as_str()))
            .take_while(|name| match name {
                None => true, // None means 'not found' which is projected to result Option
                Some(name) => *name != "stop codon",
            })
            .collect()
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo {
        map: HashMap::from_iter(pairs),
    }
}
