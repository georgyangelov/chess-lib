use core::fmt::Debug;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Piece {
    Pawn,
    Rook,
    Bishop,
    Knight,
    Queen,
    King
}

#[wasm_bindgen]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GameResult {
    Unknown,
    Draw,
    WhiteWins,
    BlackWins
}

#[wasm_bindgen]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Color {
    White,
    Black
}

impl Color {
    pub fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White
        }
    }
}

#[wasm_bindgen]
#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct Square {
    pub rank: i8,
    pub file: i8
}

#[wasm_bindgen]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct OccupiedSquare {
    pub piece: Piece,
    pub color: Color
}

#[derive(PartialEq, Eq, Clone)]
pub struct Board {
    pub squares: Vec<Option<OccupiedSquare>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Position {
    pub board: Board,

    pub next_to_move: Color,

    pub white_can_castle: bool,
    pub black_can_castle: bool,

    pub half_move_clock: i64,
    pub full_move_counter: i64,
}

static FILE_LABELS: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

impl Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        if self.file < 0 || self.file >= 8 {
            return Err(std::fmt::Error);
        }

        if self.rank < 0 || self.rank >= 8 {
            return Err(std::fmt::Error);
        }

        let file_label = FILE_LABELS[self.file as usize];

        write!(f, "{}{}", file_label, self.rank + 1)
    }
}

pub enum SquareNotationOptions {
    OnlyFile,
    OnlyRank,
    FileAndRank
}

impl Square {
    pub fn new(rank: i8, file: i8) -> Option<Square> {
        if file < 0 || file > 7 {
            None
        } else if rank < 0 || rank > 7 {
            None
        } else {
            Some(Square { file, rank })
        }
    }

    pub fn from_notation(notation: &str) -> Result<Square, ()> {
        let mut chars = notation.chars();

        let file_label = chars.next().ok_or(())?;
        let rank_char = chars.next().ok_or(())?;

        if file_label < 'a' || file_label > 'h' {
            return Err(());
        }

        if !rank_char.is_digit(10) {
            return Err(());
        }

        let file = (file_label as u8 - 'a' as u8) as i8;
        let rank = (rank_char as u8 - '0' as u8) as i8 - 1;

        if rank < 1 || rank > 8 {
            return Err(());
        }

        Ok(Square { rank, file })
    }

    pub fn to_notation(&self, options: SquareNotationOptions) -> String {
        let file_label = FILE_LABELS[self.file as usize];

        match options {
            SquareNotationOptions::OnlyFile => String::from(file_label),
            SquareNotationOptions::OnlyRank => (self.rank + 1).to_string(),
            SquareNotationOptions::FileAndRank => format!("{}{}", file_label, self.rank + 1)
        }
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        for (i, square) in self.squares.iter().enumerate() {
            let letter = match square {
                Some(OccupiedSquare { piece: Piece::Pawn, color: Color::White }) => 'P',
                Some(OccupiedSquare { piece: Piece::Knight, color: Color::White }) => 'N',
                Some(OccupiedSquare { piece: Piece::Bishop, color: Color::White }) => 'B',
                Some(OccupiedSquare { piece: Piece::Rook, color: Color::White }) => 'R',
                Some(OccupiedSquare { piece: Piece::Queen, color: Color::White }) => 'Q',
                Some(OccupiedSquare { piece: Piece::King, color: Color::White }) => 'K',

                Some(OccupiedSquare { piece: Piece::Pawn, color: Color::Black }) => 'p',
                Some(OccupiedSquare { piece: Piece::Knight, color: Color::Black }) => 'n',
                Some(OccupiedSquare { piece: Piece::Bishop, color: Color::Black }) => 'b',
                Some(OccupiedSquare { piece: Piece::Rook, color: Color::Black }) => 'r',
                Some(OccupiedSquare { piece: Piece::Queen, color: Color::Black }) => 'q',
                Some(OccupiedSquare { piece: Piece::King, color: Color::Black }) => 'k',

                None => ' '
            };

            write!(f, "|{}", letter)?;

            if i % 8 == 7 {
                write!(f, "|\n")?;
            }
        }

        Ok(())
    }
}
