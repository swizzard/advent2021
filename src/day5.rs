use std::collections::HashMap;
use std::fs;
use std::io::Read;

use anyhow::{anyhow, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Point {
    x: u32,
    y: u32,
}

type Line = Vec<Point>;

fn from_points_hv(start: Point, end: Point) -> Option<Line> {
    if start.x == end.x {
        Some(from_points_vertical(start, end))
    } else if start.y == end.y {
        Some(from_points_horizontal(start, end))
    } else {
        None
    }
}

fn from_points_hvd(start: Point, end: Point) -> Option<Line> {
    from_points_hv(start, end).or_else(|| from_points_diag(start, end))
}

fn from_points_vertical(start: Point, end: Point) -> Line {
    let x = start.x;
    let mut l = Vec::new();
    let mut st = start.y;
    let mut term = end.y + 1;
    if start.y > end.y {
        st = end.y;
        term = start.y + 1;
    }
    for y in st..term {
        l.push(Point { x, y });
    }
    l
}
fn from_points_horizontal(start: Point, end: Point) -> Line {
    let y = start.y;
    let mut l = Vec::new();
    let mut st = start.x;
    let mut term = end.x + 1;
    if start.x > end.x {
        st = end.x;
        term = start.x + 1;
    }
    for x in st..term {
        l.push(Point { x, y });
    }
    l
}

fn step_pos(v: u32) -> u32 {
    v + 1
}

fn step_neg(v: u32) -> u32 {
    v.checked_sub(1).unwrap()
}

fn from_points_diag(start: Point, end: Point) -> Option<Line> {
    let mut l = Vec::new();
    let mut x_step = step_pos as fn(u32) -> u32;
    let mut y_step = step_pos as fn(u32) -> u32;
    let mut curr_x = start.x;
    let mut curr_y = start.y;
    let end_x = end.x;
    let end_y = end.y;
    if start.x > end.x {
        x_step = step_neg;
    }
    if start.y > end.y {
        y_step = step_neg;
    }
    l.push(start);
    loop {
        curr_x = x_step(curr_x);
        curr_y = y_step(curr_y);
        if (curr_x == end_x && (curr_y != end_y)) || (curr_y == end_y && (curr_x != end_x)) {
            return None;
        } else {
            l.push(Point {
                x: curr_x,
                y: curr_y,
            });
            if curr_x == end_x && curr_y == end_y {
                return Some(l);
            }
        }
    }
}

fn parse_point(s: &str) -> IResult<&str, Point> {
    map_res(
        separated_pair(digit1, char(','), digit1),
        |(x, y): (&str, &str)| {
            Ok(Point {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }) as Result<_, Point>
        },
    )(s)
}

fn arr(s: &str) -> IResult<&str, ()> {
    map_res(tag(" -> "), |_: &str| Ok(()) as Result<_, ()>)(s)
}

fn parse_line_hv(s: &str) -> IResult<&str, Option<Line>> {
    let (s, start) = parse_point(s)?;
    let (s, _) = arr(s)?;
    let (s, end) = parse_point(s)?;
    Ok((s, from_points_hv(start, end)))
}

fn parse_line_hvd(s: &str) -> IResult<&str, Option<Line>> {
    let (s, start) = parse_point(s)?;
    let (s, _) = arr(s)?;
    let (s, end) = parse_point(s)?;
    Ok((s, from_points_hvd(start, end)))
}

fn parse_lines_hv(s: &str) -> IResult<&str, Vec<Line>> {
    map_res(separated_list1(line_ending, parse_line_hv), |v| {
        Ok(v.into_iter().flatten().collect()) as Result<_, Vec<Line>>
    })(s)
}

fn parse_lines_hvd(s: &str) -> IResult<&str, Vec<Line>> {
    map_res(separated_list1(line_ending, parse_line_hvd), |v| {
        Ok(v.into_iter().flatten().collect()) as Result<_, Vec<Line>>
    })(s)
}

fn get_input() -> Result<String> {
    let mut s = String::new();
    let mut f = fs::File::open("day5.txt")?;
    f.read_to_string(&mut s)?;
    Ok(s)
}

#[derive(Debug)]
struct Overlaps(HashMap<Point, usize>);

impl Overlaps {
    fn from_lines(lines: Vec<Line>) -> Self {
        let mut m = HashMap::new();
        for p in lines.iter().flatten() {
            let c = m.entry(*p).or_insert(0);
            *c += 1;
        }
        Self(m)
    }
    fn filter_multi(&self) -> Vec<(&Point, &usize)> {
        self.0.iter().filter(|(_, &ct)| ct > 1).collect()
    }
    fn multi_count(&self) -> usize {
        self.filter_multi().len()
    }
}

pub fn day5_1() -> Result<usize> {
    let input = get_input()?;
    let (_, lines) = parse_lines_hv(&input).map_err(|_| anyhow!("parser error"))?;
    Ok(Overlaps::from_lines(lines).multi_count())
}

pub fn day5_2() -> Result<usize> {
    let input = get_input()?;
    let (_, lines) = parse_lines_hvd(&input).map_err(|_| anyhow!("parser error"))?;
    Ok(Overlaps::from_lines(lines).multi_count())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lines() {
        let s0 = Point { x: 1, y: 1 };
        let e0 = Point { x: 1, y: 3 };
        let expected_0 = vec![
            Point { x: 1, y: 1 },
            Point { x: 1, y: 2 },
            Point { x: 1, y: 3 },
        ];
        let actual_0 = from_points_hv(s0, e0).unwrap();
        assert_eq!(expected_0, actual_0);
        let s1 = Point { x: 9, y: 7 };
        let e1 = Point { x: 7, y: 7 };
        let expected_1 = vec![
            Point { x: 7, y: 7 },
            Point { x: 8, y: 7 },
            Point { x: 9, y: 7 },
        ];
        let actual_1 = from_points_hv(s1, e1).unwrap();
        assert_eq!(expected_1, actual_1);
    }

    #[test]
    fn test_overlaps_from_lines() {
        let lines = vec![
            vec![
                Point { x: 1, y: 1 },
                Point { x: 2, y: 1 },
                Point { x: 3, y: 1 },
            ],
            vec![
                Point { x: 1, y: 1 },
                Point { x: 1, y: 2 },
                Point { x: 1, y: 3 },
            ],
            vec![
                Point { x: 1, y: 2 },
                Point { x: 2, y: 2 },
                Point { x: 3, y: 2 },
            ],
        ];
        let o = Overlaps::from_lines(lines);
        let expected_m: HashMap<Point, usize> = [
            (Point { x: 1, y: 1 }, 2),
            (Point { x: 2, y: 1 }, 1),
            (Point { x: 3, y: 1 }, 1),
            (Point { x: 1, y: 2 }, 2),
            (Point { x: 1, y: 3 }, 1),
            (Point { x: 2, y: 2 }, 1),
            (Point { x: 3, y: 2 }, 1),
        ]
        .into();
        assert_eq!(expected_m, o.0);
    }

    #[test]
    fn test_overlaps_filter_multi() {
        let lines = vec![
            vec![
                Point { x: 1, y: 1 },
                Point { x: 2, y: 1 },
                Point { x: 3, y: 1 },
            ],
            vec![
                Point { x: 1, y: 1 },
                Point { x: 1, y: 2 },
                Point { x: 1, y: 3 },
            ],
            vec![
                Point { x: 1, y: 2 },
                Point { x: 2, y: 2 },
                Point { x: 3, y: 2 },
            ],
        ];
        let o = Overlaps::from_lines(lines);
        let expected_filtered = vec![(&Point { x: 1, y: 1 }, &2), (&Point { x: 1, y: 2 }, &2)];
        let mut filtered = o.filter_multi();
        filtered.sort_by_key(|v| v.0);
        assert_eq!(filtered, expected_filtered);
    }

    #[test]
    fn test_parse_line_h() -> Result<()> {
        let s = "100,100 -> 103,100";
        let expected = vec![
            Point { x: 100, y: 100 },
            Point { x: 101, y: 100 },
            Point { x: 102, y: 100 },
            Point { x: 103, y: 100 },
        ];
        let (_, l) = parse_line_hv(s)?;
        let l = l.unwrap();
        assert_eq!(expected, l);
        Ok(())
    }

    #[test]
    fn test_parse_lines() -> Result<()> {
        let s = r#"100,100 -> 103,100
200,203 -> 200,206
203,203 -> 204,210"#;
        let expected = vec![
            vec![
                Point { x: 100, y: 100 },
                Point { x: 101, y: 100 },
                Point { x: 102, y: 100 },
                Point { x: 103, y: 100 },
            ],
            vec![
                Point { x: 200, y: 203 },
                Point { x: 200, y: 204 },
                Point { x: 200, y: 205 },
                Point { x: 200, y: 206 },
            ],
        ];
        let (_, actual) = parse_lines_hv(s)?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_from_points_diag() {
        let s = Point { x: 5, y: 5 };
        let e_both_pos = Point { x: 10, y: 10 };
        let expected_both_pos = vec![
            Point { x: 5, y: 5 },
            Point { x: 6, y: 6 },
            Point { x: 7, y: 7 },
            Point { x: 8, y: 8 },
            Point { x: 9, y: 9 },
            Point { x: 10, y: 10 },
        ];
        let actual_both_pos = from_points_diag(s, e_both_pos).expect("diag both pos is None");
        assert_eq!(actual_both_pos, expected_both_pos, "diag both pos");
        let e_x_neg = Point { x: 0, y: 10 };
        let expected_x_neg = vec![
            Point { x: 5, y: 5 },
            Point { x: 4, y: 6 },
            Point { x: 3, y: 7 },
            Point { x: 2, y: 8 },
            Point { x: 1, y: 9 },
            Point { x: 0, y: 10 },
        ];
        let actual_x_neg = from_points_diag(s, e_x_neg).expect("diag x neg is None");
        assert_eq!(expected_x_neg, actual_x_neg, "diag x neg");
        let e_y_neg = Point { x: 10, y: 0 };
        let expected_y_neg = vec![
            Point { x: 5, y: 5 },
            Point { x: 6, y: 4 },
            Point { x: 7, y: 3 },
            Point { x: 8, y: 2 },
            Point { x: 9, y: 1 },
            Point { x: 10, y: 0 },
        ];
        let actual_y_neg = from_points_diag(s, e_y_neg).expect("diag y neg is None");
        assert_eq!(expected_y_neg, actual_y_neg, "diag y neg");
        let e_both_neg = Point { x: 0, y: 0 };
        let expected_both_neg = vec![
            Point { x: 5, y: 5 },
            Point { x: 4, y: 4 },
            Point { x: 3, y: 3 },
            Point { x: 2, y: 2 },
            Point { x: 1, y: 1 },
            Point { x: 0, y: 0 },
        ];
        let actual_both_neg = from_points_diag(s, e_both_neg).expect("diag both neg is None");
        assert_eq!(expected_both_neg, actual_both_neg, "diag both neg");
    }
}
