# AGENT.md - Abjad Project Agent Configuration

This file contains configuration and guidelines for AI agents working on the Abjad programming language project.

## Project Overview

**Abjad** is an Arabic-first, high-performance programming language that compiles directly to Assembly. The project includes:
- Compiler (Rust-based)
- Standard Library
- SidraUI (GUI library with Vulkan)
- Sarab (Game development framework)
- IDE (RTL-aware)
- Various tools and utilities

## Agent Skills

The following skills are available for AI agents working on this project:

### 1. Antigravity Skill
**Purpose:** Enhances agent capabilities for complex problem-solving and architectural decisions.
**File:** `.windsurf/skills/antigravity.md`

### 2. Windsurf Skill
**Purpose:** Optimizes agent workflow and productivity for rapid development.
**File:** `.windsurf/skills/windsurf.md`

### 3. Opencode Skill
**Purpose:** Improves code quality, maintainability, and best practices.
**File:** `.windsurf/skills/opencode.md`

### 4. Rust Best Practices Skill
**Purpose:** Ensures Rust code follows idiomatic patterns and best practices.
**File:** `.windsurf/skills/rust-best-practices.md`

### 5. Git Workflow Skill
**Purpose:** Manages git operations and version control best practices.
**File:** `.windsurf/skills/git-workflow.md`

## Project Structure

```
abjadprogramminglanguage/
├── compiler/              # Rust compiler
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs
│   │   ├── cli.rs
│   │   ├── error.rs
│   │   ├── token.rs
│   │   ├── lexer.rs
│   │   ├── parser.rs
│   │   ├── ast.rs
│   │   ├── type_checker.rs
│   │   ├── borrow_checker.rs
│   │   ├── memory_safety.rs
│   │   ├── stack_allocation.rs
│   │   ├── heap_allocation.rs
│   │   ├── memory_layout.rs
│   │   ├── codegen.rs
│   │   ├── object_file.rs
│   │   ├── linker.rs
│   │   ├── package_manager.rs
│   │   ├── build_system.rs
│   │   ├── doc_generator.rs
│   │   ├── debugger.rs
│   │   ├── profiler.rs
│   │   ├── linter.rs
│   │   ├── formatter.rs
│   │   ├── optimizer.rs
│   │   ├── runtime_optimizer.rs
│   │   ├── memory_optimizer.rs
│   │   └── codegen_optimizer.rs
│   ├── Cargo.toml
│   └── README.md
├── std/                   # Standard Library
│   ├── core/
│   │   ├── mod.rs
│   │   └── types.rs
│   ├── strings/
│   │   └── mod.rs
│   ├── io/
│   │   └── mod.rs
│   └── collections/
│       └── mod.rs
├── refrences/            # Documentation
│   ├── abjad-prd.html
│   ├── phases-roadmap.md
│   ├── phase0-memory-management.md
│   ├── phase1-standard-library.md
│   ├── phase2-ecosystem.md
│   ├── phase3-advanced-tools.md
│   └── phase4-optimizations.md
└── .windsurf/
    └── skills/           # Agent skills
```

## Agent Guidelines

### Code Quality
- Follow Rust best practices (use the Rust Best Practices skill)
- Write clean, readable code (use the Opencode skill)
- Add comprehensive tests
- Document public APIs

### Workflow
- Use git for version control (use the Git Workflow skill)
- Create feature branches for new features
- Write descriptive commit messages
- Use pull requests for code review

### Problem Solving
- Use the Antigravity skill for complex architectural decisions
- Break down large tasks into smaller steps
- Test incrementally
- Ask for clarification when needed

### Productivity
- Use the Windsurf skill to optimize workflow
- Focus on one task at a time
- Use todo lists to track progress
- Take breaks when stuck

## Development Phases

Current phase: **Phase 4 - Optimizations (Q1 2026)** - COMPLETED

### Completed Phases:
- **Phase 0 - Foundation (Q1 2025)** - 100% Complete
  - Language & Grammar specification
  - Basic Compiler implementation (lexer, parser, type checker)
  - Memory Management design (borrow checker, memory safety, stack/heap allocation, memory layout)
  - Basic Tools implementation

- **Phase 1 - Standard Library (Q2 2025)** - 100% Complete
  - Core types (Option, Result, Any, Range, Pair, Triple)
  - String handling with Arabic numeral conversion
  - Collections (Array, List, Map, Set)
  - I/O operations (File, InputStream, OutputStream, Terminal, Directory)

- **Phase 2 - Ecosystem (Q3 2025)** - 100% Complete
  - Package manager with TOML support
  - Build system with target configuration
  - Documentation generator with HTML output
  - Design document

- **Phase 3 - Advanced Tools (Q4 2025)** - 100% Complete
  - Debugger with breakpoints and stack trace
  - Profiler with function and memory profiling
  - Linter with style and naming rules
  - Formatter with indentation and formatting
  - Design document

- **Phase 4 - Optimizations (Q1 2026)** - 100% Complete
  - Compiler optimizations (constant folding, propagation, dead code elimination)
  - Runtime optimizations (instruction scheduling, cache optimization)
  - Memory optimizations (stack allocation, pooling, reuse)
  - Code generation optimizations (LLVM, Cranelift, register allocation)
  - Design document

### Recent Compilation Fixes (June 2026):
- Fixed cli.rs import errors (changed from abjad_compiler:: to crate::)
- Fixed error.rs Warning enum derive (added Error derive)
- Fixed doc_generator.rs Command import
- Fixed Assignment variant errors in multiple files (Assignment is a Statement, not an Expression)
- Fixed cli.rs type mismatches (Option<&Path> vs Option<&PathBuf>)
- Fixed formatter.rs type ambiguity (specified indent_level as usize)
- Fixed linker.rs unreachable expressions (added cfg guards)
- Fixed memory_safety.rs type errors (dereferencing)
- Fixed optimizer.rs dereference error (removed extra dereference)

See `refrences/phases-roadmap.md` for detailed roadmap.

## Communication

- Use Arabic for UI strings and user-facing text
- Use English for code comments and documentation
- Be concise and direct in responses
- Provide code examples when helpful

## Testing

- Write unit tests for all modules
- Write integration tests for critical paths
- Use `cargo test` to run tests
- Aim for high test coverage

## Performance

- Profile code regularly
- Optimize hot paths
- Use benchmarks for performance-critical code
- Consider zero-cost abstractions

---

**Last Updated:** 2026-06-14  
**Version:** 0.5.0 (Phase 4 Complete)
