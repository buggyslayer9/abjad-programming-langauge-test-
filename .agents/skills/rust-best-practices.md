---
description: Ensures Rust code follows idiomatic patterns and best practices for the Abjad compiler
---

# Rust Best Practices Skill

## Purpose

This skill ensures that Rust code in the Abjad compiler follows idiomatic patterns, best practices, and leverages Rust's features effectively. It helps write safe, performant, and maintainable Rust code.

## When to Use

Use this skill when:
- Writing new Rust code
- Reviewing existing Rust code
- Refactoring Rust code
- Optimizing Rust code
- Debugging Rust-specific issues
- Learning Rust patterns

## Core Principles

### 1. Leverage the Type System
- Use types to express invariants
- Make illegal states unrepresentable
- Use enums for sum types
- Use structs for product types

### 2. Embrace Ownership and Borrowing
- Understand ownership rules
- Use references when possible
- Avoid unnecessary cloning
- Use `Cow` for conditional ownership

### 3. Use Iterators Effectively
- Prefer iterator chains over loops
- Use `collect()` sparingly
- Leverage lazy evaluation
- Use functional combinators

### 4. Error Handling with Result
- Use `Result` for fallible operations
- Use `?` operator for propagation
- Provide meaningful error types
- Avoid `unwrap()` in production code

## Type System Patterns

### Newtype Pattern
```rust
// Good: Use newtypes for type safety
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TokenId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

// Bad: Use raw types
pub type TokenId = usize;
pub struct Span {
    pub start: usize,
    pub end: usize,
}
```

### Enum for State Machines
```rust
// Good: Use enum for lexer states
#[derive(Debug, Clone, Copy)]
pub enum LexerState {
    Start,
    InIdentifier,
    InNumber,
    InString(char),  // Store quote character
    InComment,
}

// Bad: Use multiple boolean flags
pub struct LexerState {
    in_identifier: bool,
    in_number: bool,
    in_string: bool,
    in_comment: bool,
}
```

### Trait Objects vs Generics
```rust
// Good: Use generics when type is known at compile time
pub trait Parser {
    fn parse(&mut self) -> Result<AST>;
}

pub struct RecursiveDescentParser {
    // ...
}

// Use trait objects only when runtime polymorphism is needed
pub fn parse_with(parser: &mut dyn Parser) -> Result<AST> {
    parser.parse()
}
```

## Ownership and Borrowing

### Prefer References Over Cloning
```rust
// Good: Use references
pub fn count_tokens(tokens: &[Token]) -> usize {
    tokens.len()
}

// Bad: Clone unnecessarily
pub fn count_tokens(tokens: Vec<Token>) -> usize {
    tokens.len()
}
```

### Use `Cow` for Conditional Ownership
```rust
// Good: Use Cow for conditional ownership
use std::borrow::Cow;

pub fn process_string(s: &str) -> Cow<str> {
    if s.contains(' ') {
        let processed: String = s.replace(' ', "_");
        Cow::Owned(processed)
    } else {
        Cow::Borrowed(s)
    }
}
```

### Lifetime Annotations
```rust
// Good: Explicit lifetimes when needed
pub fn split_at<'a>(s: &'a str, mid: usize) -> (&'a str, &'a str) {
    (&s[..mid], &s[mid..])
}

// Good: Lifetime elision when clear
pub fn first_char(s: &str) -> Option<char> {
    s.chars().next()
}
```

### Interior Mutability
```rust
// Good: Use RefCell for interior mutability when needed
use std::cell::RefCell;

pub struct Lexer {
    tokens: RefCell<Vec<Token>>,
}

impl Lexer {
    pub fn add_token(&self, token: Token) {
        self.tokens.borrow_mut().push(token);
    }
}

// Better: Use standard borrowing when possible
pub struct Lexer<'a> {
    tokens: &'a mut Vec<Token>,
}

impl<'a> Lexer<'a> {
    pub fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }
}
```

## Iterator Patterns

### Iterator Chains
```rust
// Good: Use iterator chains
pub fn filter_keywords(tokens: &[Token]) -> Vec<Token> {
    tokens
        .iter()
        .filter(|t| t.is_keyword())
        .cloned()
        .collect()
}

// Bad: Manual loop
pub fn filter_keywords(tokens: &[Token]) -> Vec<Token> {
    let mut result = Vec::new();
    for token in tokens {
        if token.is_keyword() {
            result.push(token.clone());
        }
    }
    result
}
```

### Lazy Evaluation
```rust
// Good: Lazy evaluation with iterators
pub fn find_first_keyword(tokens: &[Token]) -> Option<&Token> {
    tokens.iter().find(|t| t.is_keyword())
}

// Bad: Collect everything first
pub fn find_first_keyword(tokens: &[Token]) -> Option<&Token> {
    let keywords: Vec<_> = tokens.iter().filter(|t| t.is_keyword()).collect();
    keywords.first().copied()
}
```

