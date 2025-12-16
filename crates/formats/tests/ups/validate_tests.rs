//! Tests for UPS patch validation

use stitchr_core::PatchFormat;
use stitchr_formats::ups::UpsPatcher;

#[test]
fn test_can_handle() {
    assert!(UpsPatcher::can_handle(b"UPS1"));
    assert!(UpsPatcher::can_handle(
        b"UPS1\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"
    ));
    assert!(!UpsPatcher::can_handle(b"PATCH"));
    assert!(!UpsPatcher::can_handle(b"UPS"));
    assert!(!UpsPatcher::can_handle(b""));
}

#[test]
fn test_validate_checks_magic() {
    let bad_patch = b"XXXX\x00\x00\x00";
    assert!(UpsPatcher::validate(bad_patch).is_err());
}

#[test]
fn test_validate_checks_size() {
    let too_small = b"UPS1";
    assert!(UpsPatcher::validate(too_small).is_err());
}

#[test]
fn test_validate_valid_patch() {
    // Minimal valid UPS patch
    let mut patch = Vec::new();
    patch.extend_from_slice(b"UPS1");
    patch.push(0x80); // Input size = 0
    patch.push(0x80); // Output size = 0

    let input_rom: Vec<u8> = vec![];
    let input_crc = crc32fast::hash(&input_rom);
    patch.extend_from_slice(&input_crc.to_le_bytes());

    let output_crc = crc32fast::hash(&input_rom);
    patch.extend_from_slice(&output_crc.to_le_bytes());

    let patch_crc = crc32fast::hash(&patch);
    patch.extend_from_slice(&patch_crc.to_le_bytes());

    assert!(UpsPatcher::validate(&patch).is_ok());
}

#[test]
fn test_validate_with_xor_records() {
    // UPS patch with XOR record
    let mut patch = Vec::new();
    patch.extend_from_slice(b"UPS1");
    patch.push(0x8A); // Input size = 10
    patch.push(0x8A); // Output size = 10
    patch.push(0x80); // Relative offset 0
    patch.push(0xFF); // XOR data
    patch.push(0x00); // Terminator

    let input_rom = vec![0u8; 10];
    let input_crc = crc32fast::hash(&input_rom);
    patch.extend_from_slice(&input_crc.to_le_bytes());

    let mut output_rom = input_rom;
    output_rom[0] = 0xFF;
    let output_crc = crc32fast::hash(&output_rom);
    patch.extend_from_slice(&output_crc.to_le_bytes());

    let patch_crc = crc32fast::hash(&patch);
    patch.extend_from_slice(&patch_crc.to_le_bytes());

    assert!(UpsPatcher::validate(&patch).is_ok());
}

#[test]
fn test_validate_corrupted_patch_crc() {
    let mut patch = Vec::new();
    patch.extend_from_slice(b"UPS1");
    patch.push(0x80);
    patch.push(0x80);

    let input_rom: Vec<u8> = vec![];
    let input_crc = crc32fast::hash(&input_rom);
    patch.extend_from_slice(&input_crc.to_le_bytes());

    let output_crc = crc32fast::hash(&input_rom);
    patch.extend_from_slice(&output_crc.to_le_bytes());

    let patch_crc = crc32fast::hash(&patch);
    patch.extend_from_slice(&patch_crc.to_le_bytes());

    // Corrupt last byte
    let last_idx = patch.len() - 1;
    patch[last_idx] ^= 0xFF;

    assert!(UpsPatcher::validate(&patch).is_err());
}
