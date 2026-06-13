---
description: Improves code quality, maintainability, and best practices for the Abjad compiler
---

# Opencode Skill

## Purpose

This skill improves code quality, maintainability, and best practices for the Abjad compiler. It ensures that code is clean, well-documented, and follows industry standards.

## When to Use

Use this skill when:
- Writing new code
- Reviewing existing code
- Refactoring
- Adding features
- Fixing bugs
- Writing documentation

## Core Principles

### 1. Code Should Be Self-Documenting
- Use meaningful names
- Avoid abbreviations
- Express intent clearly
- Reduce cognitive load

### 2. DRY (Don't Repeat Yourself)
- Extract common patterns
- Use functions and modules
- Avoid copy-paste
- Keep code DRY

### 3. KISS (Keep It Simple, Stupid)
- Prefer simple solutions
- Avoid over-engineering
- Solve the actual problem
- Don't add complexity

### 4. YAGNI (You Aren't Gonna Need It)
- Don't build for hypothetical futures
- Implement what's needed now
- Refactor when patterns emerge
- Avoid premature abstraction

## Naming Conventions

### Variables and Functions
```rust
// Good: Clear, descriptive names
let token_count = tokens.len();
fn parse_expression(&mut self) -> Result<Expression> {
    // ...
}

// Bad: Abbreviated, unclear names
let tc = tokens.len();
fn parse_expr(&mut self) -> Result<Expr> {
    // ...
}
```

### Types and Structs
```rust
// Good: PascalCase for types
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

pub enum TokenKind {
    Function,
    Identifier(String),
    // ...
}

// Bad: snake_case for types
pub struct token {
    pub kind: token_kind,
}
```

### Constants
```rust
// Good: SCREAMING_SNAKE_CASE for constants
pub const MAX_TOKEN_LENGTH: usize = 1024;
pub const DEFAULT_OPT_LEVEL: OptLevel = OptLevel::O0;

// Bad: camelCase for constants
pub const maxTokenLength: usize = 1024;
```

### Arabic Names (for user-facing strings)
```rust
// Good: Arabic for UI strings
const ERROR_MESSAGE: &str = "خطأ في التحليل المعجمي";

// Bad: English for Arabic UI
const ERROR_MESSAGE: &str = "Lexical error";
```

## Code Organization

### Module Structure
```rust
// Good: Logical module organization
pub mod lexer;
pub mod parser;
pub mod ast;
pub mod type_checker;
pub mod codegen;

// Each module has clear responsibility
// lexer.rs: Tokenization
// parser.rs: Parsing
// ast.rs: AST definitions
// type_checker.rs: Type checking
// codegen.rs: Code generation
```

### File Organization
```
src/
├── lib.rs              // Library root
├── main.rs             // CLI entry point
├── cli.rs              // CLI commands
├── error.rs            // Error types
├── token.rs            // Token definitions
├── lexer.rs            // Lexer implementation
├── parser.rs           // Parser implementation
├── ast.rs              // AST node definitions
├── type_checker.rs     // Type checker
└── codegen.rs          // Code generation
```

### Function Length
```rust
// Good: Short, focused functions
fn parse_identifier(&mut self) -> Result<String> {
    let start = self.position;
    while let Some(&c) = self.peek() {
        if c.is_alphanumeric() || c == '_' {
            self.next();
        } else {
            break;
        }
    }
    Ok(self.source[start..self.position].to_string())
}

// Bad: Long, complex functions
fn parse_complex_thing(&mut self) -> Result<ComplexThing> {
    // 100+ lines of mixed logic
}
```

## Error Handling

### Use Result Types
```rust
// Good: Explicit error handling
pub fn parse(&mut self) -> Result<AST> {
    let tokens = self.lexer.tokenize()?;
    let ast = self.build_ast(&tokens)?;
    Ok(ast)
}

// Bad: Panic on errors
pub fn parse(&mut self) -> AST {
    let tokens = self.lexer.tokenize().unwrap();
    self.build_ast(&tokens).unwrap()
}
```

### Custom Error Types
```rust
// Good: Structured errors with context
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unexpected token: expected {expected}, found {found}")]
    UnexpectedToken { expected: String, found: Token },
    
    #[error("Unterminated string at position {position}")]
    UnterminatedString { position: Position },
    
    #[error("Invalid character '{char}' at position {position}")]
    InvalidCharacter { char: char, position: Position },
}

// Bad: Generic error messages
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Parse error")]
    ParseError,
}
```

