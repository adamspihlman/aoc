//! Grid pathfinding for Day 18.
//!
//! Finds shortest path through a grid avoiding obstacles (#).

use crate::grid::{self, Location, DIRECTIONS};
use crate::pathfinding;

/// A grid with obstacles to navigate around.
#[derive(Debug)]
pub struct ByteDodge {
    grid: Vec<Vec<char>>,
}

impl From<Vec<Vec<char>>> for ByteDodge {
    fn from(value: Vec<Vec<char>>) -> Self {
        Self { grid: value }
    }
}

impl ByteDodge {
    /// Find the minimum steps from top-left to bottom-right.
    ///
    /// Returns 0 if no path exists.
    pub fn min_steps(&self) -> u64 {
        let start = Location { row: 0, col: 0 };
        let end = Location {
            row: self.grid.len() - 1,
            col: self.grid[0].len() - 1,
        };

        pathfinding::dijkstra(
            start,
            |loc| self.get_neighbors(loc),
            |loc| *loc == end,
        )
        .unwrap_or(0)
    }

    /// Get all passable neighbors of a location with uniform cost 1.
    fn get_neighbors(&self, loc: &Location) -> Vec<(Location, u64)> {
        DIRECTIONS
            .iter()
            .filter_map(|&d| {
                grid::get_location(&self.grid, *loc, d).and_then(|next| {
                    if grid::at(&self.grid, next) != '#' {
                        Some((next, 1))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }
}
