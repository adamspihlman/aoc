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
    pub fn print(&self) {
        for (before, after_set) in &self.rules {
            println!("{before}|{:?}", after_set);
        }
    }
}
