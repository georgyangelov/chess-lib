use std::vec::Vec;
use lexer::*;
use regex::Regex;
use lazy_static::lazy_static;

use super::{GameResult};

pub mod lexer;

impl GameResult {
    fn from_string(string: &str) -> Option<GameResult> {
        match string {
            "*" => Some(GameResult::Unknown),
            "1-0" => Some(GameResult::WhiteWins),
            "0-1" => Some(GameResult::BlackWins),
            "1/2-1/2" => Some(GameResult::Draw),
            _ => None
        }
    }

    fn to_string(&self) -> &'static str {
        match self {
            GameResult::Unknown   => "*",
            GameResult::WhiteWins => "1-0",
            GameResult::BlackWins => "0-1",
            GameResult::Draw      => "1/2-1/2"
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Move {
    pub number: Option<i64>,
    pub white_move: Option<String>,
    pub black_move: Option<String>
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParsedGame {
    pub tags: Vec<(String, String)>,
    pub moves: Vec<Move>,
    pub result: GameResult
}

struct TagPairSection {
    tag_pairs: Vec<(String, String)>
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Token),
    InvalidGameResult(String),
    UnexpectedEndOfFile
}

macro_rules! consume {
    ($self: ident, $pattern: pat) => {
        {
            let next_token = $self.peek();

            if let $pattern = next_token {
                $self.read()?;
            } else {
                return Err(ParseError::UnexpectedToken(next_token.clone()));
            }
        }
    };
}

macro_rules! consume_optional {
    ($self: ident, $pattern: pat) => {
        {
            let next_token = $self.peek();

            if let $pattern = next_token {
                $self.read()?;

                true
            } else {
                false
            }
        }
    };
}

macro_rules! consume_value {
    ($self: ident, $pattern: pat, $variable: ident) => {
        {
            let next_token = $self.read()?;
            if let $pattern = next_token {
                $variable
            } else {
                return Err(ParseError::UnexpectedToken(next_token.clone()));
            }
        }
    };
}

macro_rules! consume_value_optional {
    ($self: ident, $pattern: pat, $variable: ident) => {
        {
            let next_token = $self.read()?;
            if let $pattern = next_token {
                Some($variable)
            } else {
                None
            }
        }
    };
}

macro_rules! consume_value_optional_if {
    ($self: ident, $pattern: pat, $variable: ident, $condition: expr) => {
        {
            if let $pattern = $self.peek() {
                if $condition {
                    if let $pattern = $self.read()? {
                        Some($variable)
                    } else {
                        panic!("Read token != the peeked one");
                    }
                } else {
                    None
                }
            } else {
                None
            }
        }
    };
}

pub struct Parser {
    tokens: Vec<Token>
}

impl Parser {
    pub fn new(mut tokens: Vec<Token>) -> Self {
        tokens.reverse();

        Self { tokens }
    }

    pub fn parse(&mut self) -> Result<Vec<ParsedGame>, ParseError> {
        let mut games: Vec<ParsedGame> = Vec::new();

        while self.peek() != &Token::EndOfFile {
            let game = self.parse_game()?;

            games.push(game);
        }

        Ok(games)
    }

    fn parse_game(&mut self) -> Result<ParsedGame, ParseError> {
        let tag_pair_section = self.parse_tag_pair_section()?;
        let moves = self.parse_move_text_section()?;

        let result = self.parse_game_result()?;

        Ok(ParsedGame {
            tags: tag_pair_section.tag_pairs,
            moves,
            result
        })
    }

    fn parse_tag_pair_section(&mut self) -> Result<TagPairSection, ParseError> {
        let mut tag_pairs = Vec::new();

        while self.peek() == &Token::OpenBracket {
            let tag_pair = self.parse_tag_pair()?;

            tag_pairs.push(tag_pair);
        }

        Ok(TagPairSection { tag_pairs })
    }

    fn parse_tag_pair(&mut self) -> Result<(String, String), ParseError> {
        consume!(self, Token::OpenBracket);

        let name = consume_value!(self, Token::Symbol(value), value);
        let value = consume_value!(self, Token::String(value), value);

        consume!(self, Token::CloseBracket);

        Ok((name, value))
    }

    fn parse_move_text_section(&mut self) -> Result<Vec<Move>, ParseError> {
        let mut moves = Vec::new();

        while !Self::is_game_end(self.peek()) {
            let current_move = self.parse_move()?;

            moves.push(current_move);
        }

        Ok(moves)
    }

    fn is_game_end(token: &Token) -> bool {
        match token {
            Token::Symbol(result) => GameResult::from_string(result).is_some(),
            _ => false
        }
    }

    fn parse_move(&mut self) -> Result<Move, ParseError> {
        self.ignore_comments()?;

        let number = consume_value_optional!(self, Token::Integer(value), value);
        if number.is_some() {
            consume!(self, Token::Period);

            while consume_optional!(self, Token::Period) {}

            self.ignore_comments()?;
        }

        let white_move = consume_value_optional_if!(
            self, Token::Symbol(value), value,
            Self::is_possibly_a_move(value)
        );
        self.ignore_comments()?;

        let black_move = consume_value_optional_if!(
            self, Token::Symbol(value), value,
            Self::is_possibly_a_move(value)
        );
        self.ignore_comments()?;

        Ok(Move { number, white_move, black_move })
    }

    fn parse_game_result(&mut self) -> Result<GameResult, ParseError> {
        self.ignore_comments()?;

        let outcome = consume_value!(self, Token::Symbol(outcome), outcome);

        GameResult::from_string(&outcome)
            .ok_or(ParseError::InvalidGameResult(outcome))
    }

    fn is_possibly_a_move(notation: &str) -> bool {
        lazy_static! {
            static ref VALID_MOVE_REGEX: regex::Regex =
                Regex::new(r"^(?i)[PNBRQK]?([a-h]?[1-8]?)x?[a-h][1-8](=[NBRQK])?[#\+]?$")
                    .expect("Invalid regular expression");
        }

        VALID_MOVE_REGEX.is_match(notation)
    }

    fn ignore_comments(&mut self) -> Result<(), ParseError> {
        loop {
            match self.peek() {
                Token::Comment(_) => { self.read()?; },
                _ => break
            }
        }

        Ok(())
    }

    fn peek(&self) -> &Token {
        &self.tokens.last().expect("Tried to get token after the end of tokens")
    }

    fn read(&mut self) -> Result<Token, ParseError> {
        let token = self.tokens.pop();

        match token {
            None => Err(ParseError::UnexpectedEndOfFile),
            Some(token) => Ok(token)
        }
    }
}
