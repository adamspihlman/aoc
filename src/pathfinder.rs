use crate::grid::{self, Direction, Location};
use std::collections::{HashMap, HashSet};
use std::sync::mpsc;
use std::thread;

#[derive(Debug)]
pub struct Pathfinder<'a> {
    map: &'a Vec<Vec<char>>,
    path: HashMap<Location, HashSet<Direction>>,
    location: Location,
    start_location: Location,
    direction: Direction,
    start_direction: Direction,
    extra_obstacle: Option<Location>,
}

#[derive(PartialEq, Debug)]
enum PathType {
    Loop,
    Terminate,
}

impl<'a> From<&'a Vec<Vec<char>>> for Pathfinder<'a> {
    fn from(value: &'a Vec<Vec<char>>) -> Self {
        let (location, direction) = Pathfinder::find_start(value);
        Pathfinder::build_pathfinder(value, location, direction, None)
    }
}

impl Pathfinder<'_> {
    pub fn distinct_positions(&mut self) -> u64 {
        let path_type = self.populate_path();
        if path_type == PathType::Loop {
            panic!("Unexpected looping path found");
        }
        self.path.len() as u64
    }

    pub fn distinct_obstacles(&mut self) -> u64 {
        thread::scope(|s| {
            let (tx, rx) = mpsc::channel();
            while !self.is_path_end() {
                let potential_next = self.get_next_location();
                if grid::at(self.map, potential_next) == '.'
                    && !self.path.contains_key(&potential_next)
                {
                    let mut subpathfinder = Pathfinder::build_pathfinder(
                        self.map,
                        self.start_location,
                        self.start_direction,
                        Some(potential_next),
                    );
                    let tx_clone = tx.clone();
                    s.spawn(move || {
                        let pathtype = subpathfinder.populate_path();
                        tx_clone.send(pathtype).unwrap();
                    });
                }

                if self.is_obstacle(potential_next) {
                    self.direction = crate::grid::rotate_cw(self.direction);
                } else {
                    self.location = potential_next;
                }

                if self.path.contains_key(&self.location)
                    && self
                        .path
                        .get(&self.location)
                        .unwrap()
                        .contains(&self.direction)
                {
                    panic!("Found unexpected looping path");
                }
                self.path
                    .entry(self.location)
                    .or_default()
                    .insert(self.direction);
            }
            drop(tx);
            rx.iter().filter(|t| *t == PathType::Loop).count() as u64
        })
    }

    pub(self) fn populate_path(&mut self) -> PathType {
        while !self.is_path_end() {
            let potential_next = self.get_next_location();
            if self.is_obstacle(potential_next) {
                self.direction = crate::grid::rotate_cw(self.direction);
            } else {
                self.location = potential_next;
            }

            if self.path.contains_key(&self.location)
                && self
                    .path
                    .get(&self.location)
                    .unwrap()
                    .contains(&self.direction)
            {
                return PathType::Loop;
            }
            self.path
                .entry(self.location)
                .or_default()
                .insert(self.direction);
        }
        PathType::Terminate
    }

    fn build_pathfinder(
        map: &Vec<Vec<char>>,
        location: Location,
        direction: Direction,
        extra_obstacle: Option<Location>,
    ) -> Pathfinder<'_> {
        let mut path: HashMap<Location, HashSet<Direction>> = HashMap::new();
        path.insert(location, HashSet::from([direction]));
        Pathfinder {
            map,
            path,
            location,
            start_location: location,
            direction,
            start_direction: direction,
            extra_obstacle,
        }
    }

    fn find_start(map: &[Vec<char>]) -> (Location, Direction) {
        for (row_idx, row_val) in map.iter().enumerate() {
            for (col_idx, col_val) in row_val.iter().enumerate() {
                let location = Location {
                    row: row_idx,
                    col: col_idx,
                };
                match col_val {
                    '^' => return (location, Direction::Up),
                    '>' => return (location, Direction::Right),
                    '<' => return (location, Direction::Left),
                    'v' => return (location, Direction::Down),
                    _ => continue,
                }
            }
        }

        panic!("Failed to find start location in map");
    }

    fn is_path_end(&self) -> bool {
        (self.direction == Direction::Up && self.location.row == 0)
            || (self.direction == Direction::Left && self.location.col == 0)
            || (self.direction == Direction::Down && self.location.row == (self.map.len() - 1))
            || (self.direction == Direction::Right && self.location.col == (self.map[0].len() - 1))
    }

    fn get_next_location(&self) -> Location {
        let cur = &self.location;
        match self.direction {
            Direction::Up => Location {
                row: cur.row - 1,
                col: cur.col,
            },
            Direction::Down => Location {
                row: cur.row + 1,
                col: cur.col,
            },
            Direction::Left => Location {
                row: cur.row,
                col: cur.col - 1,
            },
            Direction::Right => Location {
                row: cur.row,
                col: cur.col + 1,
            },
        }
    }

    fn is_obstacle(&self, location: Location) -> bool {
        grid::at(self.map, location) == '#'
            || match self.extra_obstacle {
                Some(loc) => loc == location,
                None => false,
            }
    }
}
