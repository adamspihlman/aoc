use advent_of_code::cheat::Cheat;
use advent_of_code::input::parse_2d_vector;

advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_2d_vector(input);
    let cheat = Cheat::from(grid);
    cheat.print();
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
