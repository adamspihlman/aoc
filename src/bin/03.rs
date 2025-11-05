advent_of_code::solution!(3);

use regex::Regex;

fn get_mul_regex() -> Regex {
    Regex::new(r"mul\((\d+),(\d+)\)").unwrap()
}

fn get_do_regex() -> Regex {
    Regex::new(r"do\(\)").unwrap()
}

fn get_dont_regex() -> Regex {
    Regex::new(r"don\'t\(\)").unwrap()
}

pub fn part_one(input: &str) -> Option<u64> {
    let re = get_mul_regex();

    let result = re
        .captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [left, right])| left.parse::<u64>().unwrap() * right.parse::<u64>().unwrap())
        .sum();

    Some(result)
}

fn lower_bound(arr: &[usize], value: usize) -> Option<usize> {
    match arr.binary_search(&value) {
        Ok(_) => panic!("Matches should not have identical start indices"),
        Err(i) => {
            if i == 0 {
                None
            } else {
                Some(arr[i - 1])
            }
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let re = get_mul_regex();
    let re_do = get_do_regex();
    let re_dont = get_dont_regex();

    let do_calls: Vec<usize> = re_do.find_iter(input).map(|m| m.start()).collect();
    let dont_calls: Vec<usize> = re_dont.find_iter(input).map(|m| m.start()).collect();

    let mut result = 0;

    for mul_iter in re.captures_iter(input) {
        let mul_start = mul_iter.get(0).unwrap().start();

        let do_lower_bound = lower_bound(&do_calls, mul_start);
        let dont_lower_bound = lower_bound(&dont_calls, mul_start);

        let mul_enabled = match dont_lower_bound {
            None => true,
            Some(disable) => match do_lower_bound {
                None => false,
                Some(enable) => enable > disable,
            },
        };

        if mul_enabled {
            let (_, [left, right]) = mul_iter.extract();
            result += left.parse::<u64>().unwrap() * right.parse::<u64>().unwrap();
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
