//! Optimized pathfinder for Day 6.
//!
//! Uses bitmask arrays and parallel processing for efficient loop detection.

use crate::grid::{Direction, Location};
use rayon::prelude::*;

/// Direction bitmasks for visited tracking.
const UP: u8 = 1;
const DOWN: u8 = 2;
const LEFT: u8 = 4;
const RIGHT: u8 = 8;

/// Fast visited state using a flat bitmask array.
#[derive(Clone)]
struct VisitedGrid {
    data: Vec<u8>,
    width: usize,
}

impl VisitedGrid {
    fn new(height: usize, width: usize) -> Self {
        Self {
            data: vec![0; height * width],
            width,
        }
    }

    #[inline]
    fn index(&self, loc: Location) -> usize {
        loc.row * self.width + loc.col
    }

    #[inline]
    fn mark(&mut self, loc: Location, dir: Direction) {
        let idx = self.index(loc);
        self.data[idx] |= dir_to_bit(dir);
    }

    #[inline]
    fn has_visited(&self, loc: Location, dir: Direction) -> bool {
        let idx = self.index(loc);
        (self.data[idx] & dir_to_bit(dir)) != 0
    }

    #[inline]
    fn has_any(&self, loc: Location) -> bool {
        let idx = self.index(loc);
        self.data[idx] != 0
    }
}

#[inline]
fn dir_to_bit(dir: Direction) -> u8 {
    match dir {
        Direction::Up => UP,
        Direction::Down => DOWN,
        Direction::Left => LEFT,
        Direction::Right => RIGHT,
    }
}

#[inline]
fn rotate_cw(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

/// Candidate obstacle position with the state needed to test it.
struct ObstacleCandidate {
    obstacle_pos: Location,
    guard_pos: Location,
    guard_dir: Direction,
    visited_snapshot: VisitedGrid,
}

#[derive(Debug)]
pub struct Pathfinder<'a> {
    map: &'a [Vec<char>],
    height: usize,
    width: usize,
    start_location: Location,
    start_direction: Direction,
}

impl<'a> From<&'a Vec<Vec<char>>> for Pathfinder<'a> {
    fn from(value: &'a Vec<Vec<char>>) -> Self {
        let (location, direction) = find_start(value);
        let height = value.len();
        let width = if height > 0 { value[0].len() } else { 0 };
        Self {
            map: value,
            height,
            width,
            start_location: location,
            start_direction: direction,
        }
    }
}

impl Pathfinder<'_> {
    pub fn distinct_positions(&self) -> u64 {
        let mut visited = VisitedGrid::new(self.height, self.width);
        let mut location = self.start_location;
        let mut direction = self.start_direction;

        visited.mark(location, direction);

        while !self.is_at_edge(location, direction) {
            let next = self.get_next(location, direction);

            if self.is_obstacle(next, None) {
                direction = rotate_cw(direction);
            } else {
                location = next;
            }

            if visited.has_visited(location, direction) {
                panic!("Unexpected looping path found");
            }
            visited.mark(location, direction);
        }

        // Count cells with any visit
        visited.data.iter().filter(|&&v| v != 0).count() as u64
    }

    pub fn distinct_obstacles(&self) -> u64 {
        // Phase 1: Walk the path and collect candidate obstacle positions
        let candidates = self.collect_candidates();

        // Phase 2: Test each candidate in parallel
        candidates
            .into_par_iter()
            .filter(|candidate| self.creates_loop(candidate))
            .count() as u64
    }

    /// Walk the original path and collect all positions where placing an
    /// obstacle might create a loop.
    fn collect_candidates(&self) -> Vec<ObstacleCandidate> {
        let mut candidates = Vec::new();
        let mut visited = VisitedGrid::new(self.height, self.width);
        let mut location = self.start_location;
        let mut direction = self.start_direction;

        visited.mark(location, direction);

        while !self.is_at_edge(location, direction) {
            let next = self.get_next(location, direction);

            if self.is_obstacle(next, None) {
                direction = rotate_cw(direction);
            } else {
                // Check if placing obstacle at `next` is a valid candidate
                if !visited.has_any(next) {
                    candidates.push(ObstacleCandidate {
                        obstacle_pos: next,
                        guard_pos: location,
                        guard_dir: direction,
                        visited_snapshot: visited.clone(),
                    });
                }
                location = next;
            }

            if visited.has_visited(location, direction) {
                panic!("Unexpected looping path found");
            }
            visited.mark(location, direction);
        }

        candidates
    }

    /// Test if placing an obstacle creates a loop.
    /// Starts from the guard's position when they would have hit the obstacle.
    fn creates_loop(&self, candidate: &ObstacleCandidate) -> bool {
        let mut visited = candidate.visited_snapshot.clone();
        let mut location = candidate.guard_pos;
        // Turn right since we hit the new obstacle
        let mut direction = rotate_cw(candidate.guard_dir);

        // Mark the turn
        if visited.has_visited(location, direction) {
            return true; // Already a loop from turning
        }
        visited.mark(location, direction);

        while !self.is_at_edge(location, direction) {
            let next = self.get_next(location, direction);

            if self.is_obstacle(next, Some(candidate.obstacle_pos)) {
                direction = rotate_cw(direction);
            } else {
                location = next;
            }

            if visited.has_visited(location, direction) {
                return true; // Found a loop!
            }
            visited.mark(location, direction);
        }

        false // Exited the grid, no loop
    }

    #[inline]
    fn is_at_edge(&self, loc: Location, dir: Direction) -> bool {
        match dir {
            Direction::Up => loc.row == 0,
            Direction::Down => loc.row == self.height - 1,
            Direction::Left => loc.col == 0,
            Direction::Right => loc.col == self.width - 1,
        }
    }

    #[inline]
    fn get_next(&self, loc: Location, dir: Direction) -> Location {
        match dir {
            Direction::Up => Location {
                row: loc.row - 1,
                col: loc.col,
            },
            Direction::Down => Location {
                row: loc.row + 1,
                col: loc.col,
            },
            Direction::Left => Location {
                row: loc.row,
                col: loc.col - 1,
            },
            Direction::Right => Location {
                row: loc.row,
                col: loc.col + 1,
            },
        }
    }

    #[inline]
    fn is_obstacle(&self, loc: Location, extra: Option<Location>) -> bool {
        self.map[loc.row][loc.col] == '#' || extra == Some(loc)
    }
}

fn find_start(map: &[Vec<char>]) -> (Location, Direction) {
    for (row_idx, row_val) in map.iter().enumerate() {
        for (col_idx, col_val) in row_val.iter().enumerate() {
            let location = Location {
                row: row_idx,
                col: col_idx,
            };
            match col_val {
                '^' => return (location, Direction::Up),
                '>' => return (location, Direction::Right),
                '<' => return (location, Direction::Left),
                'v' => return (location, Direction::Down),
                _ => continue,
            }
        }
    }
    panic!("Failed to find start location in map");
}
