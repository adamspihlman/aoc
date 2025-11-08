#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
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
