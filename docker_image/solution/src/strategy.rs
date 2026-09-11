use crate::models::{Player, Turn};
use crate::placement;
use crate::scoring;

pub fn choose_move(turn: &Turn, player: &Player) -> (i32, i32) {
    let board_height = turn.board.len();
    let board_width = turn.board[0].len();
    let distances = scoring::enemy_distance_map(turn, player);

    let mut best_move = None;
    let mut best_distance = usize::MAX;
    let mut best_contacts = 0;

    for y in 0..board_height {
        for x in 0..board_width {
            if !placement::is_valid(turn, player, x, y) {
                continue;
            }

            let distance = scoring::move_distance(turn, &distances, x, y);
            let contacts = scoring::enemy_contacts(turn, player, x, y);

            if distance < best_distance
                || (distance == best_distance && contacts > best_contacts)
            {
                best_distance = distance;
                best_contacts = contacts;
                best_move = Some((x as i32, y as i32));
            }
        }
    }

    best_move.unwrap_or((0, 0))
}