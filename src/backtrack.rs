use crate::common::*;
use crate::search_line::*;

/// constraints: state はライン埋め済み and 無矛盾 and not solved
fn solve_impl(input: &Input, state: &State) -> Option<Solution> {
    assert!(deterministic_fill(input, state).as_ref() == Some(state));
    assert!(!state.solved(input));

    // todo: 仮置きの順番をいい感じにする
    for i in 0..input.height {
        for j in 0..input.width {
            if state.black[i] >> j & 1 == 0 && state.white[i] >> j & 1 == 0 {
                // (i, j) を黒と仮定する
                let mut state = state.clone();
                state.white[i] ^= 1 << j;
                if let Some(filled_state) = deterministic_fill(input, &state) {
                    // 矛盾は見つからなかった
                    if filled_state.solved(input) {
                        // 答えまでたどり着いたので解の唯一性によって黒に確定する
                        return Some(filled_state.to_solution());
                    } else {
                        // 中途半端に終わったので成果なし
                    }
                } else {
                    // 黒で矛盾が見つかったので白に確定する
                    state.black[i] ^= 1 << j;
                    state.white[i] ^= 1 << j;
                    return if let Some(filled_new_state) = deterministic_fill(input, &state) {
                        if filled_new_state.solved(input) {
                            Some(filled_new_state.to_solution())
                        } else {
                            solve_impl(input, &filled_new_state)
                        }
                    } else {
                        // 白でも矛盾なので解無し
                        None
                    };
                }
            }
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
