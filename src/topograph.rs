use std::collections::HashSet;

use crate::grid::{self, Direction, Location};

#[derive(Debug)]
pub struct Topograph {
    map: Vec<Vec<u32>>,
    heads: HashSet<Trailhead>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Trailhead {
    start: grid::Location,
}

impl From<Vec<Vec<u32>>> for Topograph {
    fn from(input: Vec<Vec<u32>>) -> Self {
        let heads = Topograph::get_trailheads(&input);
        Self { map: input, heads }
    }
}

impl Topograph {
    pub fn score(&self) -> u64 {
        self.heads.iter().map(|t| t.score(&self.map)).sum()
    }

    pub fn rating(&self) -> u64 {
        self.heads.iter().map(|t| t.rating(&self.map)).sum()
    }

    fn get_trailheads(map: &[Vec<u32>]) -> HashSet<Trailhead> {
        let mut result = HashSet::new();
        for (row, row_val) in map.iter().enumerate() {
            for (col, altitude) in row_val.iter().enumerate() {
                if altitude == &0 {
                    let start = grid::Location { row, col };
                    result.insert(Trailhead { start });
                }
            }
        }

        result
    }
}

impl Trailhead {
    pub fn score(&self, map: &[Vec<u32>]) -> u64 {
        let mut peaks = HashSet::new();
        Trailhead::get_peaks(map, &mut peaks, 0, self.start);

        peaks.len() as u64
    }

    pub fn rating(&self, map: &[Vec<u32>]) -> u64 {
        Trailhead::get_rating(map, 0, self.start)
    }

    fn get_location(
        map: &[Vec<u32>],
        location: Location,
        direction: Direction,
    ) -> Option<Location> {
        let mut row = location.row as isize;
        let mut col = location.col as isize;

        match direction {
            Direction::Up => {
                row -= 1;
            }
            Direction::Down => {
                row += 1;
            }
            Direction::Left => {
                col -= 1;
            }
            Direction::Right => {
                col += 1;
            }
        }
        if row < 0 || row >= map.len() as isize || col < 0 || col >= map[0].len() as isize {
            return None;
        }
        let result = Location {
            row: row as usize,
            col: col as usize,
        };
        Some(result)
    }

    fn get_rating(map: &[Vec<u32>], altitude: u32, location: Location) -> u64 {
        if altitude != map[location.row][location.col] {
            return 0;
        }

        if altitude == 9 {
            return 1;
        }

        let next_altitude = altitude + 1;

        let directions = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];

        directions
            .iter()
            .map(|&d| {
                let next = Trailhead::get_location(map, location, d);
                if let Some(next_location) = next {
                    return Trailhead::get_rating(map, next_altitude, next_location);
                }
                0
            })
            .sum()
    }

    fn get_peaks(
        map: &[Vec<u32>],
        peaks: &mut HashSet<Location>,
        altitude: u32,
        location: Location,
    ) {
        if altitude != map[location.row][location.col] {
            return;
        }

        if altitude == 9 {
            peaks.insert(location);
            return;
        }

        let next_altitude = altitude + 1;

        let directions = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];

        directions.iter().for_each(|&d| {
            let next = Trailhead::get_location(map, location, d);
            if let Some(next_location) = next {
                Trailhead::get_peaks(map, peaks, next_altitude, next_location);
            }
        });
    }
}
