use crate::source::SourceLocation;

pub enum Literal {
    Int(i64),
}

pub enum TokenKind {
    TokenLiteral(Literal),
    TokenNewLine, // \n
    TokenColon,   // :
    TokenAssign,  // =
    TokenEOF,     // End of file
}

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
    pub location: SourceLocation,
}
