use crate::source::Source;

use super::token::TokenLocation;

pub struct Cursor {
    source: Source,
    location: TokenLocation,
    index: usize,
    offset: usize,
}

impl Cursor {
    pub fn peek(&self) -> Option<char> {
        if self.is_eof() {
            return None;
        }
        self.source.content().chars().nth(self.offset)
    }

    pub fn source(&self) -> &Source {
        &self.source
    }

    pub fn location(&self) -> &TokenLocation {
        &self.location
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn offset(&self) -> usize {
        self.offset
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
        self.offset += 1;
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
        self.offset += 1;
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
        self.index = self.offset;
    }

    pub fn new_line(&mut self) {
        self.location.advance_line();
    }
}

impl From<&Source> for Cursor {
    fn from(source: &Source) -> Cursor {
        Cursor {
            source: source.clone(),
            location: TokenLocation::from(source.file_path()),
            index: 0,
            offset: 0,
        }
    }
}
