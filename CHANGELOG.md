# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- UPS (Universal Patching System) format implementation
- APS format implementation for N64 and GBA
- RUP (Rupture) format implementation
- PPF (PlayStation Patch Format) implementation
- xdelta format implementation
- Patch creation functionality (create patches from ROM pairs)
- SHA-1, SHA-256 hash algorithms
- Additional CLI commands (create, info, validate)

## [0.1.8] - 2025-11-05

### Added
- BPS comprehensive test suite (hardening)
  - 11 integration tests including apply, action types, validation
  - Real ROM integration test with Samurai Kid translation patch
  - Test coverage reaches parity with IPS (both have 20+ tests)
- BPS benchmark suite (apply, validate, metadata extraction)
  - Tests performance across 1KB to 1MB ROM sizes
  - Establishes baseline for future optimizations
- Total test count: 54 tests (was 36 before hardening)

### Technical
- BPS now production-ready with comprehensive test coverage
- All 54 tests passing across workspace

## [0.1.7] - 2025-11-05

### Fixed
- BPS SOURCE_READ action: Use current output position instead of source_relative_offset
  - Matches RomPatcherJS behavior (tempFile.offset is output position)
  - Fixes "SourceCopy offset out of bounds" error with real patches
- BPS TARGET_COPY action: Only validate start position, not start+length
  - Target buffer grows during RLE-style overlapping copies
  - Fixes "TargetCopy offset out of bounds" error
- BPS now works correctly with real ROM patches (tested with Samurai Kid translation)

### Added
- Expanded IPS benchmarks to test full 16MB range (IPS maximum)
  - Test sizes: 1KB, 10KB, 100KB, 1MB, 4MB, 8MB, 16MB
  - Performance: 59ns (1KB) to 304µs (16MB) with linear scaling
- Wired up BPS format in CLI apply command
  - BPS patches now functional via CLI

### Changed
- Fixed Hyprland package manager environment variables
  - Changed from $PKGMGR_CACHE expansion to hardcoded /cache paths
  - Fixes rustup/cargo detection issues

## [0.1.6] - 2025-11-05

### Added
- BPS (Beat Patching System) format support
  - Variable-length integer encoding/decoding with overflow protection
  - CRC32 validation for source, target, and patch integrity
  - Four action types: SourceRead, TargetRead, SourceCopy, TargetCopy
  - Metadata extraction with UTF-8 parsing
  - Handles RLE-style overlapping copies (TargetCopy)
- BPS module structure with subdirectories for better organization
  - apply/ subdirectory with action handlers (171 lines vs 210 previously)
  - helpers.rs for shared CRC32 and parsing functions

### Changed
- Migrated from custom CRC32 implementation to crc32fast crate (SIMD-optimized)
  - Removed 72 lines of custom CRC32 lookup table code
  - Consistent CRC32 usage across all formats (BPS, validation features)
  - Better performance through hardware acceleration on supported CPUs
- Connected MD5 algorithm to validator (was implemented but unused)

### Refactored
- IPS apply.rs split into subdirectory structure
  - apply/mod.rs (125 lines) - Main logic, validation, EOF handling
  - apply/records.rs (92 lines) - RLE and normal record handlers
  - Reduced from 212 monolithic lines to organized modules
  - No files exceed 200 lines limit
  - Consistent structure with BPS module

### Technical
- 9 BPS tests added (varint, validation, metadata extraction)
- Total code: 2784 lines
- All 36 tests passing (+ 1 ignored doctest)
- No files over 200 lines

## [0.1.5] - 2025-11-05

### Changed
- Binary size optimization: 3.9MB → 2.0MB (-48%)
- Added release profile with LTO, size optimization, and symbol stripping
- Compile time increased (more optimization passes)

### Technical
- opt-level = "z" (optimize for size)
- lto = true (link-time optimization)
- strip = true (automatic symbol stripping)
- codegen-units = 1 (better optimization)
- panic = "abort" (smaller panic handler)

## [0.1.4] - 2025-11-04

### Added
- Nintendo DS (NDS) console support for RetroAchievements
- Nintendo 3DS (3DS/CCI/CXI) console support for RetroAchievements
- File extension detection for .nds, .3ds, .cci, .cxi formats

## [0.1.3] - 2025-11-04

### Added
- MD5 hash computation tests (empty, known values, binary data)
- Console detection tests (7 tests covering all supported consoles)
- CLI library target for testing utilities

### Changed
- CLI crate now exports lib target alongside binary target

## [0.1.2] - 2025-11-04

### Changed
- Rate limiting now uses file-based storage (XDG_CACHE_HOME) instead of in-process mutex
- Rate limiting now works across multiple process invocations (multiple rompatch calls)

### Fixed
- Rate limiter now properly prevents API spam when user runs multiple patches quickly

## [0.1.1] - 2025-11-04

### Added
- MD5 hash computation for ROM validation
- RetroAchievements API integration
- Automatic game lookup by MD5 hash after patching
- Rate limiting (500ms) for RA API requests
- Console detection from file extension

### Changed
- Split hash algorithms into separate modules (md5, crc32)
- Made retroachievements feature optional (enabled with `--features retroachievements`)

### Technical
- New dependencies: md5, ureq, serde, serde_json (optional)
- Modular API client in `features/retroachievements/api.rs`
- CLI integration in `cli/utils/retroachievements.rs`

## [0.1.0-patch.1] - 2025-11-04

### Changed
- Removed all emoji characters from code and documentation
- Fixed clippy warning in IPS tests (use iterator pattern instead of range indexing)

## [0.1.0] - 2025-11-04

### Added
- Initial project structure with Cargo workspace (4 crates)
- **IPS Format** - FULLY IMPLEMENTED:
  - Patch application with RLE support
  - Truncation support
  - Validation and metadata extraction
  - Binary I/O helpers
- **CRC32 validation** with lookup table optimization
- **CLI `apply` command**:
  - Automatic format detection
  - Transactional safety
  - CRC32 checksums for input/patch/output
- Format stubs for future implementation (BPS, UPS, APS, RUP, PPF, xdelta)
- 17 integration tests for IPS format
- Criterion benchmarks
- CI/CD with GitHub Actions
- Documentation (README, ARCHITECTURE, API docs)

### Performance
- IPS patch application: ~16 µs (1MB ROM)
- CRC32 computation: ~3.3 µs (1MB ROM)

### Technical Details
- Rust Edition: 2024
- Minimum Rust Version: 1.91
- License: MIT OR Apache-2.0
- No unsafe code

### Known Limitations
- Only IPS format fully implemented
- Only CRC32 hash algorithm implemented
- No patch creation yet
- Maximum ROM size: 16MB (IPS limitation)
