use std::fs;
use std::io;
use std::io::Read;
use std::num::ParseIntError;

fn get_input() -> io::Result<String> {
    let mut s = String::new();
    let mut f = fs::File::open("day1.txt")?;
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn parse_input() -> Result<Vec<u64>, ParseIntError> {
    get_input()
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
