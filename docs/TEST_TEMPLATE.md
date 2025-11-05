# Test Structure Template

This document defines the standard test structure for ROM patch formats.

## Directory Structure

```
crates/formats/tests/
├── {format}_integration.rs     # Integration test entry point
└── {format}/
    ├── mod.rs                   # Module declarations
    ├── apply_tests.rs           # Patch application tests
    ├── validate_tests.rs        # Patch validation tests
    ├── metadata_tests.rs        # Metadata extraction tests
    └── checksum_validation_tests.rs  # Real ROM integration tests
```

## File Standards

### mod.rs
```rust
//! {FORMAT} format tests

mod apply_tests;
mod checksum_validation_tests;
mod metadata_tests;
mod validate_tests;
```

### apply_tests.rs

Tests patch application logic with inline patch construction.

**Required Tests:**
- `test_can_handle()` - Format detection
- `test_apply_simple()` - Basic patch application
- `test_apply_empty_patch()` - Identity patch (source = target)
- Format-specific action tests

**Pattern:**
```rust
//! Tests for {FORMAT} patch application

use rom_patcher_core::PatchFormat;
use rom_patcher_formats::{format}::{FormatPatcher};

#[test]
fn test_apply_simple() {
    let mut rom = vec![0x00; 10];

    // Build patch inline - NO helper functions
    let mut patch = Vec::new();
    patch.extend_from_slice(b"MAGIC");
    // ... construct patch data directly

    let patcher = FormatPatcher;
    patcher.apply(&mut rom, &patch).unwrap();

    assert_eq!(rom[5], 0xFF);
}
```

**Important:**
- NO helper functions for patch construction
- Build patches inline to show exact byte layout
- Use comments to explain binary format

### validate_tests.rs

Tests patch validation logic.

**Required Tests:**
- `test_can_handle()` - Magic number detection
- `test_validate_valid_patch()` - Valid minimal patch
- `test_validate_with_records()` - Valid patch with data
- `test_validate_invalid_magic()` - Wrong magic number
- `test_validate_checks_size()` - Truncated patch

### metadata_tests.rs

Tests metadata extraction.

**Required Tests:**
- `test_metadata_simple()` - Basic metadata
- `test_metadata_with_info()` - Metadata with additional info
- `test_metadata_invalid_patch()` - Error handling

**Note:** May use helper functions for complex encoding (e.g., varint)

### checksum_validation_tests.rs

Integration tests with real ROM patches.

**Required Tests:**
- `test_{game}_patch()` - Real ROM patching with CRC32 validation
- `test_original_rom_unchanged()` - Verify original ROM
- `test_patch_file_integrity()` - Verify patch file

**Pattern:**
```rust
#[test]
fn test_game_patch() {
    let rom_path = test_rom_path("test.rom.gb");
    let mut rom = match fs::read(&rom_path) {
        Ok(data) => data,
        Err(_) => {
            println!("Skipping test: ROM file not found");
            return;
        }
    };

    let patch_path = test_rom_path("patch.{ext}");
    let patch = fs::read(&patch_path).expect("Failed to read patch");

    // Validate
    FormatPatcher::validate(&patch).expect("Patch validation failed");

    // Apply
    let patcher = FormatPatcher;
    patcher.apply(&mut rom, &patch).expect("Failed to apply patch");

    // Verify
    let output_crc = crc32fast::hash(&rom);
    assert_eq!(output_crc, EXPECTED_CRC32);
}
```

## Integration File

`{format}_integration.rs`:

```rust
//! {FORMAT} integration tests

mod {format} {
    mod apply_tests;
    mod checksum_validation_tests;
    mod metadata_tests;
    mod validate_tests;
}
```

## Code Quality Standards

- **Line limit:** 200 lines per file maximum
- **No helper functions** in apply_tests.rs (inline construction)
- **Helper functions allowed** in metadata_tests.rs for complex encoding
- **Clear comments:** Explain binary format in tests
- **Consistent naming:** `test_` prefix for all tests
- **Error handling:** Gracefully skip tests when ROM files missing

## Test Coverage Goals

- **17+ tests per format** (parity with IPS/BPS)
- **All PatchFormat trait methods** covered
- **Error paths** tested (invalid magic, truncation, etc.)
- **Real-world validation** with actual ROM patches

## Example: Creating Tests for New Format

1. Copy structure from IPS or BPS
2. Create `mod.rs` with module declarations
3. Implement `apply_tests.rs` with inline patch construction
4. Add `validate_tests.rs` for error handling
5. Add `metadata_tests.rs` for info extraction
6. Add `checksum_validation_tests.rs` with real ROM
7. Update integration file to include new modules

## References

- IPS tests: `crates/formats/tests/ips/`
- BPS tests: `crates/formats/tests/bps/`
- Test ROMs: `test_files/{format}/test.rom.{ext}`
