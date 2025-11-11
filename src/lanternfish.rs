use crate::grid;

#[derive(Debug)]
pub struct Lanternfish {
    warehouse: Vec<Vec<char>>,
    robot_position: grid::Location,
}

impl From<Vec<Vec<char>>> for Lanternfish {
    fn from(value: Vec<Vec<char>>) -> Self {
        let robot_position = Lanternfish::find_robot(&value);
        Self {
            warehouse: value,
            robot_position,
        }
    }
}

impl Lanternfish {
    pub fn move_robot(&mut self, direction: grid::Direction) {
        let empty_space = self.find_empty_space(direction);
        if empty_space.is_none() {
            return;
        }
        let empty_space = empty_space.unwrap();

        self.warehouse[self.robot_position.row][self.robot_position.col] = '.';
        let next = grid::get_location(&self.warehouse, self.robot_position, direction).unwrap();
        self.robot_position = next;
        let overwritten = grid::at(&self.warehouse, next);
        self.warehouse[self.robot_position.row][self.robot_position.col] = '@';

        match overwritten {
            'O' => {
                self.warehouse[empty_space.row][empty_space.col] = 'O';
            }
            '.' => {}
            '#' => {
                panic!("Hit a wall when moving robot");
            }
            c => {
                panic!("Unexpected map character '{}' found", c);
            }
        }
    }

    pub fn gps_sum(&self) -> u64 {
        self.warehouse
            .iter()
            .enumerate()
            .map(|(row, row_val)| {
                row_val
                    .iter()
                    .enumerate()
                    .map(|(col, _)| self.gps_coordinate(grid::Location { row, col }))
                    .sum::<u64>()
            })
            .sum()
    }

    fn find_empty_space(&self, direction: grid::Direction) -> Option<grid::Location> {
        let mut position = self.robot_position;
        loop {
            let next = grid::get_location(&self.warehouse, position, direction).unwrap();

            match grid::at(&self.warehouse, next) {
                '.' => {
                    return Some(next);
                }
                '#' => {
                    return None;
                }
                'O' => {
                    position = next;
                }
                c => {
                    panic!("Unexpected map character '{}' found", c);
                }
            }
        }
    }

    fn gps_coordinate(&self, location: grid::Location) -> u64 {
        let c = grid::at(&self.warehouse, location);
        match c {
            'O' => (location.row * 100 + location.col) as u64,
            _ => 0,
        }
    }

    fn find_robot(warehouse: &[Vec<char>]) -> grid::Location {
        for (row, row_val) in warehouse.iter().enumerate() {
            for (col, &col_val) in row_val.iter().enumerate() {
                if col_val == '@' {
                    return grid::Location { row, col };
                }
            }
        }
        panic!("Robot not found");
    }
}
