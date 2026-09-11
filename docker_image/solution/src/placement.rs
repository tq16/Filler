use crate::models::{Player, Turn};

pub fn is_valid(turn: &Turn, player: &Player, x: usize, y: usize) -> bool {
    let board_height = turn.board.len();
    let board_width = turn.board[0].len();
    let piece_height = turn.piece.len();
    let piece_width = turn
        .piece
        .iter()
        .map(|row| row.len())
        .max()
        .unwrap();

    // The whole piece rectangle must remain inside the board.
    if x + piece_width > board_width || y + piece_height > board_height {
        return false;
    }

    let mut own_overlaps = 0;

    for (piece_y, row) in turn.piece.iter().enumerate() {
        for (piece_x, cell) in row.iter().enumerate() {
            if *cell != 'O' {
                continue;
            }

            let board_cell = turn.board[y + piece_y][x + piece_x];

            if board_cell == player.enemy_old || board_cell == player.enemy_new {
                return false;
            }

            if board_cell == player.own_old || board_cell == player.own_new {
                own_overlaps += 1;
            }
        }
    }

    own_overlaps == 1
}