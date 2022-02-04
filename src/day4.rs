use crate::get_input::get_input;
use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space0, space1},
    combinator::{eof, map_res},
    multi::{count, separated_list0, separated_list1},
    sequence::{preceded, terminated},
    IResult,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct BingoSquare {
    number: u32,
    hit: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct BingoBoard {
    squares: Vec<BingoSquare>,
    x_length: usize,
    y_length: usize,
}

type Balls = Vec<u32>;
type Boards = Vec<BingoBoard>;

impl BingoBoard {
    fn lines(&self) -> Vec<Vec<&BingoSquare>> {
        let mut ls = Vec::new();
        let mut it = self.squares.iter();
        let i = it.by_ref();
        for _ in 0..self.y_length {
            ls.push(i.take(self.x_length).collect());
        }
        ls
    }
    fn cols(&self) -> Vec<Vec<&BingoSquare>> {
        let mut cs = Vec::new();
        for x in 0..self.x_length {
            let mut c = Vec::new();
            for y in 0..self.y_length {
                c.push(self.squares.get(x + (y * self.x_length)).unwrap());
            }
            cs.push(c);
        }
        cs
    }
    fn line_wins(line: &[&BingoSquare]) -> bool {
        line.iter().all(|&sq| sq.hit)
    }
    fn is_winner(&self) -> bool {
        for line in self.lines().iter() {
            if Self::line_wins(line) {
                return true;
            }
        }
        for col in self.cols().iter() {
            if Self::line_wins(col) {
                return true;
            }
        }
        false
    }
    fn mark_hit(&mut self, num: &u32) {
        for b in self.squares.iter_mut() {
            if b.number == *num {
                b.hit = true;
            }
        }
    }
    fn sum_unmarked(&self) -> u32 {
        self.squares
            .iter()
            .filter(|&sq| !sq.hit)
            .map(|sq| sq.number)
            .sum()
    }
}

fn parse_balls(s: &str) -> IResult<&str, Balls> {
    map_res(separated_list1(tag(","), digit1), |xs: Vec<&str>| {
        xs.into_iter().map(|x| x.parse()).collect()
    })(s)
}

fn parse_bingo_num(s: &str) -> IResult<&str, u32> {
    map_res(preceded(space0, digit1), |s: &str| s.parse())(s)
}

fn parse_line(s: &str) -> IResult<&str, Vec<u32>> {
    terminated(
        separated_list1(space1, parse_bingo_num),
        alt((eof, line_ending)),
    )(s)
}

fn parse_board(s: &str) -> IResult<&str, BingoBoard> {
    let (s, lines) = count(parse_line, 5)(s)?;
    let y_length = lines.len();
    let x_length = lines.get(0).unwrap().len();
    let squares = lines
        .into_iter()
        .flatten()
        .map(|v| BingoSquare {
            number: v,
            hit: false,
        })
        .collect();
    Ok((
        s,
        BingoBoard {
            squares,
            y_length,
            x_length,
        },
    ))
}

fn board_sep(s: &str) -> IResult<&str, &str> {
    line_ending(s)
}

fn parse_boards(s: &str) -> IResult<&str, Boards> {
    separated_list0(board_sep, parse_board)(s)
}

fn parse_bingo(s: &str) -> IResult<&str, (Balls, Boards)> {
    let (s, balls) = parse_balls(s)?;
    let (s, _) = count(line_ending, 2)(s)?;
    let (_, boards) = parse_boards(s)?;
    Ok((s, (balls, boards)))
}

fn get_bingo() -> Result<(Balls, Boards)> {
    let input = get_input("day4.txt")?;
    let (balls, boards) = parse_bingo(&input).map_err(|_| anyhow!("parser error"))?.1;
    Ok((balls, boards))
}

fn get_winning_sum(balls: Balls, mut boards: Boards) -> Result<u32> {
    for ball in balls.iter() {
        for board in boards.iter_mut() {
            board.mark_hit(ball);
            if board.is_winner() {
                return Ok(board.sum_unmarked() * ball);
            }
        }
    }
    Err(anyhow!("no winner!"))
}

fn get_last_winner(balls: Balls, mut boards: Boards) -> Result<(u32, BingoBoard)> {
    let mut winners = Vec::new();
    for ball in balls.into_iter() {
        let mut non_winners = Vec::new();
        for board in boards.iter_mut() {
            board.mark_hit(&ball);
            if !board.is_winner() {
                non_winners.push(board.clone());
            } else {
                winners.push(board.clone());
            }
        }
        if non_winners.is_empty() {
            return Ok((ball, winners[winners.len() - 1].clone()));
        }
        boards = non_winners;
    }
    Err(anyhow!("no winner!"))
}

pub fn day4_1() -> Result<u32> {
    let (balls, boards) = get_bingo()?;
    get_winning_sum(balls, boards)
}

pub fn day4_2() -> Result<u32> {
    let (balls, boards) = get_bingo()?;
    let (last_ball, last_winning_board) = get_last_winner(balls, boards)?;
    Ok(last_ball * last_winning_board.sum_unmarked())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mk_test_board() -> BingoBoard {
        BingoBoard {
            squares: (0..25)
                .map(|n| {
                    let h = n % 2 == 0;
                    BingoSquare { number: n, hit: h }
                })
                .collect(),
            x_length: 5,
            y_length: 5,
        }
    }
    #[test]
    fn test_lines() {
        let b = mk_test_board();
        let lines = b.lines();
        let fst = vec![
            BingoSquare {
                number: 0,
                hit: true,
            },
            BingoSquare {
                number: 1,
                hit: false,
            },
            BingoSquare {
                number: 2,
                hit: true,
            },
            BingoSquare {
                number: 3,
                hit: false,
            },
            BingoSquare {
                number: 4,
                hit: true,
            },
        ];
        let snd = vec![
            BingoSquare {
                number: 5,
                hit: false,
            },
            BingoSquare {
                number: 6,
                hit: true,
            },
            BingoSquare {
                number: 7,
                hit: false,
            },
            BingoSquare {
                number: 8,
                hit: true,
            },
            BingoSquare {
                number: 9,
                hit: false,
            },
        ];
        let third = vec![
            BingoSquare {
                number: 10,
                hit: true,
            },
            BingoSquare {
                number: 11,
                hit: false,
            },
            BingoSquare {
                number: 12,
                hit: true,
            },
            BingoSquare {
                number: 13,
                hit: false,
            },
            BingoSquare {
                number: 14,
                hit: true,
            },
        ];
        let fourth = vec![
            BingoSquare {
                number: 15,
                hit: false,
            },
            BingoSquare {
                number: 16,
                hit: true,
            },
            BingoSquare {
                number: 17,
                hit: false,
            },
            BingoSquare {
                number: 18,
                hit: true,
            },
            BingoSquare {
                number: 19,
                hit: false,
            },
        ];
        let fifth = vec![
            BingoSquare {
                number: 20,
                hit: true,
            },
            BingoSquare {
                number: 21,
                hit: false,
            },
            BingoSquare {
                number: 22,
                hit: true,
            },
            BingoSquare {
                number: 23,
                hit: false,
            },
            BingoSquare {
                number: 24,
                hit: true,
            },
        ];
        let expected = vec![
            fst.iter().collect::<Vec<&BingoSquare>>(),
            snd.iter().collect::<Vec<&BingoSquare>>(),
            third.iter().collect::<Vec<&BingoSquare>>(),
            fourth.iter().collect::<Vec<&BingoSquare>>(),
            fifth.iter().collect::<Vec<&BingoSquare>>(),
        ];
        assert_eq!(expected, lines);
    }

    #[test]
    fn test_cols() {
        let b = mk_test_board();
        let cols = b.cols();
        let fst = vec![
            BingoSquare {
                number: 0,
                hit: true,
            },
            BingoSquare {
                number: 5,
                hit: false,
            },
            BingoSquare {
                number: 10,
                hit: true,
            },
            BingoSquare {
                number: 15,
                hit: false,
            },
            BingoSquare {
                number: 20,
                hit: true,
            },
        ];
        let snd = vec![
            BingoSquare {
                number: 1,
                hit: false,
            },
            BingoSquare {
                number: 6,
                hit: true,
            },
            BingoSquare {
                number: 11,
                hit: false,
            },
            BingoSquare {
                number: 16,
                hit: true,
            },
            BingoSquare {
                number: 21,
                hit: false,
            },
        ];
        let third = vec![
            BingoSquare {
                number: 2,
                hit: true,
            },
            BingoSquare {
                number: 7,
                hit: false,
            },
            BingoSquare {
                number: 12,
                hit: true,
            },
            BingoSquare {
                number: 17,
                hit: false,
            },
            BingoSquare {
                number: 22,
                hit: true,
            },
        ];
        let fourth = vec![
            BingoSquare {
                number: 3,
                hit: false,
            },
            BingoSquare {
                number: 8,
                hit: true,
            },
            BingoSquare {
                number: 13,
                hit: false,
            },
            BingoSquare {
                number: 18,
                hit: true,
            },
            BingoSquare {
                number: 23,
                hit: false,
            },
        ];
        let fifth = vec![
            BingoSquare {
                number: 4,
                hit: true,
            },
            BingoSquare {
                number: 9,
                hit: false,
            },
            BingoSquare {
                number: 14,
                hit: true,
            },
            BingoSquare {
                number: 19,
                hit: false,
            },
            BingoSquare {
                number: 24,
                hit: true,
            },
        ];
        let expected = vec![
            fst.iter().collect::<Vec<&BingoSquare>>(),
            snd.iter().collect::<Vec<&BingoSquare>>(),
            third.iter().collect::<Vec<&BingoSquare>>(),
            fourth.iter().collect::<Vec<&BingoSquare>>(),
            fifth.iter().collect::<Vec<&BingoSquare>>(),
        ];
        assert_eq!(expected, cols);
    }

    #[test]
    fn test_line_wins() {
        let b = BingoBoard {
            x_length: 3,
            y_length: 3,
            squares: vec![
                BingoSquare {
                    number: 0,
                    hit: false,
                },
                BingoSquare {
                    number: 1,
                    hit: true,
                },
                BingoSquare {
                    number: 2,
                    hit: false,
                },
                BingoSquare {
                    number: 3,
                    hit: true,
                },
                BingoSquare {
                    number: 4,
                    hit: true,
                },
                BingoSquare {
                    number: 5,
                    hit: true,
                },
                BingoSquare {
                    number: 6,
                    hit: false,
                },
                BingoSquare {
                    number: 7,
                    hit: false,
                },
                BingoSquare {
                    number: 8,
                    hit: false,
                },
            ],
        };
        assert!(b.is_winner());
    }

    #[test]
    fn test_col_wins() {
        let b = BingoBoard {
            x_length: 3,
            y_length: 3,
            squares: vec![
                BingoSquare {
                    number: 0,
                    hit: false,
                },
                BingoSquare {
                    number: 1,
                    hit: true,
                },
                BingoSquare {
                    number: 2,
                    hit: false,
                },
                BingoSquare {
                    number: 3,
                    hit: false,
                },
                BingoSquare {
                    number: 4,
                    hit: true,
                },
                BingoSquare {
                    number: 5,
                    hit: false,
                },
                BingoSquare {
                    number: 6,
                    hit: false,
                },
                BingoSquare {
                    number: 7,
                    hit: true,
                },
                BingoSquare {
                    number: 8,
                    hit: false,
                },
            ],
        };
        assert!(b.is_winner());
    }

    #[test]
    fn test_not_winner() {
        let b = BingoBoard {
            x_length: 3,
            y_length: 3,
            squares: vec![
                BingoSquare {
                    number: 0,
                    hit: false,
                },
                BingoSquare {
                    number: 1,
                    hit: false,
                },
                BingoSquare {
                    number: 2,
                    hit: false,
                },
                BingoSquare {
                    number: 3,
                    hit: false,
                },
                BingoSquare {
                    number: 4,
                    hit: true,
                },
                BingoSquare {
                    number: 5,
                    hit: false,
                },
                BingoSquare {
                    number: 6,
                    hit: false,
                },
                BingoSquare {
                    number: 7,
                    hit: true,
                },
                BingoSquare {
                    number: 8,
                    hit: false,
                },
            ],
        };
        assert!(!b.is_winner());
    }

    #[test]
    fn test_mark_hit() {
        let mut b = BingoBoard {
            x_length: 3,
            y_length: 3,
            squares: vec![
                BingoSquare {
                    number: 0,
                    hit: false,
                },
                BingoSquare {
                    number: 1,
                    hit: false,
                },
                BingoSquare {
                    number: 2,
                    hit: false,
                },
                BingoSquare {
                    number: 3,
                    hit: false,
                },
                BingoSquare {
                    number: 4,
                    hit: true,
                },
                BingoSquare {
                    number: 5,
                    hit: false,
                },
                BingoSquare {
                    number: 6,
                    hit: false,
                },
                BingoSquare {
                    number: 7,
                    hit: true,
                },
                BingoSquare {
                    number: 8,
                    hit: false,
                },
            ],
        };
        b.mark_hit(&6);
        let expected = BingoBoard {
            x_length: 3,
            y_length: 3,
            squares: vec![
                BingoSquare {
                    number: 0,
                    hit: false,
                },
                BingoSquare {
                    number: 1,
                    hit: false,
                },
                BingoSquare {
                    number: 2,
                    hit: false,
                },
                BingoSquare {
                    number: 3,
                    hit: false,
                },
                BingoSquare {
                    number: 4,
                    hit: true,
                },
                BingoSquare {
                    number: 5,
                    hit: false,
                },
                BingoSquare {
                    number: 6,
                    hit: true,
                },
                BingoSquare {
                    number: 7,
                    hit: true,
                },
                BingoSquare {
                    number: 8,
                    hit: false,
                },
            ],
        };
        assert_eq!(expected, b);
    }

    #[test]
    fn test_sum_unmarked() {
        let b = BingoBoard {
            x_length: 3,
            y_length: 3,
            squares: vec![
                BingoSquare {
                    number: 0,
                    hit: false,
                },
                BingoSquare {
                    number: 1,
                    hit: false,
                },
                BingoSquare {
                    number: 2,
                    hit: false,
                },
                BingoSquare {
                    number: 3,
                    hit: false,
                },
                BingoSquare {
                    number: 4,
                    hit: true,
                },
                BingoSquare {
                    number: 5,
                    hit: false,
                },
                BingoSquare {
                    number: 6,
                    hit: false,
                },
                BingoSquare {
                    number: 7,
                    hit: true,
                },
                BingoSquare {
                    number: 8,
                    hit: false,
                },
            ],
        };
        assert_eq!(25, b.sum_unmarked());
    }

    #[test]
    fn test_parse_board() -> Result<()> {
        let s = r#"0  1  2  3  4
 5  6  7  8  9
10 11 12 13 14
15 16 17 18 19
20 21 22 23 24"#;
        let (_, parsed) = parse_board(s)?;
        assert_eq!(5, parsed.y_length);
        assert_eq!(5, parsed.x_length);
        Ok(())
    }

    #[test]
    fn test_parse_boards() -> Result<()> {
        let s = r#" 0  1  2  3  4
 5  6  7  8  9
10 11 12 13 14
15 16 17 18 19
20 21 22 23 24

10 11 12 13 14
15 16 17 18 19
20 21 22 23 24
25 26 27 28 29
30 31 32 33 34"#;
        let (_, boards) = parse_boards(s)?;
        assert_eq!(2, boards.len(), "num_boards");
        assert_eq!(5, boards[0].y_length);
        assert_eq!(5, boards[1].y_length);
        assert_eq!(5, boards[0].x_length);
        assert_eq!(5, boards[1].x_length);
        Ok(())
    }

    #[test]
    fn test_get_winning_sum() -> Result<()> {
        let s = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#;
        let (balls, boards) = parse_bingo(s).map_err(|_| anyhow!("parser error"))?.1;
        let res = get_winning_sum(balls, boards)?;
        assert_eq!(4512, res);
        Ok(())
    }

    #[test]
    fn test_get_last_winner() -> Result<()> {
        let s = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7"#;
        let (balls, boards) = parse_bingo(s).map_err(|_| anyhow!("parser error"))?.1;
        let (last_winning_ball, last_winning_board) = get_last_winner(balls, boards)?;
        let expected_last_board = BingoBoard {
            x_length: 5,
            y_length: 5,
            squares: vec![
                BingoSquare {
                    number: 3,
                    hit: false,
                },
                BingoSquare {
                    number: 15,
                    hit: false,
                },
                BingoSquare {
                    number: 0,
                    hit: true,
                },
                BingoSquare {
                    number: 2,
                    hit: true,
                },
                BingoSquare {
                    number: 22,
                    hit: false,
                },
                BingoSquare {
                    number: 9,
                    hit: true,
                },
                BingoSquare {
                    number: 18,
                    hit: false,
                },
                BingoSquare {
                    number: 13,
                    hit: true,
                },
                BingoSquare {
                    number: 17,
                    hit: true,
                },
                BingoSquare {
                    number: 5,
                    hit: true,
                },
                BingoSquare {
                    number: 19,
                    hit: false,
                },
                BingoSquare {
                    number: 8,
                    hit: false,
                },
                BingoSquare {
                    number: 7,
                    hit: true,
                },
                BingoSquare {
                    number: 25,
                    hit: false,
                },
                BingoSquare {
                    number: 23,
                    hit: true,
                },
                BingoSquare {
                    number: 20,
                    hit: false,
                },
                BingoSquare {
                    number: 11,
                    hit: true,
                },
                BingoSquare {
                    number: 10,
                    hit: true,
                },
                BingoSquare {
                    number: 24,
                    hit: true,
                },
                BingoSquare {
                    number: 4,
                    hit: true,
                },
                BingoSquare {
                    number: 14,
                    hit: true,
                },
                BingoSquare {
                    number: 21,
                    hit: true,
                },
                BingoSquare {
                    number: 16,
                    hit: true,
                },
                BingoSquare {
                    number: 12,
                    hit: false,
                },
                BingoSquare {
                    number: 6,
                    hit: false,
                },
            ],
        };
        assert_eq!(13, last_winning_ball);
        assert_eq!(expected_last_board, last_winning_board);
        assert_eq!(1924, last_winning_ball * last_winning_board.sum_unmarked());
        Ok(())
    }
}
