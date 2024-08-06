use super::cursor::Cursor;
use super::token::Literal;
use super::Lexer;
use crate::lexer::token::Token;
use crate::lexer::token::TokenKind;
use std::fmt::Debug;

pub trait State: Debug {
    fn visit(&self, cursor: &mut Cursor) -> Transition;
}

#[derive(Debug)]
pub enum TransitionKind {
    Consume,
    Advance,
    EmitToken(Token),
    ErrorToken(Token),
    End,
}

impl TransitionKind {
    pub fn apply(&self, cursor: &mut Cursor) {
        match self {
            TransitionKind::Consume => {
                cursor.consume();
            }
            TransitionKind::Advance => {
                cursor.advance();
            }
            TransitionKind::EmitToken(_) => cursor.align(),
            TransitionKind::ErrorToken(_) => cursor.align(),
            TransitionKind::End => {}
        }
    }
}

#[derive(Debug)]
// TODO: Remove pub from fields
pub struct Transition {
    pub state: Box<dyn State>,
    pub consume_kind: TransitionKind,
}

impl Transition {
    pub fn new(state: Box<dyn State>, consume_kind: TransitionKind) -> Transition {
        Transition {
            state,
            consume_kind,
        }
    }
}

#[derive(Debug)]
pub struct StateStart;

impl State for StateStart {
    fn visit(&self, cursor: &mut Cursor) -> Transition {
        match cursor.peek() {
            Some(c) if c.is_whitespace() => {
                Lexer::proceed(Box::new(StateStart), TransitionKind::Consume)
            }
            Some(c) if c.is_ascii_digit() => {
                Lexer::proceed(Box::new(StateNumber), TransitionKind::Advance)
            }
            Some(c) if c.is_alphabetic() || c.eq(&'_') => {
                Lexer::proceed(Box::new(StateWord), TransitionKind::Advance)
            }
            Some(c) if StateSymbol::is_symbol(c) => {
                Lexer::proceed(Box::new(StateSymbol), TransitionKind::Advance)
            }
            Some(c) => Lexer::proceed(
                Box::new(StateEnd), // TODO: Consider to return to the StartState to continue the
                // lexing
                TransitionKind::ErrorToken(Token::new(
                    TokenKind::from(&c.to_string()),
                    c.to_string(),
                    cursor.location().clone(),
                )),
            ),
            None => Lexer::proceed(Box::new(StateEOF), TransitionKind::Consume),
        }
    }
}

#[derive(Debug)]
pub struct StateNumber;

impl State for StateNumber {
    fn visit(&self, cursor: &mut Cursor) -> Transition {
        match cursor.peek() {
            Some(c) if c.is_ascii_digit() => {
                Lexer::proceed(Box::new(StateNumber), TransitionKind::Advance)
            }
            _ => {
                let lexeme = cursor.source().content()
                    [cursor.location().column_start()..cursor.location().column_end()]
                    .to_string();
                let location = cursor.location().clone();
                Transition {
                    state: Box::new(StateStart),
                    consume_kind: TransitionKind::EmitToken(Token::new(
                        TokenKind::TokenLiteral(Literal::Int(lexeme.parse().unwrap())),
                        lexeme,
                        location,
                    )),
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct StateWord;

impl State for StateWord {
    fn visit(&self, cursor: &mut Cursor) -> Transition {
        match cursor.peek() {
            Some(c) if c.is_alphanumeric() || c.eq(&'_') => {
                Lexer::proceed(Box::new(StateWord), TransitionKind::Advance)
            }
            _ => {
                // Emit token when we encounter a non-alphabetic character
                let lexeme = cursor.source().content()
                    [cursor.location().column_start()..cursor.location().column_end()]
                    .to_string();
                let token_kind = TokenKind::from(&lexeme);
                let location = cursor.location().clone();
                Transition {
                    state: Box::new(StateStart),
                    consume_kind: TransitionKind::EmitToken(Token::new(
                        token_kind, lexeme, location,
                    )),
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct StateSymbol;

impl StateSymbol {
    fn is_symbol(c: char) -> bool {
        matches!(c, ':' | '=')
    }
}

impl State for StateSymbol {
    fn visit(&self, cursor: &mut Cursor) -> Transition {
        match cursor.peek() {
            Some(c) if StateSymbol::is_symbol(c) => {
                Lexer::proceed(Box::new(StateSymbol), TransitionKind::Advance)
            }
            _ => {
                let lexeme = cursor.source().content()
                    [cursor.location().column_start()..cursor.location().column_end()]
                    .to_string();
                let token_kind = TokenKind::from(&lexeme);
                let location = cursor.location().clone();
                Transition {
                    state: Box::new(StateStart),
                    consume_kind: TransitionKind::EmitToken(Token::new(
                        token_kind, lexeme, location,
                    )),
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct StateEOF;

impl State for StateEOF {
    fn visit(&self, cursor: &mut Cursor) -> Transition {
        cursor.align();
        Transition {
            state: Box::new(StateEnd),
            consume_kind: TransitionKind::EmitToken(Token::new(
                TokenKind::TokenEOF,
                "".to_string(),
                cursor.location().clone(),
            )),
        }
    }
}

#[derive(Debug)]
pub struct StateEnd;

impl State for StateEnd {
    fn visit(&self, _cursor: &mut Cursor) -> Transition {
        Transition {
            state: Box::new(StateEnd),
            consume_kind: TransitionKind::End,
        }
    }
}
