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
    fn from_vec(v: Vec<&Bit>) -> Self {
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

#[derive(Clone, Debug, Eq, PartialEq)]
struct Reading(Vec<Bit>);

impl Reading {
    fn len(&self) -> usize {
        self.get_reading().len()
    }
    fn get_reading(&self) -> &Vec<Bit> {
        &self.0
    }
    fn at(&self, ix: usize) -> Result<&Bit> {
        self.get_reading().get(ix).ok_or(anyhow!("bad index"))
    }
    fn parse(s: &str) -> Result<Reading> {
        s.chars()
            .map(|c| (Bit::try_from(c).map_err(|_| anyhow!("digit parsing error"))))
            .collect::<Result<Vec<Bit>>>()
            .map(Self)
    }

    fn as_number(&self) -> u32 {
        self.get_reading()
            .iter()
            .rev()
            .fold((0, 1), |acc, v| (acc.0 + v.as_number(acc.1), acc.1 * 2))
            .0
    }
    fn matches_bit_at(&self, bit: &Bit, pos: usize) -> Result<bool> {
        let b = self.at(pos)?;
        Ok(b == bit)
    }
}

fn digits_at(readings: &[Reading], ix: usize) -> Result<Count> {
    readings
        .iter()
        .map(|v| v.at(ix))
        .collect::<Result<Vec<&Bit>>>()
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
        .map(|v| v.len())
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

fn count_at(readings: &[Reading], pos: usize) -> Result<Count> {
    let bits = readings
        .iter()
        .map(|r| r.at(pos))
        .collect::<Result<Vec<&Bit>>>()?;
    Ok(Count::from_vec(bits))
}

fn filter_by_bit_at(readings: &[Reading], bit: &Bit, pos: usize) -> Result<Vec<Reading>> {
    let mut filtered = Vec::new();
    for r in readings {
        let matches = r.matches_bit_at(bit, pos)?;
        if matches {
            filtered.push(r.clone())
        }
    }
    Ok(filtered)
}

fn get_rating(readings: &[Reading], decider_fn: impl Fn(&Count) -> Bit) -> Result<u32> {
    let mut pos = 0;
    let mut filtered = Vec::from(readings);
    while filtered.len() > 1 {
        let ca = count_at(filtered.as_slice(), pos)?;
        let decider = decider_fn(&ca);
        filtered = filter_by_bit_at(filtered.as_slice(), &decider, pos)?;
        pos += 1;
    }
    Ok(filtered[0].as_number())
}

fn get_oxygen_rating(readings: &[Reading]) -> Result<u32> {
    get_rating(readings, |c| c.max())
}

fn get_co2_rating(readings: &[Reading]) -> Result<u32> {
    get_rating(readings, |c| c.min())
}

pub fn day3_2() -> Result<u32> {
    let input = get_input()?;
    let readings = parse_readings(&input)
        .map_err(|_| anyhow!("parser error"))?
        .1;
    let oxygen = get_oxygen_rating(&readings)?;
    let co2 = get_co2_rating(&readings)?;
    Ok(oxygen * co2)
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
        let z = Bit::Zero;
        let o = Bit::One;
        let v = vec![&z, &z, &o, &o, &z];
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
