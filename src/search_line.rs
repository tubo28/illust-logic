use crate::bitop::*;
use crate::common::*;
use std::collections::VecDeque;

fn search_line_impl(
    width: usize,
    cur: Mask,
    pos: usize,
    hints: &[Hint],
    must_white: Mask,
    must_black: Mask,
    and: &mut Mask,
    or: &mut Mask,
) {
    match hints.first() {
        None => {
            if subset(cur, must_black) {
                *and &= cur;
                *or |= cur;
            }
        }
        Some(&hint) => {
            let mask = all(hint);
            for i in pos..(width - hint + 1) {
                // Try to set [i, i + hint) black
                if i + hint <= width {
                    let next = cur | mask << i;
                    // [0, i + hint) satisfies must_white constraints
                    if distinct(next, must_white)
                        // [0, i + hint) satisfies must_black constraints
                        && subset(next, must_black & all(hint + i))
                        // The left cell doesn't touch must_black cell
                        && !set(must_black, i + hint)
                    {
                        search_line_impl(
                            width,
                            next,
                            i + hint + 1,
                            &hints[1..],
                            must_white,
                            must_black,
                            and,
                            or,
                        );
                    }
                }
            }
        }
    }
}

pub fn search_line(input: &Input, state: &State, row: usize) -> (Mask, Mask) {
    let mut and = all(input.width);
    let mut or = 0;
    search_line_impl(
        input.width,
        0,
        0,
        &input.row_hints[row],
        state.white[row],
        state.black[row],
        &mut and,
        &mut or,
    );
    (and, or)
}

// "Active" means that the line (row or column) has updated since the last deterministic_fill was applied
enum ActiveLine {
    Row(usize),
    Column(usize),
}

// Try all filling patterns. If some cells are black (white) in all of them,
// fix them black (white)
pub fn deterministic_fill(input: &Input, state: &State) -> Option<State> {
    let mut state = state.clone();
    let input_trans = input.transpose();

    let mut row_active = vec![true; input.height];
    let mut column_active = vec![true; input.width];
    let mut queue = VecDeque::with_capacity(input.height + input.width);
    for i in 0..input.height {
        queue.push_back(ActiveLine::Row(i));
    }
    for j in 0..input.width {
        queue.push_back(ActiveLine::Column(j));
    }

    while let Some(head) = queue.pop_front() {
        match head {
            ActiveLine::Row(row) => {
                let (and, or) = search_line(input, &state, row);
                if and == all(input.width) && or == 0 {
                    return None;
                }
                let updated = apply_search_line_result(&mut state, input, row, and, or);

                row_active[row] = false;
                for j in 0..input.width {
                    if set(updated, j) && !column_active[j] {
                        column_active[j] = true;
                        queue.push_back(ActiveLine::Column(j));
                    }
                }
            }
            ActiveLine::Column(column) => {
                let mut state_trans = state.transpose();
                let (and, or) = search_line(&input_trans, &state_trans, column);
                if and == all(input_trans.width) && or == 0 {
                    return None;
                }
                let updated =
                    apply_search_line_result(&mut state_trans, &input_trans, column, and, or);
                state = state_trans.transpose();

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
fn apply_search_line_result(
    state: &mut State,
    input: &Input,
    row: usize,
    and: Mask,
    or: Mask,
) -> Mask {
    let must_black = and;
    let must_white = (!or) & all(input.width);
    debug_assert!(must_black & must_white == 0);

    let new_black = minus(must_black, state.black[row]);
    let new_white = minus(must_white, state.white[row]);
    state.black[row] |= must_black;
    state.white[row] |= must_white;
    debug_assert!(state.black[row] & state.white[row] == 0);

    new_black | new_white
}
