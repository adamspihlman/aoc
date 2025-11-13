use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

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

#[derive(Debug)]
struct Ledger {
    weight: u64,
    predecessors: Vec<Node>,
}

impl From<Vec<Vec<char>>> for Maze {
    fn from(value: Vec<Vec<char>>) -> Self {
        Self { map: value }
    }
}

impl Ord for WeightedNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.weight.cmp(&self.weight)
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

    pub fn shortest_path_tiles(&self) -> u64 {
        let start = self.weighted_start();
        let mut nodes: HashMap<Node, Ledger> = HashMap::from([(
            start.node,
            Ledger {
                weight: start.weight,
                predecessors: Vec::new(),
            },
        )]);
        let mut unvisited: BinaryHeap<WeightedNode> = BinaryHeap::from([start]);
        let mut unvisited_ends = self.ends();
        let mut weighted_ends: HashSet<WeightedNode> = HashSet::new();

        while !unvisited_ends.is_empty() {
            let wnode = Maze::next_ledger_node(&nodes, &mut unvisited);
            if unvisited_ends.contains(&wnode.node) {
                unvisited_ends.remove(&wnode.node);
                weighted_ends.insert(wnode);
            }

            self.add_ledger_neighbors(wnode, &mut nodes, &mut unvisited);
        }

        let tiles = Maze::get_shortest_paths_tiles(&nodes, &weighted_ends);
        tiles.len() as u64
    }

    fn get_shortest_paths_tiles(
        nodes: &HashMap<Node, Ledger>,
        weighted_ends: &HashSet<WeightedNode>,
    ) -> HashSet<grid::Location> {
        let mut tiles: HashSet<grid::Location> = HashSet::new();
        let mut queue: VecDeque<Node> = VecDeque::new();

        let min_weight = weighted_ends.iter().map(|w| w.weight).min().unwrap();
        let min_ends: Vec<WeightedNode> = weighted_ends
            .iter()
            .filter(|w| w.weight == min_weight)
            .copied()
            .collect();

        for wnode in min_ends {
            tiles.insert(wnode.node.location);
            queue.extend(nodes.get(&wnode.node).unwrap().predecessors.clone());
        }

        while !queue.is_empty() {
            let next = queue.pop_front().unwrap();
            tiles.insert(next.location);
            queue.extend(nodes.get(&next).unwrap().predecessors.clone());
        }

        tiles
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

    fn add_ledger_neighbors(
        &self,
        wnode: WeightedNode,
        nodes: &mut HashMap<Node, Ledger>,
        unvisited: &mut BinaryHeap<WeightedNode>,
    ) {
        for neighbor in self.get_neighbors(wnode) {
            match nodes.entry(neighbor.node) {
                Entry::Vacant(e) => {
                    e.insert(Ledger {
                        weight: neighbor.weight,
                        predecessors: vec![wnode.node],
                    });
                    unvisited.push(neighbor);
                }
                Entry::Occupied(mut e) => {
                    let prior_weight = e.get().weight;
                    if prior_weight > neighbor.weight {
                        e.insert(Ledger {
                            weight: neighbor.weight,
                            predecessors: vec![wnode.node],
                        });
                        unvisited.push(neighbor);
                    } else if prior_weight == neighbor.weight {
                        e.get_mut().predecessors.push(wnode.node);
                    }
                }
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

    fn next_ledger_node(
        nodes: &HashMap<Node, Ledger>,
        unvisited: &mut BinaryHeap<WeightedNode>,
    ) -> WeightedNode {
        let mut wnode = unvisited.pop().unwrap();
        while nodes.get(&wnode.node).unwrap().weight < wnode.weight {
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
