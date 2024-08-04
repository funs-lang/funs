use std::fs;
use std::path::{Path, PathBuf};
use tracing::info;

#[derive(Clone)]
pub struct Source {
    file_path: PathBuf,
    content: String,
}

impl Source {
    pub fn new(file_path: impl AsRef<Path>) -> Source {
        let file_path = file_path.as_ref().to_path_buf();
        let content = fs::read_to_string(&file_path).unwrap_or_else(|e| {
            panic!("Error reading file \"{}\": {}", file_path.display(), e);
        });
        info!("Read file \"{}\"", file_path.display());
        Source { file_path, content }
    }

    pub fn file_path(&self) -> &Path {
        &self.file_path
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

pub struct SourceLocation {
    file_path: PathBuf,
    line: usize,
    column_start: usize,
    column_end: usize,
}

impl SourceLocation {
    pub fn new(file_path: &Path) -> SourceLocation {
        SourceLocation {
            file_path: file_path.to_path_buf(),
            line: 1,
            column_start: 1,
            column_end: 1,
        }
    }
}
