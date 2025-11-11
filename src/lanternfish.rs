use crate::grid;

#[derive(Debug)]
pub struct Lanternfish {
    warehouse: Vec<Vec<char>>,
    robot_steps: Vec<grid::Direction>,
}

impl From<(Vec<Vec<char>>, Vec<grid::Direction>)> for Lanternfish {
    fn from(value: (Vec<Vec<char>>, Vec<grid::Direction>)) -> Self {
        let (warehouse, robot_steps) = value;
        Self {
            warehouse,
            robot_steps,
        }
    }
}
