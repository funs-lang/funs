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
        info!("Created Source from file \"{}\"", file_path.display());
        Source { file_path, content }
    }

    pub fn file_path(&self) -> &Path {
        &self.file_path
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}
