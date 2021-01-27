use super::*;

#[test]
fn lexes_lichess_pgns() {
    expect_lexing("[Event \"XXV Open\"]", &[
        Token::OpenBracket,
        Token::Symbol(String::from("Event")),
        Token::String(String::from("XXV Open")),
        Token::CloseBracket,
        Token::EndOfFile
    ]);

    expect_lexing("
        [Event \"XXV Open\"]
        [Site \"Novi Becej SRB\"]

        {[#]} 44. Bd1 Kd4+ 45. Kf1 Nh3#   0-1
    ", &[
        Token::OpenBracket,
        Token::Symbol(String::from("Event")),
        Token::String(String::from("XXV Open")),
        Token::CloseBracket,

        Token::OpenBracket,
        Token::Symbol(String::from("Site")),
        Token::String(String::from("Novi Becej SRB")),
        Token::CloseBracket,

        Token::Comment(String::from("[#]")),

        Token::Integer(44),
        Token::Period,
        Token::Symbol(String::from("Bd1")),
        Token::Symbol(String::from("Kd4+")),

        Token::Integer(45),
        Token::Period,
        Token::Symbol(String::from("Kf1")),
        Token::Symbol(String::from("Nh3#")),

        Token::Symbol(String::from("0-1")),

        Token::EndOfFile
    ]);
}

#[test]
fn test_parse_simple_pgns() {
    expect_parse("
        [Event \"Casual Blitz game\"]

        1. e4 e5 2. Nf3 Nc6 3. Qxg7# { White wins by checkmate. } 1-0
    ", &[
        ParsedGame {
            setup: None,
            fen: None,
            other_tags: vec![(String::from("Event"), String::from("Casual Blitz game"))],
            moves: vec![
                PGNMove { number: Some(1), white_move: Some(String::from("e4")), black_move: Some(String::from("e5")) },
                PGNMove { number: Some(2), white_move: Some(String::from("Nf3")), black_move: Some(String::from("Nc6")) },
                PGNMove { number: Some(3), white_move: Some(String::from("Qxg7#")), black_move: None },
            ],
            result: GameResult::WhiteWins
        }
    ]);
}

#[test]
fn test_parse_weird_moves() {
    expect_parse("
        [Event \"Casual Blitz game\"]

        1. e4e5 e8=Q# 1-0
    ", &[
        ParsedGame {
            setup: None,
            fen: None,
            other_tags: vec![(String::from("Event"), String::from("Casual Blitz game"))],
            moves: vec![
                PGNMove { number: Some(1), white_move: Some(String::from("e4e5")), black_move: Some(String::from("e8=Q#")) },
            ],
            result: GameResult::WhiteWins
        }
    ]);
}

#[test]
fn test_parse_move_without_number() {
    expect_parse("
        [Event \"Casual Blitz game\"]

        e4e5 e8=Q# 1-0
    ", &[
        ParsedGame {
            setup: None,
            fen: None,
            other_tags: vec![(String::from("Event"), String::from("Casual Blitz game"))],
            moves: vec![
                PGNMove { number: None, white_move: Some(String::from("e4e5")), black_move: Some(String::from("e8=Q#")) },
            ],
            result: GameResult::WhiteWins
        }
    ]);
}

#[test]
fn test_pgn_to_game() {
    expect_pgn_state(
        "
        [Event \"Fool's Mate\"]

        1. f3 e5 2. g4 Qh4# 0-1
        ",
        "
        |r|n|b| |k|b|n|r|
        |p|p|p|p| |p|p|p|
        | | | | | | | | |
        | | | | |p| | | |
        | | | | | | |P|q|
        | | | | | |P| | |
        |P|P|P|P|P| | |P|
        |R|N|B|Q|K|B|N|R|
        "
    );
}

#[test]
fn test_pgn_with_set_up() {
    expect_pgn_state(
        "
        [Event \"Fool's Mate\"]
        [SetUp \"1\"]
        [FEN \"rnbqkbnr/pppp1ppp/8/4p3/8/5P2/PPPPP1PP/RNBQKBNR w KQkq - 0 2\"]

        2. g4 Qh4# 0-1
        ",
        "
        |r|n|b| |k|b|n|r|
        |p|p|p|p| |p|p|p|
        | | | | | | | | |
        | | | | |p| | | |
        | | | | | | |P|q|
        | | | | | |P| | |
        |P|P|P|P|P| | |P|
        |R|N|B|Q|K|B|N|R|
        "
    );
}

#[test]
fn test_pgn_with_set_up_start_with_black_move() {
    expect_pgn_state(
        "
        [Event \"Fool's Mate\"]
        [SetUp \"1\"]
        [FEN \"rnbqkbnr/pppp1ppp/8/4p3/6P1/5P2/PPPPP2P/RNBQKBNR b KQkq - 0 2\"]

        2... Qh4# 0-1
        ",
        "
        |r|n|b| |k|b|n|r|
        |p|p|p|p| |p|p|p|
        | | | | | | | | |
        | | | | |p| | | |
        | | | | | | |P|q|
        | | | | | |P| | |
        |P|P|P|P|P| | |P|
        |R|N|B|Q|K|B|N|R|
        "
    );
}

#[test]
fn test_pgn_with_set_up_start_with_black_move_with_no_move_number() {
    expect_pgn_state(
        "
        [Event \"Fool's Mate\"]
        [SetUp \"1\"]
        [FEN \"rnbqkbnr/pppp1ppp/8/4p3/6P1/5P2/PPPPP2P/RNBQKBNR b KQkq - 0 2\"]

        Qh4# 0-1
        ",
        "
        |r|n|b| |k|b|n|r|
        |p|p|p|p| |p|p|p|
        | | | | | | | | |
        | | | | |p| | | |
        | | | | | | |P|q|
        | | | | | |P| | |
        |P|P|P|P|P| | |P|
        |R|N|B|Q|K|B|N|R|
        "
    );
}
