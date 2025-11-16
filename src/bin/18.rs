use std::collections::VecDeque;

use advent_of_code::{bytedodge, grid};
use regex::Regex;

advent_of_code::solution!(18);

fn parse_input(input: &str) -> VecDeque<grid::Location> {
    let re = Regex::new(r"(\d+),(\d+)").unwrap();
    let mut result = VecDeque::new();

    input.lines().for_each(|line| {
        let (_, [col, row]) = re.captures(line).unwrap().extract();
        let location = grid::Location {
            row: row.parse::<usize>().unwrap(),
            col: col.parse::<usize>().unwrap(),
        };
        result.push_back(location);
    });

    result
}

fn add_corruptions(grid: &mut [Vec<char>], corruptions: &VecDeque<grid::Location>, count: usize) {
    if count >= corruptions.len() {
        panic!(
            "Requested {count} corruptions, but only {} provided",
            corruptions.len()
        );
    }

    for obstacle in corruptions.iter().take(count) {
        grid[obstacle.row][obstacle.col] = '#';
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut grid = grid::create_grid(71, 71, '.');

    let corruptions = parse_input(input);
    add_corruptions(&mut grid, &corruptions, 1024);

    let bytedodge = bytedodge::ByteDodge::from(grid);
    let result = bytedodge.min_steps();

    Some(result)
}

pub fn part_two(input: &str) -> Option<String> {
    let corruptions = parse_input(input);

    // Binary search to find first corruption that blocks all paths
    let mut left = 0;
    let mut right = corruptions.len() - 1;

    while left < right {
        let mid = left + (right - left) / 2;

        let mut grid = grid::create_grid(71, 71, '.');
        add_corruptions(&mut grid, &corruptions, mid + 1);

        let bytedodge = bytedodge::ByteDodge::from(grid);
        let result = bytedodge.min_steps();

        if result == 0 {
            // No path exists, answer is at or before mid
            right = mid;
        } else {
            // Path still exists, answer is after mid
            left = mid + 1;
        }
    }

    // left == right is the first index where no path exists
    let obstacle = corruptions[left];
    Some(format!("{},{}", obstacle.col, obstacle.row))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_solution() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(372));
    }

    #[test]
    fn test_part_two_solution() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some("25,6".to_string()));
    }
}
