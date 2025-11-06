use std::collections::HashSet;

#[derive(Debug)]
pub struct Pathfinder {
    map: Vec<Vec<char>>,
    path: HashSet<Location>,
    location: Location,
    direction: Direction,
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Location {
    row: usize,
    col: usize,
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

pub fn build_pathfinder(map: Vec<Vec<char>>) -> Pathfinder {
    let (location, direction) = find_start(&map);
    let mut path: HashSet<Location> = HashSet::new();
    path.insert(location.clone());
    Pathfinder {
        map,
        path,
        location,
        direction,
    }
}

impl Pathfinder {
    pub fn distinct_positions(&mut self) -> u64 {
        self.populate_path();
        self.path.len() as u64
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

    fn is_obstacle(&self, location: &Location) -> bool {
        self.map[location.row][location.col] == '#'
    }

    fn rotate_direction(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
    }

    fn populate_path(&mut self) {
        while !self.is_path_end() {
            let potential_next = self.get_next_location();
            if self.is_obstacle(&potential_next) {
                self.rotate_direction();
                continue;
            } else {
                self.location = potential_next;
                self.path.insert(self.location.clone());
                continue;
            }
            // get next location
            // check if location is obstacle
            //     if not obstacle, update location and go to next iteration of loop
            //     if obstacle, update direction and go to next iteration of loop
        }
    }
}
