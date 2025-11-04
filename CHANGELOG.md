# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- BPS (Beat Patching System) format implementation
- UPS (Universal Patching System) format implementation
- APS format implementation for N64 and GBA
- RUP (Rupture) format implementation
- PPF (PlayStation Patch Format) implementation
- xdelta format implementation
- Patch creation functionality (create patches from ROM pairs)
- SHA-1, SHA-256 hash algorithms
- Additional CLI commands (create, info, validate)

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
