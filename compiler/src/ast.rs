use crate::token::Token;
use std::fmt;

/// Represents a position in the source code
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        Position { line, column }
    }
}

/// Represents a span in the source code
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Span {
    pub fn new(start: Position, end: Position) -> Self {
        Span { start, end }
    }
}

/// Represents a type in the AST
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    /// Primitive types
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Bool,
    String,
    Char,
    
    /// Array type
    Array(Box<Type>, Option<usize>),
    
    /// Slice type
    Slice(Box<Type>),
    
    /// Tuple type
    Tuple(Vec<Type>),
    
    /// Function type
    Function(Vec<Type>, Box<Type>),
    
    /// User-defined type
    Named(String),
    
    /// Generic type
    Generic(String),
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::I8 => write!(f, "صحيح٨"),
            Type::I16 => write!(f, "صحيح١٦"),
            Type::I32 => write!(f, "صحيح٣٢"),
            Type::I64 => write!(f, "صحيح٦٤"),
            Type::U8 => write!(f, "طبيعي٨"),
            Type::U16 => write!(f, "طبيعي١٦"),
            Type::U32 => write!(f, "طبيعي٣٢"),
            Type::U64 => write!(f, "طبيعي٦٤"),
            Type::F32 => write!(f, "عشري٣٢"),
            Type::F64 => write!(f, "عشري٦٤"),
            Type::Bool => write!(f, "منطقي"),
            Type::String => write!(f, "نص"),
            Type::Char => write!(f, "حرف"),
            Type::Array(ty, size) => match size {
                Some(s) => write!(f, "[{}; {}]", ty, s),
                None => write!(f, "[{}]", ty),
            },
            Type::Slice(ty) => write!(f, "&[{}]", ty),
            Type::Tuple(types) => {
                write!(f, "(")?;
                for (i, t) in types.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", t)?;
                }
                write!(f, ")")
            }
            Type::Function(params, ret) => {
                write!(f, "fn(")?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, ") -> {}", ret)
            }
            Type::Named(name) => write!(f, "{}", name),
            Type::Generic(name) => write!(f, "{}", name),
        }
    }
}

/// Represents a binary operator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Power,
    
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    
    And,
    Or,
    
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOperator::Add => write!(f, "+"),
            BinaryOperator::Sub => write!(f, "-"),
            BinaryOperator::Mul => write!(f, "*"),
            BinaryOperator::Div => write!(f, "/"),
            BinaryOperator::Mod => write!(f, "%"),
            BinaryOperator::Power => write!(f, "**"),
            BinaryOperator::Equal => write!(f, "=="),
            BinaryOperator::NotEqual => write!(f, "!="),
            BinaryOperator::Less => write!(f, "<"),
            BinaryOperator::LessEqual => write!(f, "<="),
            BinaryOperator::Greater => write!(f, ">"),
            BinaryOperator::GreaterEqual => write!(f, ">="),
            BinaryOperator::And => write!(f, "و"),
            BinaryOperator::Or => write!(f, "أو"),
            BinaryOperator::BitwiseAnd => write!(f, "&"),
            BinaryOperator::BitwiseOr => write!(f, "|"),
            BinaryOperator::BitwiseXor => write!(f, "^"),
            BinaryOperator::LeftShift => write!(f, "<<"),
            BinaryOperator::RightShift => write!(f, ">>"),
        }
    }
}

/// Represents a unary operator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    Negate,
    Not,
    BitwiseNot,
    Dereference,
    Reference,
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOperator::Negate => write!(f, "-"),
            UnaryOperator::Not => write!(f, "ليس"),
            UnaryOperator::BitwiseNot => write!(f, "~"),
            UnaryOperator::Dereference => write!(f, "*"),
            UnaryOperator::Reference => write!(f, "&"),
        }
    }
}

