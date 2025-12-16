//! Validation tests for APS GBA format

use stitchr_core::PatchFormat;
use stitchr_formats::aps::gba::ApsGbaPatcher;

const BLOCK_SIZE: usize = 0x10000;

#[test]
fn test_can_handle() {
    let mut valid_patch = Vec::new();
    valid_patch.extend_from_slice(b"APS1");
    valid_patch.extend_from_slice(&512u32.to_le_bytes());
    valid_patch.extend_from_slice(&512u32.to_le_bytes());
    valid_patch.extend_from_slice(&0u32.to_le_bytes());
    valid_patch.extend_from_slice(&0u16.to_le_bytes());
    valid_patch.extend_from_slice(&0u16.to_le_bytes());
    valid_patch.extend_from_slice(&vec![0u8; BLOCK_SIZE]);

    assert!(ApsGbaPatcher::can_handle(&valid_patch));

    let invalid = b"INVALID";
    assert!(!ApsGbaPatcher::can_handle(invalid));
}

#[test]
fn test_validate_checks_magic() {
    let invalid = b"NOTAPS\x00\x00";
    let result = ApsGbaPatcher::validate(invalid);
    assert!(result.is_err());
}

#[test]
fn test_validate_checks_size() {
    let too_small = b"APS1";
    let result = ApsGbaPatcher::validate(too_small);
    assert!(result.is_err());
}

#[test]
fn test_validate_valid_patch() {
    let mut patch = Vec::new();
    patch.extend_from_slice(b"APS1");
    patch.extend_from_slice(&1024u32.to_le_bytes());
    patch.extend_from_slice(&2048u32.to_le_bytes());
    patch.extend_from_slice(&0u32.to_le_bytes());
    patch.extend_from_slice(&0u16.to_le_bytes());
    patch.extend_from_slice(&0u16.to_le_bytes());
    patch.extend_from_slice(&vec![0u8; BLOCK_SIZE]);

    let result = ApsGbaPatcher::validate(&patch);
    assert!(result.is_ok());
}

#[test]
fn test_validate_with_records() {
    let mut patch = Vec::new();
    patch.extend_from_slice(b"APS1");
    patch.extend_from_slice(&1024u32.to_le_bytes());
    patch.extend_from_slice(&1024u32.to_le_bytes());

    // Add XOR record
    patch.extend_from_slice(&0x100u32.to_le_bytes());
    patch.extend_from_slice(&0u16.to_le_bytes());
    patch.extend_from_slice(&0u16.to_le_bytes());
    patch.extend_from_slice(&vec![0xFFu8; BLOCK_SIZE]);

    assert!(ApsGbaPatcher::validate(&patch).is_ok());
}

#[test]
fn test_validate_truncated_record() {
    let mut patch = Vec::new();
    patch.extend_from_slice(b"APS1");
    patch.extend_from_slice(&1024u32.to_le_bytes());
    patch.extend_from_slice(&1024u32.to_le_bytes());

    // Truncated record (missing XOR data block)
    patch.extend_from_slice(&0x100u32.to_le_bytes());
    patch.extend_from_slice(&0u16.to_le_bytes());
    patch.extend_from_slice(&0u16.to_le_bytes());
    // Missing BLOCK_SIZE bytes of data!

    assert!(ApsGbaPatcher::validate(&patch).is_err());
}
