use std::collections::BTreeMap;
use std::collections::BTreeSet;

pub struct School(BTreeMap<u32, BTreeSet<String>>);

impl School {
    pub fn new() -> School {
        School(BTreeMap::new())
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        match self.0.get_mut(&grade) {
            Some(names) => {
                names.insert(student.to_string());
            }
            None => {
                let mut names = BTreeSet::new();
                names.insert(student.to_string());
                self.0.insert(grade, names);
            }
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        self.0
            .iter()
            .filter(|(_, names)| names.len() > 0)
            .map(|(grade, _)| *grade)
            .collect()
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.0
            .get(&grade)
            .map(|names| names.iter().map(|s| s.clone()).collect())
    }
}
