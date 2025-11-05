use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Rules {
    rules: HashMap<u64, HashSet<u64>>,
}

pub fn build_rules(input: Vec<(u64, u64)>) -> Rules {
    let mut rules: HashMap<u64, HashSet<u64>> = HashMap::new();

    for (before, after) in input {
        rules.entry(before).or_insert(HashSet::new()).insert(after);
    }
    Rules { rules }
}

impl Rules {
    pub fn is_valid(&self, update: &[u64]) -> bool {
        let mut seen: HashSet<u64> = HashSet::new();

        for &item in update {
            for &prev in &seen {
                if self.is_before(item, prev) {
                    return false;
                }
            }
            seen.insert(item);
        }
        true
    }

    fn is_before(&self, before: u64, after: u64) -> bool {
        self.rules.contains_key(&before) && self.rules.get(&before).unwrap().contains(&after)
    }
}
