//! Generic BFS and flood-fill algorithms.
//!
//! This module provides reusable breadth-first search implementations
//! for various grid-based and graph-based problems.

use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

/// Perform a breadth-first search from a start node to find a goal.
///
/// Returns the path from start to goal if found, or None if unreachable.
///
/// # Arguments
/// * `start` - The starting node
/// * `neighbors` - Function returning neighboring nodes for a given node
/// * `is_goal` - Predicate to check if a node is the goal
pub fn search<N, FN, FG>(start: N, neighbors: FN, is_goal: FG) -> Option<Vec<N>>
where
    N: Eq + Hash + Copy,
    FN: Fn(&N) -> Vec<N>,
    FG: Fn(&N) -> bool,
{
    if is_goal(&start) {
        return Some(vec![start]);
    }

    let mut visited: HashSet<N> = HashSet::from([start]);
    let mut queue: VecDeque<(N, Vec<N>)> = VecDeque::from([(start, vec![start])]);

    while let Some((current, path)) = queue.pop_front() {
        for next in neighbors(&current) {
            if visited.contains(&next) {
                continue;
            }

            let mut new_path = path.clone();
            new_path.push(next);

            if is_goal(&next) {
                return Some(new_path);
            }

            visited.insert(next);
            queue.push_back((next, new_path));
        }
    }

    None
}

/// Find all nodes reachable from start using flood-fill.
///
/// Returns a set of all connected nodes (including start).
///
/// # Arguments
/// * `start` - The starting node
/// * `neighbors` - Function returning valid neighboring nodes to explore
pub fn flood_fill<N, FN>(start: N, neighbors: FN) -> HashSet<N>
where
    N: Eq + Hash + Copy,
    FN: Fn(&N) -> Vec<N>,
{
    let mut visited: HashSet<N> = HashSet::from([start]);
    let mut queue: VecDeque<N> = VecDeque::from([start]);

    while let Some(current) = queue.pop_front() {
        for next in neighbors(&current) {
            if !visited.contains(&next) {
                visited.insert(next);
                queue.push_back(next);
            }
        }
    }

    visited
}

/// Find the shortest path length from start to goal.
///
/// Returns the number of steps if a path exists, or None if unreachable.
///
/// # Arguments
/// * `start` - The starting node
/// * `neighbors` - Function returning neighboring nodes
/// * `is_goal` - Predicate to check if a node is the goal
pub fn shortest_distance<N, FN, FG>(start: N, neighbors: FN, is_goal: FG) -> Option<usize>
where
    N: Eq + Hash + Copy,
    FN: Fn(&N) -> Vec<N>,
    FG: Fn(&N) -> bool,
{
    if is_goal(&start) {
        return Some(0);
    }

    let mut visited: HashSet<N> = HashSet::from([start]);
    let mut queue: VecDeque<(N, usize)> = VecDeque::from([(start, 0)]);

    while let Some((current, dist)) = queue.pop_front() {
        for next in neighbors(&current) {
            if visited.contains(&next) {
                continue;
            }

            if is_goal(&next) {
                return Some(dist + 1);
            }

            visited.insert(next);
            queue.push_back((next, dist + 1));
        }
    }

    None
}

/// Traverse all nodes reachable from start, calling a visitor function on each.
///
/// The visitor receives each node as it's discovered.
/// Unlike flood_fill, this doesn't return the set of nodes but allows
/// custom processing during traversal.
///
/// # Arguments
/// * `start` - The starting node
/// * `neighbors` - Function returning valid neighboring nodes
/// * `visitor` - Function called for each discovered node
pub fn traverse<N, FN, FV>(start: N, neighbors: FN, mut visitor: FV)
where
    N: Eq + Hash + Copy,
    FN: Fn(&N) -> Vec<N>,
    FV: FnMut(&N),
{
    let mut visited: HashSet<N> = HashSet::from([start]);
    let mut queue: VecDeque<N> = VecDeque::from([start]);

    visitor(&start);

    while let Some(current) = queue.pop_front() {
        for next in neighbors(&current) {
            if !visited.contains(&next) {
                visited.insert(next);
                visitor(&next);
                queue.push_back(next);
            }
        }
    }
}

/// Build a path through the grid, tracking order of visited nodes.
///
/// Similar to BFS but returns nodes in order of discovery.
/// Useful for building an ordered path through a maze with no branches.
///
/// # Arguments
/// * `start` - The starting node
/// * `neighbors` - Function returning valid neighboring nodes
/// * `is_end` - Optional predicate to stop early at a goal
pub fn build_path<N, FN, FG>(start: N, neighbors: FN, is_end: FG) -> Vec<N>
where
    N: Eq + Hash + Copy,
    FN: Fn(&N) -> Vec<N>,
    FG: Fn(&N) -> bool,
{
    let mut visited: HashSet<N> = HashSet::from([start]);
    let mut path: Vec<N> = vec![start];
    let mut queue: VecDeque<N> = VecDeque::from([start]);

    while let Some(current) = queue.pop_front() {
        if is_end(&current) {
            break;
        }

        for next in neighbors(&current) {
            if !visited.contains(&next) {
                visited.insert(next);
                path.push(next);
                queue.push_back(next);
            }
        }
    }

    path
}
