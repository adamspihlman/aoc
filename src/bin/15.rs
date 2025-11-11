use advent_of_code::grid;

advent_of_code::solution!(15);

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<grid::Direction>) {
    let map_end = input.find("\n\n").unwrap();
    let map = advent_of_code::input::parse_2d_vector(&input[0..map_end]);

    let mut steps = input[map_end..input.len()].to_string();
    steps.retain(|c| !c.is_whitespace());
    let steps: Vec<grid::Direction> = steps.chars().map(grid::to_direction).collect();

    (map, steps)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (warehouse, robot_steps) = parse_input(input);
    let mut lanternfish = advent_of_code::lanternfish::Lanternfish::from(warehouse);

    robot_steps.iter().for_each(|&d| lanternfish.move_robot(d));
    let result = lanternfish.gps_sum();

    Some(result)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_one_simple() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_one_solution() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1349898));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
