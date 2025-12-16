# CLAUDE.md - AI Assistant Guide for stitchr

This document provides comprehensive guidance for AI assistants working with the stitchr codebase.

## Project Overview

**Project Name**: stitchr (binary: `stitchr`)
**Type**: Rust CLI application with library crates
**Purpose**: Modern, modular ROM patcher supporting multiple patch formats
**Version**: 0.4.3
**Rust Edition**: 2024 (MSRV: 1.91+)
**License**: MIT OR Apache-2.0

### Key Characteristics
- **100% Safe Rust**: Zero unsafe code
- **Production-ready**: 219 tests, comprehensive validation
- **Size-optimized**: 1.4MB binary with zero runtime dependencies
- **Modular**: 4-crate workspace architecture
- **Extensible**: Trait-based plugin system for formats

### Supported Patch Formats
- **IPS** (International Patching System) - 20 tests
- **BPS** (Beat Patching System) - 28 tests
- **UPS** (Universal Patching System) - 26 tests
- **APS N64** (Nintendo 64 APS) - 24 tests
- **APS GBA** (Game Boy Advance APS) - 24 tests
- **EBP** (Extended Binary Patch) - 26 tests
- **RUP** (Rupture Patches) - 27 tests
- **PPF** (PlayStation Patch Format) - Planned
- **xdelta** (Generic binary diff) - Planned

## Codebase Structure

### Workspace Architecture

```
/home/user/stitchr/
├── Cargo.toml              # Workspace root
├── justfile                # Task automation (use `just <command>`)
├── clippy.toml             # Linter configuration
├── rustfmt.toml            # Code formatting rules
├── deny.toml               # Dependency auditing
├── .skip-ci                # CI skip marker
│
├── crates/                 # Cargo workspace members
│   ├── core/               # Layer 1: Core traits and types
│   ├── formats/            # Layer 2: Format implementations
│   ├── features/           # Layer 3: Optional features
│   └── cli/                # Layer 4: CLI interface
│
├── examples/               # Usage examples
├── test_files/             # Test patches and ROMs
│
├── README.md               # User documentation
├── CONTRIBUTING.md         # Development guidelines
└── CHANGELOG.md            # Version history
```

### Layer-Based Architecture

**Layer 1 - Core** (`crates/core/`):
- File: `crates/core/src/lib.rs`
- Defines `PatchFormat` trait
- Defines `PatchType` enum and `PatchMetadata` struct
- Defines `PatchError` enum
- Zero dependencies except `thiserror`

**Layer 2 - Formats** (`crates/formats/`):
- File: `crates/formats/src/lib.rs`
- Implements `PatchFormat` for each format
- Auto-detection via `detect_format()`
- Each format in separate module (e.g., `src/ips/`, `src/bps/`)

**Layer 3 - Features** (`crates/features/`):
- File: `crates/features/src/lib.rs`
- Optional validation (MD5, CRC32)
- RetroAchievements integration
- Feature-gated with Cargo features

**Layer 4 - CLI** (`crates/cli/`):
- File: `crates/cli/src/main.rs` (entry point)
- File: `crates/cli/src/lib.rs` (library)
- Command handlers in `src/commands/`
- Dispatches to format implementations
- Transactional safety with rollback

### Key File Locations

**Entry Points**:
- CLI: `/home/user/stitchr/crates/cli/src/main.rs`
- Core: `/home/user/stitchr/crates/core/src/lib.rs`
- Formats: `/home/user/stitchr/crates/formats/src/lib.rs`

**Core Trait**:
- `/home/user/stitchr/crates/core/src/format.rs` - `PatchFormat` trait

**Format Implementations**:
- IPS: `/home/user/stitchr/crates/formats/src/ips/`
- BPS: `/home/user/stitchr/crates/formats/src/bps/`
- UPS: `/home/user/stitchr/crates/formats/src/ups/`
- APS: `/home/user/stitchr/crates/formats/src/aps/`
- EBP: `/home/user/stitchr/crates/formats/src/ebp/`
- RUP: `/home/user/stitchr/crates/formats/src/rup/`

