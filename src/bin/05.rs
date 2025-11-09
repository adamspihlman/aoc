advent_of_code::solution!(5);

fn parse_input_rules(input: &str) -> Vec<(u64, u64)> {
    let mut result: Vec<(u64, u64)> = Vec::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            break;
        }

        let mut iter = line.split('|');
        let before = iter.next().unwrap().parse::<u64>().unwrap();
        let after = iter.next().unwrap().parse::<u64>().unwrap();
        result.push((before, after));
    }

    result
}

fn parse_input_updates(input: &str) -> Vec<Vec<u64>> {
    let mut result: Vec<Vec<u64>> = Vec::new();
    let mut is_past_rules = false;

    for line in input.lines() {
        if line.trim().is_empty() {
            is_past_rules = true;
            continue;
        } else if !is_past_rules {
            continue;
        }

        let update: Vec<u64> = line.split(',').map(|d| d.parse::<u64>().unwrap()).collect();
        result.push(update);
    }

    result
}

pub fn part_one(input: &str) -> Option<u64> {
    let rules = advent_of_code::order_rules::Rules::from(parse_input_rules(input));
    let updates = parse_input_updates(input);
    let sum = updates
        .into_iter()
        .filter(|u| rules.is_valid(u))
        .map(|u| u[u.len() / 2])
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let rules = advent_of_code::order_rules::Rules::from(parse_input_rules(input));
    let mut updates = parse_input_updates(input);
    updates.retain(|u| !rules.is_valid(u));

    let fixed: Vec<Vec<u64>> = updates.iter().map(|u| rules.make_valid(u)).collect();
    let sum = fixed.into_iter().map(|u| u[u.len() / 2]).sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }

    #[test]
    fn test_part_one_solution() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(6951));
    }

    #[test]
    fn test_part_two_solution() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(4121));
    }
}
