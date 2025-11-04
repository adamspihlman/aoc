advent_of_code::solution!(2);

fn parse_reports(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|line| {
            let parts = line.split_whitespace();
            let report: Vec<u64> = parts
                .map(|element| element.parse::<u64>().unwrap())
                .collect();
            report
        })
        .collect()
}

fn is_report_safe(report: &Vec<u64>) -> bool {
    #[derive(PartialEq)]
    enum Direction {
        Increasing,
        Decreasing,
    }

    if report.len() <= 1 {
        return true;
    }

    let mut prev = report[0];
    let mut direction = Direction::Increasing;

    for (index, current) in report.iter().enumerate() {
        if index == 0 {
            continue;
        }
        if index == 1 {
            direction = if *current > prev {
                Direction::Increasing
            } else {
                Direction::Decreasing
            };
        }

        let diff = current.abs_diff(prev);
        if diff < 1 || diff > 3 {
            return false;
        }
        if (direction == Direction::Increasing && *current < prev)
            || (direction == Direction::Decreasing && *current > prev)
        {
            return false;
        }
        prev = *current;
    }
    true
}

pub fn part_one(input: &str) -> Option<u64> {
    let reports = parse_reports(input);
    let result = reports
        .iter()
        .map(|report| if is_report_safe(report) { 1 } else { 0 })
        .sum();
    Some(result)
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