**Dispatchers**:
- `/home/user/stitchr/crates/cli/src/commands/dispatch.rs`
- `/home/user/stitchr/crates/cli/src/commands/verify.rs`

**Templates**:
- `/home/user/stitchr/crates/formats/FORMAT_TEMPLATE/`

## Core Design Patterns

### PatchFormat Trait

The `PatchFormat` trait is the core abstraction (see `crates/core/src/format.rs`):

```rust
pub trait PatchFormat: Send + Sync {
    fn can_handle(data: &[u8]) -> bool;              // Format detection
    fn apply(&self, rom: &mut Vec<u8>, patch: &[u8]) -> Result<()>;  // Apply patch
    fn metadata(patch: &[u8]) -> Result<PatchMetadata>;  // Extract metadata
    fn validate(patch: &[u8]) -> Result<()>;         // Validate integrity
    fn verify(rom: &[u8], patch: &[u8], target: Option<&[u8]>) -> Result<()>;  // Checksum
}
```

### Standard Format Structure

Every format MUST follow this structure (see `FORMAT_TEMPLATE/README.md`):

```
src/FORMAT_NAME/
├── mod.rs              # Trait implementation + exports
├── constants.rs        # Magic bytes, sizes, action codes
├── validate.rs         # can_handle() + validate() + verify()
├── metadata.rs         # Metadata extraction
├── helpers.rs          # CRC validation, parsing utilities
├── varint.rs           # Variable-length encoding (if needed)
└── apply/
    └── mod.rs          # Patch application logic
```

**CRITICAL RULE**: No file over 200 lines. Use subdirectories for complex modules.

### Format Detection Pattern

Order matters in `detect_format()` (see `crates/formats/src/lib.rs`):

```rust
pub fn detect_format(data: &[u8]) -> Option<PatchType> {
    // EBP before IPS (both use "PATCH" magic)
    if EbpPatcher::can_handle(data) { return Some(PatchType::Ebp); }
    if IpsPatcher::can_handle(data) { return Some(PatchType::Ips); }
    // ... other formats
}
```

### Error Handling

**Core Errors** (from `crates/core/src/error.rs`):
- `InvalidFormat` - Wrong patch format
- `CorruptedData` - Malformed patch data
- `ChecksumMismatch` - Verification failed
- `SizeMismatch` - Wrong ROM size
- `OutOfBounds` - Invalid offset in patch
- `UnsupportedVersion` - Unsupported format version

**CLI Errors**: Use `anyhow` for user-friendly context

### Transactional Safety

All patch operations are transactional (see `crates/cli/src/commands/apply/mod.rs`):
1. Clone ROM data
2. Apply patch to clone
3. Verify checksums (if `--verify`)
4. Write to temporary file
5. Atomic rename to final location

## Development Workflows

### Common Commands (via justfile)

```bash
just                # List all commands
just build          # Release build
just test           # Run all tests
just clippy         # Lint (warnings as errors)
just fmt            # Format code
just bench          # Run benchmarks
just ci             # All CI checks (fmt + clippy + test)
just doc            # Generate docs
just install        # Install binary
```

### Direct Cargo Commands

```bash
cargo build --release --all-features
cargo test --workspace --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all
cargo bench --all-features
```

### Testing Commands

```bash
just test                           # All tests
just watch-test                     # Watch mode
cargo test --workspace              # Direct cargo
cargo test --test ips_integration   # Specific format
```

### Pre-Commit Checklist

Before committing:
1. `cargo test --workspace` - All tests pass
2. `cargo clippy --all-targets --all-features -- -D warnings` - No warnings
3. `cargo fmt --all` - Code formatted
4. Update `CHANGELOG.md` with changes
5. **NO EMOJI CHARACTERS** in code or documentation

## Key Conventions and Rules

### CRITICAL RULES (Strictly Enforced)

1. **NO EMOJI CHARACTERS**: Absolutely forbidden in all code and documentation
2. **One Commit Per File**: Each file modification = separate commit
3. **No Files Over 200 Lines**: Split into subdirectories if needed
4. **No Unsafe Rust**: 100% safe code only
5. **No Encoding**: Only decode/apply patches, never create them

