advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u64> {
    let map = advent_of_code::input::parse_2d_digit_vector(input);
    let topograph = advent_of_code::topograph::Topograph::from(map);
    dbg!(topograph);
    None
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10));
    }

    #[test]
    #[ignore]
    fn test_part_one_score_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(1));
    }

    #[test]
    #[ignore]
    fn test_part_one_score_two() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(2));
    }

    #[test]
    #[ignore]
    fn test_part_one_two_trailheads() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(3));
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
