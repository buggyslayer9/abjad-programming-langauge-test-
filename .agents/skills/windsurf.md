---
description: Optimizes agent workflow and productivity for rapid development of the Abjad compiler
---

# Windsurf Skill

## Purpose

This skill optimizes AI agent workflow and productivity for rapid development of the Abjad compiler. It enables agents to work efficiently, maintain focus, and deliver high-quality code quickly.

## When to Use

Use this skill when:
- Starting a new development session
- Feeling stuck or blocked
- Need to prioritize tasks
- Want to optimize workflow
- Working on multiple features simultaneously
- Need to maintain momentum

## Core Principles

### 1. Work in Small Increments
- Break large tasks into smaller, manageable chunks
- Commit frequently
- Test incrementally
- Get feedback early

### 2. Maintain Flow State
- Minimize context switching
- Use todo lists to track progress
- Take breaks when stuck
- Return with fresh perspective

### 3. Focus on One Thing
- Single-task, don't multitask
- Complete one feature before starting another
- Use branches for parallel work
- Keep the main branch stable

### 4. Automate Repetitive Tasks
- Use scripts for common operations
- Configure tools for efficiency
- Set up hotkeys and aliases
- Use pre-commit hooks

## Workflow Optimization

### Session Setup
```bash
# Start a development session
cd compiler
git checkout -b feature/new-feature
cargo check
```

### Incremental Development
```bash
# Work in small steps
1. Write test
2. Implement minimal code
3. Run test
4. Commit
5. Repeat
```

### Quick Feedback Loop
```bash
# Watch for changes
cargo watch -x check -x test

# Run specific test
cargo test test_name

# Check compilation
cargo check --message-format=short
```

## Task Management

### Todo List Pattern
```markdown
## Current Session
- [ ] Implement lexer for Arabic keywords
- [ ] Add tests for lexer
- [ ] Update documentation
- [ ] Commit changes

## Next Session
- [ ] Implement parser
- [ ] Add AST nodes
- [ ] Write parser tests
```

### Priority Framework
1. **Critical:** Blocks other work
2. **High:** Important for current milestone
3. **Medium:** Nice to have
4. **Low:** Can defer

### Time Boxing
- Set time limits for tasks
- Use Pomodoro technique (25 min work, 5 min break)
- Estimate before starting
- Review estimates after completion

## Productivity Techniques

### The Two-Minute Rule
If a task takes less than two minutes, do it immediately:
- Fix a typo
- Add a simple test
- Update documentation
- Commit changes

### The "Done" Definition
Before starting a task, define what "done" means:
- Code written
- Tests passing
- Documentation updated
- Code reviewed
- Committed

### The "Stop" Criteria
Know when to stop working on a task:
- Tests pass
- Documentation complete
- No obvious improvements
- Time limit reached

## Code Writing Workflow

### 1. Understand the Requirement
```rust
// Read the spec or issue
// Ask clarifying questions
// Write a brief description
```

### 2. Write the Test First
```rust
#[test]
fn test_arabic_keyword() {
    let mut lexer = Lexer::new("دالة");
    assert_eq!(lexer.next_token(), Ok(Token::Function));
}
```

### 3. Implement Minimal Code
```rust
// Write just enough to pass the test
// Don't over-engineer
// Keep it simple
```

### 4. Run and Verify
```bash
cargo test test_arabic_keyword
```

### 5. Refactor if Needed
```rust
// Improve code quality
// Extract common patterns
// Add documentation
```

### 6. Commit
```bash
git add .
git commit -m "feat: add Arabic keyword support to lexer"
```

## Debugging Workflow

### 1. Reproduce the Issue
```bash
# Create minimal reproduction
cargo test -- --nocapture
```

### 2. Add Debug Output
```rust
eprintln!("Debug: {:?}", variable);
```

### 3. Use Debugger
```bash
cargo build
rust-gdb target/debug/abjad
```

### 4. Add Logging
```rust
log::debug!("Processing token: {:?}", token);
```

### 5. Fix and Verify
```bash
cargo test
```

## Refactoring Workflow

### 1. Ensure Tests Pass
```bash
cargo test
```

### 2. Make Small Changes
```rust
// Extract function
// Rename variable
// Simplify logic
```

