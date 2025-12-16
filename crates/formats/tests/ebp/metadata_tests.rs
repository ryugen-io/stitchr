//! Tests for IPS metadata extraction

use stitchr_core::PatchFormat;
use stitchr_formats::ebp::EbpPatcher;

#[test]
fn test_metadata_simple() {
    // Manually create IPS patch: write 0xFF at offset 50
    let mut patch = Vec::new();
    patch.extend_from_slice(b"PATCH"); // Header
    patch.extend_from_slice(&[0x00, 0x00, 0x32]); // Offset 50 (24-bit BE)
    patch.extend_from_slice(&[0x00, 0x01]); // Size 1 (16-bit BE)
    patch.push(0xFF); // Data
    patch.extend_from_slice(b"EOF"); // Footer

    let metadata = EbpPatcher::metadata(&patch).unwrap();

    assert!(metadata.target_size.is_some());
    assert!(metadata.target_size.unwrap() >= 51);
}

#[test]
fn test_metadata_with_truncation() {
    // Manually create IPS patch with truncation size 50
    let mut patch = Vec::new();
    patch.extend_from_slice(b"PATCH"); // Header
    patch.extend_from_slice(b"EOF"); // EOF marker
    patch.extend_from_slice(&[0x00, 0x00, 0x32]); // Truncate to 50 bytes (24-bit BE)

    let metadata = EbpPatcher::metadata(&patch).unwrap();

    assert_eq!(metadata.target_size, Some(50));
}

#[test]
fn test_metadata_invalid_patch() {
    let invalid_patch = b"NOTIPS";
    let result = EbpPatcher::metadata(invalid_patch);

    assert!(result.is_err());
}
