use crate::grid::{at, find_only, get_location, print_map, Location, DIRECTIONS};
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Cheat {
    grid: Vec<Vec<char>>,
    path_order: Vec<Location>,
}

impl From<Vec<Vec<char>>> for Cheat {
    fn from(grid: Vec<Vec<char>>) -> Self {
        let (start, end) = Self::find_start_and_end(&grid);
        let path_order = Self::build_path(&grid, start, end);
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
                let max_distance = std::cmp::min(end - (start + threshold_usize), max_distance);
                if self.check_cheat(start, end, max_distance) {
                    count += 1;
                }
            }
        }

        count
    }

    fn check_cheat(&self, index1: usize, index2: usize, max_distance: usize) -> bool {
        let loc1 = self.path_order[index1];
        let loc2 = self.path_order[index2];

        let row_diff = loc1.row.abs_diff(loc2.row);
        let col_diff = loc1.col.abs_diff(loc2.col);
        let manhattan_distance = row_diff + col_diff;
        let index_diff = index2.abs_diff(index1);

        manhattan_distance < index_diff && manhattan_distance <= max_distance
    }

    fn find_start_and_end(grid: &[Vec<char>]) -> (Location, Location) {
        let start = find_only(grid, 'S');
        let end = find_only(grid, 'E');
        (start, end)
    }

    fn build_path(grid: &[Vec<char>], start: Location, end: Location) -> Vec<Location> {
        let mut visited = std::collections::HashSet::new();
        let mut path_order = Vec::new();
        let mut queue = VecDeque::new();

        queue.push_back(start);
        visited.insert(start);
        path_order.push(start);

        while let Some(current) = queue.pop_front() {
            if current == end {
                break;
            }

            for direction in DIRECTIONS {
                if let Some(next_loc) = get_location(grid, current, direction) {
                    let next_char = at(grid, next_loc);
                    if (next_char == '.' || next_char == 'S' || next_char == 'E')
                        && !visited.contains(&next_loc)
                    {
                        visited.insert(next_loc);
                        path_order.push(next_loc);
                        queue.push_back(next_loc);
                    }
                }
            }
        }

        path_order
    }
}
