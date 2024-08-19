pub mod ast;

use crate::{
    lexer::{
        token::{Keyword, Literal, Token, TokenKind},
        Lexer,
    },
    source::Source,
};
use std::iter::Peekable;
use tracing::info;

pub struct Parser<I: IntoIterator> {
    lexer: Peekable<I::IntoIter>,
    curr_token: Option<Token>,
    source: Source,
}

impl<I: IntoIterator<Item = Token, IntoIter = Lexer>> Parser<I> {
    pub fn new(lexer: I) -> Parser<I> {
        let mut lexer: Lexer = lexer.into_iter();
        let source = lexer.cursor().source().clone();
        info!("Created Parser");
        let curr_token = lexer.next();
        Parser {
            lexer: lexer.peekable(),
            curr_token,
            source,
        }
    }

    fn consume(&mut self) {
        self.curr_token = self.lexer.next();
    }

    fn consume_until(&mut self, kinds: &[TokenKind]) {
        while let Some(token) = self.curr_token.clone() {
            if kinds.contains(&token.kind) {
                self.consume();
            } else {
                break;
            }
        }
    }

    // The skippable tokens are space and tab for now
    // TODO: Add continuation character `\` as skippable
    fn skip_skippable(&mut self) {
        self.consume_until(&[TokenKind::TokenSpace, TokenKind::TokenTab]);
    }

    pub fn parse(&mut self) -> ast::Ast {
        let source = self.source.clone();
        let ast::Block { stmts } = self.parse_block().unwrap();
        let root = ast::Block { stmts };
        ast::Ast::new(source, root)
    }

    fn parse_block(&mut self) -> Option<ast::Block> {
        let mut stmts = Vec::new();
        let stmt = self.parse_stmt();
        match stmt {
            Some(stmt) => stmts.push(stmt),
            None => return None,
        }
        Some(ast::Block {
            stmts: stmts.into_boxed_slice(),
        })
    }

    fn parse_stmt(&mut self) -> Option<ast::Stmt> {
        match self.curr_token {
            Some(Token {
                kind: TokenKind::TokenIdentifier,
                ..
            }) => {
                let stms = self.parse_identifier_stmt();
                info!("Parsed identifier - {:?}", stms);
                Some(stms)
            }
            Some(Token {
                kind: TokenKind::TokenEOF,
                ..
            }) => None,
            _ => todo!(),
        }
    }

    fn parse_identifier_stmt(&mut self) -> ast::Stmt {
        let lhs = self.curr_token.clone().unwrap(); // Safe to unwrap because we checked for Some
                                                    // in parse_stmt

        self.consume();
        self.skip_skippable();

        match self.curr_token {
            Some(Token {
                kind: TokenKind::TokenColon,
                ..
            }) => {
                self.consume();
                self.skip_skippable();
                match self.curr_token {
                    Some(Token {
                        kind: TokenKind::TokenKeyword(_),
                        ..
                    }) => {
                        let type_ = self.parse_type();
                        self.consume();
                        self.skip_skippable();
                        match self.curr_token {
                            Some(Token {
                                kind: TokenKind::TokenAssign,
                                ..
                            }) => {
                                self.consume();
                                self.skip_skippable();
                                let rhs = self.parse_expr();
                                self.consume();
                                self.skip_skippable();
                                match self.curr_token {
                                    Some(Token {
                                        kind: TokenKind::TokenNewLine,
                                        ..
                                    }) => {
                                        self.consume();
                                        self.skip_skippable();
                                        ast::Stmt::Assign {
                                            lhs: ast::Expr::Identifier {
                                                name: lhs.lexeme,
                                                location: lhs.location,
                                            },
                                            type_,
                                            rhs,
                                        }
                                    }
                                    _ => todo!(),
                                }
                            }
                            _ => todo!(),
                        }
                    }
                    _ => todo!(), // Match `(` and parse a function
                }
            }
            _ => todo!(),
        }
    }

    fn parse_type(&mut self) -> ast::Type {
        match &self.curr_token {
            Some(Token {
                kind: TokenKind::TokenKeyword(keyword),
                ..
            }) => match keyword {
                Keyword::IntType => ast::Type::Int,
                Keyword::FloatType => ast::Type::Float,
                Keyword::BoolType => ast::Type::Bool,
                Keyword::StrType => ast::Type::Str,
                _ => todo!(), // Error of invalid type
            },
            _ => todo!(), // Error of unexpected token
        }
    }

    fn parse_expr(&mut self) -> ast::Expr {
        match self.curr_token {
            Some(Token {
                kind: TokenKind::TokenLiteral(_),
                ..
            }) => self.parse_literal_expr(),
            // Some(Token {
            //     kind: TokenKind::TokenIdentifier,
            //     ..
            // }) => self.parse_identifier_expr(),
            _ => todo!(),
        }
    }

    fn parse_literal_expr(&mut self) -> ast::Expr {
        let token = self.curr_token.clone().unwrap(); // Safe to unwrap
        match &self.curr_token {
            Some(Token {
                kind: TokenKind::TokenLiteral(literal),
                ..
            }) => match literal {
                Literal::Int => {
                    let int = match token.lexeme.parse::<i64>() {
                        Ok(int) => int,
                        Err(_) => todo!(), // Error of invalid integer
                    };
                    ast::Expr::Literal {
                        literal: ast::Literal::Int(int),
                        location: token.location,
                    }
                }
                _ => todo!(),
            },
            _ => todo!(),
        }
    }
}

impl Iterator for Parser<Lexer> {
    type Item = ast::Block;

    fn next(&mut self) -> Option<Self::Item> {
        self.parse_block()
    }
}
