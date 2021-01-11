#![allow(dead_code)]

extern crate regex;
extern crate lazy_static;

mod models;
mod fen;

pub mod parser;
pub mod game;

pub use parser::lexer::{Lexer, Token};
pub use parser::{ParsedGame, Move, Parser};
pub use game::{Game};

pub use models::*;
pub use fen::*;

#[cfg(test)]
mod test;
