pub mod cursor;
pub mod states;
pub mod token;

use crate::source::Source;
use crate::{lexer::token::Token, utils::color};
use cursor::Cursor;
use states::{State, StateStart, Transition, TransitionKind};
use tracing::{error, info};

pub struct Lexer {
    cursor: Cursor,
    state: Box<dyn State>,
    errors: Vec<LexerError>,
}

impl Lexer {
    pub fn new(source: &Source) -> Lexer {
        let lexer = Lexer {
            cursor: Cursor::from(source),
            state: Box::new(StateStart),
            errors: Vec::new(),
        };
        info!("Created Lexer");
        lexer
    }

    pub fn cursor(&self) -> &Cursor {
        &self.cursor
    }

    fn proceed(state: Box<dyn State>, transition_kind: TransitionKind) -> Transition {
        Transition::new(state, transition_kind)
    }

    pub fn errors(&self) -> &Vec<LexerError> {
        &self.errors
    }

    pub fn emit_errors(&self) {
        if self.errors.is_empty() {
            return;
        }

        eprintln!("{}", color::red("Errors:"));
        for error in &self.errors {
            eprintln!("  {}", error);
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let transition = match self.state.visit(&mut self.cursor) {
                Ok(transition) => transition,
                Err(err) => {
                    self.errors.push(err.clone());
                    return match err {
                        LexerError::UnexpectedToken(token) => {
                            error!("Unexpected token: {:?}", token);
                            // TODO: return a transition to continue lexing (for error recovery)
                            None
                        }
                    };
                }
            };
            let (state, transition_kind) = transition.into_parts();

            self.state = state;
            transition_kind.apply(&mut self.cursor);
            if let TransitionKind::EmitToken(token) = transition_kind {
                info!("Emitting token - {:?}", token);
                return Some(token.clone());
            }
            if let TransitionKind::End = transition_kind {
                return None;
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum LexerError {
    UnexpectedToken(Token),
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LexerError::UnexpectedToken(token) => write!(f, "Unexpected token: {}", token),
        }
    }
}
