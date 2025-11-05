# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- APS format implementation for N64 and GBA
- RUP (Rupture) format implementation
- PPF (PlayStation Patch Format) implementation
- xdelta format implementation
- SHA-1, SHA-256 hash algorithms
- Additional CLI commands (info, validate)

## [0.2.3] - 2025-11-05

### Changed
- Binary size optimization: 2.0MB → 1.4MB (30% reduction)
  - Replaced ureq with minreq (lighter HTTP client with rustls)
  - Replaced serde_json with optimized manual JSON parser (35 lines)
  - Zero allocations in JSON parsing (byte-based digit scanning)
- Refactored RetroAchievements module structure
  - Split api.rs (151 → 87 lines)
  - New parser.rs module (37 lines, #[inline] optimized)
  - Moved tests to integration test file (27 lines)
  - All files under 200 lines, matches IPS/BPS/UPS patterns

### Technical
- Manual JSON parser with byte-level scanning for performance
- Direct string slice parsing without UTF-8 conversion overhead
- Modular structure improves maintainability

## [0.2.2] - 2025-11-05

### Fixed
- RetroAchievements now enabled by default (was incorrectly opt-in)
  - Every patched ROM now automatically gets RA hash lookup
  - Rate limiting (500ms) prevents API spam
  - Binary size: 2.0MB (includes ureq HTTP client + serde_json)

## [0.2.1] - 2025-11-05

### Changed
- UPS refactored to modular structure (apply/, metadata.rs, validate.rs)
  - Matches IPS/BPS architecture exactly
  - All files under 200 lines
  - Improved maintainability
- Added FORMAT_TEMPLATE directory with standard structure for new formats
  - Complete implementation guide
  - Template files for all modules
  - Ensures consistency across all format implementations

### Added
- UPS benchmark configuration in Cargo.toml
- UPS checksum_validation_tests.rs (20 total tests, parity with IPS/BPS)

## [0.2.0] - 2025-11-05

### Added
- UPS (Universal Patching System) format implementation
  - Variable-Length Value (VLV) encoding for sizes and offsets
  - XOR-based patching algorithm
  - CRC32 validation for patch, input ROM, and output ROM
  - Support for ROM resizing during patch application
  - Metadata extraction (input size, output size, checksums)
  - Full --verify flag support for optional checksum validation
- Modular CLI architecture for scalability
  - New dispatch.rs module for format routing (33 lines)
  - New paths.rs utility module for output path generation (30 lines)
  - Reduced apply.rs from 175 to 120 lines

### Technical
- UPS module structure: constants, varint, helpers, patcher
- All UPS files under 200 lines (decode-only, no patch creation)
- 17 UPS integration tests (parity with IPS/BPS)
- Tested with real 32MB ROM and 1.5MB UPS patch
- Architecture now supports adding future formats without bloating core files

## [0.1.9] - 2025-11-05

### Added
- Optional checksum verification via `--verify` flag
  - Validates patch integrity (patch CRC32)
  - Verifies source ROM checksum before patching
  - Verifies target ROM checksum after patching
  - Modular design: verify() method in PatchFormat trait with default no-op
  - Separate CLI verify module for clean separation of concerns

### Changed
- BPS apply() no longer performs CRC32 validation by default (huge performance gain)
  - Removed source ROM CRC32 check from apply()
  - Removed target ROM CRC32 check from apply()
  - Use `--verify` flag for full validation (all 3 CRC32 checks)
- Performance improvements from optional verification:
  - BPS @ 1MB: 268µs (was 382µs with CRC32) = **30% faster**
  - Apply now only does patch logic, no checksums
  - RomPatcherJS-compatible behavior (checksums optional)

### Technical
- Added verify() trait method to PatchFormat (default implementation = no-op)
- BPS implements verify() for source/target CRC32 validation
- Patch CRC32 validation remains in validate() (patch integrity check)
- CLI verify module handles all 3 CRC32 checks when --verify flag used
- Benchmark configuration improved (10s measurement time for large files)

## [0.1.8-patch.5] - 2025-11-05

### Changed
- BPS performance optimizations
  - Added #[inline] attributes to hot path functions (varint::decode, action handlers)
  - Optimized TARGET_COPY RLE loop using extend_from_within for bulk copying
  - Moved ActionContext outside main loop to reduce per-iteration allocations
  - Performance: ~6.17ms @ 16MB (minimal change - CRC32 validation dominates)

## [0.1.8-patch.4] - 2025-11-05

### Changed
- Refactored BPS tests to match IPS inline construction pattern
  - Removed helper functions from apply_tests.rs
  - Patches built inline for clarity (like IPS tests)
  - Added mod.rs to BPS test structure
  - Reduced apply_tests.rs from 174 to 138 lines

### Added
- docs/TEST_TEMPLATE.md - Standard test structure template
- Updated README.md with current stats and TEST_TEMPLATE reference

## [0.1.8-patch.3] - 2025-11-05

### Added
- BPS test coverage parity with IPS (17 tests each)
  - Added test_validate_valid_patch - validates minimal valid BPS patch
  - Added test_validate_with_actions - validates BPS patch with actions
  - Added test_apply_empty_patch - tests identity patch (source = target)
  - Total BPS tests: 17 (was 14)

## [0.1.8-patch.2] - 2025-11-05

### Fixed
- Added PSP console detection support (was in enum but missing from detect_console)
  - Supports .cso and .pbp file extensions
  - Added tests for PSP detection

## [0.1.8-patch.1] - 2025-11-05

### Changed
- Refactored BPS test structure to match IPS organization
  - Split apply_tests.rs into separate validate_tests.rs and metadata_tests.rs
  - Added 3 metadata extraction tests
  - Total BPS tests: 14 (was 11)

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
