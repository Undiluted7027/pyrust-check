// Error types
use thiserror::Error;
use std::path::PathBuf;

#[derive(Error, Debug)]
pub enum PyRustError {
    #[error("Parse error at {location}: {message}")]
    ParseError {
        location: SourceLocation,
        message: String,
    },

    #[error("Type error at {location}: {message}")]
    TypeError {
        location: SourceLocation,
        message: String,
    },

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Undefined name '{name}' at {location}")]
    UndefinedName {
        name: String,
        location: SourceLocation,
    },
}

#[derive(Debug, Clone)]
pub struct SourceLocation {
    pub file: PathBuf,
    pub line: usize,
    pub column: usize,
}

impl std::fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.file.display(), self.line, self.column)
    }
}

pub type Result<T> = std::result::Result<T, PyRustError>;