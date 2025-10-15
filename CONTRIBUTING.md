# Contributing to VS Code Latency Monitor

Thank you for your interest in contributing to the VS Code Latency Monitor! This document provides guidelines and information for contributors.

## 🚀 Getting Started

### Prerequisites
- **Rust 1.70+**: Install from [rustup.rs](https://rustup.rs/)
- **Git**: For version control
- **VS Code**: Recommended IDE with Rust analyzer extension

### Development Setup
```bash
# Clone the repository
git clone https://github.com/your-username/vscode-latency-monitor.git
cd vscode-latency-monitor

# Build the project
cargo build

# Run tests
cargo test
cargo run -- test --iterations 3

# Start development services
cargo run -- dashboard --port 3030 &
cargo run -- telemetry --port 8081 &
```

## 🏗️ Project Structure

### Core Components
- `src/main.rs` - CLI interface and command routing
- `src/monitor.rs` - Core latency monitoring engine
- `src/storage.rs` - SQLite database operations
- `src/dashboard.rs` - Web dashboard server
- `src/telemetry.rs` - LAN telemetry API server
- `src/models.rs` - Data structures and types
- `src/config.rs` - Configuration management

### Key Dependencies
- **tokio**: Async runtime
- **sqlx**: Database operations
- **axum**: Web framework
- **serde**: Serialization
- **clap**: CLI parsing
- **sysinfo**: System monitoring

## 📋 How to Contribute

### 1. Issues and Bug Reports
Before creating a new issue:
- Search existing issues to avoid duplicates
- Check the [CHANGELOG.md](CHANGELOG.md) for recent fixes
- Test with the latest version

**Bug Report Template:**
```markdown
## Bug Description
Brief description of the issue

## Steps to Reproduce
1. Step one
2. Step two
3. Expected vs actual result

## Environment
- OS: [Linux/Windows/macOS]
- Rust version: `rustc --version`
- Project version: [commit hash or tag]

## Logs
```bash
# Include relevant log output
cargo run -- status --verbose
```

### 2. Feature Requests
**Feature Request Template:**
```markdown
## Feature Description
Clear description of the proposed feature

## Use Case
Why is this feature needed?

## Implementation Ideas
Any thoughts on how this could be implemented?

## Alternatives
Other solutions you've considered
```

### 3. Pull Requests

#### Before Starting
1. **Check existing issues** - Look for related discussions
2. **Create an issue** - Discuss larger changes before implementing
3. **Fork the repo** - Create your own copy

#### Development Process
1. **Create a branch** from `main`:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** following the coding standards below

3. **Test thoroughly**:
   ```bash
   # Build and test
   cargo build --release
   cargo test
   
   # Component tests
   cargo run -- test --iterations 5
   
   # Service tests
   curl http://localhost:3030/health
   curl http://localhost:8081/health
   ```

4. **Commit with clear messages**:
   ```bash
   git commit -m "feat: add new monitoring component
   
   - Implement XYZ monitoring capability
   - Add corresponding CLI command
   - Include tests and documentation"
   ```

5. **Push and create PR**:
   ```bash
   git push origin feature/your-feature-name
   ```

#### PR Requirements
- [ ] **Tests pass**: All existing tests continue to pass
- [ ] **New tests**: Add tests for new functionality
- [ ] **Documentation**: Update README.md if needed
- [ ] **Changelog**: Add entry to CHANGELOG.md
- [ ] **No breaking changes**: Unless discussed in an issue
- [ ] **Performance**: No significant performance regressions

## 🎯 Coding Standards

### Rust Style
- Follow `rustfmt` defaults: `cargo fmt`
- Use `clippy` for linting: `cargo clippy`
- Document public APIs with `///` comments
- Use `Result<T, Error>` for error handling
- Prefer `async/await` over manual futures

### Code Organization
```rust
// File structure within modules
use std::...;
use external_crate::...;
use crate::...;

// Constants
const MAX_EVENTS: usize = 1000;

// Types and structs
#[derive(Debug, Clone, Serialize)]
pub struct MyStruct {
    pub field: String,
}

// Implementation
impl MyStruct {
    pub fn new() -> Self { ... }
    
    pub async fn async_method(&self) -> Result<()> { ... }
}
```

### Error Handling
```rust
use anyhow::{Result, Context};

pub async fn example_function() -> Result<String> {
    let data = fetch_data()
        .await
        .context("Failed to fetch data")?;
    
    Ok(data.to_string())
}
```

### Async Patterns
```rust
// Prefer structured concurrency
tokio::select! {
    result1 = task1() => handle_result1(result1),
    result2 = task2() => handle_result2(result2),
}

// Use proper error propagation
let results = futures::future::try_join_all(tasks).await?;
```

## 🧪 Testing Guidelines

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_monitoring_component() {
        let monitor = LatencyMonitor::new().await;
        let result = monitor.test_component().await;
        assert!(result.is_ok());
    }
}
```

### Integration Tests
```bash
# Test CLI commands
cargo run -- test --component vscode --iterations 1

