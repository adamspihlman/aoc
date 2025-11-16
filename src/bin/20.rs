use advent_of_code::cheat::Cheat;
use advent_of_code::input::parse_2d_vector;

advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_2d_vector(input);
    let cheat = Cheat::from(grid);
    let result = cheat.count_cheats(100);
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_one_solution() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1511));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_cheats_threshold_2() {
        let grid = parse_2d_vector(&advent_of_code::template::read_file("examples", DAY));
        let cheat = Cheat::from(grid);
        // 14 + 14 + 2 + 4 + 2 + 3 + 1 + 1 + 1 + 1 + 1 = 44 total cheats saving >= 2
        assert_eq!(cheat.count_cheats(2), 44);
    }

    #[test]
    fn test_cheats_threshold_4() {
        let grid = parse_2d_vector(&advent_of_code::template::read_file("examples", DAY));
        let cheat = Cheat::from(grid);
        // 14 + 2 + 4 + 2 + 3 + 1 + 1 + 1 + 1 + 1 = 30 total cheats saving >= 4
        assert_eq!(cheat.count_cheats(4), 30);
    }

    #[test]
    fn test_cheats_threshold_8() {
        let grid = parse_2d_vector(&advent_of_code::template::read_file("examples", DAY));
        let cheat = Cheat::from(grid);
        // 4 + 2 + 3 + 1 + 1 + 1 + 1 + 1 = 14 total cheats saving >= 8
        assert_eq!(cheat.count_cheats(8), 14);
    }

    #[test]
    fn test_cheats_threshold_10() {
        let grid = parse_2d_vector(&advent_of_code::template::read_file("examples", DAY));
        let cheat = Cheat::from(grid);
        // 2 + 3 + 1 + 1 + 1 + 1 + 1 = 10 total cheats saving >= 10
        assert_eq!(cheat.count_cheats(10), 10);
    }

    #[test]
    fn test_cheats_threshold_64() {
        let grid = parse_2d_vector(&advent_of_code::template::read_file("examples", DAY));
        let cheat = Cheat::from(grid);
        // 1 cheat saving >= 64
        assert_eq!(cheat.count_cheats(64), 1);
    }
}
