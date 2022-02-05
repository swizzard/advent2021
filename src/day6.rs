use crate::get_input::get_input;
use anyhow::{anyhow, Result};
use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map_res, multi::separated_list1,
    IResult,
};

#[derive(Debug, Eq, PartialEq)]
struct LF(u8);

impl LF {
    fn new_fish() -> Self {
        LF(8)
    }
    fn tick(&mut self) -> bool {
        if self.0 == 0 {
            self.0 = 6;
            true
        } else {
            self.0 -= 1;
            false
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct LanternFish(Vec<LF>);

impl LanternFish {
    fn tick(&mut self) {
        let mut new_fish = 0;
        for f in self.0.iter_mut() {
            if f.tick() {
                new_fish += 1;
            }
        }
        for _ in 0..new_fish {
            self.0.push(LF::new_fish())
        }
    }
    fn tick_n(&mut self, n: u32) -> usize {
        for _ in 0..n {
            self.tick();
        }
        self.0.len()
    }
    fn from_nums(ns: Vec<u8>) -> Self {
        Self(ns.into_iter().map(LF).collect())
    }
}

pub fn parse_nums(s: &str) -> IResult<&str, Vec<u8>> {
    map_res(separated_list1(tag(","), digit1), |xs: Vec<&str>| {
        xs.into_iter()
            .map(|x| x.parse().map_err(|_| anyhow!("parse error")))
            .collect::<Result<Vec<u8>>>()
    })(s)
}

fn parse_fish(s: &str) -> IResult<&str, LanternFish> {
    let (s, f) = parse_nums(s)?;
    Ok((s, LanternFish::from_nums(f)))
}

fn read_fish() -> anyhow::Result<LanternFish> {
    let s = get_input("day6.txt")?;
    parse_fish(&s)
        .map_err(|_| anyhow!("parsing error"))
        .map(|(_, fs)| fs)
}

pub fn day6_1() -> anyhow::Result<usize> {
    let mut fish = read_fish()?;
    Ok(fish.tick_n(80))
}

#[cfg(test)]
mod test {
    use super::*;
    fn starting_fish() -> LanternFish {
        LanternFish(vec![LF(3), LF(4), LF(3), LF(1), LF(2)])
    }
    #[test]
    fn test_lanternfish() {
        let mut fish = starting_fish();
        for _ in 0..18 {
            fish.tick();
        }
        assert_eq!(fish.0.len(), 26);
    }
    #[test]
    fn test_lanternfish_80() {
        let mut fish = starting_fish();
        assert_eq!(fish.tick_n(80), 5934);
    }
    #[test]
    fn test_fish_tick() {
        let mut fish = LF(0);
        let res = fish.tick();
        assert!(res);
        assert_eq!(fish.0, 6);
    }
    #[test]
    fn test_tick_1() {
        let mut fish = LanternFish(vec![LF(0)]);
        fish.tick();
        assert_eq!(fish.0.len(), 2);
    }
    #[test]
    fn test_tick_2() {
        let mut fish = starting_fish();
        let l = fish.tick_n(2);
        assert_eq!(6, l);
    }
    #[test]
    fn tt1() {
        let mut fish = starting_fish();
        fish.tick();
        assert_eq!(fish, LanternFish(vec![LF(2), LF(3), LF(2), LF(0), LF(1)]));
        fish.tick();
        assert_eq!(
            fish,
            LanternFish(vec![LF(1), LF(2), LF(1), LF(6), LF(0), LF(8)])
        );
    }
}