/// Represents an expression in the AST
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// Literal value
    Literal(Literal),
    
    /// Variable reference
    Identifier(String),
    
    /// Binary operation
    Binary {
        op: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    
    /// Unary operation
    Unary {
        op: UnaryOperator,
        operand: Box<Expression>,
    },
    
    /// Function call
    Call {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
    
    /// Array literal
    ArrayLiteral(Vec<Expression>),
    
    /// Array indexing
    Index {
        array: Box<Expression>,
        index: Box<Expression>,
    },
    
    /// Tuple literal
    TupleLiteral(Vec<Expression>),
    
    /// Block expression
    Block(Vec<Statement>),
    
    /// If expression
    If {
        condition: Box<Expression>,
        then_branch: Box<Expression>,
        else_branch: Option<Box<Expression>>,
    },
    
    /// Match expression
    Match {
        value: Box<Expression>,
        arms: Vec<MatchArm>,
    },
    
    /// Loop expression
    Loop {
        body: Box<Expression>,
    },
    
    /// While loop
    While {
        condition: Box<Expression>,
        body: Box<Expression>,
    },
    
    /// For loop
    For {
        pattern: Pattern,
        iterable: Box<Expression>,
        body: Box<Expression>,
    },
    
    /// Break expression
    Break(Option<Box<Expression>>),
    
    /// Continue expression
    Continue,
    
    /// Return expression
    Return(Option<Box<Expression>>),
    
    /// Type cast
    Cast {
        value: Box<Expression>,
        target_type: Type,
    },
    
    /// Parenthesized expression
    Parenthesized(Box<Expression>),
}

/// Represents a literal value
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    String(String),
    Char(char),
    Boolean(bool),
    Null,
}

/// Represents a pattern in match expressions
#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    /// Wildcard pattern
    Wildcard,
    
    /// Literal pattern
    Literal(Literal),
    
    /// Identifier pattern
    Identifier(String),
    
    /// Tuple pattern
    Tuple(Vec<Pattern>),
    
    /// Struct pattern
    Struct {
        name: String,
        fields: Vec<(String, Pattern)>,
    },
    
    /// Or pattern
    Or(Box<Pattern>, Box<Pattern>),
}

/// Represents a match arm
#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<Expression>,
    pub body: Expression,
}

/// Represents a statement in the AST
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    /// Expression statement
    Expression(Expression),
    
    /// Variable declaration
    Let {
        name: String,
        type_annotation: Option<Type>,
        value: Expression,
        mutable: bool,
    },
    
    /// Assignment
    Assignment {
        target: Expression,
        value: Expression,
    },
    
    /// Function declaration
    Function(FunctionDeclaration),
    
    /// Struct declaration
    Struct(StructDeclaration),
    
    /// Enum declaration
    Enum(EnumDeclaration),
    
    /// Import statement
    Import(String),
    
    /// Empty statement
    Empty,
}

/// Represents a function declaration
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Type>,
    pub body: Vec<Statement>,
}

/// Represents a function parameter
#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub type_annotation: Type,
    pub mutable: bool,
}

/// Represents a struct declaration
#[derive(Debug, Clone, PartialEq)]
pub struct StructDeclaration {
    pub name: String,
    pub fields: Vec<Field>,
}

/// Represents a struct field
#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub name: String,
    pub type_annotation: Type,
    pub mutable: bool,
}

/// Represents an enum declaration
#[derive(Debug, Clone, PartialEq)]
pub struct EnumDeclaration {
    pub name: String,
    pub variants: Vec<EnumVariant>,
}

/// Represents an enum variant
#[derive(Debug, Clone, PartialEq)]
pub struct EnumVariant {
    pub name: String,
    pub fields: Vec<Type>,
}

/// Represents the entire AST
#[derive(Debug, Clone, PartialEq)]
pub struct AST {
    pub statements: Vec<Statement>,
}

impl AST {
    pub fn new(statements: Vec<Statement>) -> Self {
        AST { statements }
    }
}

