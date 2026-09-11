use std::io::{self, BufRead};

use crate::models::{Player, Turn};

pub fn read_player<R: BufRead>(reader: &mut R, executable_path: &str) -> io::Result<Player> {
    let mut line = String::new();

    loop {
        line.clear();

        if reader.read_line(&mut line)? == 0 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Player information was not found",
            ));
        }
        if !line.contains(executable_path) {
            continue;
        }


        
        let parts: Vec<&str> = line.split_whitespace().collect();

        return match parts.get(2) {
            Some(&"p1") => Ok(Player::from_number(1)),
            Some(&"p2") => Ok(Player::from_number(2)),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid player header",
            )),
        };
    }
}

pub fn read_turn<R: BufRead>(reader: &mut R) -> io::Result<Option<Turn>> {
    let mut line = String::new();

    loop {
        line.clear();

        if reader.read_line(&mut line)? == 0 {
            return Ok(None);
        }

        if !line.starts_with("Anfield") {
            continue;
        }

        let anfield_parts: Vec<&str> = line.split_whitespace().collect();
        let board_height: usize = anfield_parts[2].trim_end_matches(':').parse().unwrap();

        // Skip the column-number line.
        line.clear();
        reader.read_line(&mut line)?;

        let mut board = Vec::new();

        for _ in 0..board_height {
            line.clear();
            reader.read_line(&mut line)?;

            // Example board row: "002 ....@...."
            let row = line.split_whitespace().nth(1).unwrap();
            board.push(row.chars().collect());
        }

        // Read: Piece <width> <height>:
        line.clear();
        reader.read_line(&mut line)?;

        let piece_parts: Vec<&str> = line.split_whitespace().collect();
        let piece_height: usize = piece_parts[2].trim_end_matches(':').parse().unwrap();

        let mut piece = Vec::new();

        for _ in 0..piece_height {
            line.clear();
            reader.read_line(&mut line)?;
            piece.push(line.trim().chars().collect());
        }

        return Ok(Some(Turn { board, piece }));
    }
}
