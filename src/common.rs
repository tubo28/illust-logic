use crate::bitop::*;
use colored::*;
use std::{io::*, mem::swap, str::FromStr};

pub type Mask = u64; // bit-mask of row
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
        input.row_hints = vec![Vec::new(); input.height];
        input.column_hints = vec![Vec::new(); input.width];
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

    pub fn transpose(&self) -> Input {
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
        debug_assert!(width < std::mem::size_of::<Mask>() * 8);
        State {
            height: height,
            width: width,
            white: vec![0; height],
            black: vec![0; height],
        }
    }

    pub fn transpose(&self) -> State {
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

    pub fn solved(&self, input: &Input) -> bool {
        for (a, b) in input.row_hints.iter().zip(self.black.iter()) {
            if a.iter().sum::<usize>() != b.count_ones() as usize {
                return false;
            }
        }
        true
    }

    pub fn white(&self, i: usize, j: usize) -> bool {
        self.white[i] >> j & 1 == 1
    }

    pub fn black(&self, i: usize, j: usize) -> bool {
        self.black[i] >> j & 1 == 1
    }

    pub fn empty(&self, i: usize, j: usize) -> bool {
        !self.white(i, j) && !self.black(i, j)
    }

    pub fn set(&mut self, i: usize, j: usize, cell: Cell) {
        debug_assert!(self.empty(i, j));
        match cell {
            Cell::Black => self.black[i] |= 1 << j,
            Cell::White => self.white[i] |= 1 << j,
        }
    }

    pub fn flip(&mut self, i: usize, j: usize) {
        debug_assert!(!self.empty(i, j));
        self.black[i] ^= 1 << j;
        self.white[i] ^= 1 << j;
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
    for i in 0..input.height {
        for j in 0..input.width {
            match solution[i][j] {
                Cell::Black => print!("{}", "x".cyan().bold()),
                Cell::White => print!("{}", "."),
            }
        }
        println!()
    }
}
