advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let (mut left_list, mut right_list): (Vec<u64>, Vec<u64>) = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let left = parts.next().unwrap().parse::<u64>().unwrap();
            let right = parts.next().unwrap().parse::<u64>().unwrap();
            (left, right)
        })
        .unzip();

    left_list.sort();
    right_list.sort();

    let mut my_result = 0;

    for index in 0..left_list.len() {
        my_result += left_list[index].abs_diff(right_list[index]);
    }
    Some(my_result)
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
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
