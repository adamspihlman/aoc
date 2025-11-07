advent_of_code::solution!(7);

use advent_of_code::equation::Equation;

fn parse_input(input: &str) -> Vec<Equation> {
    input.lines().map(Equation::from).collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let equations = parse_input(input);
    let ops = vec![
        advent_of_code::equation::Operator::Add,
        advent_of_code::equation::Operator::Multiply,
    ];
    let result = equations
        .into_iter()
        .filter(|e| e.is_solvable(&ops))
        .map(|e| e.get_result())
        .sum();
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
