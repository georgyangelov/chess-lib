use super::*;

use super::fen::*;

#[test]
fn test_initial_board_fen() {
    expect_fen(
        "
        |r|n|b|q|k|b|n|r| 8
        |p|p|p|p|p|p|p|p| 7
        | | | | | | | | | 6
        | | | | | | | | | 5
        | | | | | | | | | 4
        | | | | | | | | | 3
        |P|P|P|P|P|P|P|P| 2
        |R|N|B|Q|K|B|N|R| 1
         a b c d e f g h
        ",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    );
}

fn expect_fen(board: &str, fen: &str) {
    let position = Position {
        board: read_board(board),

        next_to_move: Color::White,

        white_can_castle: true,
        black_can_castle: true,

        half_move_clock: 0,
        full_move_counter: 1
    };

    assert_eq!(position.to_fen(), fen);
    assert_eq!(Position::from_fen(fen).expect("Cannot parse FEN").to_fen(), fen);
}
