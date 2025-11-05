//! Tests for IPS patch validation

use rom_patcher_core::PatchFormat;
use rom_patcher_formats::ips::IpsPatcher;

#[test]
fn test_validate_valid_patch() {
    let patch = b"PATCHEOF";
    assert!(IpsPatcher::validate(patch).is_ok());
}

#[test]
fn test_validate_with_records() {
    // Manually create IPS patch with one record
    let mut patch = Vec::new();
    patch.extend_from_slice(b"PATCH"); // Header
    patch.extend_from_slice(&[0x00, 0x00, 0x32]); // Offset 50
    patch.extend_from_slice(&[0x00, 0x01]); // Size 1
    patch.push(0xFF); // Data
    patch.extend_from_slice(b"EOF"); // Footer

    assert!(IpsPatcher::validate(&patch).is_ok());
}

#[test]
fn test_validate_invalid_magic() {
    let patch = b"NOTIPSEOF";
    assert!(IpsPatcher::validate(patch).is_err());
}

#[test]
fn test_validate_missing_eof() {
    let patch = b"PATCH\x00\x00\x00\x00\x01\xFF";
    assert!(IpsPatcher::validate(patch).is_err());
}

#[test]
fn test_validate_truncated() {
    let patch = b"PATCH\x00\x00";
    assert!(IpsPatcher::validate(patch).is_err());
}

#[test]
fn test_validate_incomplete_record() {
    // Missing data bytes
    let patch = b"PATCH\x00\x00\x05\x00\x03EOF";
    assert!(IpsPatcher::validate(patch).is_err());
}