### Naming Conventions

**Crate Names** (hyphenated):
- `stitchr-core`
- `stitchr-formats`
- `stitchr-features`
- `stitchr-cli`

**Binary Name** (no hyphens):
- `stitchr`

**Module Names** (snake_case):
- `apply_patch`, `validation`, `retroachievements`

**Type Names** (PascalCase):
- `PatchFormat`, `PatchType`, `IpsPatcher`

### Code Style

From `rustfmt.toml`:
- Max line width: 100 characters
- 4 spaces indentation
- Unix line endings
- Auto-reorder imports
- Wrap comments at 80 characters

From `CONTRIBUTING.md`:
- Use Rust idioms (iterators over index loops)
- Add inline docs for complex algorithms
- Keep functions focused and small
- Write tests for all new functionality

### Testing Standards

**Minimum Tests Per Format**:
- 17+ tests total
- 6+ validation tests
- 3+ metadata tests
- 8+ apply tests
- 3+ checksum validation tests (with real patches)

**Test Organization**:
```
tests/FORMAT_NAME/
├── mod.rs
├── validate_tests.rs           # Format validation
├── metadata_tests.rs            # Metadata extraction
├── apply_tests.rs              # Basic application
└── checksum_validation_tests.rs # Integration with real patches
```

### Fuzzing Guidelines

**Fuzzing Infrastructure**:
- Located in `fuzz/` directory.
- Use `just fuzz <target>` to run.
- Targets: `fuzz_detect`, `fuzz_ips`, `fuzz_bps`, etc.

**Security & Stability**:
- **Integer Overflows**: Always use `checked_add`, `checked_mul`, etc. for offsets and sizes.
- **Allocation Limits**: Always verify `target_size` against a sane limit (e.g. `MAX_TARGET_SIZE`) before allocation to avoid OOM/ASAN crashes.
- **Incremental Growth**: Check bounds/limits when extending vectors incrementally (e.g. `check_growth` pattern).

## Git Workflow

### Branch Naming

```bash
feature/add-bps-format       # New features
fix/ips-rle-overflow         # Bug fixes
docs/update-api-reference    # Documentation
refactor/simplify-validation # Refactoring
```

### One Commit Per File Strategy

**CRITICAL**: Each file gets its own commit:

```bash
# Example workflow
git add crates/formats/src/bps/mod.rs
git commit -m "Add BPS format stub implementation"

git add crates/formats/src/bps/validate.rs
git commit -m "Add BPS validation logic"

git add crates/formats/tests/bps/validate_tests.rs
git commit -m "Add BPS validation tests"

git add CHANGELOG.md
git commit -m "Update CHANGELOG for BPS format addition"
```

### Good Commit Messages

```
Add BPS format validation
Fix IPS RLE record handling for edge cases
Update API documentation for PatchFormat trait
Refactor CRC32 computation for better performance
```

### Push to Branch

For this session, push to:
```bash
git push -u origin claude/create-codebase-documentation-01W52m24JscGaKHxFCPSXwe6
```

## Adding a New Patch Format

Follow this checklist (see `FORMAT_TEMPLATE/README.md`):

### Step 1: Create File Structure

```bash
# Create format module
mkdir -p crates/formats/src/FORMAT_NAME/apply
mkdir -p crates/formats/tests/FORMAT_NAME

# Create source files
touch crates/formats/src/FORMAT_NAME/mod.rs
touch crates/formats/src/FORMAT_NAME/constants.rs
touch crates/formats/src/FORMAT_NAME/validate.rs
touch crates/formats/src/FORMAT_NAME/metadata.rs
touch crates/formats/src/FORMAT_NAME/helpers.rs
touch crates/formats/src/FORMAT_NAME/apply/mod.rs

# Create test files
touch crates/formats/tests/FORMAT_NAME/mod.rs
touch crates/formats/tests/FORMAT_NAME/validate_tests.rs
touch crates/formats/tests/FORMAT_NAME/metadata_tests.rs
touch crates/formats/tests/FORMAT_NAME/apply_tests.rs
touch crates/formats/tests/FORMAT_NAME/checksum_validation_tests.rs
```

