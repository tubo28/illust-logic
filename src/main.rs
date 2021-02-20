use colored::*;
use std::{collections::VecDeque, io::*, mem::swap, str::FromStr};

type Mask = u64;
type Hint = usize;

fn search_impl(
    width: usize,
    cur: Mask,
    pos: usize,
    hints: &[Hint],
    empty: Mask,
    filled: Mask,
    result: &mut Vec<Mask>,
) {
    match hints.first() {
        None => {
            if subset_bit(cur, filled) {
                result.push(cur);
            }
        }
        Some(&hint) => {
            let mask = all_bit(hint);
            for i in pos..(pos + width - hint + 1) {
                if i + hint <= width {
                    let next = cur | mask << i;
                    let cur_head = all_bit(hint + i);
                    if distinct_bit(empty, next)
                        && !set_bit(filled, i + hint)
                        && subset_bit(next, filled & cur_head)
                    {
                        search_impl(
                            width,
                            next,
                            i + hint + 1,
                            &hints[1..],
                            empty,
                            filled,
                            result,
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn search_works() {
    assert_eq!(
        search(8, &[2, 3], 0b0000_0000, 0b0000_0000),
        [0b00111011, 0b01110011, 0b11100011, 0b01110110, 0b11100110, 0b11101100]
    );
    assert_eq!(
        search(8, &[2, 3], 0b0000_0000, 0b0000_0100),
        [0b01110110, 0b11100110, 0b11101100]
    );
}

fn search(width: usize, hints: &[Hint], empty: Mask, filled: Mask) -> Vec<Mask> {
    let mut result = Vec::<Mask>::new();
    search_impl(width, 0, 0, hints, empty, filled, &mut result);
    result
}

fn search_row(input: &Input, state: &State, row: usize) -> Vec<Mask> {
    search(
        input.width,
        &input.row_hints[row],
        state.empty[row],
        state.filled[row],
    )
}

#[derive(Default, Debug, Clone)]
struct Input {
    width: usize,
    height: usize,
    row_hints: Vec<Vec<Hint>>,
    column_hints: Vec<Vec<Hint>>,
}

impl Input {
    fn from_stdin() -> Input {
        let mut input = Input::default();
        input.height = Self::read();
        input.width = Self::read();
        input.row_hints.resize(input.height, Vec::new());
        input.column_hints.resize(input.width, Vec::new());
        for i in 0..input.height {
            let n = Self::read();
            input.row_hints[i].resize(n, 0);
            for j in 0..n {
                input.row_hints[i][j] = Self::read();
            }
        }
        for i in 0..input.width {
            let n = Self::read();
            input.column_hints[i].resize(n, 0);
            for j in 0..n {
                input.column_hints[i][j] = Self::read();
            }
        }
        input
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

    fn flip(&self) -> Input {
        let mut result = self.clone();
        swap(&mut result.height, &mut result.width);
        swap(&mut result.row_hints, &mut result.column_hints);
        result
    }
}

struct State {
    height: usize,
    width: usize,
    empty: Vec<Mask>,
    filled: Vec<Mask>,
}

impl State {
    fn new(height: usize, width: usize) -> State {
        assert!(width < std::mem::size_of::<Mask>() * 8);
        State {
            height: height,
            width: width,
            empty: vec![0; height],
            filled: vec![0; height],
        }
    }

    fn flip(&self) -> State {
        let mut result = State {
            height: self.width,
            width: self.height,
            empty: vec![0; self.width],
            filled: vec![0; self.width],
        };
        for i in 0..self.height {
            for j in 0..self.width {
                if set_bit(self.empty[i], j) {
                    result.empty[j] |= 1 << i;
                }
                if set_bit(self.filled[i], j) {
                    result.filled[j] |= 1 << i;
                }
            }
        }
        result
    }
}

#[test]
fn all_bit_works() {
    assert_eq!(all_bit(0), 0b0);
    assert_eq!(all_bit(4), 0b1111);
}
fn all_bit(width: usize) -> Mask {
    (1 << width) - 1
}

#[test]
fn subset_bit_works() {
    assert!(subset_bit(0b0110, 0b0110));
    assert!(subset_bit(0b0110, 0b0100));
    assert!(!subset_bit(0b0101, 0b0010));
}
fn subset_bit(sup: Mask, sub: Mask) -> bool {
    sup & sub == sub
}

#[test]
fn minus_bit_works() {
    assert_eq!(minus_bit(0b1100, 0b1010), 0b0100);
}
fn minus_bit(x: Mask, y: Mask) -> Mask {
    // 0 0 -> 0
    // 0 1 -> 0
    // 1 1 -> 0
    // 1 0 -> 1
    x & !y
}

fn distinct_bit(x: Mask, y: Mask) -> bool {
    x & y == 0
}

fn set_bit(x: Mask, i: usize) -> bool {
    x >> i & 1 == 1
}

enum ActiveLine {
    Row(usize),
    Column(usize),
}

fn solve(input: &Input) {
    let flipped_input = input.flip();

    let mut state = State::new(input.height, input.width);
    let mut row_active = vec![true; input.height];
    let mut column_active = vec![true; input.width];
    let mut queue = VecDeque::new();
    for i in 0..input.height {
        queue.push_back(ActiveLine::Row(i));
    }
    for j in 0..input.width {
        queue.push_back(ActiveLine::Column(j));
    }

    while let Some(head) = queue.pop_front() {
        match head {
            ActiveLine::Row(row) => {
                let candidates = search_row(input, &state, row);
                let updated = apply_search_row_result(&mut state, input, row, &candidates);

                row_active[row] = false;
                for j in 0..input.width {
                    if set_bit(updated, j) && !column_active[j] {
                        column_active[j] = true;
                        queue.push_back(ActiveLine::Column(j));
                    }
                }
            }
            ActiveLine::Column(column) => {
                let mut flipped_state = state.flip();
                let candidates = search_row(&flipped_input, &flipped_state, column);
                let updated =
                    apply_search_row_result(&mut flipped_state, input, column, &candidates);
                state = flipped_state.flip();

                column_active[column] = false;
                for i in 0..input.height {
                    if set_bit(updated, i) && !row_active[i] {
                        row_active[i] = true;
                        queue.push_back(ActiveLine::Row(i));
                    }
                }
            }
        }
    }
    print(input, &state)
}

// returns updated bits
fn apply_search_row_result(
    state: &mut State,
    input: &Input,
    row: usize,
    candidates: &[Mask],
) -> Mask {
    assert!(!candidates.is_empty());
    let must_fill = candidates
        .iter()
        .fold(all_bit(input.width), |acc, &x| acc & x);
    let must_empty = !candidates.iter().fold(0, |acc, &x| acc | x);
    let must_empty = must_empty & all_bit(input.width);

    let filled = minus_bit(must_fill, state.filled[row]);
    let removed = minus_bit(must_empty, state.empty[row]);
    state.filled[row] |= must_fill;
    state.empty[row] |= must_empty;
    assert!(state.filled[row] & state.empty[row] == 0);

    filled | removed
}

fn print(input: &Input, state: &State) {
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
            grid[i * 2 + 1][j * 2 + 1] =
                match (set_bit(state.filled[i], j), set_bit(state.empty[i], j)) {
                    (true, false) => '#',
                    (false, true) => '/',
                    (false, false) => ' ',
                    (true, true) => panic!(),
                }
        }
    }
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == '#' {
                print!("{}", "#".cyan().bold());
            } else {
                print!("{}", grid[i][j]);
            }
        }
        println!("");
    }
}

fn main() {
    let input = Input::from_stdin();
    solve(&input);
}
