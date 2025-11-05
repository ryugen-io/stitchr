# FORMAT TEMPLATE

This directory contains the standard structure for implementing new patch formats.

## File Structure

All formats MUST follow this exact structure:

```
src/FORMAT_NAME/
├── apply/
│   └── mod.rs          # Apply logic ONLY
├── constants.rs        # Magic bytes, sizes, action types
├── helpers.rs          # CRC validation, header parsing
├── metadata.rs         # Metadata extraction
├── mod.rs              # Module exports and trait impl
├── validate.rs         # Validation and verification
└── varint.rs           # Variable-length encoding (if needed)

tests/FORMAT_NAME/
├── apply_tests.rs                  # Apply logic tests (8+ tests)
├── checksum_validation_tests.rs   # Real patch file tests (3+ tests)
├── metadata_tests.rs               # Metadata extraction tests (3+ tests)
├── mod.rs                          # Test module exports
└── validate_tests.rs               # Validation tests (6+ tests)
```

## Rules

1. **NO files over 200 lines** - Split into subdirectories if needed
2. **Modular structure** - Each concern in separate file
3. **Test parity** - Minimum 17 tests per format (like IPS/BPS/UPS)
4. **No dead code** - Only implement what's needed for applying patches
5. **No encoding** - NEVER implement patch creation functions

## Implementation Checklist

- [ ] Create src/FORMAT_NAME/constants.rs (magic, sizes)
- [ ] Create src/FORMAT_NAME/helpers.rs (checksums, parsing)
- [ ] Create src/FORMAT_NAME/validate.rs (can_handle, validate, verify)
- [ ] Create src/FORMAT_NAME/metadata.rs (extract metadata)
- [ ] Create src/FORMAT_NAME/apply/mod.rs (apply logic only)
- [ ] Create src/FORMAT_NAME/mod.rs (trait implementation)
- [ ] Create tests/FORMAT_NAME/validate_tests.rs
- [ ] Create tests/FORMAT_NAME/metadata_tests.rs
- [ ] Create tests/FORMAT_NAME/apply_tests.rs
- [ ] Create tests/FORMAT_NAME/checksum_validation_tests.rs
- [ ] Create tests/FORMAT_NAME/mod.rs
- [ ] Add to Cargo.toml features
- [ ] Add benchmark file benches/FORMAT_NAME_bench.rs
- [ ] Add to dispatch.rs and verify.rs
- [ ] Update CHANGELOG.md
- [ ] Update README.md

## Example: See IPS, BPS, or UPS implementations
