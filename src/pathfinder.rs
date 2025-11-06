use std::collections::HashSet;

#[derive(Debug)]
pub struct Pathfinder<'a> {
    map: &'a mut Vec<Vec<char>>,
    path: HashSet<PathState>,
    location: Location,
    direction: Direction,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct PathState {
    loc: Location,
    dir: Direction,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
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

#[derive(PartialEq, Debug)]
enum PathType {
    Loop,
    Terminate,
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

pub fn build_pathfinder(map: &mut Vec<Vec<char>>) -> Pathfinder<'_> {
    let (location, direction) = find_start(map);
    _build_pathfinder(map, location, direction)
}

fn _build_pathfinder(
    map: &mut Vec<Vec<char>>,
    location: Location,
    direction: Direction,
) -> Pathfinder<'_> {
    let state = PathState {
        loc: location.clone(),
        dir: direction.clone(),
    };
    let path: HashSet<PathState> = HashSet::from([state]);
    Pathfinder {
        map,
        path,
        location,
        direction,
    }
}

impl Pathfinder<'_> {
    pub fn distinct_positions(&mut self) -> u64 {
        let path_type = self.populate_path();
        if path_type == PathType::Loop {
            panic!("Unexpected looping path found");
        }
        self.path
            .iter()
            .map(|s| &s.loc)
            .collect::<HashSet<_>>()
            .len() as u64
    }

    pub fn distinct_obstacles(&mut self) -> u64 {
        // let blocked_location = self.location.clone();
        let count = 0;

        while !self.is_path_end() {
            let potential_next = self.get_next_location();
            if self.is_obstacle(&potential_next) {
                self.rotate_direction();
            } else {
                // spawn new pathfinder
                self.location = potential_next;
            }

            let state = PathState {
                loc: self.location.clone(),
                dir: self.direction.clone(),
            };
            if self.path.contains(&state) {
                // return PathType::Loop;
            }
            self.path.insert(state);
        }
        count
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

    fn populate_path(&mut self) -> PathType {
        while !self.is_path_end() {
            let potential_next = self.get_next_location();
            if self.is_obstacle(&potential_next) {
                self.rotate_direction();
            } else {
                self.location = potential_next;
            }

            let state = PathState {
                loc: self.location.clone(),
                dir: self.direction.clone(),
            };
            if self.path.contains(&state) {
                return PathType::Loop;
            }
            self.path.insert(state);
        }
        PathType::Terminate
    }
}
