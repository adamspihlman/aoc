advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u64> {
    let map = advent_of_code::input::parse_2d_vector(input);
    let mut garden = advent_of_code::garden::Garden::from(map);
    let result = garden.get_fence_price(advent_of_code::garden::PriceScale::Perimeter);
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = advent_of_code::input::parse_2d_vector(input);
    let mut garden = advent_of_code::garden::Garden::from(map);
    let result = garden.get_fence_price(advent_of_code::garden::PriceScale::Corner);
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_one_solution() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1424006));
    }

    #[test]
    fn test_part_one_simple() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(140));
    }

    #[test]
    fn test_part_one_subregions() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(772));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }

    #[test]
    fn test_part_two_simple() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(80));
    }

    #[test]
    fn test_part_two_four_by_four() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 6,
        ));
        assert_eq!(result, Some(16));
    }

    #[test]
    fn test_part_two_e_shape() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(236));
    }

    #[test]
    fn test_part_two_subregions() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(368));
    }

    #[test]
    fn test_part_two_solution() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(858684));
    }
}
