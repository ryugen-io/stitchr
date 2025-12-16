//! Tests for UPS metadata extraction

use stitchr_core::PatchFormat;
use stitchr_formats::ups::UpsPatcher;

#[test]
fn test_metadata_simple() {
    let mut patch = Vec::new();
    patch.extend_from_slice(b"UPS1");
    patch.push(0x8A); // Input size = 10
    patch.push(0x94); // Output size = 20
    patch.push(0x80); // Relative offset 0
    patch.push(0xFF);
    patch.push(0x00);

    let input_rom = vec![0u8; 10];
    let input_crc = crc32fast::hash(&input_rom);
    patch.extend_from_slice(&input_crc.to_le_bytes());

    let output_rom = vec![0xFFu8; 20];
    let output_crc = crc32fast::hash(&output_rom);
    patch.extend_from_slice(&output_crc.to_le_bytes());

    let patch_crc = crc32fast::hash(&patch);
    patch.extend_from_slice(&patch_crc.to_le_bytes());

    let metadata = UpsPatcher::metadata(&patch).unwrap();

    assert_eq!(metadata.patch_type, stitchr_core::PatchType::Ups);
    assert_eq!(metadata.source_size, Some(10));
    assert_eq!(metadata.target_size, Some(20));
    assert!(metadata.source_checksum.is_some());
    assert!(metadata.target_checksum.is_some());
}

#[test]
fn test_metadata_invalid_patch() {
    let invalid = b"IPS1234567890";
    assert!(UpsPatcher::metadata(invalid).is_err());
}

#[test]
fn test_metadata_empty_patch() {
    let mut patch = Vec::new();
    patch.extend_from_slice(b"UPS1");
    patch.push(0x80); // Input size = 0
    patch.push(0x80); // Output size = 0

    let empty: Vec<u8> = vec![];
    let input_crc = crc32fast::hash(&empty);
    patch.extend_from_slice(&input_crc.to_le_bytes());

    let output_crc = crc32fast::hash(&empty);
    patch.extend_from_slice(&output_crc.to_le_bytes());

    let patch_crc = crc32fast::hash(&patch);
    patch.extend_from_slice(&patch_crc.to_le_bytes());

    let metadata = UpsPatcher::metadata(&patch).unwrap();

    assert_eq!(metadata.source_size, Some(0));
    assert_eq!(metadata.target_size, Some(0));
}
