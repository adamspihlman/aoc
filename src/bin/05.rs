advent_of_code::solution!(5);

fn parse_input(input: &str) -> Vec<(u64, u64)> {
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

pub fn part_one(input: &str) -> Option<u64> {
    let rules = advent_of_code::order_rules::build_rules(parse_input(input));
    println!("{:?}", rules);
    // rules.print();
    None
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
