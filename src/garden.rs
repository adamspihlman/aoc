use std::collections::{BTreeSet, HashSet, VecDeque};

use crate::grid::Location;

#[derive(Debug)]
pub struct Garden {
    map: Vec<Vec<char>>,
    unvisited: BTreeSet<Location>,
}

impl From<Vec<Vec<char>>> for Garden {
    fn from(value: Vec<Vec<char>>) -> Self {
        let mut unvisited: BTreeSet<Location> = BTreeSet::new();
        for (row, row_val) in value.iter().enumerate() {
            for (col, _) in row_val.iter().enumerate() {
                unvisited.insert(Location { row, col });
            }
        }
        Self {
            map: value,
            unvisited,
        }
    }
}

impl Garden {
    pub fn get_fence_price(&mut self) -> u64 {
        let mut result = 0;

        while !self.unvisited.is_empty() {
            let start = self.unvisited.pop_first().unwrap();
            result += self.compute_region_price(start);
        }
        result
    }

    fn compute_region_price(&mut self, start: Location) -> u64 {
        let mut area = 0_u64;
        let mut perimeter = 0_u64;

        let mut visited = HashSet::from([start]);
        let mut plots: VecDeque<Location> = VecDeque::from([start]);
        while !plots.is_empty() {
            let location = plots.pop_front().unwrap();
            area += 1;
            perimeter += 4 - self.add_neighbors(location, &mut plots, &mut visited);
        }

        area * perimeter
    }

    fn add_neighbors(
        &mut self,
        location: Location,
        plots: &mut VecDeque<Location>,
        visited: &mut HashSet<Location>,
    ) -> u64 {
        let mut num_neighbors = 0;

        crate::grid::DIRECTIONS.iter().for_each(|&d| {
            let next = crate::grid::get_location(&self.map, location, d);
            if let Some(next) = next {
                if self.map[location.row][location.col] == self.map[next.row][next.col] {
                    num_neighbors += 1;
                    if !visited.contains(&next) {
                        plots.push_back(next);
                        visited.insert(next);
                        self.unvisited.remove(&next);
                    }
                }
            }
        });
        num_neighbors
    }
}
