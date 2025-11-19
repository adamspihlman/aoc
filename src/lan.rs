use std::collections::{HashMap, HashSet};

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

#[derive(Debug)]
pub struct Lan {
    connections: HashMap<Computer, HashSet<Computer>>,
}

impl From<Vec<(String, String)>> for Lan {
    fn from(connections: Vec<(String, String)>) -> Self {
        let mut map = HashMap::new();

        for (name1, name2) in connections {
            let comp1 = Computer::new(&name1);
            let comp2 = Computer::new(&name2);

            map.entry(comp1).or_insert_with(HashSet::new).insert(comp2);

            map.entry(comp2).or_insert_with(HashSet::new).insert(comp1);
        }

        Self { connections: map }
    }
}

impl Lan {
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
