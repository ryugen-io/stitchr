//! Tests for UPS checksum verification

use stitchr_core::PatchFormat;
use stitchr_formats::ups::UpsPatcher;

#[test]
fn test_verify_input_rom() {
    let rom = vec![0x12, 0x34, 0x56];

    let mut patch = Vec::new();
    patch.extend_from_slice(b"UPS1");
    patch.push(0x03); // input_size = 3
    patch.push(0x03); // output_size = 3
    patch.push(0x00); // End of patch (XOR encoded offset = 0)

    let input_crc = crc32fast::hash(&rom);
    patch.extend_from_slice(&input_crc.to_le_bytes());
    let output_crc = crc32fast::hash(&rom);
    patch.extend_from_slice(&output_crc.to_le_bytes());
    let patch_crc = crc32fast::hash(&patch);
    patch.extend_from_slice(&patch_crc.to_le_bytes());

    assert!(UpsPatcher::verify(&rom, &patch, None).is_ok());
}

#[test]
fn test_verify_wrong_input_checksum() {
    let rom = vec![0x12, 0x34, 0x56];

    let mut patch = Vec::new();
    patch.extend_from_slice(b"UPS1");
    patch.push(0x03); // input_size = 3
    patch.push(0x03); // output_size = 3
    patch.push(0x00); // End of patch

    let input_crc = crc32fast::hash(&rom);
    patch.extend_from_slice(&input_crc.to_le_bytes());
    let output_crc = crc32fast::hash(&rom);
    patch.extend_from_slice(&output_crc.to_le_bytes());
    let patch_crc = crc32fast::hash(&patch);
    patch.extend_from_slice(&patch_crc.to_le_bytes());

    let wrong_input = vec![0xFF, 0xFF, 0xFF];
    assert!(UpsPatcher::verify(&wrong_input, &patch, None).is_err());
}

#[test]
fn test_verify_output_rom() {
    let rom = vec![0x12, 0x34, 0x56];
    let input_crc = crc32fast::hash(&rom);

    let output_rom = vec![0x12, 0x34, 0x56];
    let output_crc = crc32fast::hash(&output_rom);

    let mut patch = Vec::new();
    patch.extend_from_slice(b"UPS1");
    patch.push(0x03); // input_size = 3
    patch.push(0x03); // output_size = 3
    patch.push(0x00); // End of patch
    patch.extend_from_slice(&input_crc.to_le_bytes());
    patch.extend_from_slice(&output_crc.to_le_bytes());
    let patch_crc = crc32fast::hash(&patch);
    patch.extend_from_slice(&patch_crc.to_le_bytes());

    assert!(UpsPatcher::verify(&[], &patch, Some(&output_rom)).is_ok());
}

#[test]
fn test_verify_wrong_output_checksum() {
    let rom = vec![0x12, 0x34, 0x56];
    let input_crc = crc32fast::hash(&rom);
    let output_crc = crc32fast::hash(&rom);

    let mut patch = Vec::new();
    patch.extend_from_slice(b"UPS1");
    patch.push(0x03); // input_size = 3
    patch.push(0x03); // output_size = 3
    patch.push(0x00); // End of patch
    patch.extend_from_slice(&input_crc.to_le_bytes());
    patch.extend_from_slice(&output_crc.to_le_bytes());
    let patch_crc = crc32fast::hash(&patch);
    patch.extend_from_slice(&patch_crc.to_le_bytes());

    let wrong_output = vec![0xAA, 0xBB, 0xCC];
    assert!(UpsPatcher::verify(&[], &patch, Some(&wrong_output)).is_err());
}