### Step 2: Implement Core Files

1. **constants.rs**: Define magic bytes, sizes, action codes
2. **helpers.rs**: CRC validation, header parsing
3. **validate.rs**: `can_handle()`, `validate()`, `verify()` implementations
4. **metadata.rs**: `metadata()` implementation
5. **apply/mod.rs**: `apply()` implementation
6. **mod.rs**: Struct definition + `PatchFormat` trait implementation

### Step 3: Add Tests

Minimum 17 tests:
- 6+ validation tests
- 3+ metadata tests
- 8+ apply tests
- 3+ checksum validation tests with real patches

### Step 4: Integration

1. Add to `crates/formats/src/lib.rs` - Export module
2. Add to `detect_format()` in `crates/formats/src/lib.rs`
3. Add to `crates/formats/Cargo.toml` - Feature flag
4. Add to `crates/cli/src/commands/dispatch.rs` - Apply dispatch
5. Add to `crates/cli/src/commands/verify.rs` - Verify dispatch
6. Create `crates/formats/benches/FORMAT_NAME_bench.rs`

### Step 5: Documentation

1. Update `README.md` - Add to supported formats
2. Update `CHANGELOG.md` - Document addition
3. Add doc comments to public API

### Step 6: Commit (One File at a Time)

```bash
git add crates/formats/src/FORMAT_NAME/constants.rs
git commit -m "Add FORMAT_NAME format constants"

git add crates/formats/src/FORMAT_NAME/validate.rs
git commit -m "Add FORMAT_NAME validation logic"
# ... etc for each file
```

## Important Configuration Files

### Cargo.toml (Workspace Root)

Location: `/home/user/stitchr/Cargo.toml`

**Key Settings**:
- Edition: 2024
- MSRV: 1.91
- Release profile: size-optimized (opt-level="z", lto=true, strip=true)

### clippy.toml

Location: `/home/user/stitchr/clippy.toml`

**Key Settings**:
- MSRV: 1.91
- All warnings become errors in CI

### rustfmt.toml

Location: `/home/user/stitchr/rustfmt.toml`

**Key Settings**:
- Edition: 2024
- Max width: 100
- Tab spaces: 4
- Reorder imports: true
- Comment width: 80

### deny.toml

Location: `/home/user/stitchr/deny.toml`

**Purpose**: Cargo-deny configuration for:
- License validation (MIT, Apache-2.0, MPL-2.0, ISC, Unicode-3.0)
- Security advisories
- Dependency banning
- Source verification

## Feature Flags

### Formats Crate

```toml
default = ["ips", "bps", "ups", "aps", "ebp", "rup", "ppf", "xdelta"]
ips = []
bps = []
ups = []
aps = []
ebp = []
rup = []
ppf = []
xdelta = []
```

### Features Crate

```toml
default = ["validation"]
validation = ["md5"]
retroachievements = ["md5", "minreq"]
```

### CLI Crate

```toml
default = ["validation", "retroachievements"]
validation = ["stitchr-features/validation"]
retroachievements = ["stitchr-features/retroachievements"]
```

## Dependencies

### Workspace-Level
- `thiserror = "2.0"` - Error derive macros
- `anyhow = "1.0"` - Error handling

### Core
- `thiserror = "2.0"`

### Formats
- `stitchr-core` (workspace)
- `thiserror = "2.0"`
- `crc32fast = "1.5"` - SIMD-optimized CRC32
- `crc16 = "0.4"` - CRC16 for UPS
- `md5 = "0.7"` - MD5 hashing

### Features
- `stitchr-core` (workspace)
- `thiserror = "2.0"`
- `crc32fast = "1.5"`
- `md5 = "0.7"` (optional, feature-gated)
- `minreq = "2.12"` (optional, for RetroAchievements)

### CLI
- `stitchr-core` (workspace)
- `stitchr-formats` (workspace)
- `stitchr-features` (workspace)
- `anyhow = "1.0"`
- `clap = "4.5"` - CLI parsing with derive

## Performance Characteristics

