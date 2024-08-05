use tracing::info;

use crate::lexer::token::Token;
use crate::lexer::token::{TokenKind, TokenLocation};
use crate::source::Source;
use std::fmt::Debug;

pub struct Cursor {
    source: Source,
    location: TokenLocation,
    index: usize,
}

impl Cursor {
    pub fn peek(&self) -> Option<char> {
        self.source.content().chars().nth(self.index)
    }

    pub fn source(&self) -> &Source {
        &self.source
    }

    pub fn location(&self) -> &TokenLocation {
        &self.location
    }

    pub fn is_eof(&self) -> bool {
        self.index >= self.source.content().len()
    }

    /// Consumes the current character and advances the cursor
    ///
    /// ```text
    /// Before consume:
    ///
    /// test
    /// ^_____ column_start
    /// ^_____ column_end
    ///
    /// After two consume:
    ///
    /// test
    ///   ^_____ column_start
    ///   ^_____ column_end
    /// ```
    pub fn consume(&mut self) {
        if self.is_eof() {
            return;
        }
        self.location.advance_column_start();
        self.location.advance_column_end();
        self.index += 1;
    }

    /// Advances the cursor without consuming the current character
    ///
    /// ```text
    /// Before advance:
    ///
    /// test
    /// ^_____ column_start
    /// ^_____ column_end
    ///
    /// After two advance:
    ///
    /// test
    /// ^_______ column_start
    ///   ^_____ column_end
    /// ```
    pub fn advance(&mut self) {
        if self.is_eof() {
            return;
        }

        self.location.advance_column_end();
        self.index += 1;
    }

    /// Aligns the column start with the column end
    ///
    /// ```text
    /// Before align:
    ///
    /// test
    /// ^_________ column_start
    ///     ^_____ column_end
    ///
    /// After align:
    ///
    /// test
    ///     ^_____ column_start
    ///     ^_____ column_end
    /// ```
    pub fn align(&mut self) {
        self.location.set_column_start(self.location.column_end());
    }
}

impl From<&Source> for Cursor {
    fn from(source: &Source) -> Cursor {
        Cursor {
            source: source.clone(),
            location: TokenLocation::new(source.file_path()),
            index: 0,
        }
    }
}

#[derive(Debug)]
pub enum TransitionKind {
    Consume,
    Advance,
    EmitToken(Token),
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
        }
    }
}

#[derive(Debug)]
pub struct Transition {
    state: Box<dyn State>,
    consume_kind: TransitionKind,
}

pub trait State: Debug {
    fn visit(&self, cursor: &mut Cursor) -> Transition;
}

#[derive(Debug)]
struct StateStart;

impl State for StateStart {
    fn visit(&self, cursor: &mut Cursor) -> Transition {
        match cursor.peek() {
            Some(c) if c.is_whitespace() => {
                Lexer::advance(Box::new(StateStart), TransitionKind::Consume)
            }
            Some(c) if c.is_alphabetic() || c.eq(&'_') => {
                Lexer::advance(Box::new(StateWord), TransitionKind::Advance)
            }
            _ => todo!(),
        }
    }
}

#[derive(Debug)]
struct StateWord;

impl StateWord {
    fn string_to_token_kind(string: String) -> TokenKind {
        match string {
            _ => TokenKind::TokenIdentifier,
        }
    }
}

impl State for StateWord {
    fn visit(&self, cursor: &mut Cursor) -> Transition {
        match cursor.peek() {
            Some(c) if c.is_alphabetic() || c.eq(&'_') => {
                Lexer::advance(Box::new(StateWord), TransitionKind::Advance)
            }
            _ => {
                // Emit token when we encounter a non-alphabetic character
                let lexeme = cursor.source().content()
                    [cursor.location().column_start()..cursor.location().column_end()]
                    .to_string();
                let token_kind = StateWord::string_to_token_kind(lexeme.clone());
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

pub struct Lexer {
    cursor: Cursor,
    state: Box<dyn State>,
}

impl Lexer {
    pub fn new(source: &Source) -> Lexer {
        let lexer = Lexer {
            cursor: Cursor::from(source),
            state: Box::new(StateStart),
        };
        info!("Created Lexer");
        lexer
    }

    pub fn advance(state: Box<dyn State>, consume_kind: TransitionKind) -> Transition {
        Transition {
            state,
            consume_kind,
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let state = &self.state;
            let transition = state.visit(&mut self.cursor);
            self.state = transition.state;
            if let TransitionKind::EmitToken(token) = transition.consume_kind {
                info!("Emitting token - {}", token);
                return Some(token);
            }
            transition.consume_kind.apply(&mut self.cursor);
        }
    }
}
