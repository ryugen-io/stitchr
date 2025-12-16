//! Metadata extraction tests for APS N64

use stitchr_core::PatchFormat;
use stitchr_formats::aps::n64::ApsN64Patcher;

#[test]
fn test_metadata_simple() {
    let mut patch = Vec::new();
    patch.extend_from_slice(b"APS10");
    patch.push(0x01); // Type: N64
    patch.push(0x00); // Encoding
    patch.extend_from_slice(b"Test Patch");
    patch.extend_from_slice(&[0u8; 40]); // Rest of description
    patch.push(0x01); // Original format
    patch.extend_from_slice(b"NTE"); // Cart ID
    patch.extend_from_slice(&[0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0]); // CRC
    patch.extend_from_slice(&[0u8; 5]); // Padding
    patch.extend_from_slice(&1024u32.to_le_bytes()); // Output size

    let metadata = ApsN64Patcher::metadata(&patch).expect("Failed to extract metadata");

    assert_eq!(metadata.target_size, Some(1024));
    assert!(
        metadata
            .extra
            .iter()
            .any(|(k, v)| k == "Output Size" && v == "1024")
    );
}

#[test]
fn test_metadata_with_description() {
    let mut patch = Vec::new();
    patch.extend_from_slice(b"APS10");
    patch.push(0x01);
    patch.push(0x00);
    patch.extend_from_slice(b"Spanish Translation Patch");
    patch.extend_from_slice(&[0u8; 25]);
    patch.push(0x01);
    patch.extend_from_slice(b"NTE");
    patch.extend_from_slice(&[0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0]);
    patch.extend_from_slice(&[0u8; 5]);
    patch.extend_from_slice(&2048u32.to_le_bytes());

    let metadata = ApsN64Patcher::metadata(&patch).expect("Failed to extract metadata");

    assert_eq!(metadata.target_size, Some(2048));
    assert!(
        metadata
            .extra
            .iter()
            .any(|(k, v)| k == "Output Size" && v == "2048")
    );
}

#[test]
fn test_metadata_invalid_patch() {
    let invalid = b"NOTAPS";
    let result = ApsN64Patcher::metadata(invalid);
    assert!(result.is_err());
}
