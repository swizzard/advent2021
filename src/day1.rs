use std::num::ParseIntError;

use crate::get_input::get_input;

fn parse_input() -> Result<Vec<u64>, ParseIntError> {
    get_input("day1.txt")
        .unwrap()
        .lines()
        .filter(|s| !s.is_empty())
        .map(|n| n.parse::<u64>())
        .collect::<Result<Vec<u64>, ParseIntError>>()
}

pub fn day1_1() -> Result<u64, ParseIntError> {
    let v = parse_input()?;
    Ok(v.windows(2)
        .fold(0, |acc, c| if c[0] < c[1] { acc + 1 } else { acc }))
}

pub fn day1_2() -> Result<u64, ParseIntError> {
    let v = parse_input()?;
    Ok(v.windows(3)
        .map(|w| w.iter().sum())
        .into_iter()
        .collect::<Vec<u64>>()
        .windows(2)
        .fold(0, |acc, w| if w[0] < w[1] { acc + 1 } else { acc }))
}
