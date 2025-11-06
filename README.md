# ROM Patcher RS

A modern, modular ROM patcher written in Rust supporting multiple patch formats.

**Current Status:** v0.3.1 | 183 Tests | Binary: 1.4MB (with RA)

## Supported Formats

- **IPS** (International Patching System) - Production Ready (20 tests)
- **BPS** (Beat Patching System) - Production Ready (28 tests)
- **UPS** (Universal Patching System) - Production Ready (26 tests)
- **APS N64** (Nintendo 64 APS Format) - Production Ready (24 tests)
- **APS GBA** (Game Boy Advance APS Format) - Production Ready (24 tests)
- **EBP** (Extended Binary Patch) - Production Ready (26 tests) - IPS + JSON metadata
- **RUP** (Rupture Patches) - Planned
- **PPF** (PlayStation Patch Format) - Planned
- **xdelta** (Generic binary diff) - Planned

## Features

### Implemented
- **Apply patches:** IPS, BPS, UPS, APS N64, APS GBA, EBP formats with automatic detection
- **Validation:** Optional CRC32 verification via --verify flag (patch integrity + source/target checksums)
- **Hashing:** CRC32 and MD5 computation
- **RetroAchievements:** Console detection + hash verification
- **Console Support:** GB/GBC/GBA, NDS, 3DS, PSX/PS2/PSP, SNES, NES, N64, Genesis, Master System, Game Gear
- **Safety:** Transactional patching with automatic rollback on error

### Planned
- RUP, PPF, xdelta format support
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
rompatchrs game.gb patch.ips

# Specify output path
rompatchrs game.gb patch.ips game-patched.gb

# With checksum verification (slower, safer - validates all CRC32 checksums)
rompatchrs game.gbc patch.bps game-patched.gbc --verify

# UPS patches
rompatchrs game.gba patch.ups game-patched.gba

# APS N64 patches (.z64/.n64/.v64)
rompatchrs game.z64 patch.aps game-patched.z64
```

The patcher automatically detects the patch format (IPS, BPS, UPS, APS, EBP) and applies it.

### EBP patches (IPS + JSON metadata)
```bash
# EBP is IPS-compatible with optional JSON metadata
rompatchrs game.sfc patch.ebp game-patched.sfc
```

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

Benchmarked on various ROM sizes (v0.2.9):

### Apply Performance

| Format | 1KB | 100KB | 1MB | 4MB | 16MB |
|--------|-----|-------|-----|-----|------|
| **IPS** | 59ns | 1.4µs | 15.8µs | 73µs | 291µs |
| **BPS** | 98ns | 1.4µs | 267µs | 1.2ms | 5.1ms |
| **UPS** | 67ns | 1.4µs | 16µs | 73.7µs | 292µs |
| **APS N64** | 139ns | 3.5µs | 572µs | 2.4ms | 9.7ms |
| **APS GBA** | 1.8µs | 57µs | 96µs | 227µs | 10ms |

### Validation Performance (constant time)

- **IPS validate:** ~18ns (magic + size check)
- **BPS validate:** ~37-46ns (magic + varint + bounds)
- **UPS validate:** ~18-29ns (magic + size check)
- **APS N64 validate:** ~63ns (magic + N64 header)
- **APS GBA validate:** ~3.8ns (magic + size check)

### Metadata Extraction (constant time)

- **BPS metadata:** ~18-20ns
- **UPS metadata:** ~10-11ns

### Binary

- **Size:** 1.4MB (optimized with LTO + strip + minreq)
- **Zero runtime dependencies** (static linking)

Note: BPS/UPS checksums are optional via --verify flag. Without verification, patching is fast. With --verify, all CRC32 checks are performed (patch + source + target). APS N64 includes optional ROM header verification (Cart ID, CRC).

## Project Stats

- **Version:** 0.3.1
- **Test Coverage:** 183 tests (20 IPS + 28 BPS + 26 UPS + 24 APS N64 + 24 APS GBA + 26 EBP + 7 RA CLI + 14 format helpers + 6 validation + 4 RA features + 4 ROM utils)
- **Code Quality:** All files under 100 lines (modular structure)
- **Build Time:** ~4s (release with LTO)
- **Binary Size:** 1.4MB (with RetroAchievements, optimized with minreq + manual JSON parser)
