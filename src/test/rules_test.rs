use super::*;

#[test]
fn test_debug_positions() {
    let string = "
        |r|n|b|q|k|b|n|r|
        |p|p|p|p|p|p|p|p|
        | | | | | | | | |
        | | | | | | | | |
        | | | | | | | | |
        | | | | | | | | |
        |P|P|P|P|P|P|P|P|
        |R|N|B|Q|K|B|N|R|
    ";

    let board = read_board(string);
    let board_as_string = format!("{:?}", board);

    assert_eq!(trim_lines(string), trim_lines(&board_as_string));
}

#[test]
fn test_generating_pawn_moves_with_white() {
    expect_valid_moves(
        "
        | | | | | | | | |
        | | | | |p| | | |
        | | | | |P| | | |
        | | | | | | | | |
        | | | | | | | | |
        | | |P| | | | | |
        | | | |P|P| | | |
        | | | | | | | | |
        ",
        Color::White,

        &[
            "c4",
            "d3", "d4",
            "e3", "e4",
        ]
    );
}

#[test]
fn test_generating_pawn_moves_with_black() {
    expect_valid_moves(
        "
        | | | | | | | | |
        | | |p| |p| | | |
        | | | | |P| | | |
        | | | | | | | | |
        | | | | | | |p| |
        | | |P| | | | | |
        | | | |P|P| | | |
        | | | | | | | | |
        ",
        Color::Black,

        &[
            "c6", "c5",
            "g3"
        ]
    );
}

#[test]
fn test_pawn_takes() {
    expect_valid_moves(
        "
        | | | | | | | | |
        | | | | | | | | |
        | | | | | |p| | |
        | | | | |P| | | |
        | | | | | | | | |
        | | | | | | | | |
        | | | | | | | | |
        | | | | | | | | |
        ",
        Color::White,

        &["e6", "exf6"]
    );
}

#[test]
fn test_knight_moves() {
    expect_valid_moves(
        "
        | | | | | | | | |
        | | | | | | | | |
        | | | | | | | | |
        | | | | |N| | | |
        | | | | | | | | |
        | | | | | | | | |
        | | | | | | | | |
        | | | | | | | | |
        ",
        Color::White,

        &[
                  "Nd7",    "Nf7",
            "Nc6",                "Ng6",

            "Nc4",                "Ng4",
                  "Nd3",    "Nf3"
        ]
    );
}

#[test]
fn test_knight_moves_at_edge() {
    expect_valid_moves(
        "
        | | | | | | |N| |
        | | | | | | | | |
        | | | | | | | | |
        | | | | | | | | |
        | | | | | | | | |
        | | | | | | | | |
        | | | | | | | | |
        | | | | | | | | |
        ",
        Color::White,

        &["Ne7", "Nf6", "Nh6"]
    );
}

#[test]
fn test_rook_moves() {
    expect_valid_moves(
        "
        | | | | | | | | |
        | | | | | | | | |
        | | | | | |R| | |
        | | | | | | | | |
        | | | | | |p| | |
        | | | | | | | | |
        | | | | | | | | |
        | | | | | | | | |
        ",
        Color::White,

        &[
                                               "Rf8",
                                               "Rf7",
            "Ra6", "Rb6", "Rc6", "Rd6", "Re6",        "Rg6", "Rh6",
                                               "Rf5",
                                               "Rxf4"
        ]
    );
}

#[test]
fn test_rook_moves_cannot_move_past_self_piece() {
    expect_valid_moves(
        "
        | | | | | | | | |
        | | | | | | | | |
        | | | | | |R| | |
        | | | | | | | | |
        | | | | | |P| | |
        | | | | | | | | |
        | | | | | | | | |
        | | | | | | | | |
        ",
        Color::White,

        &[
            "f5",

                                               "Rf8",
                                               "Rf7",
            "Ra6", "Rb6", "Rc6", "Rd6", "Re6",        "Rg6", "Rh6",
                                               "Rf5"
        ]
    );
}

#[test]
fn test_queen_moves() {
    expect_valid_moves(
        "
        | | | | | | | | |
        | | | | |P| | | |
        | | | | | |Q| | |
        | | | | |p| | | |
        | | | | | |P| | |
        | | | | | | | | |
        | | | | | | | | |
        | | | | | | | | |
        ",
        Color::White,

        &[
            "f5", "e8", "fxe5",

                                                "Qf8",        "Qh8",
                                                "Qf7", "Qg7",
            "Qa6", "Qb6", "Qc6", "Qd6", "Qe6",         "Qg6", "Qh6",
                                        "Qxe5", "Qf5", "Qg5",
                                                              "Qh4"
        ]
    );
}