### Custom Iterators
```rust
// Good: Implement Iterator trait
pub struct Tokenizer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        // Implementation
    }
}

// Usage
for token in Tokenizer::new(source) {
    match token {
        Ok(t) => println!("{:?}", t),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
```

## Error Handling

### Custom Error Types
```rust
// Good: Structured error types
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("Unexpected character '{0}' at position {1}")]
    UnexpectedCharacter(char, usize),
    
    #[error("Unterminated string starting at position {0}")]
    UnterminatedString(usize),
    
    #[error("Invalid escape sequence: {0}")]
    InvalidEscapeSequence(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
```

### Error Context
```rust
// Good: Add context to errors
pub fn tokenize_file(path: &Path) -> Result<Vec<Token>> {
    let source = std::fs::read_to_string(path)
        .map_err(|e| LexerError::Io(e))
        .context(format!("Failed to read file: {}", path.display()))?;
    
    tokenize(&source)
}

// Use anyhow for error context
use anyhow::Context;

pub fn tokenize_file(path: &Path) -> Result<Vec<Token>> {
    let source = std::fs::read_to_string(path)
        .context("Failed to read source file")?;
    
    tokenize(&source).context("Failed to tokenize source")?
}
```

### Error Conversion
```rust
// Good: Implement From for error conversion
impl From<std::io::Error> for LexerError {
    fn from(err: std::io::Error) -> Self {
        LexerError::Io(err)
    }
}

// Then use ? operator directly
pub fn tokenize_file(path: &Path) -> Result<Vec<Token>, LexerError> {
    let source = std::fs::read_to_string(path)?;  // Automatic conversion
    tokenize(&source)
}
```

## Concurrency

### Use Channels for Communication
```rust
// Good: Use channels for thread communication
use std::sync::mpsc;

pub fn parallel_tokenize(sources: Vec<String>) -> Vec<Result<Vec<Token>>> {
    let (tx, rx) = mpsc::channel();
    
    for source in sources {
        let tx = tx.clone();
        std::thread::spawn(move || {
            let result = tokenize(&source);
            tx.send(result).unwrap();
        });
    }
    
    drop(tx);  // Drop original sender
    
    rx.iter().collect()
}
```

### Use Arc for Shared Ownership
```rust
// Good: Use Arc for shared immutable data
use std::sync::Arc;

pub struct SharedConfig {
    settings: Arc<Config>,
}

impl SharedConfig {
    pub fn new(config: Config) -> Self {
        SharedConfig {
            settings: Arc::new(config),
        }
    }
    
    pub fn clone_config(&self) -> Arc<Config> {
        Arc::clone(&self.settings)
    }
}
```

### Use Mutex for Shared Mutable State
```rust
// Good: Use Mutex for shared mutable state
use std::sync::Mutex;

pub struct TokenCache {
    cache: Mutex<HashMap<String, Vec<Token>>>,
}

impl TokenCache {
    pub fn get(&self, source: &str) -> Option<Vec<Token>> {
        let cache = self.cache.lock().unwrap();
        cache.get(source).cloned()
    }
    
    pub fn insert(&self, source: String, tokens: Vec<Token>) {
        let mut cache = self.cache.lock().unwrap();
        cache.insert(source, tokens);
    }
}
```

## Performance

### Zero-Cost Abstractions
```rust
// Good: Use generics for zero-cost abstractions
pub fn parse<T: Parser>(parser: &mut T) -> Result<AST> {
    parser.parse()
}

// The above compiles to the same code as a direct call
```

### Stack Allocation
```rust
// Good: Use stack allocation when possible
pub fn parse_expression() -> Expression {
    Expression::Binary {
        op: Operator::Add,
        left: Box::new(Expression::Literal(1)),
        right: Box::new(Expression::Literal(2)),
    }
}

// Bad: Unnecessary heap allocation
pub fn parse_expression() -> Box<Expression> {
    Box::new(Expression::Binary {
        op: Operator::Add,
        left: Box::new(Expression::Literal(1)),
        right: Box::new(Expression::Literal(2)),
    })
}
```

### String Handling
```rust
// Good: Use &str when possible
pub fn process(input: &str) -> String {
    input.to_uppercase()
}

// Bad: Always use String
pub fn process(input: String) -> String {
    input.to_uppercase()
}
```

### Avoid Unnecessary Copies
```rust
// Good: Use references to avoid copies
pub fn analyze_tokens(tokens: &[Token]) -> Analysis {
    // ...
}

// Bad: Take ownership unnecessarily
pub fn analyze_tokens(tokens: Vec<Token>) -> Analysis {
    // ...
}
```

## Memory Safety

### Avoid Unsafe Code
```rust
// Good: Use safe Rust
pub fn concatenate(a: &str, b: &str) -> String {
    format!("{}{}", a, b)
}

// Only use unsafe when absolutely necessary
pub unsafe fn concatenate_unsafe(a: &str, b: &str) -> String {
    let mut result = String::with_capacity(a.len() + b.len());
    result.push_str(a);
    result.push_str(b);
    result
}
```

