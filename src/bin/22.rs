use advent_of_code::secret;

advent_of_code::solution!(22);

fn parse_input(input: &str) -> Vec<u64> {
    input.lines().map(|l| l.parse::<u64>().unwrap()).collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let seeds = parse_input(input);
    let result = seeds.iter().map(|&s| secret::generate(s, 2000)).sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    use advent_of_code::secret::{combine_sequence_maps, max_price, Secret};
    use std::collections::HashMap;

    let seeds = parse_input(input);

    let combined_map = seeds
        .iter()
        .map(|&seed| Secret::from(seed).generate_sequence_map(2000))
        .fold(HashMap::new(), combine_sequence_maps);

    max_price(&combined_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_one_solution() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(17005483322));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(23));
    }

    #[test]
    fn test_part_two_solution() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1910));
    }
}
