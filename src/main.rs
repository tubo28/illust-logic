use colored::*;
use std::{collections::VecDeque, io::*, mem::swap, str::FromStr};

type Mask = u64; // bit-mask of board
type Hint = usize; // size of continuous black blocks

fn search_line_sub_dfs(
    width: usize,
    cur: Mask,
    pos: usize,
    hints: &[Hint],
    white: Mask,
    black: Mask,
    result: &mut Vec<Mask>,
) {
    match hints.first() {
        None => {
            if subset_bit(cur, black) {
                result.push(cur);
            }
        }
        Some(&hint) => {
            let mask = all_bit(hint);
            for i in pos..(pos + width - hint + 1) {
                if i + hint <= width {
                    let next = cur | mask << i;
                    let cur_head = all_bit(hint + i);
                    if distinct_bit(white, next)
                        && !set_bit(black, i + hint)
                        && subset_bit(next, black & cur_head)
                    {
                        search_line_sub_dfs(
                            width,
                            next,
                            i + hint + 1,
                            &hints[1..],
                            white,
                            black,
                            result,
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn search_line_sub_works() {
    assert_eq!(
        search_line_sub(8, &[2, 3], 0b0000_0000, 0b0000_0000),
        [0b00111011, 0b01110011, 0b11100011, 0b01110110, 0b11100110, 0b11101100]
    );
    assert_eq!(
        search_line_sub(8, &[2, 3], 0b0000_0000, 0b0000_0100),
        [0b01110110, 0b11100110, 0b11101100]
    );
    assert_eq!(
        search_line_sub(8, &[], 0b0000_0000, 0b0000_0000),
        [0b0000_0000]
    );
    // no solution
    assert_eq!(search_line_sub(8, &[3, 3, 3], 0b0000_0000, 0b0000_0000), []);
}
fn search_line_sub(width: usize, hints: &[Hint], white: Mask, black: Mask) -> Vec<Mask> {
    let mut result = Vec::<Mask>::new();
    search_line_sub_dfs(width, 0, 0, hints, white, black, &mut result);
    result
}

fn search_line(input: &Input, state: &State, row: usize) -> Vec<Mask> {
    search_line_sub(
        input.width,
        &input.row_hints[row],
        state.white[row],
        state.black[row],
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

#[derive(PartialEq, Clone)]
struct State {
    height: usize,
    width: usize,
    white: Vec<Mask>,
    black: Vec<Mask>,
}

impl State {
    fn new(height: usize, width: usize) -> State {
        assert!(width < std::mem::size_of::<Mask>() * 8);
        State {
            height: height,
            width: width,
            white: vec![0; height],
            black: vec![0; height],
        }
    }

    fn flip(&self) -> State {
        let mut result = State {
            height: self.width,
            width: self.height,
            white: vec![0; self.width],
            black: vec![0; self.width],
        };
        for i in 0..self.height {
            for j in 0..self.width {
                if set_bit(self.white[i], j) {
                    result.white[j] |= 1 << i;
                }
                if set_bit(self.black[i], j) {
                    result.black[j] |= 1 << i;
                }
            }
        }
        result
    }

    fn solved(&self, input: &Input) -> bool {
        for (a, b) in input.row_hints.iter().zip(self.black.iter()) {
            if a.iter().sum::<usize>() != b.count_ones() as usize {
                return false;
            }
        }
        true
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

fn deterministic_fill(input: &Input, state: &State) -> Option<State> {
    let mut state = state.clone();
    let flipped_input = input.flip();

    //let mut state = State::new(input.height, input.width);
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
                let candidates = search_line(input, &state, row);
                if candidates.is_empty() {
                    return None;
                }
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
                let candidates = search_line(&flipped_input, &flipped_state, column);
                if candidates.is_empty() {
                    return None;
                }
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
    Some(state)
}

/// returns updated bits
fn apply_search_row_result(
    state: &mut State,
    input: &Input,
    row: usize,
    candidates: &[Mask],
) -> Mask {
    assert!(!candidates.is_empty());
    let must_black = candidates
        .iter()
        .fold(all_bit(input.width), |acc, &x| acc & x);
    let must_white = !candidates.iter().fold(0, |acc, &x| acc | x);
    let must_white = must_white & all_bit(input.width);

    let new_black = minus_bit(must_black, state.black[row]);
    let new_white = minus_bit(must_white, state.white[row]);
    state.black[row] |= must_black;
    state.white[row] |= must_white;
    assert!(state.black[row] & state.white[row] == 0);

    new_black | new_white
}

enum Solution {
    Solved(State),
    Impossible,
}

/// constraints: state はライン埋め済み and 無矛盾 and not solved
fn solve_combined(input: &Input, state: &State) -> Solution {
    print(input, state);
    assert!(deterministic_fill(input, state).as_ref() == Some(state));
    assert!(!state.solved(input));

    // todo: 仮置きの順番をいい感じにする
    for i in 0..input.height {
        for j in 0..input.width {
            if state.black[i] >> j & 1 == 0 && state.white[i] >> j & 1 == 0 {
                {
                    // (i, j) を黒と仮定する
                    let mut new_state = state.clone();
                    new_state.black[i] |= 1 << j;
                    if let Some(filled_state) = deterministic_fill(input, &new_state) {
                        // 矛盾は見つからなかった
                        if filled_state.solved(input) {
                            // 答えまでたどり着いたので解の唯一性によって黒に確定する
                            return Solution::Solved(filled_state);
                        } else {
                            // 中途半端に終わったので成果なし
                        }
                    } else {
                        // 矛盾が見つかったので白に確定する
                        let mut new_state2 = state.clone();
                        new_state2.white[i] |= 1 << j;
                        return solve_combined(input, &new_state2);
                    }
                }
            }
        }
    }
    Solution::Impossible
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
                match (set_bit(state.black[i], j), set_bit(state.white[i], j)) {
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
    let state = State::new(input.height, input.width);
    if let Some(state) = deterministic_fill(&input, &state) {
        print(&input, &state);
        if state.solved(&input) {
            print(&input, &state);
        } else {
            match solve_combined(&input, &state) {
                Solution::Solved(state) => {
                    print(&input, &state);
                }
                Solution::Impossible => {
                    println!("impossible");
                }
            }
        }
    }
}