impl fmt::Display for AST {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for stmt in &self.statements {
            writeln!(f, "{}", stmt)?;
        }
        Ok(())
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Expression(expr) => write!(f, "{};", expr),
            Statement::Let { name, type_annotation, value, mutable } => {
                if *mutable {
                    write!(f, "متغير")?;
                } else {
                    write!(f, "ثابت")?;
                }
                write!(f, " {}", name)?;
                if let Some(ty) = type_annotation {
                    write!(f, ": {}", ty)?;
                }
                write!(f, " = {};", value)
            }
            Statement::Assignment { target, value } => {
                write!(f, "{} = {};", target, value)
            }
            Statement::Function(func) => {
                write!(f, "دالة {}(", func.name)?;
                for (i, param) in func.parameters.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", param.name, param.type_annotation)?;
                }
                write!(f, ")")?;
                if let Some(ret) = &func.return_type {
                    write!(f, " -> {}", ret)?;
                }
                write!(f, " {{ /* body */ }}")
            }
            Statement::Struct(struct_) => {
                write!(f, "هيكل {} {{", struct_.name)?;
                for (i, field) in struct_.fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", field.name, field.type_annotation)?;
                }
                write!(f, " }}")
            }
            Statement::Enum(enum_) => {
                write!(f, "تعداد {} {{", enum_.name)?;
                for (i, variant) in enum_.variants.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", variant.name)?;
                }
                write!(f, " }}")
            }
            Statement::Import(path) => write!(f, "استورد \"{}\";", path),
            Statement::Empty => write!(f, ";"),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Literal(lit) => write!(f, "{}", lit),
            Expression::Identifier(name) => write!(f, "{}", name),
            Expression::Binary { op, left, right } => {
                write!(f, "{} {} {}", left, op, right)
            }
            Expression::Unary { op, operand } => {
                write!(f, "{}{}", op, operand)
            }
            Expression::Call { function, arguments } => {
                write!(f, "{}(", function)?;
                for (i, arg) in arguments.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
            Expression::ArrayLiteral(elements) => {
                write!(f, "[")?;
                for (i, elem) in elements.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", elem)?;
                }
                write!(f, "]")
            }
            Expression::Index { array, index } => {
                write!(f, "{}[{}]", array, index)
            }
            Expression::TupleLiteral(elements) => {
                write!(f, "(")?;
                for (i, elem) in elements.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", elem)?;
                }
                write!(f, ")")
            }
            Expression::Block(_) => write!(f, "{{ /* block */ }}"),
            Expression::If { condition, then_branch, else_branch } => {
                write!(f, "إذا {} {} ", condition, then_branch)?;
                if let Some(else_br) = else_branch {
                    write!(f, "وإلا {}", else_br)?;
                }
                Ok(())
            }
            Expression::Match { value, arms } => {
                write!(f, "مطابق {} {{", value)?;
                for arm in arms {
                    write!(f, "{:?} => {}, ", arm.pattern, arm.body)?;
                }
                write!(f, " }}")
            }
            Expression::Loop { body } => write!(f, "حلقة {{ {} }}", body),
            Expression::While { condition, body } => {
                write!(f, "بينما {} {{ {} }}", condition, body)
            }
            Expression::For { pattern, iterable, body } => {
                write!(f, "لكل {:?} في {} {{ {} }}", pattern, iterable, body)
            }
            Expression::Break(value) => {
                write!(f, "كسر")?;
                if let Some(v) = value {
                    write!(f, " {}", v)?;
                }
                Ok(())
            }
            Expression::Continue => write!(f, "تابع"),
            Expression::Return(value) => {
                write!(f, "أعد")?;
                if let Some(v) = value {
                    write!(f, " {}", v)?;
                }
                Ok(())
            }
            Expression::Cast { value, target_type } => {
                write!(f, "{} كـ {}", value, target_type)
            }
            Expression::Parenthesized(expr) => write!(f, "({})", expr),
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Integer(n) => write!(f, "{}", n),
            Literal::Float(n) => write!(f, "{}", n),
            Literal::String(s) => write!(f, "\"{}\"", s),
            Literal::Char(c) => write!(f, "'{}'", c),
            Literal::Boolean(b) => write!(f, "{}", if *b { "صحيح" } else { "خطأ" }),
            Literal::Null => write!(f, "لا_شيء"),
        }
    }
}
