use crate::lexer::token::Token;
use crate::source::{Source, SourceLocation};

pub struct Cursor {
    source: Source,
    index: usize,
    location: SourceLocation,
}

impl Cursor {
    // TODO: Implement the cursor methods
}

impl From<&Source> for Cursor {
    fn from(source: &Source) -> Cursor {
        Cursor {
            source: source.clone(),
            location: SourceLocation::new(source.file_path()),
            index: 0,
        }
    }
}

pub struct Lexer {
    source: Cursor,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(source: &Source) -> Lexer {
        Lexer {
            source: Cursor::from(source),
            tokens: Vec::new(),
        }
    }
}
