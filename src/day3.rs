extern crate anyhow;
extern crate nom;

use std::convert::TryFrom;
use std::fs;
use std::io::Read;

use anyhow::{anyhow, Result};
use nom::{
    character::complete::{digit1, line_ending},
    combinator::map_res,
    multi::separated_list1,
    IResult,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Bit {
    Zero,
    One,
}

impl Bit {
    fn as_number(&self, m: u32) -> u32 {
        match self {
            Self::Zero => 0,
            Self::One => m,
        }
    }
}

impl TryFrom<char> for Bit {
    type Error = ();
    fn try_from(val: char) -> Result<Self, Self::Error> {
        match val {
            '0' => Ok(Self::Zero),
            '1' => Ok(Self::One),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Count {
    zeros: usize,
    ones: usize,
}

impl Count {
    fn from_vec(v: Vec<Bit>) -> Self {
        v.iter().fold(Self { zeros: 0, ones: 0 }, |acc, v| match v {
            Bit::Zero => Self {
                zeros: acc.zeros + 1,
                ..acc
            },
            Bit::One => Self {
                ones: acc.ones + 1,
                ..acc
            },
        })
    }
    fn max(&self) -> Bit {
        if self.zeros > self.ones {
            Bit::Zero
        } else {
            Bit::One
        }
    }
    fn min(&self) -> Bit {
        if self.zeros > self.ones {
            Bit::One
        } else {
            Bit::Zero
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Reading(Vec<Bit>);

impl Reading {
    fn parse(s: &str) -> Result<Reading> {
        s.chars()
            .map(|c| (Bit::try_from(c).map_err(|_| anyhow!("digit parsing error"))))
            .collect::<Result<Vec<Bit>>>()
            .map(Self)
    }

    fn as_number(&self) -> u32 {
        self.0
            .iter()
            .rev()
            .fold((0, 1), |acc, v| (acc.0 + v.as_number(acc.1), acc.1 * 2))
            .0
    }
}

fn digits_at(readings: &[Reading], ix: usize) -> Result<Count> {
    readings
        .iter()
        .map(|v| v.0.get(ix).ok_or(anyhow!("bad index")).map(|v| *v))
        .collect::<Result<Vec<Bit>>>()
        .map(Count::from_vec)
}

fn get_input() -> Result<String> {
    let mut s = String::new();
    let mut f = fs::File::open("day3.txt")?;
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn parse_reading(s: &str) -> IResult<&str, Reading> {
    map_res(digit1, Reading::parse)(s)
}

fn parse_readings(s: &str) -> IResult<&str, Vec<Reading>> {
    separated_list1(line_ending, parse_reading)(s)
}

fn _to_ds(readings: &[Reading]) -> Result<Vec<Count>> {
    let l = readings
        .iter()
        .map(|v| v.0.len())
        .max()
        .ok_or(anyhow!("len error"))?;
    (0..l)
        .into_iter()
        .map(|ix| digits_at(readings, ix))
        .collect::<Result<Vec<Count>>>()
}

fn gamma_rate(readings: &[Reading]) -> Result<u32> {
    let ds = _to_ds(readings)?;
    let ms = ds.iter().map(|c| c.max()).collect::<Vec<Bit>>();
    let gamma = Reading(ms).as_number();
    Ok(gamma)
}

fn epsilon_rate(readings: &[Reading]) -> Result<u32> {
    let ds = _to_ds(readings)?;
    let ms = ds.iter().map(|c| c.min()).collect::<Vec<Bit>>();
    let epsilon = Reading(ms).as_number();
    Ok(epsilon)
}

pub fn day3_1() -> Result<u32> {
    let input = get_input()?;
    let readings = parse_readings(&input)
        .map_err(|_| anyhow!("parser error"))?
        .1;
    let gamma = gamma_rate(&readings)?;
    let epsilon = epsilon_rate(&readings)?;
    Ok(epsilon * gamma)
}

#[derive(Debug)]
struct BitCriterion {
    pos: usize,
    bit: Bit,
}

impl BitCriterion {
    fn filter_reading(&self, reading: &Reading) -> Result<bool> {
        let bit = reading.0.get(self.pos).ok_or(anyhow!("index error"))?;
        Ok(self.bit == *bit)
    }

    fn filter_readings<'a>(&self, readings: &'a [Reading]) -> Result<Vec<&'a Reading>> {
        let mut filtered = Vec::new();
        for reading in readings.iter() {
            if self.filter_reading(reading)? {
                filtered.push(reading);
            }
        }
        Ok(filtered)
    }

    fn from_readings(readings: &[Reading], pos: usize) -> Result<Self> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_reading() -> Result<()> {
        let s = "11001";
        let expected = Reading(vec![Bit::One, Bit::One, Bit::Zero, Bit::Zero, Bit::One]);
        let actual = parse_reading(s)?;
        assert_eq!(expected, actual.1);
        assert_eq!("", actual.0);
        Ok(())
    }

    #[test]
    fn test_from_vec() {
        let v = vec![Bit::Zero, Bit::Zero, Bit::One, Bit::One, Bit::Zero];
        let expected = Count { zeros: 3, ones: 2 };
        let actual = Count::from_vec(v);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_reading_as_number() {
        let r = Reading(vec![Bit::One, Bit::Zero, Bit::One, Bit::One, Bit::One]);
        assert_eq!(23, r.as_number());
    }
}
