advent_of_code::solution!(1);

use advent_of_code::input::{count_frequencies, parse_pairs};

pub fn part_one(input: &str) -> Option<u64> {
    let (mut left_list, mut right_list): (Vec<u64>, Vec<u64>) = parse_pairs(input);

    left_list.sort();
    right_list.sort();

    let result = left_list
        .iter()
        .zip(&right_list)
        .map(|(l, r)| l.abs_diff(*r))
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (left_list, right_list): (Vec<u64>, Vec<u64>) = parse_pairs(input);
    let freq_map = count_frequencies(&right_list);

    let result = left_list
        .iter()
        .map(|item| item * freq_map.get(item).unwrap_or(&0))
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }

    #[test]
    fn test_part_one_solution() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1506483));
    }

    #[test]
    fn test_part_two_solution() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(23126924));
    }
}
