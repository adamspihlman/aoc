advent_of_code::solution!(4);

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn count_xmas(arr: &Vec<Vec<char>>, row: usize, col: usize) -> u64 {
    0
}

pub fn part_one(input: &str) -> Option<u64> {
    let arr = parse_input(input);
    let mut count = 0;

    for (row_idx, row_val) in arr.iter().enumerate() {
        for (col_idx, col_val) in row_val.iter().enumerate() {
            if col_val == &'X' {
                count += count_xmas(&arr, row_idx, col_idx);
            }
        }
    }
    Some(count)
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
