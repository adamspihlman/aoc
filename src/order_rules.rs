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
        let mut copy = update.to_vec();
        self.attempt_fix(&mut copy);

        while !self.is_valid(&copy) {
            self.attempt_fix(&mut copy);
        }
        copy
    }

    fn attempt_fix(&self, update: &mut [u64]) {
        let mut seen: HashMap<u64, usize> = HashMap::new();

        for (index, &cur) in update.iter().enumerate() {
            for &prev in seen.keys() {
                if self.is_before(cur, prev) {
                    update.swap(index, seen.get(&prev).copied().unwrap());
                    return;
                }
            }
            seen.insert(cur, index);
        }
    }

    fn is_before(&self, before: u64, after: u64) -> bool {
        self.rules.contains_key(&before) && self.rules.get(&before).unwrap().contains(&after)
    }
}
