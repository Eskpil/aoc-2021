mod puzzle_1;
mod puzzle_2;
use std::env;
use std::process;

fn main() {
    let puzzle = env::args().nth(1);

    match puzzle.as_ref().map(|s| &s[..]) {
        Some("1") => {
            puzzle_1::puzzle_one();
        }
        Some("2") => {
            puzzle_2::puzzle_two();
        }
        Some(other) => {
            eprintln!("unknown puzzle: {:}", other);
            process::exit(1);
        }
        None => {
            eprintln!("possible puzzles are 1, 2");
            process::exit(1);
        }
    }
}
