#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone, Ord, PartialOrd)]
pub struct Location {
    pub row: usize,
    pub col: usize,
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

pub fn print_map(map: &[Vec<char>]) {
    for row in map.iter() {
        let cols: String = row.iter().collect();
        println!("{cols}");
    }
}

pub fn to_direction(c: char) -> Direction {
    match c {
        '^' => Direction::Up,
        'v' => Direction::Down,
        '>' => Direction::Right,
        '<' => Direction::Left,
        _ => panic!("Character {c} is not a direction!"),
    }
}

pub fn is_horizontal(direction: Direction) -> bool {
    direction == Direction::Left || direction == Direction::Right
}

pub fn is_vertical(direction: Direction) -> bool {
    !is_horizontal(direction)
}

pub fn rotate_direction(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

pub fn at<T: Copy>(map: &[Vec<T>], location: Location) -> T {
    map[location.row][location.col]
}

pub fn get_location<T>(
    map: &[Vec<T>],
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
