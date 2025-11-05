//! Validation tests for APS N64 format

use rom_patcher_core::PatchFormat;
use rom_patcher_formats::aps::n64::ApsN64Patcher;

#[test]
fn test_can_handle() {
    let mut valid_patch = Vec::new();
    valid_patch.extend_from_slice(b"APS10");
    valid_patch.push(0x01); // Type
    valid_patch.push(0x00); // Encoding
    valid_patch.extend_from_slice(&[0u8; 50]); // Description
    valid_patch.push(0x01); // Original format
    valid_patch.extend_from_slice(b"TST"); // Cart ID
    valid_patch.extend_from_slice(&[0u8; 8]); // CRC
    valid_patch.extend_from_slice(&[0u8; 5]); // Padding
    valid_patch.extend_from_slice(&1024u32.to_le_bytes()); // Output size

    assert!(ApsN64Patcher::can_handle(&valid_patch));

    let invalid = b"INVALID";
    assert!(!ApsN64Patcher::can_handle(invalid));
}

#[test]
fn test_validate_checks_magic() {
    let invalid = b"NOTAPS\x01\x00";
    let result = ApsN64Patcher::validate(invalid);
    assert!(result.is_err());
}

#[test]
fn test_validate_checks_size() {
    let too_small = b"APS";
    let result = ApsN64Patcher::validate(too_small);
    assert!(result.is_err());
}

#[test]
fn test_validate_valid_patch() {
    let mut patch = Vec::new();
    patch.extend_from_slice(b"APS10");
    patch.push(0x01); // Type: N64
    patch.push(0x00); // Encoding
    patch.extend_from_slice(&[0u8; 50]); // Description
    patch.push(0x01); // Original format
    patch.extend_from_slice(b"NTE"); // Cart ID
    patch.extend_from_slice(&[0u8; 8]); // CRC
    patch.extend_from_slice(&[0u8; 5]); // Padding
    patch.extend_from_slice(&1024u32.to_le_bytes()); // Output size

    let result = ApsN64Patcher::validate(&patch);
    assert!(result.is_ok());
}
