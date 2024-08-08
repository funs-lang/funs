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
    /// ^_____ index
    /// ^_____ offset
    /// ^_____ column_start
    /// ^_____ column_end
    ///
    /// After two consume:
    ///
    /// test
    ///   ^_____ index
    ///   ^_____ offset
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
    /// Before advance offset:
    ///
    /// test
    /// ^_____ index = 0
    /// ^_____ offset = 0
    /// ^_____ column_start = 0
    /// ^_____ column_end = 0
    ///
    /// After advance:
    ///
    /// test
    /// ^_____ index = 0
    ///  ^_____ offset = 1
    /// ^_______ column_start = 0
    ///  ^_____ column_end = 1
    /// ```
    pub fn advance_offset(&mut self) {
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
    /// ^_________ column_start = 0
    ///     ^_____ column_end = 3
    ///
    /// After align:
    ///
    /// test
    ///     ^_____ column_start = 3
    ///     ^_____ column_end = 3
    /// ```
    pub fn align(&mut self) {
        self.location.set_column_start(self.location.column_end());
        self.index = self.offset;
    }

    /// Advances only the cursor indexes
    pub fn remove_carriage_return(&mut self) {
        self.source.content_mut().remove(self.offset);
    }

    /// Advances the cursor to the next line
    /// ```text
    /// Before new line:
    /// test\ntest2
    ///     ^_____ index = 4
    ///     ^_____ offset = 4
    ///     ^_____ column_start = 4
    ///     ^_____ column_end = 4
    ///
    /// After new line:
    /// test\ntest2
    ///       ^_____ index = 5
    ///       ^_____ offset = 5
    ///       ^_____ column_start = 0
    ///       ^_____ column_end = 0
    pub fn new_line(&mut self) {
        if self.is_eof() {
            return;
        }

        self.location.advance_line();
        self.index = self.offset;
        self.offset += 1;
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

#[cfg(test)]
mod tests {
    use super::*;

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
        cursor.advance_offset();
        assert_eq!(cursor.location().column_start(), 0);
        assert_eq!(cursor.location().column_end(), 1);
    }

    #[test]
    fn test_lexer_cursor_align() {
        let source = Source::from("test_id".to_string());
        let mut cursor = Cursor::from(&source);
        cursor.advance_offset();
        cursor.align();
        assert_eq!(cursor.location().column_start(), 1);
        assert_eq!(cursor.location().column_end(), 1);
    }
}
