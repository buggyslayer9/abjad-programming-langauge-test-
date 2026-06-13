---
description: Enhances agent capabilities for complex problem-solving and architectural decisions in the Abjad compiler project
---

# Antigravity Skill

## Purpose

This skill enhances AI agent capabilities for complex problem-solving, architectural decisions, and deep technical challenges in the Abjad compiler project. It enables agents to think beyond conventional approaches and find innovative solutions.

## When to Use

Use this skill when:
- Facing complex architectural decisions
- Designing core compiler components (lexer, parser, type checker, code generation)
- Solving performance bottlenecks
- Making trade-offs between different implementation approaches
- Designing memory management systems
- Implementing advanced language features (generics, traits, async/await)
- Optimizing critical code paths

## Core Principles

### 1. Think in Layers
- Separate concerns into distinct layers
- Define clear interfaces between layers
- Each layer should have a single responsibility
- Minimize coupling between layers

### 2. Consider Trade-offs
- Every decision has trade-offs
- Explicitly state the trade-offs being made
- Document why a particular approach was chosen
- Be prepared to revisit decisions

### 3. Optimize for the Common Case
- Profile before optimizing
- Focus on hot paths
- Consider cache locality
- Use appropriate data structures

### 4. Design for Extensibility
- Use traits for polymorphism
- Design APIs that are easy to extend
- Avoid breaking changes when possible
- Provide sensible defaults

## Compiler Architecture Guidelines

### Lexer Design
```rust
// Good: Simple, efficient lexer
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    position: Position,
}

// Avoid: Overly complex lexer with too many features
```

### Parser Design
```rust
// Good: Recursive descent parser with clear grammar
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

// Avoid: Parser with mixed responsibilities
```

### Type System Design
```rust
// Good: Trait-based type system
pub trait Type {
    fn size(&self) -> usize;
    fn align(&self) -> usize;
}

// Avoid: Hardcoded type checks
```

### Code Generation
```rust
// Good: Separate IR from target-specific code
pub trait CodeGenerator {
    fn generate(&mut self, ir: &IR) -> Result<Assembly>;
}

// Avoid: Direct code generation without IR
```

## Memory Management Guidelines

### Ownership System
```rust
// Good: Clear ownership semantics
pub struct Owned<T> {
    data: T,
}

// Avoid: Unclear ownership
```

### Borrowing
```rust
// Good: Explicit lifetimes
pub fn borrow<'a>(&'a self) -> &'a T {
    &self.data
}

// Avoid: Implicit lifetimes where clarity is needed
```

### Lifetimes
```rust
// Good: Meaningful lifetime names
pub struct Context<'ctx> {
    // ...
}

// Avoid: Generic lifetime names
```

## Performance Optimization

### Profiling
```rust
// Use criterion for benchmarks
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_lexing(c: &mut Criterion) {
    c.bench_function("lex_simple", |b| {
        b.iter(|| lexer.tokenize(input))
    });
}
```

### Zero-Cost Abstractions
```rust
// Good: Compile-time polymorphism
pub trait Parser {
    fn parse(&mut self) -> Result<AST>;
}

// Avoid: Runtime polymorphism where not needed
```

### SIMD Optimization
```rust
// Good: Use SIMD for data-parallel operations
use std::arch::x86_64::*;

unsafe fn add_simd(a: __m256i, b: __m256i) -> __m256i {
    _mm256_add_epi32(a, b)
}
```

## Error Handling

### Error Types
```rust
// Good: Structured errors with context
#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("Lexical error at {position}: {message}")]
    Lexical { position: Position, message: String },
    
    #[error("Type error: expected {expected}, found {found}")]
    TypeMismatch { expected: Type, found: Type },
}

// Avoid: Generic error messages
```

### Error Recovery
```rust
// Good: Attempt to recover from errors
pub fn recover_from_error(&mut self, error: Error) -> Option<Token> {
    // Skip tokens until we find a synchronization point
    self.skip_to_sync_point()
}

// Avoid: Stop on first error
```

## Testing Strategy

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
}
```

### Integration Tests
```rust
#[test]
fn test_full_compilation() {
    let source = r#"
        دالة رئيسية() {
            طباعة("مرحباً")
        }
    "#;
    
    let result = compile(source);
    assert!(result.is_ok());
}
```

### Property-Based Testing
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_roundtrip(s in "\\PC*") {
        let tokens = tokenize(&s);
        let reconstructed = reconstruct(&tokens);
        assert_eq!(s, reconstructed);
    }
}
```

## Documentation

### API Documentation
```rust
/// Tokenizes the input string into a stream of tokens
///
/// # Arguments
/// * `input` - The source code to tokenize
///
/// # Returns
/// A vector of tokens
///
/// # Errors
/// Returns an error if the input contains invalid characters
///
/// # Example
/// ```
/// let mut lexer = Lexer::new("دالة");
/// let tokens = lexer.tokenize()?;
/// ```
pub fn tokenize(input: &str) -> Result<Vec<Token>> {
    // ...
}
```

### Architecture Documentation
- Document design decisions
- Explain trade-offs
- Provide examples
- Keep documentation up to date

## Common Patterns

### Visitor Pattern
```rust
pub trait Visitor {
    fn visit_function(&mut self, func: &Function);
    fn visit_struct(&mut self, struct_: &Struct);
    fn visit_expression(&mut self, expr: &Expression);
}
```

### Builder Pattern
```rust
pub struct CompilerBuilder {
    opt_level: OptLevel,
    target: Target,
}

impl CompilerBuilder {
    pub fn new() -> Self { /* ... */ }
    pub fn opt_level(mut self, level: OptLevel) -> Self { /* ... */ }
    pub fn build(self) -> Compiler { /* ... */ }
}
```

### State Machine
```rust
pub enum LexerState {
    Start,
    InIdentifier,
    InNumber,
    InString,
    // ...
}
```

## Anti-Patterns to Avoid

### Over-Engineering
- Don't add abstractions before they're needed
- Keep it simple initially
- Refactor when patterns emerge

### Premature Optimization
- Profile first
- Optimize based on measurements
- Consider maintainability

### Tight Coupling
- Use traits for loose coupling
- Dependency injection
- Clear interfaces

### Global State
- Pass context explicitly
- Use dependency injection
- Avoid mutable globals

## Decision Framework

When making architectural decisions:

1. **Define the Problem**
   - What are we trying to solve?
   - What are the constraints?

2. **Explore Options**
   - List possible approaches
   - Consider trade-offs

3. **Evaluate**
   - Prototype if needed
   - Measure performance
   - Assess maintainability

4. **Decide**
   - Choose the best option
   - Document the decision
   - Explain the rationale

5. **Review**
   - Revisit periodically
   - Be willing to change
   - Learn from experience

## Resources

- Rust Compiler Development Guide
- LLVM Documentation
- Cranelift Documentation
- "Crafting Interpreters" by Robert Nystrom
- "Engineering a Compiler" by Keith Cooper
