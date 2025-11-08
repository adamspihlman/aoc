advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let mut disk = advent_of_code::disk::Disk::from(input);
    let result = disk.compute_contiguous_checksum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut disk = advent_of_code::disk::Disk::from(input);
    let result = disk.compute_block_checksum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }

    #[test]
    fn test_part_one_solution() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(6399153661894));
    }

    #[test]
    fn test_part_two_solution() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(6421724645083));
    }
}
