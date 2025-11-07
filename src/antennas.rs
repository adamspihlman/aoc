use std::collections::{HashMap, HashSet};

use crate::grid::Location;

#[derive(Debug)]
pub struct Antennas {
    map: Vec<Vec<char>>,
    antennas: HashMap<char, Vec<Location>>,
}

impl Antennas {
    pub fn from(map: Vec<Vec<char>>) -> Antennas {
        let antennas = Antennas::find_antennas(&map);
        Antennas { map, antennas }
    }

    fn find_antennas(map: &[Vec<char>]) -> HashMap<char, Vec<Location>> {
        let mut antennas: HashMap<char, Vec<Location>> = HashMap::new();

        for (row_idx, row_val) in map.iter().enumerate() {
            for (col_idx, _) in row_val.iter().enumerate() {
                let value = map[row_idx][col_idx];
                if value == '.' {
                    continue;
                }
                let location = Location {
                    row: row_idx,
                    col: col_idx,
                };
                antennas.entry(value).or_default().push(location);
            }
        }

        antennas
    }

    fn calculate_antinodes(&self, left: &Location, right: &Location) -> Vec<Location> {
        let mut result = Vec::new();

        let height = self.map.len() as isize;
        let width = self.map[0].len() as isize;

        let right_row = right.row as isize;
        let right_col = right.col as isize;

        let left_row = left.row as isize;
        let left_col = left.col as isize;

        let row_delta = right_row - left_row;
        let col_delta = right_col - left_col;

        let first_row = right_row + row_delta;
        let first_col = right_col + col_delta;

        if first_row >= 0 && first_row < height && first_col >= 0 && first_col < width {
            result.push(Location {
                row: first_row as usize,
                col: first_col as usize,
            });
        }

        let second_row = left_row - row_delta;
        let second_col = left_col - col_delta;

        if second_row >= 0 && second_row < height && second_col >= 0 && second_col < width {
            result.push(Location {
                row: second_row as usize,
                col: second_col as usize,
            });
        }
        result
    }

    pub fn distinct_antinodes(&self) -> u64 {
        let mut antinodes: HashSet<Location> = HashSet::new();

        for antenna in self.antennas.keys() {
            let start_locations = self.antennas.get(antenna).unwrap().iter().enumerate();
            for (idx, left) in start_locations {
                let end_locations = self.antennas.get(antenna).unwrap()[idx + 1..].iter();
                for right in end_locations {
                    antinodes.extend(self.calculate_antinodes(left, right));
                }
            }
        }

        antinodes.len() as u64
    }
}
