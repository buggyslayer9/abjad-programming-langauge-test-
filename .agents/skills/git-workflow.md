---
description: Manages git operations and version control best practices for the Abjad compiler project
---

# Git Workflow Skill

## Purpose

This skill manages git operations and version control best practices for the Abjad compiler project. It ensures proper git usage, clean history, and effective collaboration.

## When to Use

Use this skill when:
- Starting new work
- Committing changes
- Creating branches
- Merging code
- Reviewing history
- Resolving conflicts
- Tagging releases

## Core Principles

### 1. Atomic Commits
- Each commit should be a logical unit
- One commit per feature/fix
- Keep commits focused
- Make commits reversible

### 2. Clear Commit Messages
- Use conventional commit format
- Be descriptive but concise
- Explain why, not just what
- Reference issues when applicable

### 3. Branch Strategy
- Use feature branches for new work
- Keep main branch stable
- Use descriptive branch names
- Delete merged branches

### 4. Code Review
- All code should be reviewed
- Use pull requests for merging
- Review should be thorough
- Address all feedback

## Branch Naming

### Feature Branches
```bash
feature/add-lexer
feature/implement-parser
feature/add-type-checker
feature/support-arabic-numbers
```

### Bug Fix Branches
```bash
fix/parser-crash
fix/memory-leak
fix-unicode-handling
fix-rtl-layout
```

### Documentation Branches
```bash
docs/update-readme
docs/add-api-docs
docs/grammar-spec
```

### Refactoring Branches
```bash
refactor/optimize-ast
refactor/simplify-lexer
refactor/improve-error-messages
```

### Release Branches
```bash
release/v0.1.0
release/v0.2.0
```

## Commit Message Format

### Conventional Commits
```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks
- `perf`: Performance improvements
- `ci`: CI/CD changes

### Examples
```bash
feat(lexer): add support for Arabic keywords

Implement parsing of Arabic keywords including:
- دالة (function)
- هيكل (struct)
- تعداد (enum)
- استورد (import)

Closes #123
```

```bash
fix(parser): resolve crash on empty input

The parser would panic when given empty input.
Now returns an empty AST instead.

Fixes #456
```

```bash
docs(readme): update installation instructions

Add instructions for building from source and
installing via package manager.
```

## Git Commands

### Starting New Work
```bash
# Update main branch
git checkout main
git pull origin main

# Create feature branch
git checkout -b feature/add-lexer

# Start working
```

### Committing Changes
```bash
# Stage changes
git add src/lexer.rs
git add tests/lexer_tests.rs

# Commit with message
git commit -m "feat(lexer): add Arabic keyword support"

# View commit
git log -1 --stat
```

### Pushing Changes
```bash
# Push to remote
git push -u origin feature/add-lexer

# If upstream is set
git push
```

### Updating Branch
```bash
# Rebase onto main
git fetch origin
git rebase origin/main

# Or merge main
git merge origin/main
```

### Merging to Main
```bash
# Create pull request via GitHub/GitLab
# Or merge directly (if approved)
git checkout main
git merge feature/add-lexer
git push origin main

# Delete feature branch
git branch -d feature/add-lexer
git push origin --delete feature/add-lexer
```

## Pull Request Workflow

### Before Creating PR
```bash
# Ensure branch is up to date
git fetch origin
git rebase origin/main

# Run tests
cargo test

# Check formatting
cargo fmt -- --check

# Run clippy
cargo clippy -- -D warnings

# Build in release mode
cargo build --release
```

### PR Description Template
```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] Manual testing performed

## Checklist
- [ ] Code follows project style guidelines
- [ ] Self-review performed
- [ ] Commented complex code
- [ ] Updated documentation
- [ ] No new warnings generated
- [ ] Added tests for new functionality
- [ ] All tests passing

