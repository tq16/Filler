mod models;
mod parser;
mod placement;
mod scoring;
mod strategy;
mod output;

#[cfg(test)]
mod tests;

use std::io::{self, BufReader, Write};

fn main() {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin.lock());

    let executable_path = std::env::current_exe().unwrap();
    let executable_path = executable_path.to_string_lossy().to_string();

    let player = parser::read_player(&mut reader, &executable_path).unwrap();

    while let Some(turn) = parser::read_turn(&mut reader).unwrap() {
        let (x, y) = strategy::choose_move(&turn, &player);

        print!("{}", output::format_move(x, y));
        io::stdout().flush().unwrap();
    }
}
