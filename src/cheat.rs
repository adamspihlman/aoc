use crate::grid::print_map;

#[derive(Debug)]
pub struct Cheat {
    grid: Vec<Vec<char>>,
}

impl From<Vec<Vec<char>>> for Cheat {
    fn from(grid: Vec<Vec<char>>) -> Self {
        Self { grid }
    }
}

impl Cheat {
    pub fn print(&self) {
        print_map(&self.grid);
    }
}
