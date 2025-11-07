advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    let map = advent_of_code::input::parse_2d_vector(input);
    let antennas = advent_of_code::antennas::Antennas::from(&map);
    let result = antennas.distinct_antinodes(advent_of_code::antennas::Antinode::Resonant);
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = advent_of_code::input::parse_2d_vector(input);
    let antennas = advent_of_code::antennas::Antennas::from(&map);
    let result = antennas.distinct_antinodes(advent_of_code::antennas::Antinode::Harmonic);
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }

    #[test]
    fn test_part_two_example_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(9));
    }

    #[test]
    fn test_part_two_slope_one() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(9));
    }
}
