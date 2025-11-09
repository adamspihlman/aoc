advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let map = advent_of_code::input::parse_2d_vector(input);
    let mut pathfinder = advent_of_code::pathfinder::Pathfinder::from(&map);
    let result = pathfinder.distinct_positions();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = advent_of_code::input::parse_2d_vector(input);
    let mut pathfinder = advent_of_code::pathfinder::Pathfinder::from(&map);
    let result = pathfinder.distinct_obstacles();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_one_solution() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(4752));
    }

    #[test]
    #[should_panic]
    fn test_part_one_loop_panic() {
        part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
    }

    #[test]
    fn test_part_two_default() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two_solution() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1719));
    }

    #[test]
    fn test_part_two_one_obstacle() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(1));
    }

    #[test]
    #[should_panic]
    fn test_part_two_loop_panic() {
        part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
    }
}