### Binary Size
- **Target**: 1.4MB
- **Method**: LTO + strip + opt-level="z" + panic="abort"

### Apply Performance (Benchmarks)
- IPS 1MB: ~15.8µs
- BPS 1MB: ~267µs (includes CRC32 verification)
- UPS 1MB: ~16µs
- APS N64 1MB: ~572µs
- RUP 1MB: ~3.1ms (includes MD5 hashing)

### Validation Performance
- IPS: ~18ns (constant time)
- BPS: ~37-46ns (constant time)
- UPS: ~18-29ns (constant time)

## AI Assistant Guidelines

### When Exploring Code

1. **Use specific file paths**: Always reference files with full paths from `/home/user/stitchr/`
2. **Understand layers**: Know which layer (core/formats/features/cli) you're working in
3. **Check existing patterns**: Look at IPS, BPS, or UPS implementations as examples
4. **Follow templates**: Use `FORMAT_TEMPLATE/` for new formats

### When Making Changes

1. **One commit per file**: Never batch multiple files in one commit
2. **Update CHANGELOG.md**: Always document changes
3. **Run tests**: Use `just test` before committing
4. **Check formatting**: Use `just fmt` and `just clippy`
5. **No emoji**: Remove any emoji characters immediately

### When Adding Features

1. **Start with tests**: Write failing tests first
2. **Follow structure**: Use standard file organization
3. **Add benchmarks**: Create benchmark file for new formats
4. **Update docs**: README.md, CHANGELOG.md, inline docs
5. **Feature flag**: Consider if it should be optional

### When Fixing Bugs

1. **Write regression test**: Add test that reproduces the bug
2. **Fix in layers**: Start from core and work up to CLI
3. **Verify checksums**: Ensure fixes don't break validation
4. **Update tests**: Adjust test expectations if needed

### When Writing Documentation

1. **No emoji**: Strictly forbidden
2. **Be specific**: Include file paths and line numbers
3. **Show examples**: Provide code snippets
4. **Link to sources**: Reference existing implementations

### Common Pitfalls to Avoid

1. **Don't use emoji** - Will be rejected
2. **Don't create encoding functions** - Only decoding/applying
3. **Don't exceed 200 lines** - Split files into subdirectories
4. **Don't batch commits** - One file per commit
5. **Don't skip tests** - Minimum 17 per format
6. **Don't add unsafe code** - 100% safe Rust only

### Useful References

**Example Implementations**:
- Simple format: `/home/user/stitchr/crates/formats/src/ips/`
- Complex format: `/home/user/stitchr/crates/formats/src/bps/`
- With JSON: `/home/user/stitchr/crates/formats/src/ebp/`
- With MD5: `/home/user/stitchr/crates/formats/src/rup/`

**Templates**:
- Format template: `/home/user/stitchr/crates/formats/FORMAT_TEMPLATE/`

**Documentation**:
- User guide: `/home/user/stitchr/README.md`
- Contributing: `/home/user/stitchr/CONTRIBUTING.md`
- Changelog: `/home/user/stitchr/CHANGELOG.md`

## Quick Reference Commands

### Build & Test
```bash
just build          # Release build
just test           # All tests
just ci             # Full CI checks
just bench          # Benchmarks
```

### Code Quality
```bash
just fmt            # Format code
just clippy         # Lint (strict)
just deny           # Check dependencies
```

### Development
```bash
just watch-test     # Watch mode
just doc            # Generate docs
just install        # Install binary
```

### Analysis
```bash
just coverage       # Test coverage
just bloat          # Binary size analysis
just geiger         # Unsafe code audit
```

## Project Statistics

- **Version**: 0.4.3
- **Total Tests**: 219
- **Total Files**: 201
- **Total Lines**: ~8,750
- **Unsafe Code**: 0%
- **Binary Size**: 1.4MB
- **Build Time**: ~4s (release with LTO)
- **MSRV**: Rust 1.91+
- **Supported Formats**: 7 production-ready

## License

This project is dual-licensed under MIT OR Apache-2.0.

---

**Last Updated**: 2025-11-15
**For Questions**: See README.md and CONTRIBUTING.md
