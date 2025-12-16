# Contributing to stitchr

## Development Workflow

### Branch Strategy

For every change, create a dedicated branch:

```bash
# Feature branches
git checkout -b feature/add-bps-format
git checkout -b feature/add-md5-hashing

# Bug fix branches
git checkout -b fix/ips-rle-overflow
git checkout -b fix/cli-error-message

# Documentation branches
git checkout -b docs/update-api-reference
git checkout -b docs/add-examples

# Refactoring branches
git checkout -b refactor/simplify-validation
git checkout -b refactor/extract-helpers
```

### Development Checklist

Before committing changes:

1. Run tests: `cargo test --workspace`
2. Run clippy: `cargo clippy --all-targets --all-features -- -D warnings`
3. Format code: `cargo fmt --all`
4. Update CHANGELOG.md with changes
5. Ensure no emoji characters in code or documentation

### Testing

Run all checks:

```bash
cargo test --workspace
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all --check
cargo bench --no-run
```

### Commit Messages

Use clear, descriptive commit messages.

**Commit Strategy: One commit per file**

Each file modification should be committed separately:

```bash
git add crates/formats/src/bps/mod.rs
git commit -m "Add BPS format stub implementation"

git add crates/formats/src/bps/validate.rs
git commit -m "Add BPS validation logic"

git add crates/formats/tests/bps/validate_tests.rs
git commit -m "Add BPS validation tests"

git add CHANGELOG.md
git commit -m "Update CHANGELOG for BPS format addition"
```

Good commit message examples:
```
Add BPS format validation
Fix IPS RLE record handling for edge cases
Update API documentation for PatchFormat trait
Refactor CRC32 computation for better performance
Remove emoji characters from examples
```

## Code Style

- No emoji characters in code or documentation
- Follow Rust idioms (use iterators over index loops)
- Add inline documentation for complex algorithms
- Keep functions focused and small
- Write tests for new functionality

## Project Structure

See [ARCHITECTURE.md](ARCHITECTURE.md) for detailed architecture documentation.
