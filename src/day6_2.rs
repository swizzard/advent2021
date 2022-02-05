use crate::day6::parse_nums;
use crate::get_input::get_input;
use anyhow::{anyhow, Result};
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
struct FH(HashMap<u8, usize>);

impl FH {
    fn from_nums(ns: Vec<u8>) -> Self {
        let mut m = HashMap::from([
            (0, 0),
            (1, 0),
            (2, 0),
            (3, 0),
            (4, 0),
            (5, 0),
            (6, 0),
            (7, 0),
            (8, 0),
        ]);
        for n in ns.iter() {
            *m.get_mut(n).unwrap() += 1;
        }
        Self(m)
    }
    fn tick(&mut self) {
        let curr = self.0.clone();
        let new = curr.get(&0).unwrap();
        for i in 0..8 {
            if i == 6 {
                *self.0.get_mut(&i).unwrap() = curr.get(&7).unwrap() + new;
            } else {
                *self.0.get_mut(&i).unwrap() = *curr.get(&(i + 1)).unwrap();
            }
        }
        *self.0.get_mut(&8).unwrap() = *new;
    }
    fn tick_n(&mut self, days: u32) -> usize {
        for _ in 0..days {
            self.tick();
        }
        self.0.values().sum()
    }
}

pub fn day6_2() -> Result<usize> {
    let input = get_input("day6.txt")?;
    let (_, ns) = parse_nums(&input).map_err(|_| anyhow!("parse err"))?;
    let mut fish = FH::from_nums(ns);
    Ok(fish.tick_n(256))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_tick_1() {
        let mut start = FH::from_nums(vec![3, 4, 3, 1, 2]);
        let expected = FH::from_nums(vec![2, 3, 2, 0, 1]);
        start.tick();
        assert_eq!(start, expected);
    }
}
