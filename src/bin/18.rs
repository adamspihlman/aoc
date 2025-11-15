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

fn add_corruptions(
    grid: &mut [Vec<char>],
    corruptions: &mut VecDeque<grid::Location>,
    count: usize,
) {
    if count >= corruptions.len() {
        panic!(
            "Requested {count} corruptions, but only {} provided",
            corruptions.len()
        );
    }

    for _ in 0..count {
        let next = corruptions.pop_front().unwrap();
        grid[next.row][next.col] = '#';
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut corruptions = parse_input(input);
    let mut grid = grid::create_grid(71, 71, '.');
    add_corruptions(&mut grid, &mut corruptions, 1024);

    let bytedodge = bytedodge::ByteDodge::from(grid);
    let result = bytedodge.min_steps();

    Some(result)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_solution() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(372));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
