use std::str::Chars;
use std::iter::Peekable;

// http://www.saremba.de/chessgml/standards/pgn/pgn-complete.htm

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    // PGN character data is organized as tokens. A token is a contiguous sequence of characters that represents a basic semantic unit. Tokens may be separated from adjacent tokens by white space characters. (White space characters include space, newline, and tab characters.) Some tokens are self delimiting and do not require white space characters.
    Comment(String),

    // A string token is a sequence of zero or more printing characters delimited by a pair of quote characters (ASCII decimal value 34, hexadecimal value 0x22). An empty string is represented by two adjacent quotes. (Note: an apostrophe is not a quote.) A quote inside a string is represented by the backslash immediately followed by a quote. A backslash inside a string is represented by two adjacent backslashes. Strings are commonly used as tag pair values (see below). Non-printing characters like newline and tab are not permitted inside of strings. A string token is terminated by its closing quote. Currently, a string is limited to a maximum of 255 characters of data.
    String(String),

    // An integer token is a sequence of one or more decimal digit characters. It is a special case of the more general "symbol" token class described below. Integer tokens are used to help represent move number indications (see below). An integer token is terminated just prior to the first non-symbol character following the integer digit sequence.
    Integer(i64),

    // A period character (".") is a token by itself. It is used for move number indications (see below). It is self terminating.
    Period,

    // An asterisk character ("*") is a token by itself. It is used as one of the possible game termination markers (see below); it indicates an incomplete game or a game with an unknown or otherwise unavailable result. It is self terminating.
    Asterisk,

    // The left and right bracket characters ("[" and "]") are tokens. They are used to delimit tag pairs (see below). Both are self terminating.
    OpenBracket,
    CloseBracket,

    // The left and right parenthesis characters ("(" and ")") are tokens. They are used to delimit Recursive Annotation Variations (see below). Both are self terminating.
    OpenParen,
    CloseParen,

    // The left and right angle bracket characters ("<" and ">") are tokens. They are reserved for future expansion. Both are self terminating.
    OpenAngleBracket,
    CloseAngleBracket,

    // A Numeric Annotation Glyph ("NAG", see below) is a token; it is composed of a dollar sign character ("$") immediately followed by one or more digit characters. It is terminated just prior to the first non-digit character following the digit sequence.
    NumericAnnotationGlyph(i64),

    // A symbol token starts with a letter or digit character and is immediately followed by a sequence of zero or more symbol continuation characters. These continuation characters are letter characters ("A-Za-z"), digit characters ("0-9"), the underscore ("_"), the plus sign ("+"), the octothorpe sign ("#"), the equal sign ("="), the colon (":"), and the hyphen ("-"). Symbols are used for a variety of purposes. All characters in a symbol are significant. A symbol token is terminated just prior to the first non-symbol character following the symbol character sequence. Currently, a symbol is limited to a maximum of 255 characters in length.
    Symbol(String),

    EndOfFile
}

#[derive(Debug)]
pub enum LexerError {
    ParseIntError(PositionInPGN),
    UnterminatedString(PositionInPGN),
    UnexpectedCharacter(PositionInPGN)
}

