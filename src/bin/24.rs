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

    #[test]
    #[ignore]
    fn visualize_circuit() {
        let input = &advent_of_code::template::read_file("inputs", DAY);
        let mut logic = parse_input(input);

        // Propagate values before visualization to see the circuit state
        logic.propagate_until_stable();

        // 1. ASCII visualization - prints directly to stdout
        println!("\n--- ASCII Visualization ---");
        logic.print_ascii();

        // 2. Graphviz DOT format - save to file and render with:
        //    dot -Tpng circuit.dot -o circuit.png
        println!("\n--- Graphviz DOT format ---");
        let dot = logic.to_dot();
        println!("{}", dot);
        // Uncomment to save to file:
        std::fs::write("circuit.dot", dot).unwrap();

        // 3. Mermaid diagram - can be viewed in GitHub/VSCode or at mermaid.live
        println!("\n--- Mermaid Diagram ---");
        let mermaid = logic.to_mermaid();
        println!("{}", mermaid);
        // Uncomment to save to file:
        std::fs::write("circuit.md", mermaid).unwrap();
    }

    #[test]
    #[ignore]
    fn test_adder_detection() {
        let input = &advent_of_code::template::read_file("inputs", DAY);
        let logic = parse_input(input);

        // Find full adders first
        let (full_adders, full_adder_indices) = logic.find_full_adders();
        println!("Found {} full adders:", full_adders.len());
        for (i, fa) in full_adders.iter().enumerate() {
            println!(
                "  FA{}: ({} + {} + {}) -> sum={}, c_out={}",
                i, fa.x, fa.y, fa.c_in, fa.sum, fa.c_out
            );
        }

        // Find half adders (excluding gates used in full adders)
        let (half_adders, half_adder_indices) = logic.find_half_adders(&full_adder_indices);
        println!("\nFound {} half adders:", half_adders.len());
        for (i, ha) in half_adders.iter().enumerate() {
            println!(
                "  HA{}: ({} + {}) -> sum={}, carry={}",
                i, ha.x, ha.y, ha.sum, ha.carry
            );
        }

        // Find unused gates
        let mut all_used_indices = full_adder_indices.clone();
        all_used_indices.extend(&half_adder_indices);
        let unused_gates = logic.find_unused_gates(&all_used_indices);
        println!("\nFound {} unused gates:", unused_gates.len());
        for gate in unused_gates.iter().take(10) {
            println!("  {:?}", gate);
        }
        if unused_gates.len() > 10 {
            println!("  ... and {} more", unused_gates.len() - 10);
        }

        // Summary
        println!("\n=== Summary ===");
        println!("Total gates: {}", logic.gate_count());
        println!("Gates in full adders: {}", full_adder_indices.len());
        println!("Gates in half adders: {}", half_adder_indices.len());
        println!("Unused gates: {}", unused_gates.len());
        println!(
            "Accounted for: {} / {}",
            all_used_indices.len() + unused_gates.len(),
            logic.gate_count()
        );
    }
}
