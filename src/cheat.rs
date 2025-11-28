//! Cheat path analysis for Day 20.
//!
//! Analyzes paths through a maze to find "cheats" (shortcuts through walls).

use crate::bfs;
use crate::grid::{at, find_only, get_location, print_map, Location, DIRECTIONS};

/// Cheat analyzer for a maze with a single path.
#[derive(Debug)]
pub struct Cheat {
    grid: Vec<Vec<char>>,
    path_order: Vec<Location>,
}

impl From<Vec<Vec<char>>> for Cheat {
    fn from(grid: Vec<Vec<char>>) -> Self {
        let start = find_only(&grid, 'S');
        let end = find_only(&grid, 'E');

        let path_order = bfs::build_path(
            start,
            |loc| {
                DIRECTIONS
                    .iter()
                    .filter_map(|&d| {
                        get_location(&grid, *loc, d).filter(|&next| {
                            let c = at(&grid, next);
                            c == '.' || c == 'S' || c == 'E'
                        })
                    })
                    .collect()
            },
            |loc| *loc == end,
        );

        Self { grid, path_order }
    }
}

impl Cheat {
    pub fn print(&self) {
        print_map(&self.grid);
    }

    pub fn print_path(&self) {
        println!(
            "Path order ({}): {:?}",
            self.path_order.len(),
            self.path_order
        );
    }

    /// Count cheats that save at least `threshold` steps.
    ///
    /// A cheat allows passing through walls for up to `max_distance` steps.
    pub fn count_cheats(&self, threshold: u64, max_distance: usize) -> u64 {
        let mut count = 0;
        let path_len = self.path_order.len();
        let threshold_usize = threshold as usize;

        for start in 0..path_len {
            let first_end = start + threshold_usize + 2;
            if first_end >= path_len {
                break;
            }

            for end in first_end..path_len {
                let max_dist = std::cmp::min(end - (start + threshold_usize), max_distance);
                if self.check_cheat(start, end, max_dist) {
                    count += 1;
                }
            }
        }

        count
    }

    /// Check if a cheat between two path indices is valid.
    fn check_cheat(&self, index1: usize, index2: usize, max_distance: usize) -> bool {
        let loc1 = self.path_order[index1];
        let loc2 = self.path_order[index2];

        let row_diff = loc1.row.abs_diff(loc2.row);
        let col_diff = loc1.col.abs_diff(loc2.col);
        let manhattan_distance = row_diff + col_diff;
        let index_diff = index2.abs_diff(index1);

        manhattan_distance < index_diff && manhattan_distance <= max_distance
    }
}
