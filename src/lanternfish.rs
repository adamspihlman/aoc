use crate::grid;

#[derive(Debug)]
pub struct Lanternfish {
    warehouse: Vec<Vec<char>>,
    layout: Layout,
    robot_position: grid::Location,
}

#[derive(Debug, PartialEq)]
enum Layout {
    Standard,
    Wide,
}

impl From<Vec<Vec<char>>> for Lanternfish {
    fn from(value: Vec<Vec<char>>) -> Self {
        let robot_position = Lanternfish::find_robot(&value);
        let layout = Lanternfish::layout(&value);
        Self {
            warehouse: value,
            layout,
            robot_position,
        }
    }
}

impl Lanternfish {
    pub fn move_robot(&mut self, direction: grid::Direction) {
        grid::print_map(&self.warehouse);
        println!("-------------------------");
        match self.layout {
            Layout::Standard => self.move_robot_standard(direction),
            Layout::Wide => self.move_robot_wide(direction),
        }
        println!("Moving {:?}", direction);
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

    fn write_robot_move(&mut self, direction: grid::Direction) -> (grid::Location, char) {
        self.warehouse[self.robot_position.row][self.robot_position.col] = '.';
        let next = grid::get_location(&self.warehouse, self.robot_position, direction).unwrap();
        self.robot_position = next;
        let overwritten = grid::at(&self.warehouse, next);
        self.warehouse[self.robot_position.row][self.robot_position.col] = '@';
        (next, overwritten)
    }

    fn write_horizontal_box_move(&mut self, start: grid::Location, direction: grid::Direction) {
        let empty_location = self.find_empty_space(start, direction).unwrap();
        let mut first_box_half = start;
        let (first_box, second_box) = if direction == grid::Direction::Left {
            (']', '[')
        } else {
            ('[', ']')
        };

        let num_boxes = empty_location.col.abs_diff(first_box_half.col).div_ceil(2);
        for _ in 0..num_boxes {
            let second_box_half =
                grid::get_location(&self.warehouse, first_box_half, direction).unwrap();

            self.warehouse[first_box_half.row][first_box_half.col] = first_box;
            self.warehouse[second_box_half.row][second_box_half.col] = second_box;

            first_box_half =
                grid::get_location(&self.warehouse, second_box_half, direction).unwrap();
        }
    }

    fn move_robot_wide(&mut self, direction: grid::Direction) {
        if !self.can_move(self.robot_position, direction) {
            return;
        }

        let (next, overwritten) = self.write_robot_move(direction);
        if overwritten == '.' {
            return;
        }

        if grid::is_horizontal(direction) {
            let start = grid::get_location(&self.warehouse, next, direction).unwrap();
            self.write_horizontal_box_move(start, direction);
            return;
        }

        panic!("Refusing vertical box move");
        // match overwritten {
        //     '[' => {
        //         self.move_wide_box(next, direction);
        //     }
        //     ']' => {
        //         let left_wall =
        //             grid::get_location(&self.warehouse, next, grid::Direction::Left).unwrap();
        //         self.move_wide_box(left_wall, direction);
        //     }
        //     e => panic!("Unexpected map character {e} found"),
        // }
    }

    fn move_robot_standard(&mut self, direction: grid::Direction) {
        let empty_space = self.find_empty_space(self.robot_position, direction);
        if empty_space.is_none() {
            return;
        }
        let empty_space = empty_space.unwrap();

        let (_, overwritten) = self.write_robot_move(direction);

        match overwritten {
            'O' => {
                self.warehouse[empty_space.row][empty_space.col] = 'O';
            }
            '.' => {}
            '#' => {
                panic!("Hit a wall when moving robot");
            }
            c => {
                panic!("Unexpected map character '{c}' found");
            }
        }
    }

    fn can_move(&self, location: grid::Location, direction: grid::Direction) -> bool {
        if self.find_empty_space(location, direction).is_none() {
            return false;
        } else if grid::is_horizontal(direction) {
            return true;
        }

        let next = grid::get_location(&self.warehouse, location, direction).unwrap();
        let c = grid::at(&self.warehouse, next);

        match c {
            '.' => true,
            '[' => {
                let right_wall =
                    grid::get_location(&self.warehouse, next, grid::Direction::Right).unwrap();
                self.can_move(right_wall, direction)
            }
            ']' => {
                let left_wall =
                    grid::get_location(&self.warehouse, next, grid::Direction::Left).unwrap();
                self.can_move(left_wall, direction)
            }
            e => panic!("Unexpected map character '{e}' found"),
        }
    }

    fn find_empty_space(
        &self,
        location: grid::Location,
        direction: grid::Direction,
    ) -> Option<grid::Location> {
        let mut position = location;
        loop {
            let next = grid::get_location(&self.warehouse, position, direction).unwrap();

            match grid::at(&self.warehouse, next) {
                '.' => {
                    return Some(next);
                }
                '#' => {
                    return None;
                }
                'O' | '[' | ']' => {
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
            'O' | '[' => (location.row * 100 + location.col) as u64,
            _ => 0,
        }
    }

    fn layout(warehouse: &[Vec<char>]) -> Layout {
        for row in warehouse.iter() {
            for &col in row.iter() {
                match col {
                    'O' => return Layout::Standard,
                    '[' => return Layout::Wide,
                    _ => {}
                }
            }
        }
        panic!("No boxes found in warehouse");
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

    // fn move_wide_box(&mut self, location: grid::Location, direction: grid::Direction) {
    //     // I think we need different functions for moving a box to the left/right vs moving a box
    //     //   up/down. We also need to be careful about when we modify the map vs when we read the
    //     //   map to determine if we need to make another call to move_wide_box
    //     //
    //     let new_left = grid::get_location(&self.warehouse, location, direction).unwrap();
    //     let old_right =
    //         grid::get_location(&self.warehouse, location, grid::Direction::Right).unwrap();
    //     let new_right = grid::get_location(&self.warehouse, old_right, direction).unwrap();
    //
    //     let new_left_val = grid::at(&self.warehouse, new_left);
    //     let new_right_val = grid::at(&self.warehouse, new_right);
    //
    //     // if new_left_val
    // }
}
