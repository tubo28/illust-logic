use crate::common::*;
use crate::search_line::*;
use std::cmp::*;

fn solve_impl(input: &Input, state: &State) -> Option<Solution> {
    debug_assert!(deterministic_fill(input, state).as_ref() == Some(state));
    debug_assert!(!state.solved(input));

    // Assume that the cells sounded by more black cells are likely to be black
    struct C {
        i: usize,
        j: usize,
        count_black: usize,
        count_white: usize,
    }
    let mut empty = Vec::new();
    for i in 0..input.height {
        for j in 0..input.width {
            if state.empty(i, j) {
                let mut count_black = 0;
                let mut count_white = 0;
                fn add(
                    i: usize,
                    j: usize,
                    state: &State,
                    count_black: &mut usize,
                    count_white: &mut usize,
                ) {
                    if state.black(i, j) {
                        *count_black += 1;
                    } else {
                        *count_white += 1;
                    }
                };
                if i + 1 < input.height {
                    add(i + 1, j, state, &mut count_black, &mut count_white);
                }
                if 1 < i {
                    add(i - 1, j, state, &mut count_black, &mut count_white);
                }
                if j + 1 < input.width {
                    add(i, j + 1, state, &mut count_black, &mut count_white);
                }
                if 1 < j {
                    add(i, j - 1, state, &mut count_black, &mut count_white);
                }
                empty.push(C {
                    i,
                    j,
                    count_black,
                    count_white,
                });
            }
        }
    }
    empty.sort_unstable_by(|a, b| {
        (a.count_black.cmp(&b.count_black).reverse()).then(a.count_white.cmp(&b.count_white))
    });
    //empty.reverse();
    let empty_cells = empty;

    for &C {
        i,
        j,
        count_black,
        count_white,
    } in empty_cells.iter()
    {
        // Assume that (i, j) is the opposite color of the surrounding cells
        let mut state = state.clone();
        if count_black > count_white {
            state.set(i, j, Cell::White);
        } else {
            state.set(i, j, Cell::Black);
        }
        if let Some(filled_state) = deterministic_fill(input, &state) {
            // Didn't reached to any contradiction
            if filled_state.solved(input) {
                // Reached the solution
                // Accept it by uniqueness of the solution
                return Some(filled_state.to_solution());
            } else {
                // Couldn't reach to the solution nor contradiction,
                // so couldn't know the assumption was correct or wrong, no outcome
            }
        } else {
            // We found a contradiction, so hypothesis was wrong
            // Fix it to the opposite color
            state.flip(i, j);
            return if let Some(filled_new_state) = deterministic_fill(input, &state) {
                if filled_new_state.solved(input) {
                    Some(filled_new_state.to_solution())
                } else {
                    solve_impl(input, &filled_new_state)
                }
            } else {
                // Still a contradiction, so no solution
                None
            };
        }
    }
    None
}

pub fn solve(input: &Input) -> Option<Solution> {
    let state = State::new(input.height, input.width);
    if let Some(state) = deterministic_fill(&input, &state) {
        if state.solved(&input) {
            Some(state.to_solution())
        } else {
            solve_impl(&input, &state)
        }
    } else {
        None
    }
}
