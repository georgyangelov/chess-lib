use super::*;

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

#[test]
fn test_simple_board_fen() {
    expect_fen(
        "
        |r|n|b|q|k|b|n|r| 8
        |p|p|p|p|p| |p|p| 7
        | | | | | |p| | | 6
        | | | | |p| |K| | 5
        | | | |P| | | | | 4
        | | | | | | | | | 3
        |P|P|P|P| |P|P|P| 2
        |R|N|B|Q| |B|N|R| 1
         a b c d e f g h
        ",
        "rnbqkbnr/ppppp1pp/5p2/4p1K1/3P4/8/PPPP1PPP/RNBQ1BNR w KQkq - 0 1"
    );
}

#[test]
fn test_en_passant_square() {
    expect_fen_moves(
        "
        |r|n|b|q|k|b|n|r| 8
        |p|p|p|p|p|p|p| | 7
        | | | | | | | |p| 6
        | | | | | | | | | 5
        | | | | |P| | | | 4
        | | | | | | | | | 3
        |P|P|P|P| |P|P|P| 2
        |R|N|B|Q|K|B|N|R| 1
         a b c d e f g h
        ",

        &["e5", "f5"],

        "rnbqkbnr/ppppp1p1/7p/4Pp2/8/8/PPPP1PPP/RNBQKBNR w KQkq f6 2 2"
    );
}

fn expect_fen(board: &str, fen: &str) {
    let position = Position {
        board: read_board(board),

        next_to_move: Color::White,

        white_can_castle_king_side: true,
        white_can_castle_queen_side: true,
        black_can_castle_king_side: true,
        black_can_castle_queen_side: true,

        en_passant_square: None,

        half_move_clock: 0,
        full_move_counter: 1
    };

    assert_eq!(position.to_fen(), fen);
    assert_eq!(Position::from_fen(fen).expect("Cannot parse FEN").to_fen(), fen);
}

fn expect_fen_moves(board: &str, moves: &[&str], fen: &str) {
    let position = Position {
        board: read_board(board),

        next_to_move: Color::White,

        white_can_castle_king_side: true,
        white_can_castle_queen_side: true,
        black_can_castle_king_side: true,
        black_can_castle_queen_side: true,

        en_passant_square: None,

        half_move_clock: 0,
        full_move_counter: 1
    };

    let mut game = Game::new(position);

    for m in moves {
        game = game.make_move(m).expect(&format!("Cannot make move {}", m));
    }

    assert_eq!(game.position_to_fen(), fen);
    assert_eq!(Position::from_fen(fen).expect("Cannot parse FEN").to_fen(), fen);
}