### Error Propagation
```rust
// Good: Use ? operator for clean error propagation
pub fn compile(source: &str) -> Result<Assembly> {
    let tokens = tokenize(source)?;
    let ast = parse(&tokens)?;
    let typed = type_check(&ast)?;
    let ir = generate_ir(&typed)?;
    let assembly = codegen(&ir)?;
    Ok(assembly)
}

// Bad: Manual error handling
pub fn compile(source: &str) -> Result<Assembly> {
    let tokens = match tokenize(source) {
        Ok(t) => t,
        Err(e) => return Err(e),
    };
    // ...
}
```

## Documentation

### Function Documentation
```rust
/// Tokenizes the input source code into a stream of tokens.
///
/// This function processes the input string character by character,
/// identifying keywords, identifiers, literals, and operators.
///
/// # Arguments
///
/// * `source` - The source code to tokenize
///
/// # Returns
///
/// A `Result` containing either a vector of tokens or a `LexicalError`
///
/// # Errors
///
/// Returns an error if:
/// - An invalid character is encountered
/// - A string literal is not properly terminated
/// - An invalid escape sequence is found
///
/// # Examples
///
/// ```
/// use abjad::lexer::Lexer;
///
/// let mut lexer = Lexer::new("دالة رئيسي() {}");
/// let tokens = lexer.tokenize()?;
/// assert_eq!(tokens[0], Token::Function);
/// ```
///
/// # Performance
///
/// This function runs in O(n) time where n is the length of the input.
pub fn tokenize(source: &str) -> Result<Vec<Token>, LexicalError> {
    // Implementation
}
```

### Struct Documentation
```rust
/// Represents a single token in the Abjad language.
///
/// Each token has a kind (e.g., keyword, identifier, literal) and
/// a span indicating its position in the source code.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    /// The kind of token (keyword, identifier, literal, etc.)
    pub kind: TokenKind,
    
    /// The span (start and end positions) in the source code
    pub span: Span,
}
```

### Module Documentation
```rust
//! Lexical analysis for the Abjad programming language.
//!
//! This module provides the lexer which converts source code into
//! a stream of tokens. The lexer handles:
//!
//! - Arabic keywords and identifiers
//! - Numeric literals (including Arabic numerals)
//! - String and character literals
//! - Operators and delimiters
//! - Comments (single-line and multi-line)
//!
//! # Example
//!
//! ```
//! use abjad::lexer::Lexer;
//!
//! let mut lexer = Lexer::new("دالة جمع(أ، ب) { أعد أ + ب }");
//! let tokens = lexer.tokenize()?;
//! ```

//! # Error Handling
//!
//! The lexer returns detailed error messages including position
//! information to help with debugging.
```

## Code Style

### Formatting
```rust
// Good: Consistent formatting
pub fn parse_expression(&mut self) -> Result<Expression> {
    let left = self.parse_term()?;
    
    while let Some(op) = self.peek_operator() {
        self.next();
        let right = self.parse_term()?;
        let left = Expression::Binary {
            op,
            left: Box::new(left),
            right: Box::new(right),
        };
    }
    
    Ok(left)
}

// Bad: Inconsistent formatting
pub fn parse_expression(&mut self) -> Result<Expression> {
    let left=self.parse_term()?;
    while let Some(op)=self.peek_operator(){
        self.next();
        let right=self.parse_term()?;
        let left=Expression::Binary{op,left:Box::new(left),right:Box::new(right)};
    }
    Ok(left)
}
```

### Line Length
```rust
// Good: Keep lines under 100 characters
pub fn parse_function_declaration(&mut self) -> Result<FunctionDeclaration> {
    // ...
}

// Bad: Very long lines
pub fn parse_function_declaration_with_very_long_name(&mut self) -> Result<FunctionDeclaration> { // ... }
```

### Imports
```rust
// Good: Organized imports
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::ast::{Expression, Statement};
use crate::error::{Error, Result};

// Bad: Disorganized imports
use std::collections::HashMap;
use crate::ast::Expression;
use std::fs::File;
use crate::error::Error;
use std::io::BufRead;
use crate::ast::Statement;
```

## Testing

### Test Naming
```rust
// Good: Descriptive test names
#[test]
fn test_arabic_keyword_parsing() {
    // ...
}

#[test]
fn test_string_literal_with_escape_sequences() {
    // ...
}

// Bad: Vague test names
#[test]
fn test_1() {
    // ...
}

#[test]
fn test_stuff() {
    // ...
}
```

### Test Structure
```rust
// Good: Arrange, Act, Assert pattern
#[test]
fn test_arabic_keyword_parsing() {
    // Arrange
    let source = "دالة";
    let mut lexer = Lexer::new(source);
    
    // Act
    let token = lexer.next_token().unwrap();
    
    // Assert
    assert_eq!(token.kind, TokenKind::Function);
}

// Bad: Mixed structure
#[test]
fn test_arabic_keyword_parsing() {
    let mut lexer = Lexer::new("دالة");
    assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Function);
}
```

### Test Coverage
```rust
// Good: Test edge cases
#[test]
fn test_empty_string() {
    let result = Lexer::new("").tokenize();
    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}