# Test API endpoints
curl -f http://localhost:8081/health

# Test database operations
cargo run -- status --verbose
```

### Performance Tests
```bash
# Memory usage test
cargo build --release
time ./target/release/vscode-latency-monitor test --iterations 100

# Load test APIs
for i in {1..50}; do
  curl -s http://localhost:8081/api/telemetry >/dev/null &
done
```

## 📦 Release Process

### Version Numbering
- **Major** (1.x.x): Breaking changes
- **Minor** (x.1.x): New features, backward compatible
- **Patch** (x.x.1): Bug fixes, backward compatible

### Release Steps
1. **Update version** in `Cargo.toml`
2. **Update CHANGELOG.md** with new version
3. **Test release build**: `cargo build --release`
4. **Create git tag**: `git tag v1.0.1`
5. **Push tag**: `git push origin v1.0.1`
6. **Create GitHub release** with changelog notes

## 🔍 Code Review Guidelines

### For Reviewers
- **Functionality**: Does the code work as intended?
- **Performance**: No significant regressions?
- **Security**: No obvious vulnerabilities?
- **Style**: Follows project conventions?
- **Tests**: Adequate test coverage?
- **Documentation**: Clear and up-to-date?

### For Authors
- **Self-review**: Review your own PR first
- **Small PRs**: Keep changes focused and reviewable
- **Clear description**: Explain what and why
- **Responsive**: Address feedback promptly

## 🏷️ Issue Labels

- `bug`: Something isn't working
- `enhancement`: New feature or request
- `documentation`: Improvements to documentation
- `performance`: Performance-related issues
- `security`: Security-related issues
- `good first issue`: Good for newcomers
- `help wanted`: Extra attention is needed
- `question`: Further information is requested

## 🤝 Community Guidelines

### Be Respectful
- Use welcoming and inclusive language
- Respect differing viewpoints and experiences
- Focus on what's best for the community
- Show empathy towards other community members

### Communication Channels
- **Issues**: Technical discussions and bug reports
- **Pull Requests**: Code review and implementation discussion
- **Discussions**: General questions and community interaction

## 📚 Resources

### Learning Rust
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/)
- [Async Book](https://rust-lang.github.io/async-book/)

### Project-Specific
- **tokio**: [tokio.rs](https://tokio.rs/)
- **sqlx**: [docs.rs/sqlx](https://docs.rs/sqlx/)
- **axum**: [docs.rs/axum](https://docs.rs/axum/)
- **sysinfo**: [docs.rs/sysinfo](https://docs.rs/sysinfo/)

## 🆘 Getting Help

1. **Documentation**: Check README.md and project docs
2. **Issues**: Search existing issues for similar problems
3. **Discussions**: Use GitHub Discussions for questions
4. **Code**: Look at existing implementations for patterns

Thank you for contributing to the VS Code Latency Monitor! 🎉