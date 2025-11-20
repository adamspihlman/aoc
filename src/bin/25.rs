use advent_of_code::{
    input,
    lock::{Key, Lock},
};

advent_of_code::solution!(25);

fn to_key(grid: Vec<Vec<char>>) -> Key {
    let heights: Vec<usize> = (0..grid[0].len())
        .map(|col| {
            let mut height = 1;
            loop {
                if grid[6 - height][col] == '.' {
                    break;
                }
                height += 1;
            }
            height - 1
        })
        .collect();
    Key::from(heights)
}

fn to_lock(grid: Vec<Vec<char>>) -> Lock {
    let heights: Vec<usize> = (0..grid[0].len())
        .map(|col| {
            let mut height = 1;
            loop {
                if grid[height][col] == '.' {
                    break;
                }
                height += 1;
            }
            height - 1
        })
        .collect();
    Lock::from(heights)
}

fn parse_input(input: &str) -> (Vec<Lock>, Vec<Key>) {
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    let sections: Vec<&str> = input.split("\n\n").collect();
    for section in sections {
        let grid = input::parse_2d_vector(section);
        if grid[0][0] == '#' {
            locks.push(to_lock(grid));
        } else {
            keys.push(to_key(grid));
        }
    }

    (locks, keys)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut result = 0;
    let (locks, keys) = parse_input(input);
    for lock in &locks {
        for key in &keys {
            if lock.fits(key) {
                result += 1;
            }
        }
    }
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
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_one_solution() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(3451));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
