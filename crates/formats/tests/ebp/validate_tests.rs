//! Tests for EBP patch validation

use rom_patcher_core::PatchFormat;
use rom_patcher_formats::ebp::EbpPatcher;

#[test]
fn test_validate_valid_patch() {
    let patch = b"PATCHEOF";
    assert!(EbpPatcher::validate(patch).is_ok());
}

#[test]
fn test_validate_with_records() {
    let mut patch = Vec::new();
    patch.extend_from_slice(b"PATCH");
    patch.extend_from_slice(&[0x00, 0x00, 0x32]);
    patch.extend_from_slice(&[0x00, 0x01]);
    patch.push(0xFF);
    patch.extend_from_slice(b"EOF");

    assert!(EbpPatcher::validate(&patch).is_ok());
}

#[test]
fn test_validate_invalid_magic() {
    let patch = b"NOTEBPEOF";
    assert!(EbpPatcher::validate(patch).is_err());
}

#[test]
fn test_validate_missing_eof() {
    let patch = b"PATCH\x00\x00\x00\x00\x01\xFF";
    assert!(EbpPatcher::validate(patch).is_err());
}

#[test]
fn test_validate_truncated() {
    let patch = b"PATCH\x00\x00";
    assert!(EbpPatcher::validate(patch).is_err());
}
