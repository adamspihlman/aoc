advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let words =
        advent_of_code::word_search::Words::from(advent_of_code::input::parse_2d_vector(input));

    let result = words.word_search();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let words =
        advent_of_code::word_search::Words::from(advent_of_code::input::parse_2d_vector(input));

    let result = words.x_search();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_one_solution() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(2397));
    }

    #[test]
    fn test_part_two() {
        let result_1 = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result_1, Some(9));
        let result_2 = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result_2, Some(0));
    }

    #[test]
    fn test_part_two_solution() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1824));
    }
}