impl std::convert::Into<String> for LexerError {
    fn into(self) -> String {
        match self {
            LexerError::ParseIntError(position) => format!("Could not parse int @ {:?}", position),
            LexerError::UnterminatedString(position) => format!("Unterminated string literal @ {:?}", position),
            LexerError::UnexpectedCharacter(position) => format!("Unexpected character @ {:?}", position),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PositionInPGN {
    pub line: i32,
    pub column: i32
}

pub struct Lexer<'a> {
    pgn: Peekable<Chars<'a>>,

    line: i32,
    column: i32
}

impl<'a> Lexer<'a>  {
    pub fn new(pgn: &'a str) -> Lexer<'a> {
        Self {
            pgn: pgn.chars().peekable(),
            line: 1,
            column: 0
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens: Vec<Token> = Vec::new();

        loop {
            let next_char = self.pgn.peek();

            match next_char {
                None => {
                    tokens.push(Token::EndOfFile);
                    break
                },

                Some('%') if self.column == 0 => {
                    loop {
                        match self.next() {
                            None => break,
                            Some('\n') => break,
                            _ => ()
                        }
                    }
                },

                Some(';') if self.column == 0 => {
                    let mut string = String::new();

                    self.next();

                    loop {
                        let c = self.next();

                        match c {
                            None => break,
                            Some('\n') => break,
                            Some(c) => string.push(c)
                        }
                    }

                    tokens.push(Token::Comment(string));
                },

                Some('{') => {
                    let mut string = String::new();

                    self.next();

                    loop {
                        let c = self.next();

                        match c {
                            None => break,
                            Some('}') => break,
                            Some(c) => string.push(c)
                        }
                    }

                    tokens.push(Token::Comment(string));
                },

                // Some(c) if c.is_digit(10) => {
                //     let int = self.read_int();
                //
                //     match int {
                //         Ok(value) => tokens.push(Token::Integer(value)),
                //         Err(_) => return Err(
                //             LexerError::ParseIntError(self.position())
                //         )
                //     }
                // },

                Some('"') => {
                    let mut string = String::new();
                    let mut in_escape_sequence = false;

                    self.next(); // "

                    loop {
                        let c = self.next();

                        match c {
                            None => return Err(LexerError::UnterminatedString(self.position())),

                            Some('\\') => {
                                if in_escape_sequence {
                                    in_escape_sequence = false;
                                    string.push('\\')
                                } else {
                                    in_escape_sequence = true;
                                }
                            },

                            Some('"') => {
                                if in_escape_sequence {
                                    in_escape_sequence = false;
                                    string.push('"');
                                } else {
                                    break
                                }
                            },

                            Some(c) => {
                                if in_escape_sequence {
                                    in_escape_sequence = false;
                                }

                                string.push(c)
                            }
                        }
                    }

                    tokens.push(Token::String(string))
                },

                Some(' ') | Some('\n') => { self.next(); },

                Some('.') => { self.next(); tokens.push(Token::Period) },
                Some('*') => { self.next(); tokens.push(Token::Asterisk) },

                Some('[') => { self.next(); tokens.push(Token::OpenBracket) },
                Some(']') => { self.next(); tokens.push(Token::CloseBracket) },

                Some('(') => { self.next(); tokens.push(Token::OpenParen) },
                Some(')') => { self.next(); tokens.push(Token::CloseParen) },

                Some('<') => { self.next(); tokens.push(Token::OpenAngleBracket) },
                Some('>') => { self.next(); tokens.push(Token::CloseAngleBracket) },

                Some('$') => {
                    self.next(); // $

                    let int = self.read_int();

                    match int {
                        Ok(value) => tokens.push(Token::NumericAnnotationGlyph(value)),
                        Err(_) => return Err(
                            LexerError::ParseIntError(self.position())
                        )
                    }
                },

                Some(c) if Self::is_symbol_start(c) => {
                    let string = self.read_symbol();
                    let is_integer = string.chars().all( |c| c.is_digit(10) );

                    if is_integer {
                        let int = string.parse::<i64>();

                        match int {
                            Ok(value) => tokens.push(Token::Integer(value)),
                            Err(_) => return Err(
                                LexerError::ParseIntError(self.position())
                            )
                        }
                    } else {
                        tokens.push(Token::Symbol(string))
                    }
                },

                Some(_) => return Err(LexerError::UnexpectedCharacter(self.position()))
            }
        }

        Ok(tokens)
    }

    fn is_symbol_start(c: &char) -> bool {
        c.is_alphanumeric()
    }

    fn is_symbol_continuation(c: &char) -> bool {
        match c {
            '_' | '+' | '#' | '=' | ':' | '-' => true,
            _ => c.is_alphanumeric()
        }
    }

    fn read_int(&mut self) -> Result<i64, std::num::ParseIntError> {
        let mut string = String::new();

        loop {
            let c = self.peek();

            match c {
                None => break,
                Some(c) if c.is_digit(10) => string.push(self.next().unwrap()),
                Some(_) => break
            }
        }

        string.parse::<i64>()
    }

    fn read_symbol(&mut self) -> String {
        let mut string = String::new();

        loop {
            let c = self.peek();

            match c {
                None => break,
                Some(c) if Self::is_symbol_continuation(c) => string.push(self.next().unwrap()),
                Some(_) => break
            }
        }

        string
    }

    fn next(&mut self) -> Option<char> {
        let char = self.pgn.next();

        if Some('\n') == char {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }

        char
    }

    fn peek(&mut self) -> Option<&char> {
        self.pgn.peek()
    }

    fn position(&self) -> PositionInPGN {
        PositionInPGN {
            line: self.line,
            column: self.column
        }
    }
}
