use crate::get_input::get_input;
use anyhow::{anyhow, Result};
use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map_res, multi::separated_list1,
    IResult,
};
use std::collections::HashMap;

fn parse_positions(s: &str) -> IResult<&str, Vec<i32>> {
    map_res(separated_list1(tag(","), digit1), |xs: Vec<&str>| {
        xs.into_iter()
            .map(|x| x.parse().map_err(|_| anyhow!("parse error")))
            .collect::<Result<Vec<i32>>>()
    })(s)
}

fn fuel_needed(start: &i32, end: &i32) -> i32 {
    (start - end).abs()
}

fn total_for_start(start: &i32, others: &[i32]) -> i32 {
    let mut total = 0;
    for v in others.iter() {
        total += fuel_needed(start, v);
    }
    total
}

type FuelNeeds = HashMap<i32, i32>;

fn get_needed_fuel(starting_positions: Vec<i32>) -> FuelNeeds {
    let mut m = HashMap::new();
    for n in starting_positions.iter() {
        if m.get(n).is_none() {
            m.insert(*n, total_for_start(n, &starting_positions));
        }
    }
    m
}

fn cheapest_cost(costs: FuelNeeds) -> i32 {
    let mut curr_cost = i32::MAX;
    for cost in costs.values() {
        if cost < &curr_cost {
            curr_cost = *cost;
        }
    }
    curr_cost
}

fn get_positions() -> Result<Vec<i32>> {
    let input = get_input("day7.txt")?;
    let (_, positions) = parse_positions(&input).map_err(|_| anyhow!("parse error"))?;
    Ok(positions)
}

pub fn day7_1() -> Result<i32> {
    let positions = get_positions()?;
    let costs = get_needed_fuel(positions);
    Ok(cheapest_cost(costs))
}

fn part2_cost(start: &i32, end: &i32) -> i32 {
    if start == end {
        0
    } else {
        let end = (start - end).abs() + 1;
        (1..end).sum()
    }
}

fn total_for_start_2(start: &i32, others: &[i32]) -> i32 {
    let mut total = 0;
    for v in others.iter() {
        total += part2_cost(start, v);
    }
    total
}

fn get_needed_fuel_2(starting_positions: Vec<i32>) -> FuelNeeds {
    let mut m = HashMap::new();
    let max = starting_positions
        .iter()
        .fold(&i32::MIN, |a, v| if v > a { v } else { a });
    for n in 0..*max {
        if m.get(&n).is_none() {
            m.insert(n, total_for_start_2(&n, &starting_positions));
        }
    }
    m
}

pub fn day7_2() -> Result<i32> {
    let positions = get_positions()?;
    let costs = get_needed_fuel_2(positions);
    Ok(cheapest_cost(costs))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_cheapest_cost() {
        let positions = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        let costs = get_needed_fuel(positions);
        assert_eq!(37, cheapest_cost(costs));
    }
    #[test]
    fn test_part2_cost() {
        assert_eq!(66, part2_cost(&16, &5));
        assert_eq!(10, part2_cost(&1, &5));
        assert_eq!(6, part2_cost(&2, &5));
        assert_eq!(15, part2_cost(&0, &5));
        assert_eq!(1, part2_cost(&4, &5));
        assert_eq!(3, part2_cost(&7, &5));
        assert_eq!(45, part2_cost(&14, &5));
    }
    #[test]
    fn test_cheapest_cost_2() {
        let positions = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        let costs = get_needed_fuel_2(positions);
        assert_eq!(168, cheapest_cost(costs));
    }
}
