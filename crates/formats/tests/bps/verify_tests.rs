//! Tests for BPS checksum verification

use stitchr_core::PatchFormat;
use stitchr_formats::bps::BpsPatcher;

#[test]
fn test_verify_input_rom() {
    let rom = vec![0x12, 0x34, 0x56];

    let mut patch = Vec::new();
    patch.extend_from_slice(b"BPS1");
    patch.push(0x83); // source_size = 3
    patch.push(0x83); // target_size = 3
    patch.push(0x80); // metadata_size = 0
    patch.push(0x88); // SOURCE_READ length=3
    let source_crc = crc32fast::hash(&rom);
    patch.extend_from_slice(&source_crc.to_le_bytes());
    let target_crc = crc32fast::hash(&rom);
    patch.extend_from_slice(&target_crc.to_le_bytes());
    let patch_crc = crc32fast::hash(&patch);
    patch.extend_from_slice(&patch_crc.to_le_bytes());

    assert!(BpsPatcher::verify(&rom, &patch, None).is_ok());
}

#[test]
fn test_verify_wrong_input_checksum() {
    let rom = vec![0x12, 0x34, 0x56];

    let mut patch = Vec::new();
    patch.extend_from_slice(b"BPS1");
    patch.push(0x83); // source_size = 3
    patch.push(0x83); // target_size = 3
    patch.push(0x80); // metadata_size = 0
    patch.push(0x88); // SOURCE_READ length=3
    let source_crc = crc32fast::hash(&rom);
    patch.extend_from_slice(&source_crc.to_le_bytes());
    let target_crc = crc32fast::hash(&rom);
    patch.extend_from_slice(&target_crc.to_le_bytes());
    let patch_crc = crc32fast::hash(&patch);
    patch.extend_from_slice(&patch_crc.to_le_bytes());

    let wrong_input = vec![0xFF, 0xFF, 0xFF];
    assert!(BpsPatcher::verify(&wrong_input, &patch, None).is_err());
}

#[test]
fn test_verify_output_rom() {
    let empty_rom: Vec<u8> = vec![];
    let source_crc = crc32fast::hash(&empty_rom); // CRC of empty source

    let output_rom = vec![0x12, 0x34, 0x56];
    let target_crc = crc32fast::hash(&output_rom);

    let mut patch = Vec::new();
    patch.extend_from_slice(b"BPS1");
    patch.push(0x80); // source_size = 0
    patch.push(0x83); // target_size = 3
    patch.push(0x80); // metadata_size = 0
    patch.push(0x88); // SOURCE_READ length=3
    patch.extend_from_slice(&source_crc.to_le_bytes());
    patch.extend_from_slice(&target_crc.to_le_bytes());
    let patch_crc = crc32fast::hash(&patch);
    patch.extend_from_slice(&patch_crc.to_le_bytes());

    assert!(BpsPatcher::verify(&[], &patch, Some(&output_rom)).is_ok());
}

#[test]
fn test_verify_wrong_output_checksum() {
    let rom = vec![0x12, 0x34, 0x56];
    let source_crc = crc32fast::hash(&rom);
    let target_crc = crc32fast::hash(&rom);

    let mut patch = Vec::new();
    patch.extend_from_slice(b"BPS1");
    patch.push(0x83); // source_size = 3
    patch.push(0x83); // target_size = 3
    patch.push(0x80); // metadata_size = 0
    patch.push(0x88); // SOURCE_READ length=3
    patch.extend_from_slice(&source_crc.to_le_bytes());
    patch.extend_from_slice(&target_crc.to_le_bytes());
    let patch_crc = crc32fast::hash(&patch);
    patch.extend_from_slice(&patch_crc.to_le_bytes());

    let wrong_output = vec![0xAA, 0xBB, 0xCC];
    assert!(BpsPatcher::verify(&[], &patch, Some(&wrong_output)).is_err());
}
