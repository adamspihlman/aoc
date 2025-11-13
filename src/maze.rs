use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::grid::{self, DIRECTIONS};

#[derive(Debug)]
pub struct Maze {
    map: Vec<Vec<char>>,
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Node {
    location: grid::Location,
    direction: grid::Direction,
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct WeightedNode {
    weight: u64,
    node: Node,
}

impl From<Vec<Vec<char>>> for Maze {
    fn from(value: Vec<Vec<char>>) -> Self {
        Self { map: value }
    }
}

impl Ord for WeightedNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl PartialOrd for WeightedNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Maze {
    pub fn shortest_path(&self) -> u64 {
        let start = self.weighted_start();
        let mut nodes: HashMap<Node, u64> = HashMap::from([(start.node, start.weight)]);
        let mut unvisited: BinaryHeap<WeightedNode> = BinaryHeap::from([start]);
        let mut unvisited_ends = self.ends();
        let mut weighted_ends: HashSet<WeightedNode> = HashSet::new();

        while !unvisited_ends.is_empty() {
            let wnode = Maze::next_node(&nodes, &mut unvisited);
            if unvisited_ends.contains(&wnode.node) {
                unvisited_ends.remove(&wnode.node);
                weighted_ends.insert(wnode);
            }

            self.add_neighbors(wnode, &mut nodes, &mut unvisited);
        }

        weighted_ends.iter().map(|w| w.weight).min().unwrap()
    }

    fn add_neighbors(
        &self,
        wnode: WeightedNode,
        nodes: &mut HashMap<Node, u64>,
        unvisited: &mut BinaryHeap<WeightedNode>,
    ) {
        for neighbor in self.get_neighbors(wnode) {
            match nodes.entry(neighbor.node) {
                Entry::Vacant(e) => {
                    e.insert(neighbor.weight);
                    unvisited.push(neighbor);
                }
                Entry::Occupied(mut e) if *e.get() > neighbor.weight => {
                    e.insert(neighbor.weight);
                    unvisited.push(neighbor);
                }
                _ => {}
            }
        }
    }

    fn get_neighbors(&self, wnode: WeightedNode) -> Vec<WeightedNode> {
        let mut result = vec![
            Maze::clockwise_wnode(wnode),
            Maze::counterclockwise_wnode(wnode),
        ];

        if let Some(neighbor) = self.neighbor_wnode(wnode) {
            result.push(neighbor);
        }
        result
    }

    fn neighbor_wnode(&self, wnode: WeightedNode) -> Option<WeightedNode> {
        let location =
            grid::get_location(&self.map, wnode.node.location, wnode.node.direction).unwrap();
        if grid::at(&self.map, location) == '#' {
            return None;
        }
        Some(WeightedNode {
            weight: wnode.weight + 1,
            node: Node {
                location,
                direction: wnode.node.direction,
            },
        })
    }

    fn clockwise_wnode(wnode: WeightedNode) -> WeightedNode {
        let clockwise = grid::rotate_cw(wnode.node.direction);
        let clockwise_node = Node {
            location: wnode.node.location,
            direction: clockwise,
        };
        WeightedNode {
            weight: wnode.weight + 1000,
            node: clockwise_node,
        }
    }

    fn counterclockwise_wnode(wnode: WeightedNode) -> WeightedNode {
        let counterclockwise = grid::rotate_ccw(wnode.node.direction);
        let counterclockwise_node = Node {
            location: wnode.node.location,
            direction: counterclockwise,
        };
        WeightedNode {
            weight: wnode.weight + 1000,
            node: counterclockwise_node,
        }
    }

    fn next_node(
        nodes: &HashMap<Node, u64>,
        unvisited: &mut BinaryHeap<WeightedNode>,
    ) -> WeightedNode {
        let mut wnode = unvisited.pop().unwrap();
        while nodes.get(&wnode.node).unwrap() < &wnode.weight {
            wnode = unvisited.pop().unwrap();
        }
        wnode
    }

    fn weighted_start(&self) -> WeightedNode {
        WeightedNode {
            weight: 0,
            node: Node {
                location: grid::find_only(&self.map, 'S'),
                direction: grid::Direction::Right,
            },
        }
    }

    fn ends(&self) -> HashSet<Node> {
        let end = grid::find_only(&self.map, 'E');
        DIRECTIONS
            .iter()
            .map(|&d| Node {
                location: end,
                direction: d,
            })
            .collect()
    }
}
