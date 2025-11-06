advent_of_code::solution!(6);

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut map = parse_input(input);
    let mut pathfinder = advent_of_code::pathfinder::build_pathfinder(&mut map);
    let result = pathfinder.distinct_positions();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut map = parse_input(input);
    let mut pathfinder = advent_of_code::pathfinder::build_pathfinder(&mut map);
    let result = pathfinder.distinct_positions();
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
    #[should_panic]
    fn test_part_one_loop_panic() {
        part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
    }

    #[test]
    #[ignore]
    fn test_part_two_default() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    #[ignore]
    fn test_part_two_one_obstacle() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(1));
    }
}