#[test]
fn test_bishop_moves() {
    expect_valid_moves(
        "
        | | | | | | | | |
        | | | | |P| | | |
        | | | | | |B| | |
        | | | | |p| | | |
        | | | | | |P| | |
        | | | | | | | | |
        | | | | | | | | |
        | | | | | | | | |
        ",
        Color::White,

        &[
            "f5", "e8", "fxe5",

                                                              "Bh8",
                                                       "Bg7",

                                        "Bxe5",        "Bg5",
                                                              "Bh4"
        ]
    );
}

#[test]
fn test_king_moves() {
    expect_valid_moves(
        "
        | | | | | | | | |
        | | | | |P| | | |
        | | | | | |K| | |
        | | | | |p| | | |
        | | | | | |P| | |
        | | | | | | | | |
        | | | | | | | | |
        | | | | | | | | |
        ",
        Color::White,

        &[
                    "Kf7", "Kg7",
            "Ke6",         "Kg6",
            "Kxe5", "Kf5", "Kg5",

            "f5", "e8", "fxe5"
        ]
    );
}

#[test]
fn test_moving_into_check() {
    expect_valid_moves(
        "
        | | | | | | | | | 8
        | | | | | | |r| | 7
        | | | | | |K| |n| 6
        | | | | | | | | | 5
        | | | |Q| | | | | 4
        | | |b| | | | | | 3
        | | | | | | | | | 2
        | | | | | | | | | 1
         a b c d e f g h
        ",
        Color::White,

        &[
            "Kxg7", "Ke6", "Ke5",
            "Qxc3", "Qe5"
        ]
    );
}

#[test]
fn test_need_to_move_out_of_check() {
    expect_valid_moves(
        "
        | | | | | | | | | 8
        | | | | | | | |n| 7
        | | | | | |K| | | 6
        | | | | | | | | | 5
        | | | |Q| | | | | 4
        | | |b| | | | | | 3
        | | | | | | | | | 2
        | | | | | | | | | 1
         a b c d e f g h
        ",
        Color::White,

        &[
            "Ke7", "Kf7", "Kg7",
            "Ke6",        "Kg6",
            "Ke5", "Kf5",
        ]
    );
}

#[test]
fn test_simple_moves() {
    expect_game_state(
        "
        |r|n|b|q|k|b|n|r|
        |p|p|p|p|p|p|p|p|
        | | | | | | | | |
        | | | | | | | | |
        | | | | | | | | |
        | | | | | | | | |
        |P|P|P|P|P|P|P|P|
        |R|N|B|Q|K|B|N|R|
        ",

        &["e4", "Nf6"],

        "
        |r|n|b|q|k|b| |r|
        |p|p|p|p|p|p|p|p|
        | | | | | |n| | |
        | | | | | | | | |
        | | | | |P| | | |
        | | | | | | | | |
        |P|P|P|P| |P|P|P|
        |R|N|B|Q|K|B|N|R|
        ",
    );
}

#[test]
fn test_more_complex_moves() {
    expect_game_state(
        "
        |r|n|b|q|k|b|n|r|
        |p|p|p|p|p|p|p|p|
        | | | | | | | | |
        | | | | | | | | |
        | | | | | | | | |
        | | | | | | | | |
        |P|P|P|P|P|P|P|P|
        |R|N|B|Q|K|B|N|R|
        ",

        &[
            "e4", "e5",
            "Bc4", "Nf6",
            "Nc3", "Nc6",
            "Qh5", "Nxh5"
        ],

        "
        |r| |b|q|k|b| |r|
        |p|p|p|p| |p|p|p|
        | | |n| | | | | |
        | | | | |p| | |n|
        | | |B| |P| | | |
        | | |N| | | | | |
        |P|P|P|P| |P|P|P|
        |R| |B| |K| |N|R|
        ",
    );
}


#[test]
fn test_simple_checks() {
    let game = read_game(
        "
        |r|n|b| |k|b|n|r|
        |p|p|p|p| |p|p|p|
        | | | | | | | | |
        | | | | | | | | |
        | | | | | | | |q|
        | | | | | | | | |
        |P|P|P|P|P| |P|P|
        |R|N|B|Q|K|B|N|R|
        ",
        Color::White
    );

    assert!(game.in_check(Color::White));
}
