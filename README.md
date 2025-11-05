# ROM Patcher RS

A modern, modular ROM patcher written in Rust supporting multiple patch formats.

**Current Status:** v0.1.9 | 60 Tests | Binary: 588KB (stripped)

## Supported Formats

- **IPS** (International Patching System) - Production Ready (17 tests)
- **BPS** (Beat Patching System) - Production Ready (17 tests, v0.1.9)
- **UPS** (Universal Patching System) - Planned
- **APS** (Nintendo 64 APS Format) - Planned
- **RUP** (Rupture Patches) - Planned
- **PPF** (PlayStation Patch Format) - Planned
- **xdelta** (Generic binary diff) - Planned

## Features

### Implemented
- **Apply patches:** IPS, BPS formats with automatic detection
- **Validation:** Optional CRC32 verification via --verify flag (patch integrity + source/target checksums)
- **Hashing:** CRC32 and MD5 computation
- **RetroAchievements:** Console detection + hash verification
- **Console Support:** GB/GBC/GBA, NDS, 3DS, PSX/PS2/PSP, SNES, NES, N64, Genesis, Master System, Game Gear
- **Safety:** Transactional patching with automatic rollback on error

### Planned
- UPS, APS, RUP, PPF, xdelta format support
- SHA-1, SHA-256 checksums
- Additional output options and verbosity controls

## Architecture

The project is organized as a Cargo workspace with 4 crates:

```
rom-patcher-rs/
├── crates/
│   ├── core/           # Core traits and types
│   ├── formats/        # Patch format implementations
│   ├── features/       # Extended features (validation, hash checking)
│   └── cli/            # Command-line interface
```

### Design Principles

1. **Modular** - Each format and feature is independently implemented
2. **Extensible** - Easy to add new formats via trait implementation
3. **Type-safe** - Leverages Rust's type system for safety
4. **Zero-copy** - Efficient memory usage with slice references
5. **Performance** - SIMD-optimized CRC32 (crc32fast), ~16µs per 1MB ROM
6. **Clean Code** - No file exceeds 200 lines, subdirectory organization

## Building

Requires Rust 1.91+ with 2024 edition support:

```bash
cargo build --release
```

The binary will be at `target/release/rompatchrs`.

## Usage

### Apply a patch

```bash
# Basic usage (auto-generates output path)
rompatchrs game.smc hack.ips

# Specify output path
rompatchrs game.smc hack.ips game-patched.smc

# With checksum verification (slower, safer - validates all CRC32 checksums)
rompatchrs game.smc hack.bps game-patched.smc --verify
```

The patcher automatically detects the patch format (IPS, BPS) and applies it.

## Development

### Prerequisites

Install development tools:

```bash
cargo install just cargo-watch cargo-audit cargo-outdated cargo-tarpaulin
```

### Common Tasks

```bash
just              # Show all available commands
just build        # Build in release mode
just test         # Run all tests
just clippy       # Run linter (warnings as errors)
just fmt          # Format code
just bench        # Run benchmarks
just ci           # Run all CI checks
just doc          # Generate and open documentation
```

### Adding a New Format

1. Create a new module in `crates/formats/src/`
2. Implement the `PatchFormat` trait
3. Add format detection in `detect_format()`
4. Add CLI support in `crates/cli/src/commands/apply.rs`
5. Follow test structure from [docs/TEST_TEMPLATE.md](docs/TEST_TEMPLATE.md)
6. Add benchmarks following IPS/BPS patterns

### Testing

```bash
just test                 # Run all tests
just watch-test           # Watch and run tests on changes
cargo test --all-features # Direct cargo command
```

See [docs/TEST_TEMPLATE.md](docs/TEST_TEMPLATE.md) for test structure guidelines.

## Documentation

- [CHANGELOG.md](CHANGELOG.md) - Version history and release notes
- [ARCHITECTURE.md](ARCHITECTURE.md) - Detailed design documentation
- [docs/TEST_TEMPLATE.md](docs/TEST_TEMPLATE.md) - Test structure guidelines
- [docs/API.md](docs/API.md) - Complete API reference
- [docs/CLI_USAGE.md](docs/CLI_USAGE.md) - CLI usage guide
- [examples/](examples/) - Code examples

## License

MIT OR Apache-2.0

## Performance

Benchmarked on various ROM sizes:

- **IPS apply (16MB):** 304 µs
- **BPS apply (1MB):** 268 µs (30% faster with optional verification)
- **BPS apply (16MB):** 4.8 ms
- **BPS validate:** 42 ns (constant time - patch CRC32 only)
- **Binary size:** 588KB (optimized with LTO + strip)
- **Zero runtime dependencies** (static linking)

Note: BPS checksums are optional via --verify flag. Without verification, BPS is fast. With --verify, all 3 CRC32 checks are performed (patch + source + target).

## Project Stats

- **Version:** 0.1.9
- **Test Coverage:** 60 tests (17 IPS + 17 BPS + 7 RA + others)
- **Code Quality:** All files under 200 lines
- **Build Time:** ~4s (release with LTO)
