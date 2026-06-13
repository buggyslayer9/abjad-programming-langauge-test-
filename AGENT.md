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
│   │   └── lexer.rs
│   ├── Cargo.toml
│   └── README.md
├── refrences/            # Documentation
│   ├── abjad-prd.html
│   └── phases-roadmap.md
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

Current phase: **Phase 0 - Foundation (Q1 2025)**

Focus areas:
1. Language & Grammar specification
2. Basic Compiler implementation
3. Memory Management design
4. Basic Tools implementation

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

**Last Updated:** 2025-01-14  
**Version:** 0.1.0
