use super::models::*;
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug)]
pub struct Game {
    position: Position
}

#[derive(Debug, PartialEq, Eq)]
pub enum InvalidMoveError {
}

#[derive(Debug, PartialEq, Eq)]
pub struct ValidMove {
    color: Color,

    from: Square,
    to: Square,

    piece: Piece,

    takes: Option<Piece>
}

#[derive(Debug, PartialEq, Eq)]
pub struct PartialSquare {
    rank: Option<i8>,
    file: Option<i8>
}

#[derive(Debug, PartialEq, Eq)]
pub struct PartialMove {
    piece: Piece,

    from: Option<PartialSquare>,
    to: Square,

    takes: Option<bool>,
    check_or_mate: Option<Option<CheckOrMate>>,
    castles: Option<Option<CastlesDirection>>
}

struct SquareAttackCheck<'a> {
    pieces: &'a [Piece],
    color: Color,
    squares: &'a [Option<Square>]
}

struct LinesAttackCheck<'a> {
    pieces: &'a [Piece],
    color: Color,
    squares: &'a [Vec<Square>]
}

enum AttackCheck<'a> {
    Squares(SquareAttackCheck<'a>),
    Lines(LinesAttackCheck<'a>)
}

impl Game {
    pub fn new(board: Board, next_to_move: Color) -> Self {
        Self {
            // TODO: Pass position directly
            position: Position {
                board,

                next_to_move,

                white_can_castle: true,
                black_can_castle: true,

                half_move_clock: 0,
                full_move_counter: 0
            }
        }
    }

    pub fn board(&self) -> &Board {
        &self.position.board
    }

    pub fn in_mate(&self) -> bool {
        self.in_check(self.position.next_to_move) && self.valid_moves().len() == 0
    }

    pub fn in_check(&self, color: Color) -> bool {
        // TODO: Cache this lookup in Game
        let king_square = self.position.board.squares.iter()
            .enumerate()
            .find( |(_i, occupancy)|
                match occupancy {
                    Some(occupancy) =>
                        // This assumes only one king, but oh well...
                        occupancy.piece == Piece::King &&
                        occupancy.color == color,
                    None => false
                }
            );

        match king_square {
            Some((i, _)) => self.square_attacked(
                Square { rank: 7 - i as i8 / 8, file: i as i8 % 8 },
                color.opposite()
            ).is_some(),
            None => false
        }
    }

    fn square_attacked(&self, square: Square, by_color: Color) -> Option<ValidMove> {
        let opposite_color_moves = self.valid_moves_for_color(by_color, false);

        // TODO: And not castles
        opposite_color_moves.into_iter().find( |valid_move| valid_move.to == square )
    }

    // fn square_attacked(&self, square: Square, by_color: Color) -> bool {
    //     let attacked_by_king = AttackCheck::Squares(SquareAttackCheck {
    //         pieces: &[Piece::King],
    //         color: by_color,
    //         squares: &[
    //             Square::new(square.rank - 1, square.file - 1),
    //             Square::new(square.rank - 1, square.file    ),
    //             Square::new(square.rank - 1, square.file + 1),
    //
    //             Square::new(square.rank    , square.file - 1),
    //             Square::new(square.rank    , square.file + 1),
    //
    //             Square::new(square.rank + 1, square.file - 1),
    //             Square::new(square.rank + 1, square.file    ),
    //             Square::new(square.rank + 1, square.file + 1),
    //         ]
    //     });
    //
    //     false
    // }

    // fn attack_check(&self, check: AttackCheck) -> Some((Square, OccupiedSquare)) {
    //     match check {
    //         AttackCheck::Squares(square_check) => {
    //             square_check.squares.iter()
    //                 .filter_map( |s| *s )
    //                 .filter_map( |square| self.square_occupied(square) )
    //                 .any( |occupancy|
    //                     occupancy.color == square_check.color &&
    //                     square_check.pieces.iter().find( |piece| piece ==  )
    //                 )
    //         }
    //     }
    // }

    pub fn valid_moves(&self) -> Vec<ValidMove> {
        self.valid_moves_for_color(self.position.next_to_move, true)
    }

    fn valid_moves_for_color(&self, for_color: Color, filter_out_discover_checks: bool) -> Vec<ValidMove> {
        let mut valid_moves = Vec::new();

        for (i, occupied_square) in self.position.board.squares.iter().enumerate() {
            let square = Square { rank: 7 - i as i8 / 8, file: i as i8 % 8 };

            let occupied_square = match occupied_square {
                Some(occupied_square) => occupied_square,
                None => continue
            };

            let piece = match occupied_square {
                OccupiedSquare { piece, color } if color == &for_color => piece,
                _ => continue
            };

            let mut moves = self.possible_moves_for_piece(*piece, square, for_color);

            valid_moves.append(&mut moves);
        }

        if filter_out_discover_checks {
            // Filter out moves that result in a check
            valid_moves.into_iter()
                .filter( |valid_move| !self.make_valid_move(valid_move).in_check(for_color) )
                .collect()
        } else {
            valid_moves
        }
    }

    pub fn make_move(&self, notation: &str) -> Result<Self, ()> {
        let move_to_make = ValidMove::from_notation(self, notation)?;

        Ok(self.make_valid_move(&move_to_make))
    }

    fn make_valid_move(&self, move_to_make: &ValidMove) -> Self {
        let mut new_squares = self.position.board.squares.clone();

        let from = move_to_make.from;
        let to = move_to_make.to;

        new_squares[((7 - from.rank) * 8 + from.file) as usize] = None;
        new_squares[((7 - to.rank) * 8 + to.file) as usize] = Some(OccupiedSquare {
            piece: move_to_make.piece,
            color: move_to_make.color
        });

        Game {
            position: Position {
                board: Board {
                    squares: new_squares
                },

                next_to_move: self.position.next_to_move.opposite(),

                white_can_castle: self.position.white_can_castle,
                black_can_castle: self.position.black_can_castle,

                // TODO
                half_move_clock: self.position.half_move_clock + 1,
                full_move_counter: if move_to_make.color == Color::White {
                    self.position.full_move_counter
                } else {
                    self.position.full_move_counter + 1
                }
            }
        }
    }

    pub fn find_moves(&self, template: PartialMove) -> Vec<ValidMove> {
        let moves = self.valid_moves();

        moves.into_iter()
            .filter( |valid_move| Self::move_matches(valid_move, &template))
            .collect()
    }

    pub fn filter_moves<'a>(moves: &'a [ValidMove], template: PartialMove) -> Vec<&'a ValidMove> {
        moves.iter()
            .filter( |valid_move| Self::move_matches(valid_move, &template))
            .collect()
    }

    fn move_matches(m: &ValidMove, template: &PartialMove) -> bool {
        if template.piece != m.piece {
            return false;
        }

        match &template.from {
            Some(partial_square) => {
                if let Some(rank) = partial_square.rank {
                    if rank != m.from.rank {
                        return false;
                    }
                }

                if let Some(file) = partial_square.file {
                    if file != m.from.file {
                        return false;
                    }
                }
            },
            None => ()
        }

        if template.to != m.to {
            return false;
        }

        match template.takes {
            Some(takes) => {
                if m.takes.is_some() != takes {
                    return false
                }
            },
            None => ()
        }

        // TODO
        // match template.check_or_mate
        // match template.castles

        true
    }

    fn possible_moves_for_piece(&self, piece: Piece, from: Square, color: Color) -> Vec<ValidMove> {
        let moves = match piece {
            Piece::Pawn   => self.possible_pawn_moves(from, color),
            Piece::Knight => self.possible_knight_moves(from, color),
            Piece::Rook   => self.possible_rook_moves(from, color),
            Piece::Bishop => self.possible_bishop_moves(from, color),
            Piece::Queen  => self.possible_queen_moves(from, color),
            Piece::King   => self.possible_king_moves(from, color)
        };

        println!(
            "{:?} {:?} {:?} -> {:?}",
            piece,
            from,
            color,
            moves.iter().map( |m| m.notation() ).collect::<Vec<String>>()
        );

        moves
    }

    fn can_pawn_double_move(square: Square, side_to_move: Color) -> bool {
        side_to_move == Color::White && square.rank == 1 ||
        side_to_move == Color::Black && square.rank == 6
    }

    fn square_occupied(&self, square: Square) -> Option<&OccupiedSquare> {
        self.position.board.squares[((7 - square.rank) * 8 + square.file) as usize].as_ref()
    }

    fn possible_pawn_moves(&self, from: Square, color: Color) -> Vec<ValidMove> {
        let direction = match color {
            Color::White => 1,
            Color::Black => -1
        };

        let next_square = Square::new(from.rank + direction, from.file);
        let can_move_forward = if let Some(next_square) = next_square {
            self.square_occupied(next_square) == None
        } else { false };

        let forward_squares = [
            Square::new(from.rank + direction, from.file).filter( |_| can_move_forward ),
            Square::new(from.rank + 2 * direction, from.file).filter( |to|
                can_move_forward &&
                    Self::can_pawn_double_move(from, color) &&
                    self.square_occupied(*to) == None
            ),
        ];

        let forward_moves = forward_squares.iter()
            .filter_map( |to| *to )
            .map( |to| ValidMove::new(&self.position, Piece::Pawn, color, from, to, None));

        // TODO: En passant takes
        let take_squares = [
            Square::new(from.rank + direction, from.file - 1),
            Square::new(from.rank + direction, from.file + 1)
        ];

        let take_moves = take_squares.iter()
            .map( |square|
                square.map( |square|
                    (
                        square,
                        self.square_occupied(square).filter( |occupancy| occupancy.color != color )
                    )
                )
            )
            .filter_map( |to_and_occupancy|
                match to_and_occupancy {
                    Some((to, Some(occupancy))) => Some(ValidMove::new(&self.position, Piece::Pawn, color, from, to, Some(occupancy))),
                    Some(_) => None,
                    None => None
                }
            );

        forward_moves.into_iter()
            .chain(take_moves)
            .collect()
    }

    fn possible_knight_moves(&self, from: Square, color: Color) -> Vec<ValidMove> {
        let reachable_squares = [
            Square::new(from.rank - 2, from.file - 1),
            Square::new(from.rank - 2, from.file + 1),

            Square::new(from.rank + 2, from.file - 1),
            Square::new(from.rank + 2, from.file + 1),

            Square::new(from.rank - 1, from.file - 2),
            Square::new(from.rank - 1, from.file + 2),

            Square::new(from.rank + 1, from.file - 2),
            Square::new(from.rank + 1, from.file + 2),
        ];

        reachable_squares.iter()
            .filter_map( |square| *square )
            .filter_map( |to| {
                let occupancy = self.square_occupied(to);

                match occupancy {
                    Some(OccupiedSquare { piece: _, color: other_piece_color }) if other_piece_color != &color =>
                        Some(ValidMove::new(&self.position, Piece::Knight, color, from, to, occupancy)),

                    None => Some(ValidMove::new(&self.position, Piece::Knight, color, from, to, occupancy)),

                    _ => None,
                }
            })
            .collect()
    }

    fn possible_rook_moves(&self, from: Square, color: Color) -> Vec<ValidMove> {
        let lines = [
            self.squares_in_a_line(from, -1, 0),
            self.squares_in_a_line(from, 1, 0),
            self.squares_in_a_line(from, 0, -1),
            self.squares_in_a_line(from, 0, 1),
        ];

        lines.iter()
            .flat_map( |line| self.valid_moves_in_a_line(line, Piece::Rook, from, color) )
            .collect()
    }

    fn possible_queen_moves(&self, from: Square, color: Color) -> Vec<ValidMove> {
        let lines = [
            self.squares_in_a_line(from, -1, 0),
            self.squares_in_a_line(from, 1, 0),
            self.squares_in_a_line(from, 0, -1),
            self.squares_in_a_line(from, 0, 1),

            self.squares_in_a_line(from, -1, -1),
            self.squares_in_a_line(from, 1, -1),
            self.squares_in_a_line(from, 1, 1),
            self.squares_in_a_line(from, -1, 1),
        ];

        lines.iter()
            .flat_map( |line| self.valid_moves_in_a_line(line, Piece::Queen, from, color) )
            .collect()
    }

    fn possible_bishop_moves(&self, from: Square, color: Color) -> Vec<ValidMove> {
        let lines = [
            self.squares_in_a_line(from, -1, -1),
            self.squares_in_a_line(from, 1, -1),
            self.squares_in_a_line(from, 1, 1),
            self.squares_in_a_line(from, -1, 1),
        ];

        lines.iter()
            .flat_map( |line| self.valid_moves_in_a_line(line, Piece::Bishop, from, color) )
            .collect()
    }

    fn possible_king_moves(&self, from: Square, color: Color) -> Vec<ValidMove> {
        let adjacent_squares = [
            Square::new(from.rank - 1, from.file - 1),
            Square::new(from.rank - 1, from.file    ),
            Square::new(from.rank - 1, from.file + 1),

            Square::new(from.rank    , from.file - 1),
            Square::new(from.rank    , from.file + 1),

            Square::new(from.rank + 1, from.file - 1),
            Square::new(from.rank + 1, from.file    ),
            Square::new(from.rank + 1, from.file + 1),
        ];

        adjacent_squares.iter().filter_map( |to| *to ).filter_map( |to| {
            let occupancy = self.square_occupied(to);

            match occupancy {
                Some(OccupiedSquare { piece: _, color: piece_color }) if color != *piece_color => {
                    Some(ValidMove::new(&self.position, Piece::King, color, from, to, occupancy))
                },

                Some(_) => None,

                None => Some(ValidMove::new(&self.position, Piece::King, color, from, to, occupancy))
            }
        }).collect()
    }

    fn valid_moves_in_a_line(&self, line: &[Square], piece: Piece, from: Square, color: Color) -> Vec<ValidMove> {
        let mut valid_moves = Vec::new();

        for &to in line {
            let occupancy = self.square_occupied(to);

            match occupancy {
                Some(OccupiedSquare { piece: _, color: piece_color }) if color != *piece_color => {
                    valid_moves.push(ValidMove::new(&self.position, piece, color, from, to, occupancy));
                    break
                },

                Some(_) => break,

                None => valid_moves.push(ValidMove::new(&self.position, piece, color, from, to, occupancy))
            }
        }

        valid_moves
    }

    fn squares_in_a_line(&self, from: Square, rank_delta: i8, file_delta: i8) -> Vec<Square> {
        let mut squares = Vec::new();
        let mut current_square = Self::advance_square(from, rank_delta, file_delta);

        loop {
            if let Some(square) = current_square {
                squares.push(square);
                current_square = Self::advance_square(square, rank_delta, file_delta);
            } else {
                break;
            }
        }

        squares
    }

    fn advance_square(square: Square, rank_delta: i8, file_delta: i8) -> Option<Square> {
        Square::new(square.rank + rank_delta, square.file + file_delta)
    }
}

