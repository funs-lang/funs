pub mod cursor;
pub mod states;
pub mod token;

use crate::lexer::token::Token;
use crate::source::Source;
use cursor::Cursor;
use states::{State, StateStart, Transition, TransitionKind};
use tracing::{error, info};

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

    pub fn cursor(&self) -> &Cursor {
        &self.cursor
    }

    fn proceed(state: Box<dyn State>, transition_kind: TransitionKind) -> Transition {
        Transition::new(state, transition_kind)
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let transition = match self.state.visit(&mut self.cursor) {
                Ok(transition) => transition,
                Err(err) => {
                    error!("{}", err);
                    return None;
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
    LexerError,
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "LexerError")
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{
        lexer::{token::Token, Lexer},
        source::Source,
        utils::file_handler::collect_fs_files,
    };
    use pretty_assertions::assert_eq;
    use tracing::info;

    #[test]
    fn test_lexer_native_types() {
        let fs_files = collect_fs_files("./testdata/native_types", true);
        assert_eq!(fs_files.len(), 15);

        for path in fs_files {
            info!("file -> {:?}", path);
            eprintln!("file -> {:?}", path);
            let input = std::fs::File::open(path.clone()).unwrap();
            let content = std::io::read_to_string(input).unwrap();
            let source = Source::from(content);
            let mut lexer = Lexer::new(&source);
            let output_tokens = (&mut lexer).collect::<Vec<Token>>();

            let fs_file = path.to_str().unwrap();
            let tokens_file = fs_file.to_string().replace(".fs", ".tokens.json");
            let tokens = std::fs::File::open(tokens_file.clone()).unwrap();
            let expected_tokens: Vec<Token> = serde_json::from_reader(tokens).unwrap();
            // println!("{}", serde_json::to_string(&output_tokens).unwrap());
            assert_eq!(output_tokens, expected_tokens);
        }
    }

    #[test]
    fn test_lexer_functions() {
        let fs_files = collect_fs_files("./testdata/functions", true);
        assert_eq!(fs_files.len(), 9);

        for path in fs_files {
            info!("file -> {:?}", path);
            eprintln!("file -> {:?}", path);
            let input = std::fs::File::open(path.clone()).unwrap();
            let content = std::io::read_to_string(input).unwrap();
            let source = Source::from(content);
            let lexer = Lexer::new(&source);
            let output_tokens = lexer.collect::<Vec<Token>>();

            let fs_file = path.to_str().unwrap();
            let tokens_file = fs_file.to_string().replace(".fs", ".tokens.json");
            let tokens = std::fs::File::open(tokens_file).unwrap();
            let expected_tokens: Vec<Token> = serde_json::from_reader(tokens).unwrap();
            assert_eq!(output_tokens, expected_tokens);
        }
    }

    #[test]
    fn test_lexer_lists() {
        let fs_files = collect_fs_files("./testdata/lists", true);
        assert_eq!(fs_files.len(), 3);

        for path in fs_files {
            info!("file -> {:?}", path);
            eprintln!("file -> {:?}", path);
            let input = std::fs::File::open(path.clone()).unwrap();
            let content = std::io::read_to_string(input).unwrap();
            let source = Source::from(content);
            let lexer = Lexer::new(&source);
            let output_tokens = lexer.collect::<Vec<Token>>();

            let fs_file = path.to_str().unwrap();
            let tokens_file = fs_file.to_string().replace(".fs", ".tokens.json");
            let tokens = std::fs::File::open(tokens_file).unwrap();
            let expected_tokens: Vec<Token> = serde_json::from_reader(tokens).unwrap();
            assert_eq!(output_tokens, expected_tokens);
        }
    }

    #[test]
    fn test_lexer_tuples() {
        let fs_files = collect_fs_files("./testdata/tuples", true);
        assert_eq!(fs_files.len(), 3);

        for path in fs_files {
            info!("file -> {:?}", path);
            eprintln!("file -> {:?}", path);
            let input = std::fs::File::open(path.clone()).unwrap();
            let content = std::io::read_to_string(input).unwrap();
            let source = Source::from(content);
            let lexer = Lexer::new(&source);
            let output_tokens = lexer.collect::<Vec<Token>>();

            let fs_file = path.to_str().unwrap();
            let tokens_file = fs_file.to_string().replace(".fs", ".tokens.json");
            let tokens = std::fs::File::open(tokens_file).unwrap();
            let expected_tokens: Vec<Token> = serde_json::from_reader(tokens).unwrap();
            assert_eq!(output_tokens, expected_tokens);
        }
    }

    #[test]
    fn test_lexer_records() {
        let fs_files = collect_fs_files("./testdata/records", true);
        assert_eq!(fs_files.len(), 3);

        for path in fs_files {
            info!("file -> {:?}", path);
            eprintln!("file -> {:?}", path);
            let input = std::fs::File::open(path.clone()).unwrap();
            let content = std::io::read_to_string(input).unwrap();
            let source = Source::from(content);
            let lexer = Lexer::new(&source);
            let output_tokens = lexer.collect::<Vec<Token>>();

            let fs_file = path.to_str().unwrap();
            let tokens_file = fs_file.to_string().replace(".fs", ".tokens.json");
            let tokens = std::fs::File::open(tokens_file).unwrap();
            let expected_tokens: Vec<Token> = serde_json::from_reader(tokens).unwrap();
            assert_eq!(output_tokens, expected_tokens);
        }
    }

    #[test]
    fn test_lexer_variants() {
        let fs_files = collect_fs_files("./testdata/variants", true);
        assert_eq!(fs_files.len(), 1);

        for path in fs_files {
            info!("file -> {:?}", path);
            eprintln!("file -> {:?}", path);
            let input = std::fs::File::open(path.clone()).unwrap();
            let content = std::io::read_to_string(input).unwrap();
            let source = Source::from(content);
            let lexer = Lexer::new(&source);
            let output_tokens = lexer.collect::<Vec<Token>>();

            let fs_file = path.to_str().unwrap();
            let tokens_file = fs_file.to_string().replace(".fs", ".tokens.json");
            let tokens = std::fs::File::open(tokens_file).unwrap();
            let expected_tokens: Vec<Token> = serde_json::from_reader(tokens).unwrap();
            assert_eq!(output_tokens, expected_tokens);
        }
    }

    #[test]
    fn test_lexer_errors() {
        let fs_files = collect_fs_files("./testdata/errors", true);
        assert_eq!(fs_files.len(), 2);

        for path in fs_files {
            info!("file -> {:?}", path);
            eprintln!("file -> {:?}", path);
            let input = std::fs::File::open(path.clone()).unwrap();
            let content = std::io::read_to_string(input).unwrap();
            let source = Source::from(content);
            let lexer = Lexer::new(&source);
            let output_tokens = lexer.collect::<Vec<Token>>();

            let fs_file = path.to_str().unwrap();
            let tokens_file = fs_file.to_string().replace(".fs", ".tokens.json");
            let tokens = std::fs::File::open(tokens_file).unwrap();
            let expected_tokens: Vec<Token> = serde_json::from_reader(tokens).unwrap();
            assert_eq!(output_tokens, expected_tokens);
        }
    }
}
