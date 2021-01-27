#![allow(non_snake_case)]

use wasm_bindgen::prelude::*;
use js_sys::Array;

use serde::Serialize;

use super::*;

#[wasm_bindgen]
pub struct JsGame {
    game: Game
}

#[wasm_bindgen]
pub struct JsValidMove {
    valid_move: ValidMove
}

#[derive(Serialize)]
pub struct JsError {
    pub message: String
}

#[wasm_bindgen]
impl JsGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> JsGame {
        JsGame {
            game: Game::new(Game::standard_position())
        }
    }

    // TODO: Return multiple games?
    pub fn fromPGN(pgn: &str) -> Result<JsGame, JsValue> {
        let games = Game::new_from_pgn(pgn).map_err( |e| Self::js_error(e) )?;

        if games.len() == 0 {
            return Err(Self::js_error(String::from("No games inside PGN")));
        }

        if games.len() > 1 {
            return Err(Self::js_error(String::from("More than one game inside PGN")));
        }

        games.into_iter().next().unwrap()
            .map( |game| JsGame { game })
            .map_err( |e| Self::js_error(e) )
    }

    pub fn fromFEN(fen: &str) -> Result<JsGame, JsValue> {
        let game_result = Game::new_from_fen(fen);

        match game_result {
            Ok(game) => Ok(JsGame { game }),
            Err(parse_error) => Err(JsValue::from_serde(&parse_error).expect("Cannot serialize ParseError to JSValue"))
        }
    }

    pub fn validMoves(&self) -> Array {
        let valid_moves = self.game.valid_moves();

        valid_moves.into_iter()
            .map( |valid_move| JsValue::from_serde(&valid_move).unwrap() )
            .collect()
    }

    fn js_error(message: String) -> JsValue {
        JsValue::from_serde(&JsError { message }).expect("Cannot serialize JS error to JSValue")
    }
}