#[test]
fn test_unterminated_string() {
    let result = Lexer::new("\"unterminated").tokenize();
    assert!(result.is_err());
}

// Bad: Only happy path tests
#[test]
fn test_simple_string() {
    let result = Lexer::new("\"hello\"").tokenize();
    assert!(result.is_ok());
}
```

## Performance

### Avoid Unnecessary Allocations
```rust
// Good: Reuse buffers
fn process_tokens(tokens: &[Token]) -> Vec<Token> {
    tokens.iter().filter(|t| !t.is_whitespace()).cloned().collect()
}

// Bad: Create unnecessary intermediate allocations
fn process_tokens(tokens: &[Token]) -> Vec<Token> {
    let filtered: Vec<_> = tokens.iter().filter(|t| !t.is_whitespace()).collect();
    let mapped: Vec<_> = filtered.iter().map(|t| t.clone()).collect();
    mapped
}
```

### Use Iterators
```rust
// Good: Use iterators for lazy evaluation
let sum: i32 = numbers.iter().sum();

// Bad: Collect unnecessarily
let numbers_vec: Vec<_> = numbers.iter().collect();
let sum: i32 = numbers_vec.iter().sum();
```

### Profile Before Optimizing
```rust
// Use criterion for benchmarks
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_lexing(c: &mut Criterion) {
    let source = include_str!("../tests/large_source.abjad");
    c.bench_function("lex_large_source", |b| {
        b.iter(|| Lexer::new(source).tokenize())
    });
}

criterion_group!(benches, benchmark_lexing);
criterion_main!(benches);
```

## Security

### Validate Input
```rust
// Good: Validate input bounds
pub fn read_byte(buffer: &[u8], offset: usize) -> Result<u8> {
    if offset >= buffer.len() {
        return Err(Error::OutOfBounds);
    }
    Ok(buffer[offset])
}

// Bad: No bounds checking
pub fn read_byte(buffer: &[u8], offset: usize) -> u8 {
    buffer[offset]  // May panic
}
```

### Avoid Unsafe Code
```rust
// Good: Use safe Rust when possible
pub fn concatenate_strings(a: &str, b: &str) -> String {
    format!("{}{}", a, b)
}

// Bad: Unnecessary unsafe code
pub fn concatenate_strings(a: &str, b: &str) -> String {
    unsafe {
        let mut result = String::with_capacity(a.len() + b.len());
        result.push_str(a);
        result.push_str(b);
        result
    }
}
```

## Code Review Checklist

### Before Submitting Code
- [ ] Code follows naming conventions
- [ ] Functions are short and focused
- [ ] Error handling is explicit
- [ ] Documentation is complete
- [ ] Tests are added
- [ ] No debug code left
- [ ] No commented-out code
- [ ] Code is formatted
- [ ] No clippy warnings
- [ ] Performance is acceptable

### Reviewing Others' Code
- [ ] Understand the intent
- [ ] Check for bugs
- [ ] Suggest improvements
- [ ] Verify tests
- [ ] Check documentation
- [ ] Be constructive
- [ ] Ask questions
- [ ] Explain reasoning

## Refactoring Guidelines

### When to Refactor
- Code is hard to understand
- Duplicated code exists
- Function is too long
- Complex logic can be simplified
- Performance needs improvement

### Refactoring Steps
1. Ensure tests pass
2. Make small changes
3. Run tests after each change
4. Commit frequently
5. Verify no regressions

### Common Refactorings
```rust
// Extract function
fn parse_expression(&mut self) -> Result<Expression> {
    let left = self.parse_primary()?;
    self.parse_binary_ops(left)
}

fn parse_binary_ops(&mut self, left: Expression) -> Result<Expression> {
    // ...
}

// Rename for clarity
fn tok(&mut self) -> Token {
    self.next_token().unwrap()
}

// Extract constant
const MAX_IDENTIFIER_LENGTH: usize = 255;

if identifier.len() > MAX_IDENTIFIER_LENGTH {
    return Err(Error::IdentifierTooLong);
}
```

## Resources

- "Clean Code" by Robert C. Martin
- "Refactoring" by Martin Fowler
- "The Pragmatic Programmer" by Andrew Hunt
- Rust API Guidelines
- Effective Rust
