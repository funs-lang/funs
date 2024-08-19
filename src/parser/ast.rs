use crate::{lexer::token::TokenLocation, source::Source};

#[derive(Debug)]
pub struct Ast {
    pub source: Source,
    pub root: Block,
}

#[derive(Debug)]
pub struct Block {
    pub stmts: Box<[Stmt]>,
}

#[derive(Debug)]
pub enum Stmt {
    Assign { lhs: Expr, type_: Type, rhs: Expr },
    Expr(Expr),
}

#[derive(Debug)]
pub enum Type {
    Int,
    Float,
    Bool,
    Str,
}

#[derive(Debug)]
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

#[derive(Debug)]
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
