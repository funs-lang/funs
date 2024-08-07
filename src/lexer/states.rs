use super::cursor::Cursor;
use super::token::Literal;
use super::Lexer;
use super::LexerError;
use crate::lexer::token::Token;
use crate::lexer::token::TokenKind;
use std::fmt::Debug;

pub trait State: Debug {
    fn visit(&self, cursor: &mut Cursor) -> Result<Transition, LexerError>;
}

#[derive(Debug)]
pub enum TransitionKind {
    Consume,
    Advance,
    Empty, // Keep cursors in the same position
    EmitToken(Token),
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
            TransitionKind::Empty => {}
            TransitionKind::EmitToken(_) => cursor.align(),
            TransitionKind::End => {}
        }
    }
}

#[derive(Debug)]
pub struct Transition {
    state: Box<dyn State>,
    transition_kind: TransitionKind,
}

impl Transition {
    pub fn new(state: Box<dyn State>, consume_kind: TransitionKind) -> Transition {
        Transition {
            state,
            transition_kind: consume_kind,
        }
    }

    pub fn into_parts(self) -> (Box<dyn State>, TransitionKind) {
        (self.state, self.transition_kind)
    }
}

#[derive(Debug)]
pub struct StateStart;

impl State for StateStart {
    fn visit(&self, cursor: &mut Cursor) -> Result<Transition, LexerError> {
        match cursor.peek() {
            Some(c) if c.eq(&' ') || c.eq(&'\t') || c.eq(&'\r') => Ok(Lexer::proceed(
                Box::new(StateStart),
                TransitionKind::Consume,
            )),
            Some('#') => Ok(Lexer::proceed(
                Box::new(StateComment),
                TransitionKind::Consume,
            )),
            Some(c) if c.is_ascii_digit() => Ok(Lexer::proceed(
                Box::new(StateNumber),
                TransitionKind::Advance,
            )),
            Some(c) if c.is_alphabetic() || c.eq(&'_') => {
                Ok(Lexer::proceed(Box::new(StateWord), TransitionKind::Advance))
            }
            Some(c) if StateSymbol::is_symbol(c) => {
                Ok(Lexer::proceed(Box::new(StateSymbol), TransitionKind::Empty))
            }
            Some(c) => Err(LexerError::UnexpectedToken(Token::new(
                TokenKind::from(&c.to_string()),
                c.to_string(),
                cursor.location().clone(),
            ))),
            None => Ok(Lexer::proceed(Box::new(StateEOF), TransitionKind::Consume)),
        }
    }
}

#[derive(Debug)]
pub struct StateComment;
impl State for StateComment {
    fn visit(&self, cursor: &mut Cursor) -> Result<Transition, LexerError> {
        match cursor.peek() {
            Some('\n') => Ok(Lexer::proceed(Box::new(StateStart), TransitionKind::Empty)),
            _ => Ok(Lexer::proceed(
                Box::new(StateComment),
                TransitionKind::Consume,
            )),
        }
    }
}

#[derive(Debug)]
pub struct StateNumber;

impl State for StateNumber {
    fn visit(&self, cursor: &mut Cursor) -> Result<Transition, LexerError> {
        match cursor.peek() {
            Some(c) if c.is_ascii_digit() => Ok(Lexer::proceed(
                Box::new(StateNumber),
                TransitionKind::Advance,
            )),
            _ => {
                let lexeme = cursor.source().content()[cursor.index()..cursor.offset()].to_string();
                let location = cursor.location().clone();
                Ok(Transition {
                    state: Box::new(StateStart),
                    transition_kind: TransitionKind::EmitToken(Token::new(
                        TokenKind::TokenLiteral(Literal::Int(lexeme.parse().unwrap())),
                        lexeme,
                        location,
                    )),
                })
            }
        }
    }
}

#[derive(Debug)]
pub struct StateWord;

impl State for StateWord {
    fn visit(&self, cursor: &mut Cursor) -> Result<Transition, LexerError> {
        match cursor.peek() {
            Some(c) if c.is_alphanumeric() || c.eq(&'_') => {
                Ok(Lexer::proceed(Box::new(StateWord), TransitionKind::Advance))
            }
            _ => {
                // Emit token when we encounter a non-alphabetic character
                let lexeme = cursor.source().content()[cursor.index()..cursor.offset()].to_string();
                let token_kind = TokenKind::from(&lexeme);
                let location = cursor.location().clone();
                Ok(Transition {
                    state: Box::new(StateStart),
                    transition_kind: TransitionKind::EmitToken(Token::new(
                        token_kind, lexeme, location,
                    )),
                })
            }
        }
    }
}

#[derive(Debug)]
pub struct StateSymbol;

impl StateSymbol {
    fn is_symbol(c: char) -> bool {
        matches!(c, ':' | '=' | '\n')
    }
}

impl State for StateSymbol {
    fn visit(&self, cursor: &mut Cursor) -> Result<Transition, LexerError> {
        match cursor.peek() {
            Some('\n') => {
                let transition = Lexer::proceed(
                    Box::new(StateStart),
                    TransitionKind::EmitToken(Token::new(
                        TokenKind::TokenNewLine,
                        "\\n".to_string(),
                        cursor.location().clone(),
                    )),
                );
                cursor.new_line();
                Ok(transition)
            }
            Some(c) if StateSymbol::is_symbol(c) => Ok(Lexer::proceed(
                Box::new(StateSymbol),
                TransitionKind::Advance,
            )),
            _ => {
                let lexeme = cursor.source().content()[cursor.index()..cursor.offset()].to_string();
                let token_kind = TokenKind::from(&lexeme);
                let location = cursor.location().clone();
                Ok(Transition {
                    state: Box::new(StateStart),
                    transition_kind: TransitionKind::EmitToken(Token::new(
                        token_kind, lexeme, location,
                    )),
                })
            }
        }
    }
}

#[derive(Debug)]
pub struct StateEOF;

impl State for StateEOF {
    fn visit(&self, cursor: &mut Cursor) -> Result<Transition, LexerError> {
        cursor.align();
        Ok(Transition {
            state: Box::new(StateEnd),
            transition_kind: TransitionKind::EmitToken(Token::new(
                TokenKind::TokenEOF,
                "".to_string(),
                cursor.location().clone(),
            )),
        })
    }
}

#[derive(Debug)]
pub struct StateEnd;

impl State for StateEnd {
    fn visit(&self, _cursor: &mut Cursor) -> Result<Transition, LexerError> {
        Ok(Transition {
            state: Box::new(StateEnd),
            transition_kind: TransitionKind::End,
        })
    }
}
