use std::fmt;

/// Represents a token in the Abjad language
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    // Keywords
    Function,      // دالة
    Struct,        // هيكل
    Enum,          // تعداد
    Import,        // استورد
    Var,           // متغير
    Const,         // ثابت
    Return,        // أعد
    If,            // إذا
    Else,          // وإلا
    Match,         // مطابق
    For,           // حلقة
    While,         // بينما
    ForEach,       // لكل
    Break,         // كسر
    Continue,      // تابع
    And,           // و
    Or,            // أو
    Not,           // ليس
    True,          // صحيح
    False,         // خطأ
    Null,          // لا شيء
    Try,           // حاول
    Catch,         // التقط
    Throw,         // أطلق
    Impl,          // نفِّذ
    ForTrait,      // لـ
    In,            // في
    Async,         // غير_متزامن
    Await,         // انتظر
    Move,          // نقل
    Borrow,        // استعارة
    Mut,           // متغير
    Ref,           // مرجع
    Self_,         // ذات
    Trait,         // سمة
    Generic,       // عام
    Where,         // حيث

    // Types
    I8,    // صحيح٨
    I16,   // صحيح١٦
    I32,   // صحيح٣٢
    I64,   // صحيح٦٤
    U8,    // طبيعي٨
    U16,   // طبيعي١٦
    U32,   // طبيعي٣٢
    U64,   // طبيعي٦٤
    F32,   // عشري٣٢
    F64,   // عشري٦٤
    Bool,  // منطقي
    String,// نص
    Char,  // حرف

    // Literals
    Integer(i64),
    Float(f64),
    StringLiteral(String),
    CharLiteral(char),
    Boolean(bool),

    // Identifiers
    Identifier(String),

    // Operators
    Plus,          // +
    Minus,         // -
    Star,          // *
    Slash,         // /
    Percent,       // %
    Power,         // **
    Equal,         // =
    EqualEqual,    // ==
    NotEqual,      // !=
    Less,          // <
    Greater,       // >
    LessEqual,     // <=
    GreaterEqual,  // >=
    Ampersand,     // &
    Pipe,          // |
    Caret,         // ^
    Tilde,         // ~
    LeftShift,     // <<
    RightShift,    // >>
    PlusEqual,     // +=
    MinusEqual,    // -=
    StarEqual,     // *=
    SlashEqual,    // /=
    PercentEqual,  // %=
    Arrow,         // ->
    DoubleDot,     // ..
    TripleDot,     // ...

    // Delimiters
    LeftParen,     // (
    RightParen,    // )
    LeftBrace,     // {
    RightBrace,    // }
    LeftBracket,   // [
    RightBracket,  // ]
    Comma,         // ,
    Semicolon,     // ;
    Colon,         // :
    DoubleColon,   // ::
    Dot,           // .

    // Special
    Comment(String),
    Whitespace,
    Newline,
    EOF,
}

impl Token {
    /// Get the Arabic representation of the token
    pub fn arabic_name(&self) -> &'static str {
        match self {
            Token::Function => "دالة",
            Token::Struct => "هيكل",
            Token::Enum => "تعداد",
            Token::Import => "استورد",
            Token::Var => "متغير",
            Token::Const => "ثابت",
            Token::Return => "أعد",
            Token::If => "إذا",
            Token::Else => "وإلا",
            Token::Match => "مطابق",
            Token::For => "حلقة",
            Token::While => "بينما",
            Token::ForEach => "لكل",
            Token::Break => "كسر",
            Token::Continue => "تابع",
            Token::And => "و",
            Token::Or => "أو",
            Token::Not => "ليس",
            Token::True => "صحيح",
            Token::False => "خطأ",
            Token::Null => "لا_شيء",
            Token::Try => "حاول",
            Token::Catch => "التقط",
            Token::Throw => "أطلق",
            Token::Impl => "نفِّذ",
            Token::ForTrait => "لـ",
            Token::In => "في",
            Token::Async => "غير_متزامن",
            Token::Await => "انتظر",
            Token::Move => "نقل",
            Token::Borrow => "استعارة",
            Token::Mut => "متغير",
            Token::Ref => "مرجع",
            Token::Self_ => "ذات",
            Token::Trait => "سمة",
            Token::Generic => "عام",
            Token::Where => "حيث",
            Token::I8 => "صحيح٨",
            Token::I16 => "صحيح١٦",
            Token::I32 => "صحيح٣٢",
            Token::I64 => "صحيح٦٤",
            Token::U8 => "طبيعي٨",
            Token::U16 => "طبيعي١٦",
            Token::U32 => "طبيعي٣٢",
            Token::U64 => "طبيعي٦٤",
            Token::F32 => "عشري٣٢",
            Token::F64 => "عشري٦٤",
            Token::Bool => "منطقي",
            Token::String => "نص",
            Token::Char => "حرف",
            _ => "غير_معروف",
        }
    }

    /// Check if the token is a keyword
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Token::Function | Token::Struct | Token::Enum | Token::Import |
            Token::Var | Token::Const | Token::Return | Token::If |
            Token::Else | Token::Match | Token::For | Token::While |
            Token::ForEach | Token::Break | Token::Continue | Token::And |
            Token::Or | Token::Not | Token::True | Token::False |
            Token::Null | Token::Try | Token::Catch | Token::Throw |
            Token::Impl | Token::ForTrait | Token::In | Token::Async |
            Token::Await | Token::Move | Token::Borrow | Token::Mut |
            Token::Ref | Token::Self_ | Token::Trait | Token::Generic |
            Token::Where
        )
    }

    /// Check if the token is a type
    pub fn is_type(&self) -> bool {
        matches!(
            self,
            Token::I8 | Token::I16 | Token::I32 | Token::I64 |
            Token::U8 | Token::U16 | Token::U32 | Token::U64 |
            Token::F32 | Token::F64 | Token::Bool | Token::String |
            Token::Char
        )
    }

    /// Check if the token is an operator
    pub fn is_operator(&self) -> bool {
        matches!(
            self,
            Token::Plus | Token::Minus | Token::Star | Token::Slash |
            Token::Percent | Token::Power | Token::EqualEqual |
            Token::NotEqual | Token::Less | Token::Greater |
            Token::LessEqual | Token::GreaterEqual | Token::Ampersand |
            Token::Pipe | Token::Caret | Token::Tilde |
            Token::LeftShift | Token::RightShift | Token::PlusEqual |
            Token::MinusEqual | Token::StarEqual | Token::SlashEqual |
            Token::PercentEqual | Token::Arrow | Token::DoubleDot |
            Token::TripleDot
        )
    }

    /// Check if the token is a delimiter
    pub fn is_delimiter(&self) -> bool {
        matches!(
            self,
            Token::LeftParen | Token::RightParen | Token::LeftBrace |
            Token::RightBrace | Token::LeftBracket | Token::RightBracket |
            Token::Comma | Token::Semicolon | Token::Colon |
            Token::DoubleColon | Token::Dot
        )
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Integer(n) => write!(f, "{}", n),
            Token::Float(n) => write!(f, "{}", n),
            Token::StringLiteral(s) => write!(f, "\"{}\"", s),
            Token::CharLiteral(c) => write!(f, "'{}'", c),
            Token::Boolean(b) => write!(f, "{}", b),
            Token::Identifier(s) => write!(f, "{}", s),
            Token::Comment(s) => write!(f, "// {}", s),
            _ => write!(f, "{:?}", self),
        }
    }
}
