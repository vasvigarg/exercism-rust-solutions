use std::collections::{BTreeMap, BTreeSet};

pub struct School {
    roster: BTreeMap<u32, BTreeSet<String>>,
    all_students: BTreeSet<String>,
}

impl School {
    pub fn new() -> School {
        School {
            roster: BTreeMap::new(),
            all_students: BTreeSet::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        // If student already exists in any grade, ignore
        if self.all_students.contains(student) {
            return;
        }

        self.roster
            .entry(grade)
            .or_insert_with(BTreeSet::new)
            .insert(student.to_string());
        self.all_students.insert(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.roster
            .iter()
            .filter(|(_, students)| !students.is_empty())
            .map(|(grade, _)| *grade)
            .collect()
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.roster
            .get(&grade)
            .map(|students| students.iter().cloned().collect())
            .unwrap_or_default()
    }
}
