# Just commands for rom-patcher-rs
# Install just: cargo install just
# Run: just <command>

# Show all available commands
default:
    @just --list

# Build all crates in release mode
build:
    cargo build --release --all-features

# Build release with auditable info (for security auditing)
build-auditable:
    cargo auditable build --release

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

# Run cargo-deny checks (licenses, advisories, bans, sources)
deny:
    cargo deny check

# Check only security advisories
deny-advisories:
    cargo deny check advisories

# Check only licenses
deny-licenses:
    cargo deny check licenses

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

# Audit unsafe code usage (run from crates/cli)
geiger:
    cd crates/cli && cargo geiger

# Analyze binary size (top 25 functions)
bloat:
    cargo bloat --release -n 25

# Check for API breaking changes (requires baseline commit hash)
semver-check BASELINE:
    cargo semver-checks --baseline-rev {{BASELINE}}

# Generate flamegraph for performance profiling
flamegraph ROM PATCH:
    cargo flamegraph --bin rompatchrs -- {{ROM}} {{PATCH}}
