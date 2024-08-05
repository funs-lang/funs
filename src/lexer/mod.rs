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

#[cfg(test)]
mod tests {
    use token::TokenLocation;

    use super::*;
    use crate::lexer::token::TokenKind;
    use crate::source::Source;
    use crate::utils::file_handler::{create_tmp_file, remove_tmp_file};
    use std::path::Path;

    #[test]
    fn test_lexer_tokenize_var_int_with_spaces() {
        let file_path = "test_var_int_with_spaces.tmp";
        let file_content = "    _x_int:   int  =  0   ";
        create_tmp_file(file_path, file_content);
        let source = Source::new(file_path);
        let lexer = Lexer::new(&source);
        let tokens = lexer.collect::<Vec<_>>();
        assert_eq!(tokens.len(), 6);
        assert_eq!(
            (
                &tokens[0].kind,
                &tokens[1].kind,
                &tokens[2].kind,
                &tokens[3].kind,
                &tokens[4].kind,
                &tokens[5].kind
            ),
            (
                &TokenKind::TokenIdentifier,
                &TokenKind::TokenColon,
                &TokenKind::TokenKeyword,
                &TokenKind::TokenAssign,
                &TokenKind::TokenLiteral(token::Literal::Int(0)),
                &TokenKind::TokenEOF
            )
        );
        assert_eq!(
            (
                &tokens[0].lexeme,
                &tokens[1].lexeme,
                &tokens[2].lexeme,
                &tokens[3].lexeme,
                &tokens[4].lexeme,
                &tokens[5].lexeme
            ),
            (
                &"_x_int".to_string(),
                &":".to_string(),
                &"int".to_string(),
                &"=".to_string(),
                &"0".to_string(),
                &"".to_string()
            )
        );
        assert_eq!(
            (
                &tokens[0].location,
                &tokens[1].location,
                &tokens[2].location,
                &tokens[3].location,
                &tokens[4].location,
                &tokens[5].location
            ),
            (
                &TokenLocation::new(&Path::new(file_path))
                    .with_line(0)
                    .with_column_start(4)
                    .with_column_end(10),
                &TokenLocation::new(&Path::new(file_path))
                    .with_line(0)
                    .with_column_start(10)
                    .with_column_end(11),
                &TokenLocation::new(&Path::new(file_path))
                    .with_line(0)
                    .with_column_start(14)
                    .with_column_end(17),
                &TokenLocation::new(&Path::new(file_path))
                    .with_line(0)
                    .with_column_start(19)
                    .with_column_end(20),
                &TokenLocation::new(&Path::new(file_path))
                    .with_line(0)
                    .with_column_start(22)
                    .with_column_end(23),
                &TokenLocation::new(&Path::new(file_path))
                    .with_line(0)
                    .with_column_start(26)
                    .with_column_end(26)
            )
        );
        remove_tmp_file(file_path);
    }

    #[test]
    fn test_lexer_tokenize_var_int() {
        let file_path = "test_var_int.tmp";
        let file_content = "_x_int: int = 0";
        create_tmp_file(file_path, file_content);
        let source = Source::new(file_path);
        let lexer = Lexer::new(&source);
        let tokens = lexer.collect::<Vec<_>>();
        assert_eq!(tokens.len(), 6);
        assert_eq!(
            (
                &tokens[0].kind,
                &tokens[1].kind,
                &tokens[2].kind,
                &tokens[3].kind,
                &tokens[4].kind,
                &tokens[5].kind
            ),
            (
                &TokenKind::TokenIdentifier,
                &TokenKind::TokenColon,
                &TokenKind::TokenKeyword,
                &TokenKind::TokenAssign,
                &TokenKind::TokenLiteral(token::Literal::Int(0)),
                &TokenKind::TokenEOF
            )
        );
        assert_eq!(
            (
                &tokens[0].lexeme,
                &tokens[1].lexeme,
                &tokens[2].lexeme,
                &tokens[3].lexeme,
                &tokens[4].lexeme,
                &tokens[5].lexeme
            ),
            (
                &"_x_int".to_string(),
                &":".to_string(),
                &"int".to_string(),
                &"=".to_string(),
                &"0".to_string(),
                &"".to_string()
            )
        );
        assert_eq!(
            (
                &tokens[0].location,
                &tokens[1].location,
                &tokens[2].location,
                &tokens[3].location,
                &tokens[4].location,
                &tokens[5].location
            ),
            (
                &TokenLocation::new(&Path::new(file_path))
                    .with_line(0)
                    .with_column_start(0)
                    .with_column_end(6),
                &TokenLocation::new(&Path::new(file_path))
                    .with_line(0)
                    .with_column_start(6)
                    .with_column_end(7),
                &TokenLocation::new(&Path::new(file_path))
                    .with_line(0)
                    .with_column_start(8)
                    .with_column_end(11),
                &TokenLocation::new(&Path::new(file_path))
                    .with_line(0)
                    .with_column_start(12)
                    .with_column_end(13),
                &TokenLocation::new(&Path::new(file_path))
                    .with_line(0)
                    .with_column_start(14)
                    .with_column_end(15),
                &TokenLocation::new(&Path::new(file_path))
                    .with_line(0)
                    .with_column_start(15)
                    .with_column_end(15)
            )
        );
        remove_tmp_file(file_path);
    }

