use crate::grid::{at, find_only, get_location, print_map, Location, DIRECTIONS};
use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
pub struct Cheat {
    grid: Vec<Vec<char>>,
    path: HashSet<Location>,
    path_order: Vec<Location>,
}

impl From<Vec<Vec<char>>> for Cheat {
    fn from(grid: Vec<Vec<char>>) -> Self {
        let (start, end) = Self::find_start_and_end(&grid);
        let (path, path_order) = Self::build_path(&grid, start, end);
        Self {
            grid,
            path,
            path_order,
        }
    }
}

impl Cheat {
    pub fn print(&self) {
        print_map(&self.grid);
    }

    pub fn print_path(&self) {
        println!("Path order ({}): {:?}", self.path_order.len(), self.path_order);
    }

    fn find_start_and_end(grid: &[Vec<char>]) -> (Location, Location) {
        let start = find_only(grid, 'S');
        let end = find_only(grid, 'E');
        (start, end)
    }

    fn build_path(
        grid: &[Vec<char>],
        start: Location,
        end: Location,
    ) -> (HashSet<Location>, Vec<Location>) {
        let mut visited = HashSet::new();
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

        (visited, path_order)
    }
}
