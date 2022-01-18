use anyhow::Result;

use advent2021::*;

fn main() -> Result<()> {
    println!("day1 1: {:?}", day1_1());
    println!("day1 2: {:?}", day1_2());
    println!("day2 1: {:?}", day2_1()?);
    println!("day2 2: {:?}", day2_2()?);
    println!("day3 1: {:?}", day3_1()?);
    Ok(())
}
