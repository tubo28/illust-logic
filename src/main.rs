mod backtrack;
mod bitop;
mod common;
mod search_line;

use common::*;

fn main() {
    let input = Input::from_stdin();
    if let Some(solution) = backtrack::solve(&input) {
        print(&input, &solution);
    } else {
        println!("impossible");
    }
}
