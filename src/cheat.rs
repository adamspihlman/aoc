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
        println!("Path order ({}): {:?}", self.path_order.len(), self.path_order);
    }

    pub fn count_cheats(&self, threshold: u64) -> u64 {
        let mut count = 0;
        let path_len = self.path_order.len();
        let threshold_usize = threshold as usize;

        for start in 0..path_len {
            let first_end = start + threshold_usize + 2;
            if first_end >= path_len {
                break;
            }

            for end in first_end..path_len {
                if self.check_cheat(start, end) {
                    count += 1;
                }
            }
        }

        count
    }

    fn check_cheat(&self, index1: usize, index2: usize) -> bool {
        let loc1 = self.path_order[index1];
        let loc2 = self.path_order[index2];

        let same_row = loc1.row == loc2.row;
        let same_col = loc1.col == loc2.col;
        let row_diff = loc1.row.abs_diff(loc2.row);
        let col_diff = loc1.col.abs_diff(loc2.col);

        if (same_row && col_diff == 2) || (same_col && row_diff == 2) {
            // Check that the location halfway between has a wall
            let mid_row = (loc1.row + loc2.row) / 2;
            let mid_col = (loc1.col + loc2.col) / 2;
            let mid_loc = Location { row: mid_row, col: mid_col };
            let mid_char = at(&self.grid, mid_loc);
            return mid_char == '#';
        }

        false
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
