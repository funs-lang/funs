use crate::utils::color;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

const KEYWORD_INT: &str = "int";
const KEYWORD_FLOAT: &str = "float";
const KEYWORD_BOOL: &str = "bool";
const KEYWORD_BOOL_TRUE: &str = "true";
const KEYWORD_BOOL_FALSE: &str = "false";
const SEPARATOR_COLON: &str = ":";
const SEPARATOR_ASSIGN: &str = "=";

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum TokenKind {
    TokenLiteral(Literal),
    TokenIdentifier,
    TokenKeyword,
    TokenNewLine, // \n
    TokenColon,   // :
    TokenAssign,  // =
    TokenEOF,     // End of file
}

impl TokenKind {
    fn match_keyword(lexeme: &str) -> Option<TokenKind> {
        match lexeme {
            KEYWORD_INT => Some(TokenKind::TokenKeyword),
            KEYWORD_FLOAT => Some(TokenKind::TokenKeyword),
            KEYWORD_BOOL => Some(TokenKind::TokenKeyword),
            KEYWORD_BOOL_TRUE => Some(TokenKind::TokenLiteral(Literal::Bool(true))),
            KEYWORD_BOOL_FALSE => Some(TokenKind::TokenLiteral(Literal::Bool(false))),
            _ => None,
        }
    }

    fn match_number(lexeme: &str) -> Option<TokenKind> {
        if lexeme.chars().all(char::is_numeric) {
            return Some(TokenKind::TokenLiteral(Literal::Int(
                lexeme.parse().unwrap(),
            )));
        }

        if lexeme.contains('.') {
            return Some(TokenKind::TokenLiteral(Literal::Float(
                lexeme.parse().unwrap(),
            )));
        }

        None
    }

    fn match_separator(lexeme: &str) -> Option<TokenKind> {
        match lexeme {
            SEPARATOR_COLON => Some(TokenKind::TokenColon),
            SEPARATOR_ASSIGN => Some(TokenKind::TokenAssign),
            _ => None,
        }
    }
}

impl From<&String> for TokenKind {
    fn from(lexeme: &String) -> TokenKind {
        if lexeme.eq(&'\n'.to_string()) {
            return TokenKind::TokenNewLine;
        }

        if let Some(keyword) = TokenKind::match_keyword(lexeme) {
            return keyword;
        }

        if let Some(separator) = TokenKind::match_separator(lexeme) {
            return separator;
        }

        if let Some(number) = TokenKind::match_number(lexeme) {
            return number;
        }

        TokenKind::TokenIdentifier
    }
}

/// The location of a token in the source code in a uman-readable format
#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct TokenLocation {
    file_path: PathBuf,
    line: usize,
    column_start: usize, // Inclusive
    column_end: usize,   // Exclusive
}

impl TokenLocation {
    pub fn new(
        file_path: PathBuf,
        line: usize,
        column_start: usize,
        column_end: usize,
    ) -> TokenLocation {
        TokenLocation {
            file_path,
            line,
            column_start,
            column_end,
        }
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column_start(&self) -> usize {
        self.column_start
    }

    pub fn column_end(&self) -> usize {
        self.column_end
    }

    pub fn advance_line(&mut self) {
        self.line += 1;
        self.column_start = 0;
        self.column_end = 0;
    }

    pub fn advance_column_start(&mut self) {
        self.column_start += 1;
    }

    pub fn advance_column_end(&mut self) {
        self.column_end += 1;
    }

    pub fn set_column_start(&mut self, new_column_start: usize) {
        self.column_start = new_column_start;
    }

    pub fn with_file_path(&self, file_path: &Path) -> TokenLocation {
        TokenLocation {
            file_path: file_path.to_path_buf(),
            line: self.line,
            column_start: self.column_start,
            column_end: self.column_end,
        }
    }

    pub fn with_line(&self, line: usize) -> TokenLocation {
        TokenLocation {
            file_path: self.file_path.clone(),
            line,
            column_start: self.column_start,
            column_end: self.column_end,
        }
    }

    pub fn with_column_start(&self, column_start: usize) -> TokenLocation {
        TokenLocation {
            file_path: self.file_path.clone(),
            line: self.line,
            column_start,
            column_end: self.column_end,
        }
    }

    pub fn with_column_end(&self, column_end: usize) -> TokenLocation {
        TokenLocation {
            file_path: self.file_path.clone(),
            line: self.line,
            column_start: self.column_start,
            column_end,
        }
    }
}

impl From<&PathBuf> for TokenLocation {
    fn from(file_path: &PathBuf) -> TokenLocation {
        TokenLocation {
            file_path: file_path.clone(),
            line: 0,
            column_start: 0,
            column_end: 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Token {
    /// The kind of the token
    kind: TokenKind,
    /// The lexeme is the string representation of the token
    ///
    /// For example:
    /// - the lexeme of the token `TokenLiteral(Literal::Int(42))` is "42"
    /// - the lexeme of the token `TokenColon` is ":"
    lexeme: String,
    /// The location of the token in the source code
    location: TokenLocation,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, location: TokenLocation) -> Token {
        Token {
            kind,
            lexeme,
            location,
        }
    }

    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }

    pub fn lexeme(&self) -> &String {
        &self.lexeme
    }

    pub fn location(&self) -> &TokenLocation {
        &self.location
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Literal::Int(value) => write!(f, "Int({})", value),
            Literal::Float(value) => write!(f, "Float({})", value),
            Literal::Bool(value) => write!(f, "Bool({})", value),
        }
    }
}
impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TokenKind::TokenLiteral(literal) => write!(f, "TokenLiteral({})", literal),
            TokenKind::TokenIdentifier => write!(f, "TokenIdentifier"),
            TokenKind::TokenKeyword => write!(f, "TokenKeyword"),
            TokenKind::TokenNewLine => write!(f, "TokenNewLine"),
            TokenKind::TokenColon => write!(f, "TokenColon"),
            TokenKind::TokenAssign => write!(f, "TokenAssign"),
            TokenKind::TokenEOF => write!(f, "TokenEOF"),
        }
    }
}

impl std::fmt::Display for TokenLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "TokenLocation {{ \"{}\", {}, {}, {} }}",
            self.file_path.display(),
            self.line,
            self.column_start,
            self.column_end
        )
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} {{ {}, \"{}\", {} }}",
            color::cyan("Token"),
            color::yellow(&format!("{}", self.kind)),
            color::magenta(&self.lexeme),
            color::blue(&format!("{}", self.location))
        )
    }
}