## Related Issues
Closes #123
Related to #456
```

## Git Configuration

### User Configuration
```bash
git config --global user.name "Your Name"
git config --global user.email "your.email@example.com"
git config --global core.editor "vim"
git config --global init.defaultBranch "main"
```

### Aliases
```bash
# Common aliases
git config --global alias.co checkout
git config --global alias.br branch
git config --global alias.ci commit
git config --global alias.st status
git config --global alias.unstage 'reset HEAD --'
git config --global alias.last 'log -1 HEAD'
git config --global alias.visual '!gitk'
git config --global alias.graph 'log --graph --oneline --decorate'

# Commit aliases
git config --global alias.amend 'commit --amend --no-edit'
git config --global alias.commit 'commit -v'
git config --global alias.comm 'commit -m'

# Branch aliases
git config --global alias.nb 'checkout -b'
git config --global alias.bd 'branch -d'
git config --global alias.bD 'branch -D'

# Diff aliases
git config --global alias.d diff
git config --global alias.ds 'diff --staged'
git config --global alias.dc 'diff --cached'

# Log aliases
git config --global alias.lg 'log --graph --oneline --decorate --all'
git config --global alias.ll 'log --pretty=format:"%h %ad %s" --date=short'

# Stash aliases
git config --global alias.save 'stash save'
git config --global alias.pop 'stash pop'
git config --global alias.list 'stash list'
git config --global alias.show 'stash show'
git config --global alias.drop 'stash drop'
```

### .gitignore
```gitignore
# Rust
/target/
**/*.rs.bk
*.pdb
Cargo.lock

# IDE
.vscode/
.idea/
*.swp
*.swo
*~

# OS
.DS_Store
Thumbs.db

# Build
/dist/
/build/

# Test
/*.profraw
```

## History Management

### Interactive Rebase
```bash
# Rebase last 3 commits
git rebase -i HEAD~3

# Rebase onto main
git rebase origin/main
```

### Squashing Commits
```bash
# Interactive rebase to squash
git rebase -i HEAD~3

# Mark commits as squash (s) or fixup (f)
# Save and exit
```

### Amending Commits
```bash
# Amend last commit (add forgotten files)
git add forgotten_file.rs
git commit --amend --no-edit

# Amend last commit (change message)
git commit --amend -m "new message"
```

### Cherry-Picking
```bash
# Cherry-pick a commit
git cherry-pick <commit-hash>

# Cherry-pick multiple commits
git cherry-pick <commit1> <commit2>
```

## Conflict Resolution

### Resolving Conflicts
```bash
# During rebase or merge
# Git will pause and show conflicted files

# View conflicts
git status

# Edit conflicted files
# Look for <<<<, ====, >>>> markers

# Mark as resolved
git add <resolved-file>

# Continue rebase/merge
git rebase --continue
# or
git merge --continue

# Abort if needed
git rebase --abort
# or
git merge --abort
```

### Conflict Resolution Tools
```bash
# Use merge tool
git mergetool

# Use diff tool
git difftool

# View conflict markers
git diff --check
```

## Tagging

### Creating Tags
```bash
# Annotated tag (recommended)
git tag -a v0.1.0 -m "Version 0.1.0"

# Lightweight tag
git tag v0.1.0

# Tag specific commit
git tag -a v0.1.0 <commit-hash> -m "Version 0.1.0"
```

### Pushing Tags
```bash
# Push all tags
git push origin --tags

# Push specific tag
git push origin v0.1.0
```

### Listing Tags
```bash
# List all tags
git tag

# List tags with messages
git tag -n

# Show tag details
git show v0.1.0
```

### Deleting Tags
```bash
# Delete local tag
git tag -d v0.1.0

# Delete remote tag
git push origin --delete v0.1.0
```

## Release Workflow

### Pre-Release Checklist
- [ ] All tests passing
- [ ] Documentation updated
- [ ] CHANGELOG updated
- [ ] Version bumped
- [ ] No debug code left
- [ ] Performance acceptable
- [ ] Security review complete

### Release Process
```bash
# Create release branch
git checkout -b release/v0.1.0

# Update version in Cargo.toml
# Update CHANGELOG.md

# Commit changes
git commit -am "chore: prepare for v0.1.0 release"

# Tag release
git tag -a v0.1.0 -m "Release v0.1.0"

# Push to remote
git push origin release/v0.1.0
git push origin v0.1.0

# Merge to main
git checkout main
git merge release/v0.1.0
git push origin main

# Create GitHub release
# Upload binaries
# Announce release
```

## Stashing

### Stashing Changes
```bash
# Stash current work
git stash

# Stash with message
git stash save "work in progress on lexer"

# Stash including untracked files
git stash -u

# Stash including ignored files
git stash -a
```

### Applying Stashes
```bash
# Apply most recent stash
git stash pop

# Apply specific stash
git stash pop stash@{2}

# Apply without removing from stash
git stash apply

# List stashes
git stash list

# Drop stash
git stash drop stash@{0}

# Clear all stashes
git stash clear
```

## Bisecting

### Finding Bugs
```bash
# Start bisect
git bisect start

# Mark current as bad
git bisect bad

# Mark known good commit
git bisect good <commit-hash>

# Git will checkout middle commit
# Test the code
# Mark as good or bad
git bisect good
# or
git bisect bad

# Continue until bug found
# When done, reset
git bisect reset
```

## Submodules

### Adding Submodules
```bash
# Add submodule
git submodule add <repository-url> <path>

# Initialize submodules
git submodule init

# Update submodules
git submodule update

# Clone with submodules
git clone --recursive <repository-url>
```

### Updating Submodules
```bash
# Update all submodules
git submodule update --remote

# Update specific submodule
git submodule update --remote <path>
```

## Git Hooks

### Pre-Commit Hook
```bash
#!/bin/bash
# .git/hooks/pre-commit

# Run tests
cargo test --quiet
if [ $? -ne 0 ]; then
    echo "Tests failed. Aborting commit."
    exit 1
fi

# Check formatting
cargo fmt -- --check
if [ $? -ne 0 ]; then
    echo "Code not formatted. Run 'cargo fmt'. Aborting commit."
    exit 1
fi

# Run clippy
cargo clippy -- -D warnings
if [ $? -ne 0 ]; then
    echo "Clippy found warnings. Aborting commit."
    exit 1
fi
```

### Pre-Push Hook
```bash
#!/bin/bash
# .git/hooks/pre-push

# Run full test suite
cargo test
if [ $? -ne 0 ]; then
    echo "Tests failed. Aborting push."
    exit 1
fi
```

## Best Practices

### Commit Frequency
- Commit often, commit early
- Don't leave uncommitted work overnight
- Keep commits atomic
- Write meaningful messages

### Branch Hygiene
- Delete merged branches
- Keep branches short-lived
- Rebase before merging
- Resolve conflicts promptly

### History Cleanliness
- Avoid merge commits in feature branches
- Use rebase to keep history linear
- Squash related commits
- Remove sensitive data

### Security
- Never commit secrets
- Use .gitignore appropriately
- Review changes before committing
- Use signed commits for releases

## Troubleshooting

### Undo Last Commit
```bash
# Soft reset (keep changes)
git reset --soft HEAD~1

# Hard reset (discard changes)
git reset --hard HEAD~1

# Mixed reset (unstage changes)
git reset HEAD~1
```

### Recover Lost Commits
```bash
# View reflog
git reflog

# Recover lost commit
git checkout <commit-hash>
git checkout -b recovery-branch
```

### Fix Corrupted Repository
```bash
# Check repository integrity
git fsck

# Repair repository
git gc --prune=now
```

## Resources

- Pro Git Book
- GitHub Git Cheat Sheet
- Atlassian Git Tutorial
- Git SCM Documentation