### Use Slices
```rust
// Good: Use slices for views into data
pub fn process_data(data: &[u8]) -> ProcessedData {
    // ...
}

// Bad: Use Vec when slice is sufficient
pub fn process_data(data: Vec<u8>) -> ProcessedData {
    // ...
}
```

### Bounds Checking
```rust
// Good: Use get for safe access
pub fn get_token(tokens: &[Token], index: usize) -> Option<&Token> {
    tokens.get(index)
}

// Bad: Manual indexing may panic
pub fn get_token(tokens: &[Token], index: usize) -> &Token {
    &tokens[index]  // May panic
}
```

## Idiomatic Patterns

### Builder Pattern
```rust
// Good: Builder pattern for complex construction
pub struct CompilerBuilder {
    opt_level: OptLevel,
    target: Target,
    debug_symbols: bool,
}

impl CompilerBuilder {
    pub fn new() -> Self {
        CompilerBuilder {
            opt_level: OptLevel::O0,
            target: Target::Native,
            debug_symbols: false,
        }
    }
    
    pub fn opt_level(mut self, level: OptLevel) -> Self {
        self.opt_level = level;
        self
    }
    
    pub fn target(mut self, target: Target) -> Self {
        self.target = target;
        self
    }
    
    pub fn debug_symbols(mut self, debug: bool) -> Self {
        self.debug_symbols = debug;
        self
    }
    
    pub fn build(self) -> Compiler {
        Compiler {
            opt_level: self.opt_level,
            target: self.target,
            debug_symbols: self.debug_symbols,
        }
    }
}

// Usage
let compiler = CompilerBuilder::new()
    .opt_level(OptLevel::O2)
    .target(Target::X86_64)
    .debug_symbols(true)
    .build();
```

### Default Trait
```rust
// Good: Implement Default for sensible defaults
impl Default for CompilerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for Lexer {
    fn default() -> Self {
        Lexer::new("")
    }
}
```

### From and Into Traits
```rust
// Good: Implement From for conversions
impl From<TokenKind> for Token {
    fn from(kind: TokenKind) -> Self {
        Token {
            kind,
            span: Span::default(),
        }
    }
}

// Then you can use .into()
let token: Token = TokenKind::Function.into();
```

### Display and Debug Traits
```rust
// Good: Implement Display for user-facing output
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

// Good: Implement Debug for debugging
impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Token")
            .field("kind", &self.kind)
            .field("span", &self.span)
            .finish()
    }
}
```

## Testing

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_identifier() {
        let mut lexer = Lexer::new("اسم");
        assert_eq!(lexer.next_token(), Ok(Token::Identifier("اسم".to_string())));
    }

    #[test]
    fn test_arabic_numbers() {
        let mut lexer = Lexer::new("١٢٣");
        assert_eq!(lexer.next_token(), Ok(Token::Integer(123)));
    }
}
```

### Integration Tests
```rust
// tests/integration_test.rs
use abjad::Compiler;

#[test]
fn test_full_compilation() {
    let source = r#"
        دالة رئيسي() {
            طباعة("مرحباً")
        }
    "#;
    
    let compiler = Compiler::new();
    let result = compiler.compile(source);
    assert!(result.is_ok());
}
```

### Property-Based Testing
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_roundtrip(s in "\\PC*") {
        let tokens = tokenize(&s)?;
        let reconstructed = reconstruct(&tokens);
        prop_assert_eq!(s, reconstructed);
    }
}
```

## Documentation

### Module Documentation
```rust
//! Lexical analysis for the Abjad programming language.
//!
//! This module provides the lexer which converts source code into
//! a stream of tokens.
//!
//! # Example
//!
//! ```
//! use abjad::lexer::Lexer;
//!
//! let mut lexer = Lexer::new("دالة");
//! let tokens = lexer.tokenize()?;
//! ```
```

### Function Documentation
```rust
/// Tokenizes the input source code.
///
/// # Arguments
///
/// * `source` - The source code to tokenize
///
/// # Returns
///
/// A vector of tokens
///
/// # Errors
///
/// Returns an error if the source contains invalid characters
///
/// # Example
///
/// ```
/// let tokens = Lexer::new("دالة").tokenize()?;
/// ```
pub fn tokenize(source: &str) -> Result<Vec<Token>> {
    // Implementation
}
```

## Cargo Configuration

### Dependencies
```toml
[dependencies]
# Use specific versions for reproducibility
clap = { version = "4.5", features = ["derive"] }
thiserror = "1.0"
anyhow = "1.0"

# Use workspace dependencies for shared crates
# [dependencies]
# abjad-ast = { path = "../ast" }
```

### Features
```toml
[features]
default = []
llvm = ["inkwell"]
cranelift = ["cranelift"]
```

### Dev Dependencies
```toml
[dev-dependencies]
criterion = "0.5"
proptest = "1.4"
```

## Resources

- The Rust Book
- Rust by Example
- Rust API Guidelines
- Effective Rust
- Rust Compiler Development Guide
