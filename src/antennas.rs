use std::collections::{HashMap, HashSet};

use crate::grid::Location;

#[derive(Debug)]
pub struct Antennas {
    height: isize,
    width: isize,
    antennas: HashMap<char, Vec<Location>>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Antinode {
    Resonant,
    Harmonic,
}

impl Antennas {
    pub fn from(map: &[Vec<char>]) -> Antennas {
        let height = map.len() as isize;
        let width = map[0].len() as isize;
        let antennas = Antennas::find_antennas(map);
        Antennas {
            height,
            width,
            antennas,
        }
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

    fn is_valid_location(&self, row: isize, col: isize) -> bool {
        row >= 0 && row < self.height && col >= 0 && col < self.width
    }

    fn calculate_resonant_antinodes(&self, left: &Location, right: &Location) -> Vec<Location> {
        let mut result = Vec::new();

        let left_row = left.row as isize;
        let left_col = left.col as isize;

        let right_row = right.row as isize;
        let right_col = right.col as isize;

        let row_delta = right_row - left_row;
        let col_delta = right_col - left_col;

        let first_row = right_row + row_delta;
        let first_col = right_col + col_delta;

        if self.is_valid_location(first_row, first_col) {
            result.push(Location {
                row: first_row as usize,
                col: first_col as usize,
            });
        }

        let second_row = left_row - row_delta;
        let second_col = left_col - col_delta;

        if self.is_valid_location(second_row, second_col) {
            result.push(Location {
                row: second_row as usize,
                col: second_col as usize,
            });
        }
        result
    }

    fn calculate_harmonic_antinodes(&self, left: &Location, right: &Location) -> Vec<Location> {
        let mut result = Vec::new();

        let left_row = left.row as isize;
        let left_col = left.col as isize;

        let right_row = right.row as isize;
        let right_col = right.col as isize;

        let mut row_delta = right_row - left_row;
        let mut col_delta = right_col - left_col;

        let gcd = num_integer::gcd(row_delta, col_delta);
        row_delta /= gcd;
        col_delta /= gcd;

        let mut antinode_row = left_row;
        let mut antinode_col = left_col;
        while self.is_valid_location(antinode_row, antinode_col) {
            result.push(Location {
                row: antinode_row as usize,
                col: antinode_col as usize,
            });
            antinode_row += row_delta;
            antinode_col += col_delta;
        }
        antinode_row = left_row - row_delta;
        antinode_col = left_col - col_delta;
        while self.is_valid_location(antinode_row, antinode_col) {
            result.push(Location {
                row: antinode_row as usize,
                col: antinode_col as usize,
            });
            antinode_row -= row_delta;
            antinode_col -= col_delta;
        }

        result
    }

    fn calculate_antinodes(
        &self,
        left: &Location,
        right: &Location,
        antinode: Antinode,
    ) -> Vec<Location> {
        match antinode {
            Antinode::Resonant => self.calculate_resonant_antinodes(left, right),
            Antinode::Harmonic => self.calculate_harmonic_antinodes(left, right),
        }
    }

    pub fn distinct_antinodes(&self, antinode: Antinode) -> u64 {
        let mut antinodes: HashSet<Location> = HashSet::new();

        for antenna in self.antennas.keys() {
            let start_locations = self.antennas.get(antenna).unwrap();
            for (idx, left) in start_locations.iter().enumerate() {
                for right in start_locations[idx + 1..].iter() {
                    antinodes.extend(self.calculate_antinodes(left, right, antinode));
                }
            }
        }

        antinodes.len() as u64
    }
}
