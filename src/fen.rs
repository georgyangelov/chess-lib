use super::models::*;
use std::vec::Vec;

#[derive(Debug, PartialEq, Eq)]
pub struct FenParseError {
    message: String
}

impl Position {
    pub fn from_fen(fen: &str) -> Result<Position, FenParseError> {
        let mut chars = fen.chars();
        let mut squares: Vec<Option<OccupiedSquare>> = Vec::with_capacity(64);

        let mut i = 0;
        loop {
            if i >= 8 * 8 {
                break;
            }

            let square = Square { rank: 7 - i as i8 / 8, file: i as i8 % 8 };
            let first_square_in_rank = square.file == 0;

            match chars.next() {
                Some('/') if first_square_in_rank => continue,

                Some(c) if c.is_digit(10) => {
                    let number_of_empty_squares = c as u8 - '0' as u8;

                    i += number_of_empty_squares;

                    for _ in 0..number_of_empty_squares {
                        squares.push(None);
                    }
                },

                Some(c) if c.is_alphabetic() => {
                    let occupancy = Self::occupancy_from_char(c)?;

                    i += 1;

                    squares.push(Some(occupancy));
                },

                Some(c) => return Err(FenParseError {
                    message: format!("Unexpected character '{}'", c)
                }),

                None => return Err(FenParseError {
                    message: String::from("Unexpected end of FEN string")
                })
            }
        }

        if chars.next() != Some(' ') {
            return Err(FenParseError {
                message: String::from("Expected ' ' after the piece positions")
            });
        }

        let next_to_move = match chars.next() {
            Some('w') => Color::White,
            Some('b') => Color::Black,
            Some(c) => return Err(FenParseError {
                message: format!("Unexpected char '{}' instead of player to move", c)
            }),
            None => return Err(FenParseError {
                message: String::from("Unexpected end of FEN string")
            })
        };

        if chars.next() != Some(' ') {
            return Err(FenParseError {
                message: String::from("Expected ' ' after the player to move")
            });
        }

        let mut white_can_castle_king_side  = false;
        let mut white_can_castle_queen_side = false;
        let mut black_can_castle_king_side  = false;
        let mut black_can_castle_queen_side = false;

        loop {
            match chars.next() {
                Some(' ') => break,
                Some('-') => continue,

                Some('K') => white_can_castle_king_side = true,
                Some('Q') => white_can_castle_queen_side = true,

                Some('k') => black_can_castle_king_side = true,
                Some('q') => black_can_castle_queen_side = true,

                Some(c) => return Err(FenParseError {
                    message: format!("Unexpected character '{}'", c)
                }),

                None => return Err(FenParseError {
                    message: String::from("Unexpected end of FEN string")
                })
            }
        }

        // TODO: Read en-passant square
        // if chars.next() != Some('-') {
        //     return Err(FenParseError {
        //         message: String::from("En-passant square not supported yet")
        //     });
        // }

        let en_passant_square = match chars.next() {
            Some(file_char) if file_char.is_alphabetic() => {
                if let Some(rank_char) = chars.next() {
                    Some(
                        Square::from_notation(&format!("{}{}", file_char, rank_char))
                            .map_err( |_notation_error| FenParseError {
                                message: String::from("Invalid en-passant notation")
                            } )?
                    )
                } else {
                    return Err(FenParseError {
                        message: String::from("Unexpected end of FEN string while reading en-passant notation")
                    });
                }
            },

            Some('-') => None,

            // TODO: Code duplication
            Some(c) => return Err(FenParseError {
                message: format!("Unexpected character '{}'", c)
            }),

            // TODO: Code duplication
            None => return Err(FenParseError {
                message: String::from("Unexpected end of FEN string")
            })
        };

        if chars.next() != Some(' ') {
            return Err(FenParseError {
                message: String::from("Expected ' ' after the castling flags")
            });
        }

        let half_move_clock = {
            let mut num_string = String::new();

            loop {
                match chars.next() {
                    Some(c) if c.is_digit(10) => num_string.push(c),
                    Some(' ') => break,
                    Some(c) => return Err(FenParseError {
                        message: format!("Unexpected character '{}'", c)
                    }),

                    None => return Err(FenParseError {
                        message: String::from("Unexpected end of FEN string")
                    })
                }
            }

            let int = num_string.parse::<i64>();

            match int {
                Ok(value) => value,
                Err(_) => return Err(FenParseError {
                    message: String::from("Cannot parse half-move clock as int")
                })
            }
        };

        // TODO: Fix code duplication
        let full_move_counter = {
            let mut num_string = String::new();

            loop {
                match chars.next() {
                    Some(c) if c.is_digit(10) => num_string.push(c),
                    Some(c) => return Err(FenParseError {
                        message: format!("Unexpected character '{}'", c)
                    }),

                    None => break
                }
            }

            let int = num_string.parse::<i64>();

            match int {
                Ok(value) => value,
                Err(_) => return Err(FenParseError {
                    message: String::from("Cannot parse half-move clock as int")
                })
            }
        };

        Ok(Position {
            board: Board { squares },

            next_to_move,

            en_passant_square,

            white_can_castle_king_side,
            white_can_castle_queen_side,

            black_can_castle_king_side,
            black_can_castle_queen_side,

            half_move_clock,
            full_move_counter
        })
    }

