advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let words =
        advent_of_code::word_search::build_words(advent_of_code::input::parse_2d_vector(input));

    Some(words.word_search())
}

pub fn part_two(input: &str) -> Option<u64> {
    let words =
        advent_of_code::word_search::build_words(advent_of_code::input::parse_2d_vector(input));

    Some(words.x_search())
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
}
