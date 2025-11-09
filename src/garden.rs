use std::collections::{BTreeSet, HashSet, VecDeque};

use crate::grid;
use crate::grid::Location;

#[derive(Debug)]
pub struct Garden {
    map: Vec<Vec<char>>,
    unvisited: BTreeSet<Location>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PriceScale {
    Perimeter,
    Corner,
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
    pub fn get_fence_price(&mut self, scale: PriceScale) -> u64 {
        let mut result = 0;

        while !self.unvisited.is_empty() {
            let start = self.unvisited.pop_first().unwrap();
            result += self.compute_price(start, scale);
        }
        result
    }

    fn compute_price(&mut self, start: Location, scale: PriceScale) -> u64 {
        let mut area: u64 = 0;
        let mut perimeter: u64 = 0;
        let mut corners: u64 = 0;

        let mut visited = HashSet::from([start]);
        let mut plots: VecDeque<Location> = VecDeque::from([start]);
        while !plots.is_empty() {
            let location = plots.pop_front().unwrap();
            area += 1;
            perimeter += 4 - self.add_neighbors(location, &mut plots, &mut visited);
            if scale == PriceScale::Corner {
                corners += self.count_corners(location);
            }
        }

        match scale {
            PriceScale::Perimeter => area * perimeter,
            PriceScale::Corner => area * corners,
        }
    }

    fn add_neighbors(
        &mut self,
        location: Location,
        plots: &mut VecDeque<Location>,
        visited: &mut HashSet<Location>,
    ) -> u64 {
        let mut num_neighbors = 0;

        grid::DIRECTIONS.iter().for_each(|&d| {
            let next = grid::get_location(&self.map, location, d);
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

    fn count_corners_at(&self, location: Location, first_direction: grid::Direction) -> u64 {
        let mut num_corners: u64 = 0;
        let plot = grid::at(&self.map, location);
        let second_direction = grid::rotate_direction(first_direction);

        let first_location = grid::get_location(&self.map, location, first_direction);
        let second_location = grid::get_location(&self.map, location, second_direction);

        match (first_location, second_location) {
            (None, None) => num_corners += 1,
            (Some(first_location), None) => {
                if grid::at(&self.map, first_location) != plot {
                    num_corners += 1;
                }
            }
            (None, Some(second_location)) => {
                if grid::at(&self.map, second_location) != plot {
                    num_corners += 1;
                }
            }
            (Some(first_location), Some(second_location)) => {
                let first_plot = grid::at(&self.map, first_location);
                let second_plot = grid::at(&self.map, second_location);

                if first_plot != plot && second_plot != plot {
                    num_corners += 1;
                } else if first_plot == plot && second_plot == plot {
                    let diagonal_location =
                        grid::get_location(&self.map, first_location, second_direction).unwrap();
                    if grid::at(&self.map, diagonal_location) != plot {
                        num_corners += 1;
                    }
                }
            }
        }
        num_corners
    }

    fn count_corners(&self, location: Location) -> u64 {
        grid::DIRECTIONS
            .iter()
            .map(|&d| self.count_corners_at(location, d))
            .sum()
    }
}
