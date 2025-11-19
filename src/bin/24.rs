use advent_of_code::logic::{Gate, Logic, Operation};
use std::collections::HashMap;

advent_of_code::solution!(24);

fn parse_input(input: &str) -> Logic {
    let sections: Vec<&str> = input.split("\n\n").collect();

    let mut wire_values: HashMap<String, Option<bool>> = HashMap::new();
    for line in sections[0].lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let name = parts[0].to_string();
        let value = parts[1] == "1";
        wire_values.insert(name, Some(value));
    }

    let mut gates = Vec::new();
    for line in sections[1].lines() {
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(" -> ").collect();
        let left = parts[0];
        let output_name = parts[1].to_string();

        let left_parts: Vec<&str> = left.split_whitespace().collect();
        let input1_name = left_parts[0].to_string();
        let op_str = left_parts[1];
        let input2_name = left_parts[2].to_string();

        let op = match op_str {
            "AND" => Operation::And,
            "XOR" => Operation::Xor,
            "OR" => Operation::Or,
            _ => panic!("Unknown operation: {}", op_str),
        };

        wire_values.entry(input1_name.clone()).or_insert(None);
        wire_values.entry(input2_name.clone()).or_insert(None);
        wire_values.entry(output_name.clone()).or_insert(None);

        let gate = Gate::new(input1_name, input2_name, output_name, op);
        gates.push(gate);
    }

    Logic::new(gates, wire_values)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut logic = parse_input(input);
    logic.propagate_until_stable();
    Some(logic.get_output_number())
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
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_one_simple() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_solution() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(56620966442854));
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
