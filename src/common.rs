use crate::bitop::*;
use colored::*;
use std::{io::*, mem::swap, str::FromStr};

pub type Mask = u64; // bit-mask of board
pub type Hint = usize; // size of continuous black blocks

#[derive(Default, Debug, Clone)]
pub struct Input {
    pub width: usize,
    pub height: usize,
    pub row_hints: Vec<Vec<Hint>>,
    pub column_hints: Vec<Vec<Hint>>,
}

impl Input {
    pub fn from_stdin() -> Input {
        let mut input = Input::default();
        input.height = read();
        input.width = read();
        input.row_hints.resize(input.height, Vec::new());
        input.column_hints.resize(input.width, Vec::new());
        for i in 0..input.height {
            let n = read();
            input.row_hints[i].resize(n, 0);
            for j in 0..n {
                input.row_hints[i][j] = read();
            }
        }
        for i in 0..input.width {
            let n = read();
            input.column_hints[i].resize(n, 0);
            for j in 0..n {
                input.column_hints[i][j] = read();
            }
        }
        input
    }

    pub fn flip(&self) -> Input {
        let mut result = self.clone();
        swap(&mut result.height, &mut result.width);
        swap(&mut result.row_hints, &mut result.column_hints);
        result
    }
}

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

#[derive(PartialEq, Clone)]
pub struct State {
    pub height: usize,
    pub width: usize,
    pub white: Vec<Mask>,
    pub black: Vec<Mask>,
}

impl State {
    pub fn new(height: usize, width: usize) -> State {
        assert!(width < std::mem::size_of::<Mask>() * 8);
        State {
            height: height,
            width: width,
            white: vec![0; height],
            black: vec![0; height],
        }
    }

    pub fn flip(&self) -> State {
        let mut result = State {
            height: self.width,
            width: self.height,
            white: vec![0; self.width],
            black: vec![0; self.width],
        };
        for i in 0..self.height {
            for j in 0..self.width {
                if set(self.white[i], j) {
                    result.white[j] |= 1 << i;
                }
                if set(self.black[i], j) {
                    result.black[j] |= 1 << i;
                }
            }
        }
        result
    }

    // todo: is_solved
    pub fn solved(&self, input: &Input) -> bool {
        for (a, b) in input.row_hints.iter().zip(self.black.iter()) {
            if a.iter().sum::<usize>() != b.count_ones() as usize {
                return false;
            }
        }
        true
    }

    pub fn to_solution(&self) -> Solution {
        let mut result = vec![vec![Cell::Black; self.width]; self.height];
        for i in 0..self.height {
            for j in 0..self.width {
                result[i][j] = if set(self.white[i], j) {
                    Cell::White
                } else if set(self.black[i], j) {
                    Cell::Black
                } else {
                    panic!("empty cell exists");
                }
            }
        }
        result
    }
}

#[derive(Clone, Copy)]
pub enum Cell {
    Black,
    White,
}

pub type Solution = Vec<Vec<Cell>>;

pub fn print(input: &Input, solution: &Solution) {
    let h = input.height * 2 + 1;
    let w = input.width * 2 + 1;
    let mut grid = vec![vec![' '; w]; h];
    for i in 0..h {
        for j in 0..w {
            grid[i][j] = match (i % 2, j % 2) {
                (0, 0) => '+',
                (0, 1) => '-',
                (1, 0) => '|',
                _ => grid[i][j],
            };
        }
    }
    for i in 0..input.height {
        for j in 0..input.width {
            grid[i * 2 + 1][j * 2 + 1] = match solution[i][j] {
                Cell::White => ' ',
                Cell::Black => 'x',
            }
        }
    }
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == 'x' {
                print!("{}", "x".cyan().bold());
            } else {
                print!("{}", grid[i][j]);
            }
        }
        println!("");
    }
}
