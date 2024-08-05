pub mod cursor;
pub mod states;
pub mod token;

use crate::lexer::token::Token;
use crate::source::Source;
use cursor::Cursor;
use states::{State, StateStart, Transition, TransitionKind};
use tracing::info;

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
        Transition::new(state, consume_kind)
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let transition = self.state.visit(&mut self.cursor);
            self.state = transition.state;
            transition.consume_kind.apply(&mut self.cursor);
            if let TransitionKind::EmitToken(token) = transition.consume_kind {
                info!("Emitting token - {}", token);
                return Some(token);
            }
            if let TransitionKind::End = transition.consume_kind {
                return None;
            }
        }
    }
}
