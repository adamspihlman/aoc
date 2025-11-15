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
    let grid = grid::create_grid(71, 71, '.');
    let corruptions = parse_input(input);
    let mut num_corruptions = 1;
    loop {
        let mut grid_copy = grid.clone();
        add_corruptions(&mut grid_copy, &corruptions, num_corruptions);

        let bytedodge = bytedodge::ByteDodge::from(grid_copy);
        let result = bytedodge.min_steps();

        if result != 0 {
            num_corruptions += 1;
        } else {
            let index = num_corruptions - 1;
            let obstacle = corruptions[index];
            let mut result = String::new();
            result.push_str(&obstacle.col.to_string());
            result.push(',');
            result.push_str(&obstacle.row.to_string());
            return Some(result);
        }
    }
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
    #[ignore]
    fn test_part_two_solution() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some("25,6".to_string()));
    }
}
