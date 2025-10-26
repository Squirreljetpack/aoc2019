use crate::template::{aoc_cli, Day};
use std::process;

pub fn handle(day: Day, puzzle_only: bool) {
    if aoc_cli::check().is_err() {
        eprintln!("command \"aoc\" not found or not callable. Try running \"cargo install aoc-cli\" to install it.");
        process::exit(1);
    }

    if let Err(e) = aoc_cli::download(day, puzzle_only) {
        eprintln!("failed to call aoc-cli: {e}");
        process::exit(1);
    };
}
