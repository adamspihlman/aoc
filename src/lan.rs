use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Lan {
    connections: HashMap<Computer, HashSet<Computer>>,
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

        Self { connections }
    }
}

impl Lan {
    /// Find the maximal clique using the Bron-Kerbosch algorithm with pivot optimization.
    /// Returns the single largest clique in the graph.
    pub fn find_maximal_clique(&self) -> HashSet<Computer> {
        let mut max_clique = HashSet::new();
        let all_vertices: HashSet<Computer> = self.connections.keys().copied().collect();

        self.bron_kerbosch(
            HashSet::new(), // R: current clique
            all_vertices,   // P: candidate vertices
            HashSet::new(), // X: excluded vertices
            &mut max_clique,
        );

        max_clique
    }

    /// Recursive Bron-Kerbosch algorithm with pivot optimization.
    ///
    /// # Arguments
    /// * `r` - Current clique being built
    /// * `p` - Candidate vertices that could extend the clique
    /// * `x` - Vertices already processed (to avoid duplicates)
    /// * `max_clique` - Tracks the largest clique found so far
    fn bron_kerbosch(
        &self,
        r: HashSet<Computer>,
        mut p: HashSet<Computer>,
        mut x: HashSet<Computer>,
        max_clique: &mut HashSet<Computer>,
    ) {
        // Base case: if P and X are both empty, R is a maximal clique
        if p.is_empty() && x.is_empty() {
            if r.len() > max_clique.len() {
                *max_clique = r;
            }
            return;
        }

        // Pivot optimization: choose vertex from P ∪ X with most neighbors in P
        // This minimizes the number of recursive calls
        let pivot = p
            .union(&x)
            .max_by_key(|&&v| {
                self.connections
                    .get(&v)
                    .map(|neighbors| p.intersection(neighbors).count())
                    .unwrap_or(0)
            })
            .copied();

        // Get pivot's neighbors (or empty set if no pivot)
        let pivot_neighbors = pivot
            .and_then(|v| self.connections.get(&v))
            .cloned()
            .unwrap_or_else(HashSet::new);

        // Process vertices in P \ N(pivot)
        // We only need to consider vertices not connected to the pivot
        let candidates: Vec<Computer> = p.difference(&pivot_neighbors).copied().collect();

        for v in candidates {
            // Get neighbors of v
            let neighbors = match self.connections.get(&v) {
                Some(n) => n,
                None => continue,
            };

            // Build R ∪ {v}
            let mut r_new = r.clone();
            r_new.insert(v);

            // Build P ∩ N(v) - only neighbors of v can extend the clique
            let p_new: HashSet<Computer> = p.intersection(neighbors).copied().collect();

            // Build X ∩ N(v) - only track excluded vertices that are neighbors
            let x_new: HashSet<Computer> = x.intersection(neighbors).copied().collect();

            // Recurse with updated sets
            self.bron_kerbosch(r_new, p_new, x_new, max_clique);

            // Move v from P to X (mark as processed)
            p.remove(&v);
            x.insert(v);
        }
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
