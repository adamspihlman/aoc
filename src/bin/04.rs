advent_of_code::solution!(4);

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let words = advent_of_code::word_search::build_words(parse_input(input));

    Some(words.word_search())
}

pub fn part_two(input: &str) -> Option<u64> {
    let words = advent_of_code::word_search::build_words(parse_input(input));

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
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(9));
    }
}
