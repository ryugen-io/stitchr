//! Tests for BPS patch validation

use stitchr_core::PatchFormat;
use stitchr_formats::bps::BpsPatcher;

#[test]
fn test_can_handle() {
    assert!(BpsPatcher::can_handle(b"BPS1"));
    assert!(BpsPatcher::can_handle(
        b"BPS1\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"
    ));
    assert!(!BpsPatcher::can_handle(b"PATCH"));
    assert!(!BpsPatcher::can_handle(b"BPS"));
    assert!(!BpsPatcher::can_handle(b""));
}

#[test]
fn test_validate_checks_magic() {
    let bad_patch = b"XXXX\x00\x00\x00";
    assert!(BpsPatcher::validate(bad_patch).is_err());
}

#[test]
fn test_validate_checks_size() {
    let too_small = b"BPS1";
    assert!(BpsPatcher::validate(too_small).is_err());
}

#[test]
fn test_validate_valid_patch() {
    // Minimal valid BPS patch: source=0, target=0, no actions
    let mut patch = Vec::new();
    patch.extend_from_slice(b"BPS1");
    patch.push(0x80); // source_size = 0 (varint)
    patch.push(0x80); // target_size = 0 (varint)
    patch.push(0x80); // metadata_size = 0 (varint)
    // No actions
    patch.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // source CRC32
    patch.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // target CRC32
    // Patch CRC32
    let patch_crc = crc32fast::hash(&patch);
    patch.extend_from_slice(&patch_crc.to_le_bytes());

    assert!(BpsPatcher::validate(&patch).is_ok());
}

#[test]
fn test_validate_with_actions() {
    // BPS patch with SOURCE_READ action
    let mut patch = Vec::new();
    patch.extend_from_slice(b"BPS1");
    patch.push(0x85); // source_size = 5 (varint)
    patch.push(0x83); // target_size = 3 (varint)
    patch.push(0x80); // metadata_size = 0 (varint)
    // SOURCE_READ action: length=3 -> ((3-1)<<2) | 0 = 8
    patch.push(0x88); // varint(8) with end bit
    // Checksums
    patch.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // source CRC32
    patch.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // target CRC32
    // Patch CRC32
    let patch_crc = crc32fast::hash(&patch);
    patch.extend_from_slice(&patch_crc.to_le_bytes());

    assert!(BpsPatcher::validate(&patch).is_ok());
}

#[test]
fn test_validate_corrupted_patch_crc() {
    let mut patch = Vec::new();
    patch.extend_from_slice(b"BPS1");
    patch.push(0x85); // source_size = 5
    patch.push(0x83); // target_size = 3
    patch.push(0x80); // metadata_size = 0
    patch.push(0x88); // SOURCE_READ length=3
    patch.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // source CRC32
    patch.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // target CRC32
    // Wrong patch CRC - not calculated from patch data
    patch.extend_from_slice(&[0xFF, 0xFF, 0xFF, 0xFF]);

    assert!(BpsPatcher::validate(&patch).is_err());
}
