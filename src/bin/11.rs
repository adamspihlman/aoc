advent_of_code::solution!(11);

fn parse_input(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|i| i.parse::<u64>().unwrap())
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let stones = parse_input(input);
    let mut stonemason = advent_of_code::stone::StoneMason::from(stones);

    stonemason.blink(25);

    let result = stonemason.get_num_stones();
    Some(result)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_one_solution() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(183620));
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
