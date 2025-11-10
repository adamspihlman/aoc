use crate::grid;

#[derive(Debug)]
pub struct Security {
    robots: Vec<Robot>,
    num_rows: usize,
    num_cols: usize,
}

#[derive(Debug, Default)]
pub struct SecurityBuilder {
    robots: Vec<Robot>,
    num_rows: usize,
    num_cols: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Robot {
    pos: grid::Location,
    v: Velocity,
}

#[derive(Debug, Clone, Copy)]
pub struct Velocity {
    pub col_v: i32,
    pub row_v: i32,
}

impl From<(grid::Location, Velocity)> for Robot {
    fn from(value: (grid::Location, Velocity)) -> Self {
        let (pos, v) = value;
        Self { pos, v }
    }
}

impl SecurityBuilder {
    pub fn robots(mut self, robots: Vec<Robot>) -> Self {
        self.robots = robots;
        self
    }

    pub fn num_rows(mut self, num_rows: usize) -> Self {
        self.num_rows = num_rows;
        self
    }

    pub fn num_cols(mut self, num_cols: usize) -> Self {
        self.num_cols = num_cols;
        self
    }

    pub fn build(&self) -> Security {
        Security {
            robots: self.robots.clone(),
            num_rows: self.num_rows,
            num_cols: self.num_cols,
        }
    }
}
