#![allow(non_snake_case)]

use regex::Regex;
use wasm_bindgen::prelude::*;
use js_sys::Array;
use serde::{Serialize, Serializer, Deserialize};
use serde::ser::SerializeStruct;

use super::*;

#[wasm_bindgen]
pub struct JsGame {
    game: Game
}

#[wasm_bindgen]
pub struct JsValidMove {
    valid_move: ValidMove
}

#[wasm_bindgen]
impl JsGame {
    #[wasm_bindgen(constructor)]
    pub fn new(board_string: &str) -> Self {
        let board = read_board(board_string);

        Self {
            game: Game::new_for_test(board, Color::White)
        }
    }

    pub fn validMoves(&self) -> Array {
        let valid_moves = self.game.valid_moves();

        valid_moves.into_iter()
            .map( |valid_move| JsValue::from_serde(&valid_move).unwrap() )
            .collect()
    }
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
