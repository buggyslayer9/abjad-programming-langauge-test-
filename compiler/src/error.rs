use std::fmt;
use thiserror::Error;

/// Warning type for the Abjad compiler
#[derive(Debug, Clone, Error)]
pub enum Warning {
    #[error("تحذير: {0}")]
    General(String),
    
    #[error("متغير غير مستخدم: {0}")]
    UnusedVariable(String),
    
    #[error("دالة غير مستخدمة: {0}")]
    UnusedFunction(String),
    
    #[error("تحويل ضمني من {0} إلى {1}")]
    ImplicitConversion(String, String),
    
    #[error("قيمة ميتة (dead code)")]
    DeadCode,
}

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

impl Warning {
    /// Create a new general warning
    pub fn general(msg: impl Into<String>) -> Self {
        Warning::General(msg.into())
    }

    /// Create an unused variable warning
    pub fn unused_variable(name: impl Into<String>) -> Self {
        Warning::UnusedVariable(name.into())
    }

    /// Create an unused function warning
    pub fn unused_function(name: impl Into<String>) -> Self {
        Warning::UnusedFunction(name.into())
    }

    /// Create an implicit conversion warning
    pub fn implicit_conversion(from: impl Into<String>, to: impl Into<String>) -> Self {
        Warning::ImplicitConversion(from.into(), to.into())
    }

    /// Get the Arabic warning message
    pub fn arabic_message(&self) -> String {
        match self {
            Warning::General(msg) => format!("تحذير: {}", msg),
            Warning::UnusedVariable(name) => format!("متغير غير مستخدم: {}", name),
            Warning::UnusedFunction(name) => format!("دالة غير مستخدمة: {}", name),
            Warning::ImplicitConversion(from, to) => format!("تحويل ضمني من {} إلى {}", from, to),
            Warning::DeadCode => "قيمة ميتة (dead code)".to_string(),
        }
    }
}
