advent_of_code::solution!(2);

#[derive(PartialEq)]
enum ReportSafety {
    Safe,                   // nothing to do, safe
    DiffTooLarge,           // nothing to do, impossible to make safe
    DiffTooSmall(usize),    // check 1 possibility, index where 0 diff found
    DirectionChange(usize), // check n - 2, n - 1, and n removed
}

fn parse_reports(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|element| element.parse::<u64>().unwrap())
                .collect()
        })
        .collect()
}

fn get_report_safety(report: &[u64]) -> ReportSafety {
    #[derive(PartialEq)]
    enum Direction {
        Increasing,
        Decreasing,
    }

    if report.len() <= 1 {
        return ReportSafety::Safe;
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
        if diff < 1 {
            return ReportSafety::DiffTooSmall(index);
        } else if diff > 3 {
            return ReportSafety::DiffTooLarge;
        } else if (direction == Direction::Increasing && *current < prev)
            || (direction == Direction::Decreasing && *current > prev)
        {
            return ReportSafety::DirectionChange(index);
        }
        prev = *current;
    }
    ReportSafety::Safe
}

fn is_report_safe(report: &[u64]) -> bool {
    get_report_safety(report) == ReportSafety::Safe
}

fn is_dampened_report_safe(report: &[u64]) -> bool {
    match get_report_safety(report) {
        ReportSafety::Safe => true,
        ReportSafety::DiffTooLarge => false,
        ReportSafety::DiffTooSmall(index) => {
            let mut my_copy = report.to_vec();
            my_copy.remove(index);
            is_report_safe(&my_copy)
        }
        ReportSafety::DirectionChange(index) => {
            for permutation in 0..=index {
                let mut my_copy = report.to_vec();
                my_copy.remove(permutation);
                if is_report_safe(&my_copy) {
                    return true;
                }
            }
            false
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let reports = parse_reports(input);
    let result = reports.iter().filter(|r| is_report_safe(r)).count() as u64;
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let reports = parse_reports(input);
    let result = reports
        .iter()
        .filter(|r| is_dampened_report_safe(r))
        .count() as u64;
    Some(result)
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
        assert_eq!(result, Some(4));
    }
}