    #[test]
    fn test_lexer_token_identifier_file() {
        let file_path = "test_token_identifier.tmp";
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
    fn test_lexer_space_before_identifier() {
        let file_path = "test_lexer_space_before_identifier.tmp";
        let file_content = "         __test_id";
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
            (&"__test_id".to_string(), &"".to_string())
        );
        assert_eq!(
            (&tokens[0].location, &tokens[1].location),
            (
                &TokenLocation::new(&Path::new(file_path))
                    .with_line(0)
                    .with_column_start(9)
                    .with_column_end(18),
                &TokenLocation::new(&Path::new(file_path))
                    .with_line(0)
                    .with_column_start(18)
                    .with_column_end(18)
            )
        );
        remove_tmp_file(file_path);
    }

    #[test]
    fn test_lexer_space_after_identifier() {
        let file_path = "test_lexer_space_after_identifier.tmp";
        let file_content = "__test_id         ";
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
            (&"__test_id".to_string(), &"".to_string())
        );
        assert_eq!(
            (&tokens[0].location, &tokens[1].location),
            (
                &TokenLocation::new(&Path::new(file_path))
                    .with_line(0)
                    .with_column_start(0)
                    .with_column_end(9),
                &TokenLocation::new(&Path::new(file_path))
                    .with_line(0)
                    .with_column_start(18)
                    .with_column_end(18)
            )
        );
        remove_tmp_file(file_path);
    }

    #[test]
    fn test_lexer_space_before_and_after_identifier() {
        let file_path = "test_lexer_space_before_and_after_identifier.tmp";
        let file_content = "         __test_id         ";
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
            (&"__test_id".to_string(), &"".to_string())
        );
        assert_eq!(
            (&tokens[0].location, &tokens[1].location),
            (
                &TokenLocation::new(&Path::new(file_path))
                    .with_line(0)
                    .with_column_start(9)
                    .with_column_end(18),
                &TokenLocation::new(&Path::new(file_path))
                    .with_line(0)
                    .with_column_start(27)
                    .with_column_end(27)
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
        assert_eq!(cursor.location().column_start(), 1);
        assert_eq!(cursor.location().column_end(), 1);
    }

    #[test]
    fn test_lexer_cursor_advance() {
        let source = Source::from("test_id".to_string());
        let mut cursor = Cursor::from(&source);
        cursor.advance();
        assert_eq!(cursor.location().column_start(), 0);
        assert_eq!(cursor.location().column_end(), 1);
    }

    #[test]
    fn test_lexer_cursor_align() {
        let source = Source::from("test_id".to_string());
        let mut cursor = Cursor::from(&source);
        cursor.advance();
        cursor.align();
        assert_eq!(cursor.location().column_start(), 1);
        assert_eq!(cursor.location().column_end(), 1);
    }

    #[test]
    fn test_lexer_transition_apply() {
        let source = Source::from("test_id".to_string());
        let mut cursor = Cursor::from(&source);
        let transition_kind = TransitionKind::Consume;
        transition_kind.apply(&mut cursor);
        assert_eq!(cursor.location().column_start(), 1);
        assert_eq!(cursor.location().column_end(), 1);
    }

    #[test]
    fn test_lexer_transition_apply_advance() {
        let source = Source::from("test_id".to_string());
        let mut cursor = Cursor::from(&source);
        let transition_kind = TransitionKind::Advance;
        transition_kind.apply(&mut cursor);
        assert_eq!(cursor.location().column_start(), 0);
        assert_eq!(cursor.location().column_end(), 1);
    }

    #[test]
    fn test_lexer_token_identifier_string() {
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
