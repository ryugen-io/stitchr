# Just commands for rom-patcher-rs
# Install just: cargo install just
# Run: just <command>

# Show all available commands
default:
    @just --list

# Build all crates in release mode
build:
    cargo build --release --all-features

# Run all tests
test:
    cargo test --all-features

# Check all crates
check:
    cargo check --all-features

# Run clippy with all warnings as errors
clippy:
    cargo clippy --all-features --all-targets -- -D warnings

# Format all code
fmt:
    cargo fmt --all

# Check formatting without modifying files
fmt-check:
    cargo fmt --all -- --check

# Run benchmarks
bench:
    cargo bench --all-features

# Clean build artifacts
clean:
    cargo clean

# Install the binary
install:
    cargo install --path crates/cli --locked

# Run all CI checks (fmt-check, clippy, test)
ci: fmt-check clippy test
    @echo "All CI checks passed"

# Update dependencies
update:
    cargo update

# Check for outdated dependencies
outdated:
    cargo outdated

# Run cargo audit for security vulnerabilities
audit:
    cargo audit

# Generate documentation
doc:
    cargo doc --all-features --no-deps --open

# Run tests with coverage (requires cargo-tarpaulin)
coverage:
    cargo tarpaulin --all-features --workspace --timeout 300 --out html

# Watch for changes and run tests
watch-test:
    cargo watch -x "test --all-features"

# Watch for changes and run checks
watch-check:
    cargo watch -x "check --all-features"
