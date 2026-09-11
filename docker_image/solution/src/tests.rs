use std::io::Cursor;

use crate::models::{Player, Turn};
use crate::{output, parser, placement};

fn player_one() -> Player {
    Player::from_number(1)
}

fn test_turn() -> Turn {
    Turn {
        board: vec![
            vec!['.', '.', '.', '.'],
            vec!['.', '@', '.', '.'],
            vec!['.', '.', '.', '$'],
        ],
        piece: vec![
            vec!['.', 'O'],
            vec!['O', 'O'],
        ],
    }
}

#[test]
fn parses_anfield_and_piece() {
    let input = "\
Anfield 4 3:
    0123
000 ....
001 .@..
002 ...$
Piece 2 2:
.O
OO
";

    let mut reader = Cursor::new(input);
    let turn = parser::read_turn(&mut reader).unwrap().unwrap();

    assert_eq!(turn.board[1][1], '@');
    assert_eq!(turn.board[2][3], '$');
    assert_eq!(turn.piece[0][1], 'O');
    assert_eq!(turn.piece[1][0], 'O');
}

#[test]
fn accepts_exactly_one_own_overlap() {
    let turn = test_turn();

    assert!(placement::is_valid(&turn, &player_one(), 0, 0));
}

#[test]
fn rejects_two_own_overlaps() {
    let turn = Turn {
        board: vec![vec!['@', '@', '.']],
        piece: vec![vec!['O', 'O']],
    };

    assert!(!placement::is_valid(&turn, &player_one(), 0, 0));
}

#[test]
fn rejects_enemy_overlap() {
    let turn = test_turn();

    assert!(!placement::is_valid(&turn, &player_one(), 2, 1));
}

#[test]
fn rejects_out_of_bounds_move() {
    let turn = test_turn();

    assert!(!placement::is_valid(&turn, &player_one(), 3, 0));
}

#[test]
fn formats_coordinate_output() {
    assert_eq!(output::format_move(7, 2), "7 2\n");
}