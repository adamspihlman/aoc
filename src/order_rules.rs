use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Rules {
    rules: HashMap<u64, HashSet<u64>>,
}

pub fn build_rules(input: Vec<(u64, u64)>) -> Rules {
    let mut rules: HashMap<u64, HashSet<u64>> = HashMap::new();

    for (before, after) in input {
        rules.entry(before).or_default().insert(after);
    }

    Rules { rules }
}

impl Rules {
    pub fn is_valid(&self, update: &[u64]) -> bool {
        let mut seen: HashSet<u64> = HashSet::new();

        for &cur in update {
            for &prev in &seen {
                if self.is_before(cur, prev) {
                    return false;
                }
            }
            seen.insert(cur);
        }
        true
    }

    pub fn make_valid(&self, update: &[u64]) -> Vec<u64> {
        let mut result = update.to_vec();
        result.sort_by(|a, b| {
            if self.is_before(*a, *b) {
                Ordering::Less
            } else if self.is_before(*b, *a) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
        result
    }

    fn is_before(&self, before: u64, after: u64) -> bool {
        self.rules.contains_key(&before) && self.rules.get(&before).unwrap().contains(&after)
    }
}
