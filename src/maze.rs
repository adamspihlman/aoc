//! Maze pathfinding for Day 16.
//!
//! Finds shortest paths through a maze where turning costs 1000
//! and moving forward costs 1.

use crate::grid::{self, Direction, Location};
use crate::pathfinding;

/// A maze with start (S) and end (E) positions.
#[derive(Debug)]
pub struct Maze {
    map: Vec<Vec<char>>,
    start: Location,
    end: Location,
}

/// A node in the maze search: position + facing direction.
#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Node {
    location: Location,
    direction: Direction,
}

impl From<Vec<Vec<char>>> for Maze {
    fn from(value: Vec<Vec<char>>) -> Self {
        let start = grid::find_only(&value, 'S');
        let end = grid::find_only(&value, 'E');
        Self {
            map: value,
            start,
            end,
        }
    }
}

impl Maze {
    /// Find the cost of the shortest path through the maze.
    pub fn shortest_path(&self) -> u64 {
        let start = Node {
            location: self.start,
            direction: Direction::Right,
        };

        pathfinding::dijkstra(
            start,
            |node| self.get_neighbors(node),
            |node| node.location == self.end,
        )
        .expect("maze should have a solution")
    }

    /// Count all tiles that lie on any shortest path through the maze.
    pub fn shortest_path_tiles(&self) -> u64 {
        let start = Node {
            location: self.start,
            direction: Direction::Right,
        };

        let result = pathfinding::dijkstra_all_paths(
            start,
            |node| self.get_neighbors(node),
            |node| node.location == self.end,
            |node| node.location,
        )
        .expect("maze should have a solution");

        result.path_locations.len() as u64
    }

    /// Get all neighbors of a node with their movement costs.
    fn get_neighbors(&self, node: &Node) -> Vec<(Node, u64)> {
        let mut neighbors = vec![
            // Turning costs 1000
            (
                Node {
                    location: node.location,
                    direction: grid::rotate_cw(node.direction),
                },
                1000,
            ),
            (
                Node {
                    location: node.location,
                    direction: grid::rotate_ccw(node.direction),
                },
                1000,
            ),
        ];

        // Moving forward costs 1 (if not blocked)
        if let Some(next_loc) = grid::get_location(&self.map, node.location, node.direction) {
            if grid::at(&self.map, next_loc) != '#' {
                neighbors.push((
                    Node {
                        location: next_loc,
                        direction: node.direction,
                    },
                    1,
                ));
            }
        }

        neighbors
    }
}
