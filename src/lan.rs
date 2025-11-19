use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Debug)]
pub struct Lan {
    connections: HashMap<Computer, HashSet<Computer>>,
    counts_map: HashMap<u32, HashSet<Computer>>,
    counts_heap: BinaryHeap<u32>,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Computer {
    name: [u8; 2],
}

impl Computer {
    pub fn new(name: &str) -> Self {
        let bytes = name.as_bytes();
        Self {
            name: [bytes[0], bytes[1]],
        }
    }

    pub fn name(&self) -> &str {
        std::str::from_utf8(&self.name).unwrap()
    }
}

impl From<Vec<(String, String)>> for Lan {
    fn from(value: Vec<(String, String)>) -> Self {
        let mut connections = HashMap::new();

        for (name1, name2) in value {
            let comp1 = Computer::new(&name1);
            let comp2 = Computer::new(&name2);

            connections
                .entry(comp1)
                .or_insert_with(HashSet::new)
                .insert(comp2);

            connections
                .entry(comp2)
                .or_insert_with(HashSet::new)
                .insert(comp1);
        }

        let mut counts_map = HashMap::new();
        for (computer, connections) in &connections {
            let count = connections.len() as u32;
            counts_map
                .entry(count)
                .or_insert_with(HashSet::new)
                .insert(*computer);
        }

        let counts_heap: BinaryHeap<u32> = counts_map.keys().copied().collect();

        Self {
            connections,
            counts_map,
            counts_heap,
        }
    }
}

impl Lan {
    pub fn find_largest_group(&mut self) -> HashSet<Computer> {
        while !self.counts_heap.is_empty() {
            let next = self.counts_heap.pop().unwrap();
            println!("next largest counts: {next}");
            println!(
                "computers with that count: {:?}",
                self.counts_map.get(&next).unwrap()
            );
        }
        HashSet::new()
    }

    pub fn find_groups(&self) -> Vec<(Computer, Computer, Computer)> {
        let mut groups = Vec::new();
        let mut visited = HashSet::new();

        for (first, first_connections) in &self.connections {
            visited.insert(*first);

            let mut inner_visited = HashSet::new();
            for second in first_connections {
                if visited.contains(second) || inner_visited.contains(second) {
                    continue;
                }
                inner_visited.insert(*second);

                let second_connections = self.connections.get(second).unwrap();
                let intersection: HashSet<_> = first_connections
                    .intersection(second_connections)
                    .copied()
                    .collect();

                for third in intersection {
                    if visited.contains(&third) || inner_visited.contains(&third) {
                        continue;
                    }

                    if first.name[0] == b't' || second.name[0] == b't' || third.name[0] == b't' {
                        let group = (*first, *second, third);
                        groups.push(group);
                    }
                }
            }
        }

        groups
    }
}
