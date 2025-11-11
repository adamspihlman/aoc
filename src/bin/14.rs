advent_of_code::solution!(14);

use regex::Regex;

fn parse_input(input: &str) -> Vec<advent_of_code::security::Robot> {
    let mut result = Vec::new();
    let re = Regex::new(r"^p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    for line in input.lines() {
        let (_, [col_pos, row_pos, col_v, row_v]) =
            re.captures_iter(line).next().unwrap().extract();

        let col_pos = col_pos.parse::<usize>().unwrap();
        let row_pos = row_pos.parse::<usize>().unwrap();
        let position = advent_of_code::grid::Location {
            row: row_pos,
            col: col_pos,
        };

        let col_v = col_v.parse::<i32>().unwrap();
        let row_v = row_v.parse::<i32>().unwrap();
        let velocity = advent_of_code::security::Velocity { col_v, row_v };

        result.push(advent_of_code::security::Robot::from((position, velocity)));
    }

    result
}

pub fn part_one(input: &str) -> Option<u64> {
    let robots = parse_input(input);
    let builder = advent_of_code::security::SecurityBuilder::default();
    let mut security = builder.robots(robots).num_rows(103).num_cols(101).build();
    security.elapse(100);
    let result = security.safety_factor();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let robots = parse_input(input);
    let builder = advent_of_code::security::SecurityBuilder::default();
    let mut security = builder.robots(robots).num_rows(103).num_cols(101).build();
    let result = security.find_image();
    // security.draw(); // uncomment to see the image!

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_solution() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(230461440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(6668));
    }
}
