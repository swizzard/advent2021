extern crate anyhow;
extern crate nom;

use std::fs;
use std::io;
use std::io::Read;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

use anyhow::{anyhow, Result};

#[derive(Eq, Debug, PartialEq)]
struct Pos {
    vertical: u32,
    horizontal: u32,
}

#[derive(Eq, Debug, PartialEq)]
enum Dir {
    Forward,
    Up,
    Down,
}

impl From<&str> for Dir {
    fn from(i: &str) -> Self {
        match i {
            "forward" => Self::Forward,
            "up" => Self::Up,
            "down" => Self::Down,
            _ => panic!("bad dir"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Msg {
    dir: Dir,
    n: u32,
}

impl Msg {
    fn parse_msg(i: &str) -> IResult<&str, Msg> {
        let dir_parser = alt((tag("forward"), tag("down"), tag("up")));
        let mut p = separated_pair(dir_parser, char(' '), digit1);
        let (rem, (d, n)) = p(i)?;
        Ok((
            rem,
            Msg {
                dir: Dir::from(d),
                n: n.parse::<u32>().unwrap(),
            },
        ))
    }
}

fn step(pos: Pos, msg: &Msg) -> Pos {
    match msg.dir {
        Dir::Forward => Pos {
            horizontal: pos.horizontal + msg.n,
            ..pos
        },
        Dir::Down => Pos {
            vertical: pos.vertical + msg.n,
            ..pos
        },
        Dir::Up => Pos {
            vertical: pos.vertical - msg.n,
            ..pos
        },
    }
}

fn get_input() -> io::Result<String> {
    let mut s = String::new();
    let mut f = fs::File::open("day2.txt")?;
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn parse_msgs(s: &str) -> IResult<&str, Vec<Msg>> {
    separated_list1(line_ending, Msg::parse_msg)(s)
}

pub fn day2_1() -> Result<u32> {
    let s = get_input()?;
    let msgs = parse_msgs(&s).map_err(|_| anyhow!("parser error"))?.1;
    let start = Pos {
        horizontal: 0,
        vertical: 0,
    };
    let end = msgs.iter().fold(start, step);
    Ok(end.horizontal * end.vertical)
}

#[derive(Debug, Eq, PartialEq)]
struct Day2Pos {
    horizontal: u32,
    vertical: u32,
    aim: u32,
}

fn day2_step(pos: Day2Pos, msg: &Msg) -> Day2Pos {
    match msg.dir {
        Dir::Forward => Day2Pos {
            horizontal: pos.horizontal + msg.n,
            vertical: pos.vertical + (pos.aim * msg.n),
            ..pos
        },
        Dir::Up => Day2Pos {
            aim: pos.aim - msg.n,
            ..pos
        },
        Dir::Down => Day2Pos {
            aim: pos.aim + msg.n,
            ..pos
        },
    }
}

pub fn day2_2() -> Result<u32> {
    let s = get_input()?;
    let msgs = parse_msgs(&s).map_err(|_| anyhow!("parser error"))?.1;
    let start = Day2Pos {
        horizontal: 0,
        vertical: 0,
        aim: 0,
    };
    let end = msgs.iter().fold(start, day2_step);
    Ok(end.horizontal * end.vertical)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_msg() {
        let s = "forward 2";
        let (_rem, actual) = Msg::parse_msg(s).unwrap();
        let expected = Msg {
            dir: Dir::Forward,
            n: 2,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_step_fwd() {
        let msg = Msg {
            dir: Dir::Forward,
            n: 2,
        };
        let start = Pos {
            vertical: 0,
            horizontal: 0,
        };
        let expected = Pos {
            vertical: 0,
            horizontal: 2,
        };
        let actual = step(start, &msg);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_step_down() {
        let msg = Msg {
            dir: Dir::Down,
            n: 2,
        };
        let start = Pos {
            vertical: 0,
            horizontal: 0,
        };
        let expected = Pos {
            vertical: 2,
            horizontal: 0,
        };
        let actual = step(start, &msg);
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_step_up() {
        let msg = Msg { dir: Dir::Up, n: 2 };
        let start = Pos {
            vertical: 2,
            horizontal: 0,
        };
        let expected = Pos {
            vertical: 0,
            horizontal: 0,
        };
        let actual = step(start, &msg);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_day2_step_forward() {
        let msg = Msg {
            dir: Dir::Forward,
            n: 2,
        };
        let start = Day2Pos {
            vertical: 0,
            horizontal: 0,
            aim: 2,
        };
        let expected = Day2Pos {
            vertical: 4,
            horizontal: 2,
            aim: 2,
        };
        let actual = day2_step(start, &msg);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_day2_step_up() {
        let msg = Msg { dir: Dir::Up, n: 2 };
        let start = Day2Pos {
            vertical: 0,
            horizontal: 0,
            aim: 2,
        };
        let expected = Day2Pos {
            horizontal: 0,
            vertical: 0,
            aim: 0,
        };
        let actual = day2_step(start, &msg);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_day2_step_down() {
        let msg = Msg {
            dir: Dir::Down,
            n: 2,
        };
        let start = Day2Pos {
            vertical: 0,
            horizontal: 0,
            aim: 0,
        };
        let expected = Day2Pos {
            vertical: 0,
            horizontal: 0,
            aim: 2,
        };
        let actual = day2_step(start, &msg);
        assert_eq!(actual, expected);
    }
}
