mod backtrack;
mod bitop;
mod common;
mod search_line;

use common::*;

fn main() {
    let input = Input::from_stdin();
    let solution = backtrack::solve(&input);
    if let Solution::Solved(state) = solution {
        print(&input, &state);
    }
}
