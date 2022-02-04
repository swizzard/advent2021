use anyhow::Result;

use std::fs;
use std::io::Read;

pub fn get_input(pth: &str) -> Result<String> {
    let mut s = String::new();
    let mut f = fs::File::open(pth)?;
    f.read_to_string(&mut s)?;
    Ok(s)
}
