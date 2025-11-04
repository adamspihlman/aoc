use std::collections::HashMap;

advent_of_code::solution!(1);

fn parse_lists(input: &str) -> (Vec<u64>, Vec<u64>) {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let left = parts.next().unwrap().parse::<u64>().unwrap();
            let right = parts.next().unwrap().parse::<u64>().unwrap();
            (left, right)
        })
        .unzip()
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut left_list, mut right_list) = parse_lists(input);

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
    let (left_list, right_list) = parse_lists(input);

    let mut freq_map: HashMap<u64, u64> = HashMap::new();

    for &item in &right_list {
        freq_map
            .entry(item)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

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
}