impl ValidMove {
    fn new(
        _position: &Position,
        piece: Piece,
        color: Color,
        from: Square,
        to: Square,
        to_occupancy: Option<&OccupiedSquare>
    ) -> Self {
        Self {
            color,

            from,
            to,

            piece,
            takes: to_occupancy.map( |occupancy| occupancy.piece )
        }
    }

    pub fn notation(&self) -> String {
        // TODO: Disambiguation square
        // TODO: Promotion
        // TODO: Castling

        let piece = match self.piece {
            Piece::Pawn   => "",
            Piece::Bishop => "B",
            Piece::Knight => "N",
            Piece::Rook   => "R",
            Piece::Queen  => "Q",
            Piece::King   => "K"
        };

        // TODO: This only works for pawns for now
        let disambiguation = if self.piece == Piece::Pawn && self.takes.is_some() {
            self.from.to_notation(SquareNotationOptions::OnlyFile)
        } else { String::from("") };

        let takes = if self.takes.is_some() { "x" } else { "" };
        let to_square = self.to.to_notation(SquareNotationOptions::FileAndRank);

        format!(
            "{}{}{}{}",
            piece,
            disambiguation,
            takes,
            to_square
        )
    }

    pub fn from_notation(game: &Game, notation: &str) -> Result<ValidMove, ()> {
        lazy_static! {
            static ref NOTATION_REGEX: regex::Regex =
                Regex::new(r"^((?P<piece>[PNBRQK])?(?P<from>[a-h]?[1-8]?)(?P<takes>x)?(?P<to>[a-h][1-8])(=(?P<promotion>[PNBRQK]))?)|(?P<castles>O\-O(\-O))(?P<check_or_mate>[#\+])?$")
                    .expect("Invalid regular expression");
        }

        let chars: Vec<char> = notation.chars().collect();
        let matches = NOTATION_REGEX.captures(notation).ok_or(())?;

        let piece = matches.name("piece")
            .map( |m| m.as_str() )
            .and_then( |piece| Self::parse_piece_letter(piece) );

        // let from_square = matches.name("from").map( |m| m.as_str() );
        let takes = matches.name("takes").filter( |m| m.as_str().len() > 0 ) != None;

        let to = matches.name("to").ok_or(())?;
        let to = Square::from_notation(to.as_str())?;

        let promotion_piece = matches.name("promotion").and_then( |m| Self::parse_piece_letter(m.as_str()) );
        let check_or_mate   = matches.name("check_or_mate").and_then( |m|
            match m.as_str() {
                "#" => Some(CheckOrMate::Mate),
                "+" => Some(CheckOrMate::Check),
                _   => None
            }
        );
        let castles = matches.name("castles").and_then( |m|
            match m.as_str() {
                "O-O"   => Some(CastlesDirection::KingSide),
                "O-O-O" => Some(CastlesDirection::QueenSide),
                _       => None
            }
        );

        let mut valid_moves = game.find_moves(PartialMove {
            piece: match piece {
                Some(piece) => piece,
                None => Piece::Pawn
            },

            // TODO
            from: None,
            to,

            castles: Some(castles),
            check_or_mate: Some(check_or_mate),

            takes: Some(takes)
        });

        if valid_moves.len() == 1 {
            Ok(valid_moves.pop().unwrap())
        } else {
            Err(())
        }
    }

    fn parse_piece_letter(letter: &str) -> Option<Piece> {
        match letter.to_uppercase().as_str() {
            "N" => Some(Piece::Knight),
            "P" => Some(Piece::Pawn),
            "B" => Some(Piece::Bishop),
            "R" => Some(Piece::Rook),
            "Q" => Some(Piece::Queen),
            "K" => Some(Piece::King),

            _ => None
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum CheckOrMate {
    Check,
    Mate
}

#[derive(Debug, PartialEq, Eq)]
enum CastlesDirection {
    KingSide,
    QueenSide
}
