![stitchr](header.svg)

[![Rust 2024](https://img.shields.io/badge/rust-1.91%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](LICENSE)
[![Tests](https://img.shields.io/badge/tests-282%20passing-success)](crates/)
[![Safe Rust](https://img.shields.io/badge/unsafe-0%25-success)](crates/)

**stitchr** is a fast, safe ROM patcher written in Rust. It applies patches to ROM files with automatic format detection and optional checksum verification.

## Installation

### One-liner (Linux/macOS)

```bash
curl -sSL https://raw.githubusercontent.com/ryugen/stitchr/main/install.sh | bash
```

### From source

```bash
git clone https://github.com/ryugen/stitchr.git
cd stitchr
cargo install --path crates/cli
```

### Pre-built binaries

Download from [Releases](https://github.com/ryugen/stitchr/releases).

## Quick Start

```bash
# Apply a patch (format auto-detected)
stitchr game.gb patch.ips

# With output path
stitchr game.gb patch.ips patched.gb

# With checksum verification
stitchr game.gba patch.bps --verify

# Check ROM against RetroAchievements
stitchr game.snes --only ra
```

## Supported Formats

| Format | Description | Typical Use |
|--------|-------------|-------------|
| **IPS** | International Patching System | SNES, GB/GBC, NES |
| **BPS** | Beat Patching System | GBA, SNES (with checksums) |
| **UPS** | Universal Patching System | GBA, NDS |
| **APS N64** | Nintendo 64 APS | N64 (.z64/.n64/.v64) |
| **APS GBA** | Game Boy Advance APS | GBA |
| **EBP** | Extended Binary Patch | IPS + JSON metadata |
| **RUP** | Rupture Patches | Multi-file with MD5 |
| **PPF** | PlayStation Patch Format | PSX, PS2 |
| **xdelta** | VCDIFF (RFC 3284) | Large files (NDS, PS2, PSP) |
| **BDF** | Binary Diff Format | BSDIFF40 compatible |

## Features

- **Auto-detection**: Identifies patch format from magic bytes
- **Verification**: Optional CRC32/Adler32/MD5 checksum validation
- **RetroAchievements**: ROM hash lookup for 15+ consoles
- **Transactional**: Atomic writes with rollback on error
- **Fast**: SIMD-optimized hashing, ~27us for 1MB ROM (IPS)

## Usage

### Basic patching

```bash
stitchr <rom> <patch> [output]
```

If no output is specified, creates `patched/<rom>.patched.<ext>`.

### Verification modes

```bash
# Verify checksums without applying
stitchr game.gbc patch.bps --only verify

# Check RetroAchievements hash
stitchr game.sfc --only ra

# Both
stitchr game.gbc patch.bps --only verify ra
```

### Verbosity

```bash
stitchr game.gb patch.ips -v      # Verbose output
stitchr game.gb patch.ips -vv     # Extra verbose
stitchr game.gb patch.ips -q      # Quiet (errors only)
```

## Architecture

```
stitchr/
├── crates/
│   ├── core/       # PatchFormat trait, error types
│   ├── formats/    # IPS, BPS, UPS, APS, EBP, RUP, PPF, xdelta, BDF
│   ├── features/   # Validation, hashing, RetroAchievements
│   └── cli/        # Binary entry point
```

Each format implements the `PatchFormat` trait:

```rust
pub trait PatchFormat {
    fn can_handle(data: &[u8]) -> bool;
    fn apply(&self, rom: &mut Vec<u8>, patch: &[u8]) -> Result<()>;
    fn metadata(patch: &[u8]) -> Result<PatchMetadata>;
    fn validate(patch: &[u8]) -> Result<()>;
}
```

## Performance

| Format | 1MB ROM | Notes |
|--------|---------|-------|
| IPS | ~27us | Simple format |
| BPS | ~335us | Includes CRC32 |
| UPS | ~27us | XOR-based |
| PPF | ~32us | PlayStation |
| xdelta | ~32ms | Complex VCDIFF |

Binary size: **1.4MB** (static, LTO-optimized)

## Development

```bash
just build    # Release build
just test     # Run 282 tests
just clippy   # Lint (strict)
just bench    # Benchmarks
```

## License

MIT OR Apache-2.0
