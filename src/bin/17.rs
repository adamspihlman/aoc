use regex::Regex;

advent_of_code::solution!(17);

fn parse_input(input: &str) -> (advent_of_code::assembly::Computer, Vec<u8>) {
    let register_re = Regex::new(r"Register [ABC]: (\d+)").unwrap();
    let program_re = Regex::new(r"^Program: (\d(?:,\d)*)$").unwrap();

    let mut lines = input.lines();
    let a = register_re.captures(lines.next().unwrap()).unwrap()[1]
        .parse::<u64>()
        .unwrap();
    let b = register_re.captures(lines.next().unwrap()).unwrap()[1]
        .parse::<u64>()
        .unwrap();
    let c = register_re.captures(lines.next().unwrap()).unwrap()[1]
        .parse::<u64>()
        .unwrap();
    lines.next();

    let program: Vec<u8> = program_re.captures(lines.next().unwrap()).unwrap()[1]
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect();

    (advent_of_code::assembly::Computer::new(a, b, c), program)
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut computer, program) = parse_input(input);
    let output = computer.execute(&program);

    let mut result = String::new();
    output
        .iter()
        .for_each(|e| result.push_str(&(e.to_string() + ",")));
    result.pop();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, program) = parse_input(input);
    let result = advent_of_code::assembly::magic_register(program);

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_one_solution() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some("7,6,5,3,6,5,7,0,4".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(117440));
    }

    #[test]
    fn test_part_two_solution() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(190615597431823));
    }
}
