use crate::{
    lexer::token::{Token, TokenLocation},
    source::Source,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum AstError {
    UnexpectedToken { token: Token },
}

impl std::fmt::Display for AstError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AstError::UnexpectedToken { token } => {
                write!(f, "Unexpected token: {}", token)
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Ast {
    pub source: Source,
    pub root: Block,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Block {
    pub stmts: Box<[Stmt]>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Stmt {
    Assign {
        lhs: Expr,
        type_: Type,
        rhs: Result<Expr, AstError>,
    },
    Expr(Expr),
    Comment {
        comment: String,
        location: TokenLocation,
    },
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Type {
    Int,
    Float,
    Bool,
    Str,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Expr {
    Literal {
        literal: Literal,
        location: TokenLocation,
    },
    Identifier {
        name: String,
        location: TokenLocation,
    },
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
}

impl Ast {
    pub fn new(source: Source, root: Block) -> Ast {
        Ast { source, root }
    }
}
