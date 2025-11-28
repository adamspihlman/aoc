use std::collections::VecDeque;

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
        let robot_position = grid::find_only(&value, '@');
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
        match self.layout {
            Layout::Standard => self.move_robot_standard(direction),
            Layout::Wide => self.move_robot_wide(direction),
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

    fn write_robot_move(&mut self, direction: grid::Direction) -> (grid::Location, char) {
        self.warehouse[self.robot_position.row][self.robot_position.col] = '.';
        let next = grid::get_location(&self.warehouse, self.robot_position, direction)
            .expect("robot move should be in bounds");
        self.robot_position = next;
        let overwritten = grid::at(&self.warehouse, next);
        self.warehouse[self.robot_position.row][self.robot_position.col] = '@';
        (next, overwritten)
    }

    fn write_horizontal_box_move(&mut self, start: grid::Location, direction: grid::Direction) {
        let empty_location = self
            .find_empty_space(start, direction)
            .expect("should have empty space for box move");
        let mut first_box_half = start;
        let (first_box, second_box) = if direction == grid::Direction::Left {
            (']', '[')
        } else {
            ('[', ']')
        };

        let num_boxes = empty_location.col.abs_diff(first_box_half.col).div_ceil(2);
        for _ in 0..num_boxes {
            let second_box_half = grid::get_location(&self.warehouse, first_box_half, direction)
                .expect("box half should be in bounds");

            self.warehouse[first_box_half.row][first_box_half.col] = first_box;
            self.warehouse[second_box_half.row][second_box_half.col] = second_box;

            first_box_half = grid::get_location(&self.warehouse, second_box_half, direction)
                .expect("next box half should be in bounds");
        }
    }

    fn write_vertical_box_moves(
        &mut self,
        direction: grid::Direction,
        mut boxes: VecDeque<grid::Location>,
    ) {
        while let Some(left_box_half) = boxes.pop_front() {
            let new_left = grid::get_location(&self.warehouse, left_box_half, direction)
                .expect("vertical box move should be in bounds");
            let new_right = grid::get_location(&self.warehouse, new_left, grid::Direction::Right)
                .expect("right box half should be in bounds");

            let overwritten_left = grid::at(&self.warehouse, new_left);
            let overwritten_right = grid::at(&self.warehouse, new_right);

            if overwritten_left == '[' {
                boxes.push_back(new_left);
                continue;
            } else if overwritten_left == ']' {
                let displaced_left =
                    grid::get_location(&self.warehouse, new_left, grid::Direction::Left)
                        .expect("displaced left should be in bounds");
                self.warehouse[displaced_left.row][displaced_left.col] = '.';
                self.warehouse[new_left.row][new_left.col] = '[';
                boxes.push_back(displaced_left);
            } else if overwritten_left == '.' {
                self.warehouse[new_left.row][new_left.col] = '[';
            }
            if overwritten_right == '[' {
                let displaced_right =
                    grid::get_location(&self.warehouse, new_right, grid::Direction::Right)
                        .expect("displaced right should be in bounds");
                self.warehouse[displaced_right.row][displaced_right.col] = '.';
                self.warehouse[new_right.row][new_right.col] = ']';
                boxes.push_back(new_right);
            } else if overwritten_right == '.' {
                self.warehouse[new_right.row][new_right.col] = ']';
            }
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
            let start = grid::get_location(&self.warehouse, next, direction)
                .expect("horizontal move should be in bounds");
            self.write_horizontal_box_move(start, direction);
            return;
        }

        let left_box_half = if overwritten == '[' {
            next
        } else {
            grid::get_location(&self.warehouse, next, grid::Direction::Left)
                .expect("left box half should be in bounds")
        };
        let empty_space = if overwritten == '[' {
            grid::get_location(&self.warehouse, next, grid::Direction::Right)
                .expect("right side should be in bounds")
        } else {
            left_box_half
        };
        self.warehouse[empty_space.row][empty_space.col] = '.';
        let boxes: VecDeque<grid::Location> = VecDeque::from([left_box_half]);
        self.write_vertical_box_moves(direction, boxes);
    }

    fn move_robot_standard(&mut self, direction: grid::Direction) {
        let Some(empty_space) = self.find_empty_space(self.robot_position, direction) else {
            return;
        };

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

        let next = grid::get_location(&self.warehouse, location, direction)
            .expect("can_move check should be in bounds");
        let c = grid::at(&self.warehouse, next);

        match c {
            '.' => true,
            '[' => {
                let right_wall =
                    grid::get_location(&self.warehouse, next, grid::Direction::Right)
                        .expect("right side of box should be in bounds");
                self.can_move(right_wall, direction) && self.can_move(next, direction)
            }
            ']' => {
                let left_wall = grid::get_location(&self.warehouse, next, grid::Direction::Left)
                    .expect("left side of box should be in bounds");
                self.can_move(left_wall, direction) && self.can_move(next, direction)
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
            let next = grid::get_location(&self.warehouse, position, direction)
                .expect("search for empty space should stay in bounds");

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
}
