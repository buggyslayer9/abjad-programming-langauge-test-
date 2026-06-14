use std::fmt;
use thiserror::Error;

/// Error type for the Abjad compiler
#[derive(Error, Debug)]
pub enum AbjadError {
    #[error("خطأ معجمي: {0}")]
    Lexical(String),

    #[error("خطأ نحوي: {0}")]
    Syntax(String),

    #[error("خطأ في الأنواع: {0}")]
    Type(String),

    #[error("خطأ في الإدخال/الإخراج: {0}")]
    Io(#[from] std::io::Error),

    #[error("الملف غير موجود: {0}")]
    FileNotFound(String),

    #[error("خطأ داخلي: {0}")]
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

    /// Get the Arabic error message
    pub fn arabic_message(&self) -> &str {
        match self {
            AbjadError::Lexical(msg) => msg,
            AbjadError::Syntax(msg) => msg,
            AbjadError::Type(msg) => msg,
            AbjadError::Io(_) => "خطأ في قراءة الملف",
            AbjadError::FileNotFound(_) => "الملف غير موجود",
            AbjadError::Internal(msg) => msg,
        }
    }
}
