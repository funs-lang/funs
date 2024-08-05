use super::cursor::Cursor;
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
                Lexer::advance(Box::new(StateStart), TransitionKind::Consume)
            }
            Some(c) if c.is_alphabetic() || c.eq(&'_') => {
                Lexer::advance(Box::new(StateWord), TransitionKind::Advance)
            }
            Some(_) => todo!(),
            None => Lexer::advance(Box::new(StateEOF), TransitionKind::Consume),
        }
    }
}

#[derive(Debug)]
pub struct StateWord;

impl StateWord {
    fn string_to_token_kind(_string: String) -> TokenKind {
        TokenKind::TokenIdentifier
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::TokenKind;
    use crate::source::Source;
    use crate::utils::file_handler::{create_tmp_file, remove_tmp_file};
    use std::path::Path;

    #[test]
    fn test_token_identifier() {
        let file_path = "test.tmp";
        let file_content = "test_id";
        create_tmp_file(file_path, file_content);
        let source = Source::new(file_path);
        let lexer = Lexer::new(&source);
        let tokens = lexer.collect::<Vec<_>>();
        assert_eq!(tokens.len(), 2);
        assert_eq!(
            (&tokens[0].kind, &tokens[1].kind),
            (&TokenKind::TokenIdentifier, &TokenKind::TokenEOF)
        );
        assert_eq!(
            (&tokens[0].lexeme, &tokens[1].lexeme),
            (&"test_id".to_string(), &"".to_string())
        );
        assert_eq!(
            (&tokens[0].location, &tokens[1].location),
            (
                &TokenLocation::new(&Path::new(file_path))
                    .with_line(0)
                    .with_column_start(0)
                    .with_column_end(7),
                &TokenLocation::new(&Path::new(file_path))
                    .with_line(0)
                    .with_column_start(7)
                    .with_column_end(7)
            )
        );
        remove_tmp_file(file_path);
    }

    #[test]
    fn test_lexer_cursor_peek() {
        let source = Source::from("test_id".to_string());
        let cursor = Cursor::from(&source);
        assert_eq!(cursor.peek(), Some('t'));
    }

    #[test]
    fn test_lexer_cursor_consume() {
        let source = Source::from("test_id".to_string());
        let mut cursor = Cursor::from(&source);
        cursor.consume();
        assert_eq!(cursor.location.column_start(), 1);
        assert_eq!(cursor.location.column_end(), 1);
    }

    #[test]
    fn test_lexer_cursor_advance() {
        let source = Source::from("test_id".to_string());
        let mut cursor = Cursor::from(&source);
        cursor.advance();
        assert_eq!(cursor.location.column_start(), 0);
        assert_eq!(cursor.location.column_end(), 1);
    }

    #[test]
    fn test_lexer_cursor_align() {
        let source = Source::from("test_id".to_string());
        let mut cursor = Cursor::from(&source);
        cursor.advance();
        cursor.align();
        assert_eq!(cursor.location.column_start(), 1);
        assert_eq!(cursor.location.column_end(), 1);
    }

    #[test]
    fn test_lexer_transition_apply() {
        let source = Source::from("test_id".to_string());
        let mut cursor = Cursor::from(&source);
        let transition_kind = TransitionKind::Consume;
        transition_kind.apply(&mut cursor);
        assert_eq!(cursor.location.column_start(), 1);
        assert_eq!(cursor.location.column_end(), 1);
    }

    #[test]
    fn test_lexer_transition_apply_advance() {
        let source = Source::from("test_id".to_string());
        let mut cursor = Cursor::from(&source);
        let transition_kind = TransitionKind::Advance;
        transition_kind.apply(&mut cursor);
        assert_eq!(cursor.location.column_start(), 0);
        assert_eq!(cursor.location.column_end(), 1);
    }

    #[test]
    fn test_token_identifier_using_string() {
        let source = Source::from("test_id".to_string());
        let lexer = Lexer::new(&source);
        let tokens = lexer.collect::<Vec<_>>();
        assert_eq!(tokens.len(), 2);
        assert_eq!(
            (&tokens[0].kind, &tokens[1].kind),
            (&TokenKind::TokenIdentifier, &TokenKind::TokenEOF)
        );
        assert_eq!(
            (&tokens[0].lexeme, &tokens[1].lexeme),
            (&"test_id".to_string(), &"".to_string())
        );
        assert_eq!(
            (&tokens[0].location, &tokens[1].location),
            (
                &TokenLocation::new(&Path::new(""))
                    .with_line(0)
                    .with_column_start(0)
                    .with_column_end(7),
                &TokenLocation::new(&Path::new(""))
                    .with_line(0)
                    .with_column_start(7)
                    .with_column_end(7)
            )
        );
    }
}
