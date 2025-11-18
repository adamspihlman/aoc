use advent_of_code::keypad::Keypad;

advent_of_code::solution!(21);

fn parse_input(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let keypad = Keypad::new();
    let codes = parse_input(input);
    let depth = 2;

    let sum = codes.iter().map(|code| keypad.complexity(code, depth)).sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let keypad = Keypad::new();
    let codes = parse_input(input);
    let depth = 25;

    let sum = codes.iter().map(|code| keypad.complexity(code, depth)).sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_one_solution() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(156714));
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
