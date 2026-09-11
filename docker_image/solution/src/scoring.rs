use crate::models::{Player, Turn};

pub fn enemy_distance_map(turn: &Turn, player: &Player) -> Vec<Vec<usize>> {
    let height = turn.board.len();
    let width = turn.board[0].len();
    let far = height + width + 1;

    let mut distances = vec![vec![far; width]; height];

    for y in 0..height {
        for x in 0..width {
            let cell = turn.board[y][x];

            if cell == player.enemy_old || cell == player.enemy_new {
                distances[y][x] = 0;
            }
        }
    }

    for y in 0..height {
        for x in 0..width {
            if y > 0 {
                distances[y][x] = distances[y][x].min(distances[y - 1][x] + 1);
            }

            if x > 0 {
                distances[y][x] = distances[y][x].min(distances[y][x - 1] + 1);
            }
        }
    }

    for y in (0..height).rev() {
        for x in (0..width).rev() {
            if y + 1 < height {
                distances[y][x] = distances[y][x].min(distances[y + 1][x] + 1);
            }

            if x + 1 < width {
                distances[y][x] = distances[y][x].min(distances[y][x + 1] + 1);
            }
        }
    }

    distances
}

pub fn move_distance(turn: &Turn, distances: &[Vec<usize>], x: usize, y: usize) -> usize {
    let mut closest = usize::MAX;

    for (piece_y, row) in turn.piece.iter().enumerate() {
        for (piece_x, cell) in row.iter().enumerate() {
            if *cell == 'O' {
                closest = closest.min(distances[y + piece_y][x + piece_x]);
            }
        }
    }

    closest
}

pub fn enemy_contacts(turn: &Turn, player: &Player, x: usize, y: usize) -> usize {
    let height = turn.board.len();
    let width = turn.board[0].len();
    let mut contacts = 0;

    for (piece_y, row) in turn.piece.iter().enumerate() {
        for (piece_x, cell) in row.iter().enumerate() {
            if *cell != 'O' {
                continue;
            }

            let board_x = x + piece_x;
            let board_y = y + piece_y;

            let neighbours = [
                (board_x.wrapping_sub(1), board_y),
                (board_x + 1, board_y),
                (board_x, board_y.wrapping_sub(1)),
                (board_x, board_y + 1),
            ];

            for (next_x, next_y) in neighbours {
                if next_x >= width || next_y >= height {
                    continue;
                }

                let board_cell = turn.board[next_y][next_x];

                if board_cell == player.enemy_old || board_cell == player.enemy_new {
                    contacts += 1;
                }
            }
        }
    }

    contacts
}