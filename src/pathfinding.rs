//! Generic pathfinding algorithms for grid-based problems.
//!
//! This module provides a trait-based Dijkstra implementation that can be
//! used for various pathfinding scenarios.

use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::hash::Hash;

/// A node in the search graph with its neighbors and costs.
///
/// Implement this trait for your specific pathfinding problem.
pub trait SearchNode: Copy + Eq + Hash {
    /// Returns all neighboring nodes and the cost to reach them.
    fn neighbors(&self) -> Vec<(Self, u64)>;

    /// Returns true if this node is a goal state.
    fn is_goal(&self) -> bool;
}

/// Weighted node for priority queue ordering.
#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub struct WeightedNode<N> {
    pub weight: u64,
    pub node: N,
}

impl<N: Eq> Ord for WeightedNode<N> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.weight.cmp(&self.weight)
    }
}

impl<N: Eq> PartialOrd for WeightedNode<N> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Ledger entry for tracking predecessors in all-paths search.
#[derive(Debug)]
struct Ledger<N> {
    weight: u64,
    predecessors: Vec<N>,
}

/// Find the shortest path cost from start to any goal node.
///
/// Returns `None` if no path exists.
pub fn dijkstra<N: SearchNode>(start: N) -> Option<u64> {
    let start_weighted = WeightedNode {
        weight: 0,
        node: start,
    };
    let mut visited: HashMap<N, u64> = HashMap::from([(start, 0)]);
    let mut queue: BinaryHeap<WeightedNode<N>> = BinaryHeap::from([start_weighted]);

    while let Some(current) = pop_best(&visited, &mut queue) {
        if current.node.is_goal() {
            return Some(current.weight);
        }

        for (neighbor, cost) in current.node.neighbors() {
            let new_weight = current.weight + cost;
            match visited.entry(neighbor) {
                Entry::Vacant(e) => {
                    e.insert(new_weight);
                    queue.push(WeightedNode {
                        weight: new_weight,
                        node: neighbor,
                    });
                }
                Entry::Occupied(mut e) if *e.get() > new_weight => {
                    e.insert(new_weight);
                    queue.push(WeightedNode {
                        weight: new_weight,
                        node: neighbor,
                    });
                }
                _ => {}
            }
        }
    }

    None
}

/// Find all nodes that lie on any shortest path from start to goal.
///
/// Returns the set of nodes on optimal paths and the minimum cost.
/// Returns `None` if no path exists.
pub fn dijkstra_all_paths<N, L>(start: N, locator: L) -> Option<(HashSet<N::Location>, u64)>
where
    N: SearchNode + AllPathsNode,
    L: Fn(&N) -> N::Location,
{
    let start_weighted = WeightedNode {
        weight: 0,
        node: start,
    };
    let mut visited: HashMap<N, Ledger<N>> = HashMap::from([(
        start,
        Ledger {
            weight: 0,
            predecessors: Vec::new(),
        },
    )]);
    let mut queue: BinaryHeap<WeightedNode<N>> = BinaryHeap::from([start_weighted]);
    let mut goal_nodes: HashSet<WeightedNode<N>> = HashSet::new();
    let mut found_goals: HashSet<N> = HashSet::new();

    // First pass: find all goal nodes with their weights
    while !queue.is_empty() {
        let current = pop_best_ledger(&visited, &mut queue);

        if current.node.is_goal() && !found_goals.contains(&current.node) {
            found_goals.insert(current.node);
            goal_nodes.insert(current);
        }

        // Stop exploring once we've found goals and current weight exceeds minimum
        if !goal_nodes.is_empty() {
            let min_goal_weight = goal_nodes.iter().map(|g| g.weight).min().unwrap_or(u64::MAX);
            if current.weight > min_goal_weight {
                break;
            }
        }

        for (neighbor, cost) in current.node.neighbors() {
            let new_weight = current.weight + cost;
            match visited.entry(neighbor) {
                Entry::Vacant(e) => {
                    e.insert(Ledger {
                        weight: new_weight,
                        predecessors: vec![current.node],
                    });
                    queue.push(WeightedNode {
                        weight: new_weight,
                        node: neighbor,
                    });
                }
                Entry::Occupied(mut e) => {
                    let prior_weight = e.get().weight;
                    if prior_weight > new_weight {
                        e.insert(Ledger {
                            weight: new_weight,
                            predecessors: vec![current.node],
                        });
                        queue.push(WeightedNode {
                            weight: new_weight,
                            node: neighbor,
                        });
                    } else if prior_weight == new_weight {
                        e.get_mut().predecessors.push(current.node);
                    }
                }
            }
        }
    }

    if goal_nodes.is_empty() {
        return None;
    }

    // Find minimum weight goal(s)
    let min_weight = goal_nodes
        .iter()
        .map(|g| g.weight)
        .min()
        .expect("goal_nodes is not empty");
    let min_goals: Vec<WeightedNode<N>> = goal_nodes
        .iter()
        .filter(|g| g.weight == min_weight)
        .copied()
        .collect();

    // Backtrack to find all nodes on shortest paths
    let mut path_nodes: HashSet<N::Location> = HashSet::new();
    let mut backtrack_queue: VecDeque<N> = VecDeque::new();

    for goal in min_goals {
        path_nodes.insert(locator(&goal.node));
        if let Some(ledger) = visited.get(&goal.node) {
            backtrack_queue.extend(ledger.predecessors.iter().copied());
        }
    }

    while let Some(node) = backtrack_queue.pop_front() {
        path_nodes.insert(locator(&node));
        if let Some(ledger) = visited.get(&node) {
            backtrack_queue.extend(ledger.predecessors.iter().copied());
        }
    }

    Some((path_nodes, min_weight))
}

/// Trait for nodes that can extract a location for all-paths tracking.
pub trait AllPathsNode {
    type Location: Eq + Hash;
}

/// Pop the next node from the queue, skipping outdated entries.
fn pop_best<N: SearchNode>(
    visited: &HashMap<N, u64>,
    queue: &mut BinaryHeap<WeightedNode<N>>,
) -> Option<WeightedNode<N>> {
    while let Some(node) = queue.pop() {
        if let Some(&best_weight) = visited.get(&node.node) {
            if best_weight == node.weight {
                return Some(node);
            }
        }
    }
    None
}

/// Pop the next node from the queue for ledger-based search.
fn pop_best_ledger<N: SearchNode>(
    visited: &HashMap<N, Ledger<N>>,
    queue: &mut BinaryHeap<WeightedNode<N>>,
) -> WeightedNode<N> {
    let mut node = queue.pop().expect("queue should not be empty");
    while visited
        .get(&node.node)
        .map(|l| l.weight < node.weight)
        .unwrap_or(false)
    {
        node = queue.pop().expect("queue should not be empty");
    }
    node
}
