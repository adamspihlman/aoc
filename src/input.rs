use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;

pub fn parse_2d_vector(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn parse_2d_digit_vector(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).expect("invalid digit character"))
                .collect()
        })
        .collect()
}

/// Parse each line of input into type T using FromStr.
///
/// Skips empty lines and trims whitespace from each line.
pub fn parse_lines<T>(input: &str) -> Vec<T>
where
    T: FromStr,
    T::Err: std::fmt::Debug,
{
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.trim()
                .parse()
                .expect("failed to parse line")
        })
        .collect()
}

/// Parse pairs of values separated by whitespace from each line.
///
/// Returns two vectors: all left values and all right values.
pub fn parse_pairs<T>(input: &str) -> (Vec<T>, Vec<T>)
where
    T: FromStr,
    T::Err: std::fmt::Debug,
{
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut parts = line.split_whitespace();
            let left: T = parts
                .next()
                .expect("missing left value")
                .parse()
                .expect("failed to parse left value");
            let right: T = parts
                .next()
                .expect("missing right value")
                .parse()
                .expect("failed to parse right value");
            (left, right)
        })
        .unzip()
}

/// Count the frequency of each item in a slice.
///
/// Returns a HashMap where keys are items and values are counts.
pub fn count_frequencies<T>(items: &[T]) -> HashMap<T, u64>
where
    T: Eq + Hash + Copy,
{
    let mut counts = HashMap::new();
    for &item in items {
        counts
            .entry(item)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    counts
}
