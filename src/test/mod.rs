use super::*;
use regex::Regex;
use std::collections::HashSet;

mod pgn_test;
mod rules_test;

#[test]
fn test_reading_positions() {
    let board = read_board("
        |r|n|b|q|k|b|n|r|
        |p|p|p|p|p|p|p|p|
        | | | | | | | | |
        | | | | | | | | |
        | | | | | | | | |
        | | | | | | | | |
        |P|P|P|P|P|P|P|P|
        |R|N|B|Q|K|B|N|R|
    ");

    assert_eq!(board, Board {
        squares: vec![
            Some(OccupiedSquare { piece: Piece::Rook, color: Color::Black }),
            Some(OccupiedSquare { piece: Piece::Knight, color: Color::Black }),
            Some(OccupiedSquare { piece: Piece::Bishop, color: Color::Black }),
            Some(OccupiedSquare { piece: Piece::Queen, color: Color::Black }),
            Some(OccupiedSquare { piece: Piece::King, color: Color::Black }),
            Some(OccupiedSquare { piece: Piece::Bishop, color: Color::Black }),
            Some(OccupiedSquare { piece: Piece::Knight, color: Color::Black }),
            Some(OccupiedSquare { piece: Piece::Rook, color: Color::Black }),

            Some(OccupiedSquare { piece: Piece::Pawn, color: Color::Black }),
            Some(OccupiedSquare { piece: Piece::Pawn, color: Color::Black }),
            Some(OccupiedSquare { piece: Piece::Pawn, color: Color::Black }),
            Some(OccupiedSquare { piece: Piece::Pawn, color: Color::Black }),
            Some(OccupiedSquare { piece: Piece::Pawn, color: Color::Black }),
            Some(OccupiedSquare { piece: Piece::Pawn, color: Color::Black }),
            Some(OccupiedSquare { piece: Piece::Pawn, color: Color::Black }),
            Some(OccupiedSquare { piece: Piece::Pawn, color: Color::Black }),

            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,

            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,

            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,

            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,

            Some(OccupiedSquare { piece: Piece::Pawn, color: Color::White }),
            Some(OccupiedSquare { piece: Piece::Pawn, color: Color::White }),
            Some(OccupiedSquare { piece: Piece::Pawn, color: Color::White }),
            Some(OccupiedSquare { piece: Piece::Pawn, color: Color::White }),
            Some(OccupiedSquare { piece: Piece::Pawn, color: Color::White }),
            Some(OccupiedSquare { piece: Piece::Pawn, color: Color::White }),
            Some(OccupiedSquare { piece: Piece::Pawn, color: Color::White }),
            Some(OccupiedSquare { piece: Piece::Pawn, color: Color::White }),

            Some(OccupiedSquare { piece: Piece::Rook, color: Color::White }),
            Some(OccupiedSquare { piece: Piece::Knight, color: Color::White }),
            Some(OccupiedSquare { piece: Piece::Bishop, color: Color::White }),
            Some(OccupiedSquare { piece: Piece::Queen, color: Color::White }),
            Some(OccupiedSquare { piece: Piece::King, color: Color::White }),
            Some(OccupiedSquare { piece: Piece::Bishop, color: Color::White }),
            Some(OccupiedSquare { piece: Piece::Knight, color: Color::White }),
            Some(OccupiedSquare { piece: Piece::Rook, color: Color::White }),
        ]
    })
}

pub fn expect_lexing(pgn: &str, expected_tokens: &[Token]) {
    let mut lexer = Lexer::new(pgn);
    let tokens = lexer.lex().expect("Cannot lex pgn");

    assert_eq!(tokens.as_slice(), expected_tokens)
}

pub fn expect_parse(pgn: &str, expected_games: &[ParsedGame]) {
    let mut lexer = Lexer::new(pgn);
    let tokens = lexer.lex().expect("Cannot lex pgn");

    let mut parser = Parser::new(tokens);
    let games = parser.parse().expect("Cannot parse pgn");

    assert_eq!(games, expected_games)
}

pub fn expect_game_state(starting_board: &str, moves: &[&str], expected_board: &str) {
    let mut game = Game::new(read_board(starting_board), Color::White);

    for next_move in moves {
        game = game.make_move(next_move).expect("Invalid move");
    }

    let board_debug_string = format!("{:?}", game.board());

    assert_eq!(trim_lines(expected_board), trim_lines(&board_debug_string));
}

pub fn read_game(board: &str, next_to_move: Color) -> Game {
    Game::new(read_board(board), next_to_move)
}

pub fn expect_valid_moves(board: &str, next_to_move: Color, moves: &[&str]) {
    let game = Game::new(read_board(board), next_to_move);

    let actual_moves: HashSet<String> = game.valid_moves().into_iter()
        .map( |valid_move| valid_move.notation() )
        .collect();

    let expected_moves: HashSet<String> = moves.iter()
        .map( |s| s.clone().into() )
        .collect();

    assert_eq!(expected_moves, actual_moves);
}

fn trim_lines(string: &str) -> String {
    string.lines()
        .map( |line| line.trim() )
        .filter( |line| line.len() > 0 )
        .collect::<Vec<&str>>()
        .join("\n")
}

fn read_board(string: &str) -> Board {
    let mut squares: Vec<Option<OccupiedSquare>> = Vec::new();
    let rank_regex = Regex::new(r"^(?i)\s*\|+\s*([PNBRQK])?\s*\|+\s*([PNBRQK])?\s*\|+\s*([PNBRQK])?\s*\|+\s*([PNBRQK])?\s*\|+\s*([PNBRQK])?\s*\|+\s*([PNBRQK])?\s*\|+\s*([PNBRQK])?\s*\|+\s*([PNBRQK])?").expect("Invalid regex");

    for line in string.lines().filter( |line| line.trim().len() > 0 ).take(8) {
        let pieces = rank_regex.captures(line).expect(&format!("Invalid line '{}'", line));

        for piece in pieces.iter().skip(1) {
            squares.push(piece.map( |m| {
                let letter = m.as_str();

                match letter {
                    "P" => OccupiedSquare { piece: Piece::Pawn, color: Color::White },
                    "N" => OccupiedSquare { piece: Piece::Knight, color: Color::White },
                    "B" => OccupiedSquare { piece: Piece::Bishop, color: Color::White },
                    "R" => OccupiedSquare { piece: Piece::Rook, color: Color::White },
                    "Q" => OccupiedSquare { piece: Piece::Queen, color: Color::White },
                    "K" => OccupiedSquare { piece: Piece::King, color: Color::White },

                    "p" => OccupiedSquare { piece: Piece::Pawn, color: Color::Black },
                    "n" => OccupiedSquare { piece: Piece::Knight, color: Color::Black },
                    "b" => OccupiedSquare { piece: Piece::Bishop, color: Color::Black },
                    "r" => OccupiedSquare { piece: Piece::Rook, color: Color::Black },
                    "q" => OccupiedSquare { piece: Piece::Queen, color: Color::Black },
                    "k" => OccupiedSquare { piece: Piece::King, color: Color::Black },

                    _ => panic!(format!("Invalid piece letter '{}'", letter))
                }
            }));
        }
    }

    Board { squares }
}
