use crate::common::*;
use crate::search_line::*;

/// constraints: state はライン埋め済み and 無矛盾 and not solved
fn solve_combined(input: &Input, state: &State) -> Solution {
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

pub fn solve(input: &Input) -> Solution {
    let state = State::new(input.height, input.width);
    if let Some(state) = deterministic_fill(&input, &state) {
        if state.solved(&input) {
            Solution::Solved(state)
        } else {
            solve_combined(&input, &state)
        }
    } else {
        Solution::Impossible
    }
}
