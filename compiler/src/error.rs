use std::fmt;
use thiserror::Error;

/// Error type for the Abjad compiler
#[derive(Error, Debug)]
pub enum AbjadError {
    #[error("Lexical error: {0}")]
    Lexical(String),

    #[error("Syntax error: {0}")]
    Syntax(String),

    #[error("Type error: {0}")]
    Type(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

/// Result type alias for Abjad compiler operations
pub type Result<T> = std::result::Result<T, AbjadError>;

impl AbjadError {
    /// Create a new lexical error
    pub fn lexical(msg: impl Into<String>) -> Self {
        AbjadError::Lexical(msg.into())
    }

    /// Create a new syntax error
    pub fn syntax(msg: impl Into<String>) -> Self {
        AbjadError::Syntax(msg.into())
    }

    /// Create a new type error
    pub fn type_error(msg: impl Into<String>) -> Self {
        AbjadError::Type(msg.into())
    }

    /// Create a new internal error
    pub fn internal(msg: impl Into<String>) -> Self {
        AbjadError::Internal(msg.into())
    }
}
