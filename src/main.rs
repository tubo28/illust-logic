use std::{
    collections::{BTreeSet, HashSet, VecDeque},
    io::*,
    unimplemented,
};
use std::{mem::swap, str::FromStr};

type Mask = u64;
type BlockSize = usize;

fn search_impl(
    width: usize,
    cur: Mask,
    pos: usize,
    blocks: &[BlockSize],
    empty: Mask,
    filled: Mask,
    result: &mut Vec<Mask>,
) {
    if blocks.is_empty() {
        if subset_bit(cur, filled) {
            result.push(cur);
        }
        return;
    }

    let cur_block = blocks[0];
    let mask = all_bit(cur_block);
    for i in pos..(pos + width - cur_block + 1) {
        if i + cur_block <= width {
            let next = cur | mask << i;
            let cur_head = all_bit(cur_block + i);
            if distinct_bit(empty, next)
                && !set_bit(filled, i + cur_block)
                && subset_bit(next, filled & cur_head)
            {
                search_impl(
                    width,
                    next,
                    i + cur_block + 1,
                    &blocks[1..],
                    empty,
                    filled,
                    result,
                );
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
fn search(width: usize, blocks: &[BlockSize], empty: Mask, filled: Mask) -> Vec<Mask> {
    let mut result = Vec::<Mask>::new();
    search_impl(width, 0, 0, blocks, empty, filled, &mut result);
    result
}

fn search_row(input: &Input, state: &State, row: usize) -> Vec<Mask> {
    search(
        input.width,
        &input.row_blocks[row],
        state.empty[row],
        state.filled[row],
    )
}

fn search_column(input: &Input, column: usize, empty: Mask, filled: Mask) -> Vec<Mask> {
    search(input.height, &input.column_blocks[column], empty, filled)
}

#[derive(Default, Debug, Clone)]
struct Input {
    width: usize,
    height: usize,
    row_blocks: Vec<Vec<BlockSize>>,
    column_blocks: Vec<Vec<BlockSize>>,
}

impl Input {
    fn from_stdin() -> Input {
        let mut input = Input::default();
        input.height = Self::read();
        input.width = Self::read();
        input.row_blocks.resize(input.height, Vec::new());
        input.column_blocks.resize(input.width, Vec::new());
        for i in 0..input.height {
            let n = Self::read();
            input.row_blocks[i].resize(n, 0);
            for j in 0..n {
                input.row_blocks[i][j] = Self::read();
            }
        }
        for i in 0..input.width {
            let n = Self::read();
            input.column_blocks[i].resize(n, 0);
            for j in 0..n {
                input.column_blocks[i][j] = Self::read();
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
        swap(&mut result.row_blocks, &mut result.column_blocks);
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

fn distinct_bit(x: Mask, y: Mask) -> bool {
    x & y == 0
}

fn set_bit(x: Mask, i: usize) -> bool {
    x >> i & 1 == 1
}

enum SearchObject {
    Row(usize),
    Column(usize),
}

fn solve(input: &Input) {
    let flipped_input = input.flip();

    let mut state = State::new(input.height, input.width);
    let mut row_active = vec![true; input.height];
    let mut col_active = vec![true; input.width];
    let mut queue = VecDeque::new();
    for i in 0..input.height {
        queue.push_back(SearchObject::Row(i));
    }
    for j in 0..input.width {
        queue.push_back(SearchObject::Column(j));
    }

    while let Some(head) = queue.pop_front() {
        match head {
            SearchObject::Row(row) if row_active[row] => {
                let candidates = search_row(input, &state, row);
                apply_search_result(&mut state, input, row, &candidates);
            }
            SearchObject::Column(column) if col_active[column] => {
                let mut flipped_state = state.flip();
                let candidates = search_row(&flipped_input, &flipped_state, column);
                println!("{}", candidates.len());
                apply_search_result(&mut flipped_state, input, column, &candidates);
                state = flipped_state.flip();
            }
            _ => continue,
        }
        print(input, &state)
    }
}

fn apply_search_result(state: &mut State, input: &Input, row: usize, candidates: &[Mask]) {
    assert!(!candidates.is_empty());
    let and = candidates
        .iter()
        .fold(all_bit(input.width), |acc, &x| acc & x);
    let or = candidates.iter().fold(0, |acc, &x| acc | x);
    state.filled[row] |= and;
    state.empty[row] |= !or & all_bit(input.width);
    assert!(state.filled[row] & state.empty[row] == 0);
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
                    (true, false) => 'o',
                    (false, true) => 'x',
                    (false, false) => ' ',
                    (true, true) => panic!(),
                }
        }
    }
    for i in 0..h {
        for j in 0..w {
            print!("{}", grid[i][j]);
        }
        println!("");
    }
}

fn main() {
    let input = Input::from_stdin();
    solve(&input);
}
