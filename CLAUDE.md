# Claude Code - Project Instructions

## Build Process

### Release Builds

Always use `cargo auditable build --release` for release builds.

This embeds dependency information in the binary for supply-chain security auditing.

```bash
# Build release binary with auditable info
cargo auditable build --release

# Verify auditable info is present
cargo audit bin target/release/rompatchrs
```

The binary should show: `Found 'cargo auditable' data in target/release/rompatchrs (41 dependencies)`

### Benefits

- Enables security scanning of deployed binaries
- No separate SBOM file needed
- Minimal overhead (~1KB compressed JSON)
- Compliance and audit trail built-in

## Development Tools

### Security and Compliance

#### cargo-deny
License and security advisory checking.

```bash
# Run all checks (licenses, advisories, bans, sources)
cargo deny check

# Check only licenses
cargo deny check licenses

# Check only security advisories
cargo deny check advisories
```

Configuration in `deny.toml`. Allowed licenses: MIT, Apache-2.0, MPL-2.0, ISC, Unicode-3.0.

#### cargo-geiger
Unsafe code audit - measures unsafe code usage.

```bash
# Run from crates/cli (workspace root doesn't work)
cd crates/cli && cargo geiger
```

Project status: All rom-patcher crates are 0% unsafe. Only dependencies use unsafe.

### Binary Analysis

#### cargo-bloat
Analyze what takes space in the binary.

```bash
# Show top 25 functions by size
cargo bloat --release -n 25
```

Current binary: 70% of .text section is rustls/ring (HTTPS crypto for RetroAchievements).

### API Stability

#### cargo-semver-checks
Check for breaking API changes between versions.

```bash
# Compare against previous git commit
cargo semver-checks --baseline-rev COMMIT_HASH

# Example: compare v0.2.8-patch.8 to current
cargo semver-checks --baseline-rev 4544ad7
```

Use before bumping version to verify semver compliance.

### Performance Profiling

#### cargo-flamegraph
Generate flamegraph for performance analysis.

```bash
# Profile applying a patch (without RA to avoid rate limits)
cargo flamegraph --bin rompatchrs -- test_files/bps/rom.gb test_files/bps/patch.bps

# Output: flamegraph.svg
```

Note: Use local test files without RetroAchievements checks to avoid API rate limits.
