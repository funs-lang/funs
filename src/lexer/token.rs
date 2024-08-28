use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

const KEYWORD_UNIT: &str = "unit";
const KEYWORD_INT: &str = "int";
const KEYWORD_FLOAT: &str = "float";
const KEYWORD_BOOL: &str = "bool";
const KEYWORD_STR: &str = "str";
const KEYWORD_BOOL_TRUE: &str = "true";
const KEYWORD_BOOL_FALSE: &str = "false";
const KEYWORD_MATCH: &str = "match";
const KEYWORD_IF: &str = "if";
const KEYWORD_THEN: &str = "then";
const KEYWORD_ELSE: &str = "else";
const KEYWORD_DATA: &str = "data";

const DOT: &str = ".";
const COLON: &str = ":";
const SEMICOLON: &str = ";";
const ASSIGN: &str = "=";
const NEW_LINE: &str = "\n";
const SINGLE_QUOTE: &str = "'";
const DOUBLE_QUOTE: &str = "\"";
const OPEN_PAREN: &str = "(";
const CLOSE_PAREN: &str = ")";
const OPEN_BRACKET: &str = "{";
const CLOSE_BRACKET: &str = "}";
const OPEN_BRACE: &str = "[";
const CLOSE_BRACE: &str = "]";
const COMMA: &str = ",";
const MINUS: &str = "-";
const PLUS: &str = "+";
const STAR: &str = "*";
const SLASH: &str = "/";
const GREATER: &str = ">";
const RIGHT_ARROW: &str = "->";
const RIGHT_DOUBLE_ARROW: &str = "=>";
const PLUS_PLUS: &str = "++"; // concat for list
const UNDERSCORE: &str = "_";
const PIPE: &str = "|";

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum Literal {
    Int,
    Float,
    Bool,
    Str,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum Keyword {
    UnitType,
    IntType,
    FloatType,
    BoolType,
    StrType,
    Match,
    If,
    Then,
    Else,
    Data,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum TokenKind {
    TokenLiteral(Literal),
    TokenKeyword(Keyword),
    TokenIdentifier,
    TokenComment,
    TokenSpace,            // ' '
    TokenTab,              // \t
    TokenNewLine,          // \n
    TokenDot,              // .
    TokenColon,            // :
    TokenSemicolon,        // ;
    TokenAssign,           // =
    TokenSingleQuote,      // '
    TokenDoubleQuote,      // "
    TokenOpenParen,        // (
    TokenCloseParen,       // )
    TokenOpenBrace,        // {
    TokenCloseBrace,       // }
    TokenOpenBracket,      // [
    TokenCloseBracket,     // ]
    TokenComma,            // ,
    TokenGreater,          // >
    TokenRightArrow,       // ->
    TokenRightDoubleArrow, // =>
    TokenPlusPlus,         // ++
    TokenUnderscore,       // _
    TokenPipe,             // |
    TokenEOF,              // End of file
    // Operators
    TokenPlus,  // +
    TokenMinus, // -
    TokenStar,  // *
    TokenSlash, // /
    TokenUnknown,
}

impl TokenKind {
    pub fn can_be_followed_by_another_symbol(c: &str) -> bool {
        matches!(c, MINUS | ASSIGN | PLUS)
    }

    pub fn is_symbol(c: &str) -> bool {
        matches!(
            c,
            COLON
                | DOT
                | SEMICOLON
                | ASSIGN
                | SINGLE_QUOTE
                | DOUBLE_QUOTE
                | OPEN_PAREN
                | CLOSE_PAREN
                | OPEN_BRACE
                | CLOSE_BRACE
                | OPEN_BRACKET
                | CLOSE_BRACKET
                | UNDERSCORE
                | PIPE
                | COMMA
                | MINUS
                | PLUS
                | STAR
                | SLASH
                | GREATER
                | NEW_LINE
        )
    }

    fn match_keyword(lexeme: &str) -> Option<TokenKind> {
        match lexeme {
            KEYWORD_BOOL_TRUE => Some(TokenKind::TokenLiteral(Literal::Bool)),
            KEYWORD_BOOL_FALSE => Some(TokenKind::TokenLiteral(Literal::Bool)),
            KEYWORD_UNIT => Some(TokenKind::TokenKeyword(Keyword::UnitType)),
            KEYWORD_INT => Some(TokenKind::TokenKeyword(Keyword::IntType)),
            KEYWORD_FLOAT => Some(TokenKind::TokenKeyword(Keyword::FloatType)),
            KEYWORD_BOOL => Some(TokenKind::TokenKeyword(Keyword::BoolType)),
            KEYWORD_STR => Some(TokenKind::TokenKeyword(Keyword::StrType)),
            KEYWORD_MATCH => Some(TokenKind::TokenKeyword(Keyword::Match)),
            KEYWORD_IF => Some(TokenKind::TokenKeyword(Keyword::If)),
            KEYWORD_THEN => Some(TokenKind::TokenKeyword(Keyword::Then)),
            KEYWORD_ELSE => Some(TokenKind::TokenKeyword(Keyword::Else)),
            KEYWORD_DATA => Some(TokenKind::TokenKeyword(Keyword::Data)),
            _ => None,
        }
    }

    fn match_number(lexeme: &str) -> Option<TokenKind> {
        if lexeme.chars().all(char::is_numeric) {
            return Some(TokenKind::TokenLiteral(Literal::Int));
        }

        if lexeme.contains('.') {
            return Some(TokenKind::TokenLiteral(Literal::Float));
        }

        None
    }

    fn match_separator(lexeme: &str) -> Option<TokenKind> {
        match lexeme {
            DOT => Some(TokenKind::TokenDot),
            COLON => Some(TokenKind::TokenColon),
            SEMICOLON => Some(TokenKind::TokenSemicolon),
            ASSIGN => Some(TokenKind::TokenAssign),
            SINGLE_QUOTE => Some(TokenKind::TokenSingleQuote),
            DOUBLE_QUOTE => Some(TokenKind::TokenDoubleQuote),
            OPEN_PAREN => Some(TokenKind::TokenOpenParen),
            CLOSE_PAREN => Some(TokenKind::TokenCloseParen),
            OPEN_BRACE => Some(TokenKind::TokenOpenBrace),
            CLOSE_BRACE => Some(TokenKind::TokenCloseBrace),
            OPEN_BRACKET => Some(TokenKind::TokenOpenBracket),
            CLOSE_BRACKET => Some(TokenKind::TokenCloseBracket),
            UNDERSCORE => Some(TokenKind::TokenUnderscore),
            COMMA => Some(TokenKind::TokenComma),
            PLUS => Some(TokenKind::TokenPlus),
            MINUS => Some(TokenKind::TokenMinus),
            STAR => Some(TokenKind::TokenStar),
            SLASH => Some(TokenKind::TokenSlash),
            RIGHT_ARROW => Some(TokenKind::TokenRightArrow),
            RIGHT_DOUBLE_ARROW => Some(TokenKind::TokenRightDoubleArrow),
            PLUS_PLUS => Some(TokenKind::TokenPlusPlus),
            PIPE => Some(TokenKind::TokenPipe),
            _ => None,
        }
    }
}

impl From<&String> for TokenKind {
    fn from(lexeme: &String) -> TokenKind {
        if lexeme.eq(&'\n'.to_string()) {
            return TokenKind::TokenNewLine;
        }
        if lexeme.eq(&'\t'.to_string()) {
            return TokenKind::TokenTab;
        }
        if lexeme.eq(&' '.to_string()) {
            return TokenKind::TokenSpace;
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
    pub file_path: PathBuf,
    pub line: usize,
    pub column_start: usize, // Inclusive
    pub column_end: usize,   // Exclusive
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
    pub kind: TokenKind,
    /// The lexeme is the string representation of the token
    ///
    /// For example:
    /// - the lexeme of the token `TokenLiteral(Literal::Int(42))` is "42"
    /// - the lexeme of the token `TokenColon` is ":"
    pub lexeme: String,
    /// The location of the token in the source code
    pub location: TokenLocation,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, location: TokenLocation) -> Token {
        Token {
            kind,
            lexeme,
            location,
        }
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Literal::Int => write!(f, "Int"),
            Literal::Float => write!(f, "Float"),
            Literal::Bool => write!(f, "Bool"),
            Literal::Str => write!(f, "Str"),
        }
    }
}

impl std::fmt::Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Keyword::UnitType => write!(f, "UnitType"),
            Keyword::IntType => write!(f, "IntType"),
            Keyword::FloatType => write!(f, "FloatType"),
            Keyword::BoolType => write!(f, "BoolType"),
            Keyword::StrType => write!(f, "StrType"),
            Keyword::Match => write!(f, "Match"),
            Keyword::If => write!(f, "If"),
            Keyword::Then => write!(f, "Then"),
            Keyword::Else => write!(f, "Else"),
            Keyword::Data => write!(f, "Data"),
        }
    }
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TokenKind::TokenLiteral(literal) => write!(f, "TokenLiteral({})", literal),
            TokenKind::TokenIdentifier => write!(f, "TokenIdentifier"),
            TokenKind::TokenKeyword(keyword) => write!(f, "TokenKeyword({})", keyword),
            TokenKind::TokenComment => write!(f, "TokenComment"),
            TokenKind::TokenSpace => write!(f, "TokenSpace"),
            TokenKind::TokenTab => write!(f, "TokenTab"),
            TokenKind::TokenNewLine => write!(f, "TokenNewLine"),
            TokenKind::TokenDot => write!(f, "TokenDot"),
            TokenKind::TokenSemicolon => write!(f, "TokenSemicolon"),
            TokenKind::TokenColon => write!(f, "TokenColon"),
            TokenKind::TokenAssign => write!(f, "TokenAssign"),
            TokenKind::TokenSingleQuote => write!(f, "TokenTick"),
            TokenKind::TokenDoubleQuote => write!(f, "TokenDoubleTick"),
            TokenKind::TokenOpenParen => write!(f, "TokenOpenParen"),
            TokenKind::TokenCloseParen => write!(f, "TokenCloseParen"),
            TokenKind::TokenOpenBrace => write!(f, "TokenOpenBrace"),
            TokenKind::TokenCloseBrace => write!(f, "TokenCloseBrace"),
            TokenKind::TokenOpenBracket => write!(f, "TokenOpenBracket"),
            TokenKind::TokenCloseBracket => write!(f, "TokenCloseBracket"),
            TokenKind::TokenGreater => write!(f, "TokenGreater"),
            TokenKind::TokenComma => write!(f, "TokenComma"),
            TokenKind::TokenRightArrow => write!(f, "TokenRightArrow"),
            TokenKind::TokenRightDoubleArrow => write!(f, "TokenRightDoubleArrow"),
            TokenKind::TokenPlusPlus => write!(f, "TokenPlusPlus"),
            TokenKind::TokenUnderscore => write!(f, "TokenUnderscore"),
            TokenKind::TokenPipe => write!(f, "TokenPipe"),
            TokenKind::TokenEOF => write!(f, "TokenEOF"),
            TokenKind::TokenPlus => write!(f, "TokenPlus"),
            TokenKind::TokenMinus => write!(f, "TokenMinus"),
            TokenKind::TokenStar => write!(f, "TokenMultiply"),
            TokenKind::TokenSlash => write!(f, "TokenDivide"),
            TokenKind::TokenUnknown => write!(f, "TokenUnknown"),
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
        // use crate::utils::color;
        write!(
            f,
            "Token {{ {}, \"{}\", {} }}",
            // color::cyan("Token"),
            self.kind,     // color::yellow(&format!("{}", self.kind)),
            self.lexeme,   // color::magenta(&self.lexeme),
            self.location, // color::blue(&format!("{}", self.location))
        )
    }
}
