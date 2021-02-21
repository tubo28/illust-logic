use crate::bitop::*;
use crate::common::*;
use std::collections::VecDeque;

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
            let mask = all(hint);
            for i in pos..(pos + width - hint + 1) {
                if i + hint <= width {
                    let next = cur | mask << i;
                    let cur_head = all(hint + i);
                    if distinct(white, next)
                        && !set(black, i + hint)
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

pub fn search_line(input: &Input, state: &State, row: usize) -> Vec<Mask> {
    search_line_sub(
        input.width,
        &input.row_hints[row],
        state.white[row],
        state.black[row],
    )
}

enum ActiveLine {
    Row(usize),
    Column(usize),
}

pub fn deterministic_fill(input: &Input, state: &State) -> Option<State> {
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
                    if set(updated, j) && !column_active[j] {
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
                    if set(updated, i) && !row_active[i] {
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
    let must_black = candidates.iter().fold(all(input.width), |acc, &x| acc & x);
    let must_white = !candidates.iter().fold(0, |acc, &x| acc | x);
    let must_white = must_white & all(input.width);

    let new_black = minus(must_black, state.black[row]);
    let new_white = minus(must_white, state.white[row]);
    state.black[row] |= must_black;
    state.white[row] |= must_white;
    assert!(state.black[row] & state.white[row] == 0);

    new_black | new_white
}
