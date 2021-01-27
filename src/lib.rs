#![allow(dead_code)]

extern crate regex;
extern crate lazy_static;
extern crate wasm_bindgen;

mod models;
mod fen;

pub mod parser;
pub mod game;
pub mod wasm;

pub use parser::lexer::{Lexer, Token};
pub use parser::{ParsedGame, PGNMove, Parser};
pub use game::{Game, ValidMove};

pub use models::*;
pub use fen::*;

// pub use wasm::*;

// use wasm_bindgen::prelude::*;
//
// #[wasm_bindgen]
// pub fn hellow_world() -> String {
//     String::from("Hellow World")
// }

#[cfg(test)]
mod test;