### 3. Run Tests After Each Change
```bash
cargo test
```

### 4. Commit Frequently
```bash
git commit -m "refactor: extract token parsing logic"
```

## Documentation Workflow

### 1. Document as You Code
```rust
/// Tokenizes Arabic keywords
///
/// # Examples
/// ```
/// let lexer = Lexer::new("دالة");
/// assert_eq!(lexer.next_token(), Token::Function);
/// ```
pub fn parse_keyword(&mut self) -> Token {
    // ...
}
```

### 2. Update README
```markdown
## New Features
- Added support for Arabic keywords
```

### 3. Update CHANGELOG
```markdown
## [0.1.0] - 2025-01-14
### Added
- Arabic keyword support in lexer
```

## Git Workflow

### Branch Naming
```bash
feature/add-lexer
fix/parser-error
docs/update-readme
refactor/optimize-ast
```

### Commit Messages
```bash
feat: add Arabic keyword support to lexer
fix: resolve parser error with nested expressions
docs: update README with new features
refactor: optimize AST node allocation
test: add integration tests for compiler
```

### Commit Frequency
- Commit after each logical change
- Don't leave uncommitted work overnight
- Keep commits atomic
- Write clear commit messages

## Code Review Workflow

### Before Submitting
```bash
# Run all tests
cargo test

# Check formatting
cargo fmt -- --check

# Run clippy
cargo clippy -- -D warnings

# Build in release mode
cargo build --release
```

### Review Checklist
- [ ] Tests pass
- [ ] Code is formatted
- [ ] No clippy warnings
- [ ] Documentation updated
- [ ] Commit messages are clear
- [ ] No debug code left

## Collaboration

### Asking for Help
1. Describe the problem clearly
2. Show what you've tried
3. Provide minimal reproduction
4. Include error messages
5. Ask specific questions

### Code Review Etiquette
- Be constructive
- Explain your reasoning
- Ask questions
- Suggest improvements
- Be respectful

## Tool Configuration

### VS Code Settings
```json
{
    "rust-analyzer.checkOnSave.command": "clippy",
    "rust-analyzer.cargo.loadOutDirsFromCheck": true,
    "editor.formatOnSave": true,
    "editor.rulers": [80, 100]
}
```

### Git Aliases
```bash
git config --global alias.co checkout
git config --global alias.br branch
git config --global alias.ci commit
git config --global alias.st status
git config --global alias.unstage 'reset HEAD --'
git config --global alias.last 'log -1 HEAD'
```

### Cargo Aliases
```toml
[alias]
b = "build"
c = "check"
t = "test"
r = "run"
fmt = "fmt --all"
clippy = "clippy -- -D warnings"
```

## Time Management

### Daily Routine
1. **Morning:** Review progress, plan tasks
2. **Mid-day:** Deep work on main task
3. **Afternoon:** Code review, meetings
4. **Evening:** Wrap up, commit changes

### Weekly Routine
1. **Monday:** Plan week, review roadmap
2. **Tuesday-Thursday:** Deep work
3. **Friday:** Code review, planning next week
4. **Weekend:** Rest, explore new ideas

### Estimation
- Break tasks into hours
- Multiply by 1.5 for buffer
- Track actual time
- Adjust estimates based on experience

## Burnout Prevention

### Signs of Burnout
- Feeling constantly tired
- Losing motivation
- Making more mistakes
- Irritability
- Difficulty concentrating

### Prevention Strategies
- Take regular breaks
- Maintain work-life balance
- Set realistic goals
- Celebrate small wins
- Ask for help when needed

### Recovery
- Take time off
- Reduce workload temporarily
- Focus on different tasks
- Seek support
- Reassess priorities

## Continuous Improvement

### Weekly Review
- What went well?
- What didn't go well?
- What can be improved?
- What to focus on next week?

### Monthly Review
- Progress toward goals
- Skill development
- Tool improvements
- Process optimization

### Learning
- Read documentation
- Study open source code
- Attend conferences
- Participate in communities
- Share knowledge

## Resources

- "Deep Work" by Cal Newport
- "The Pragmatic Programmer" by Andrew Hunt
- "Clean Code" by Robert C. Martin
- Rust Compiler Development Guide
- Effective Rust Blog