    pub fn to_fen(&self) -> String {
        let mut fen = String::new();
        let mut blank_square_count = 0;

        for (i, occupancy) in self.board.squares.iter().enumerate() {
            let square = Square { rank: 7 - i as i8 / 8, file: i as i8 % 8 };
            let last_square_in_rank = square.file == 7;

            match occupancy {
                Some(occupancy) => {
                    if blank_square_count > 0 {
                        fen.push_str(&blank_square_count.to_string());
                        blank_square_count = 0;
                    }

                    fen.push(Self::occupancy_to_char(occupancy))
                },
                None => blank_square_count += 1
            }

            if last_square_in_rank && blank_square_count > 0 {
                fen.push_str(&blank_square_count.to_string());
                blank_square_count = 0;
            }

            if last_square_in_rank && square.rank != 0 {
                fen.push('/');
            }
        }

        fen.push(' ');
        fen.push(match self.next_to_move {
            Color::White => 'w',
            Color::Black => 'b'
        });

        fen.push(' ');

        let mut some_castling_possible = false;
        if self.white_can_castle_king_side  { fen.push('K'); some_castling_possible = true }
        if self.white_can_castle_queen_side { fen.push('Q'); some_castling_possible = true }
        if self.black_can_castle_king_side  { fen.push('k'); some_castling_possible = true }
        if self.black_can_castle_queen_side { fen.push('q'); some_castling_possible = true }
        if !some_castling_possible { fen.push('-'); }

        // TODO: en-passant target square
        fen.push(' ');


        match self.en_passant_square {
            Some(square) => fen.push_str(&square.to_notation(SquareNotationOptions::FileAndRank)),
            None => fen.push('-')
        }

        fen.push(' ');
        fen.push_str(&self.half_move_clock.to_string());
        fen.push(' ');
        fen.push_str(&self.full_move_counter.to_string());

        fen
    }

    fn occupancy_to_char(occupancy: &OccupiedSquare) -> char {
        match occupancy {
            OccupiedSquare { piece: Piece::Pawn,   color: Color::White } => 'P',
            OccupiedSquare { piece: Piece::Knight, color: Color::White } => 'N',
            OccupiedSquare { piece: Piece::Bishop, color: Color::White } => 'B',
            OccupiedSquare { piece: Piece::Rook,   color: Color::White } => 'R',
            OccupiedSquare { piece: Piece::Queen,  color: Color::White } => 'Q',
            OccupiedSquare { piece: Piece::King,   color: Color::White } => 'K',

            OccupiedSquare { piece: Piece::Pawn,   color: Color::Black } => 'p',
            OccupiedSquare { piece: Piece::Knight, color: Color::Black } => 'n',
            OccupiedSquare { piece: Piece::Bishop, color: Color::Black } => 'b',
            OccupiedSquare { piece: Piece::Rook,   color: Color::Black } => 'r',
            OccupiedSquare { piece: Piece::Queen,  color: Color::Black } => 'q',
            OccupiedSquare { piece: Piece::King,   color: Color::Black } => 'k'
        }
    }

    fn occupancy_from_char(letter: char) -> Result<OccupiedSquare, FenParseError> {
        match letter {
            'P' => Ok(OccupiedSquare { piece: Piece::Pawn,   color: Color::White }),
            'N' => Ok(OccupiedSquare { piece: Piece::Knight, color: Color::White }),
            'B' => Ok(OccupiedSquare { piece: Piece::Bishop, color: Color::White }),
            'R' => Ok(OccupiedSquare { piece: Piece::Rook,   color: Color::White }),
            'Q' => Ok(OccupiedSquare { piece: Piece::Queen,  color: Color::White }),
            'K' => Ok(OccupiedSquare { piece: Piece::King,   color: Color::White }),

            'p' => Ok(OccupiedSquare { piece: Piece::Pawn,   color: Color::Black }),
            'n' => Ok(OccupiedSquare { piece: Piece::Knight, color: Color::Black }),
            'b' => Ok(OccupiedSquare { piece: Piece::Bishop, color: Color::Black }),
            'r' => Ok(OccupiedSquare { piece: Piece::Rook,   color: Color::Black }),
            'q' => Ok(OccupiedSquare { piece: Piece::Queen,  color: Color::Black }),
            'k' => Ok(OccupiedSquare { piece: Piece::King,   color: Color::Black }),

            _ => Err(FenParseError {
                message: format!("Invalid piece letter '{}'", letter)
            })
        }
    }
}
