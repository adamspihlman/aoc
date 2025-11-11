use std::collections::HashMap;

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

impl Security {
    pub fn elapse(&mut self, time: usize) {
        self.robots
            .iter_mut()
            .for_each(|r| r.move_location(self.num_rows, self.num_cols, time));
    }

    pub fn safety_factor(&self) -> u64 {
        let mut quadrant_counts: HashMap<i32, u64> =
            HashMap::from([(1, 0), (2, 0), (3, 0), (4, 0)]);

        self.robots.iter().for_each(|r| {
            if let Some(quadrant) = r.quadrant(self.num_rows, self.num_cols) {
                *quadrant_counts.entry(quadrant).or_insert(0) += 1;
            }
        });

        quadrant_counts.values().product::<u64>()
    }
}

impl Robot {
    pub fn move_location(&mut self, num_rows: usize, num_cols: usize, time: usize) {
        self.pos.row = Robot::get_final_dimension(self.pos.row, num_rows, self.v.row_v, time);
        self.pos.col = Robot::get_final_dimension(self.pos.col, num_cols, self.v.col_v, time);
    }

    pub fn quadrant(&self, num_rows: usize, num_cols: usize) -> Option<i32> {
        let mid_row = num_rows / 2;
        let mid_col = num_cols / 2;

        match (self.pos.row.cmp(&mid_row), self.pos.col.cmp(&mid_col)) {
            (_, std::cmp::Ordering::Equal) => None,
            (std::cmp::Ordering::Equal, _) => None,
            (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => Some(1),
            (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => Some(2),
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => Some(3),
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => Some(4),
        }
    }

    fn get_final_dimension(
        initial: usize,
        dimension_size: usize,
        velocity: i32,
        time: usize,
    ) -> usize {
        let offset = (time as i32 * velocity).unsigned_abs() as usize % dimension_size;
        if velocity < 0 {
            return initial + if offset > initial { dimension_size } else { 0 } - offset;
        }
        initial + offset
            - if initial + offset >= dimension_size {
                dimension_size
            } else {
                0
            }
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
