use std::collections::{HashSet, VecDeque};

use advent_of_code::{bytedodge, grid, path_blocking_analyzer::PathBlockingAnalyzer};
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

/// Encodes a 3x3 grid around a location into an 8-bit value
/// where each bit represents if that position is accessible (not '#' and in bounds)
fn encode_3x3_grid(grid: &[Vec<char>], center: grid::Location) -> u8 {
    let mut config = 0u8;

    // Position mapping: [0,1,2,3,5,6,7,8] excluding center (4)
    // Grid positions:
    // 0 1 2
    // 3 4 5  (4 is center)
    // 6 7 8
    let offsets = [
        (-1, -1),
        (0, -1),
        (1, -1), // top row: positions 0,1,2
        (-1, 0),
        (1, 0), // middle row: positions 3,5 (skip center)
        (-1, 1),
        (0, 1),
        (1, 1), // bottom row: positions 6,7,8
    ];

    for (bit_idx, &(col_offset, row_offset)) in offsets.iter().enumerate() {
        let new_row = center.row as isize + row_offset;
        let new_col = center.col as isize + col_offset;

        // Check if position is in bounds and accessible
        if new_row >= 0
            && new_row < grid.len() as isize
            && new_col >= 0
            && new_col < grid[0].len() as isize
        {
            let pos_char = grid[new_row as usize][new_col as usize];
            if pos_char != '#' {
                config |= 1 << bit_idx;
            }
        }
        // If out of bounds or '#', bit remains 0 (inaccessible)
    }

    config
}

pub fn part_two(input: &str) -> Option<String> {
    // Precompute all blocking configurations
    let analyzer = PathBlockingAnalyzer::new();
    let blocking_configs: HashSet<u8> = analyzer
        .get_blocking_configurations()
        .iter()
        .copied()
        .collect();

    // Initialize grid and parse corruptions
    let mut grid = grid::create_grid(71, 71, '.');
    let corruptions = parse_input(input);

    // Process corruptions starting from 0
    for &obstacle in &corruptions {
        grid[obstacle.row][obstacle.col] = '#';

        // Check if this obstacle blocks a local path
        let config = encode_3x3_grid(&grid, obstacle);

        if blocking_configs.contains(&config) {
            // Local path blocked - check if overall path still exists
            let bytedodge = bytedodge::ByteDodge::from(grid.clone());
            let result = bytedodge.min_steps();

            if result == 0 {
                // No path exists in the overall grid
                return Some(format!("{},{}", obstacle.col, obstacle.row));
            }
        }
    }

    None
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
