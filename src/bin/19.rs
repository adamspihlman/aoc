use advent_of_code::towel::{Pattern, Towel};

advent_of_code::solution!(19);

fn parse_input(input: &str) -> (Vec<Pattern>, Vec<Towel>) {
    let mut lines = input.lines();

    let patterns: Vec<Pattern> = lines
        .next()
        .unwrap()
        .split(", ")
        .map(|s| Pattern::from(s.to_string()))
        .collect();

    lines.next(); // skip empty line

    let towels: Vec<Towel> = lines
        .map(|line| Towel::from(line.to_string()))
        .collect();

    (patterns, towels)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (patterns, towels) = parse_input(input);

    let result = towels
        .iter()
        .filter(|towel| towel.is_possible(&patterns))
        .count() as u64;

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (patterns, towels) = parse_input(input);

    let result = towels
        .iter()
        .map(|towel| towel.count_possibilities(&patterns))
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_one_solution() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(228));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
