advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u64> {
    let mut claw = advent_of_code::claw::ClawBuilder::default()
        .machines(input)
        .build();
    let result = claw.min_cost();
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
        assert_eq!(result, Some(480));
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
