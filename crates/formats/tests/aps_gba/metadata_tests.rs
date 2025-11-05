//! Metadata extraction tests for APS GBA

use rom_patcher_core::PatchFormat;
use rom_patcher_formats::aps::gba::ApsGbaPatcher;

const BLOCK_SIZE: usize = 0x10000;

#[test]
fn test_metadata_simple() {
    let mut patch = Vec::new();
    patch.extend_from_slice(b"APS1");
    patch.extend_from_slice(&1024u32.to_le_bytes());
    patch.extend_from_slice(&2048u32.to_le_bytes());
    patch.extend_from_slice(&0u32.to_le_bytes());
    patch.extend_from_slice(&0u16.to_le_bytes());
    patch.extend_from_slice(&0u16.to_le_bytes());
    patch.extend_from_slice(&vec![0u8; BLOCK_SIZE]);

    let metadata = ApsGbaPatcher::metadata(&patch).expect("Failed to extract metadata");

    assert_eq!(metadata.source_size, Some(1024));
    assert_eq!(metadata.target_size, Some(2048));
    assert!(
        metadata
            .extra
            .iter()
            .any(|(k, v)| k == "Source Size" && v == "1024")
    );
    assert!(
        metadata
            .extra
            .iter()
            .any(|(k, v)| k == "Target Size" && v == "2048")
    );
}

#[test]
fn test_metadata_with_multiple_records() {
    let mut patch = Vec::new();
    patch.extend_from_slice(b"APS1");
    patch.extend_from_slice(&512u32.to_le_bytes());
    patch.extend_from_slice(&1024u32.to_le_bytes());
    patch.extend_from_slice(&0u32.to_le_bytes());
    patch.extend_from_slice(&0u16.to_le_bytes());
    patch.extend_from_slice(&0u16.to_le_bytes());
    patch.extend_from_slice(&vec![0u8; BLOCK_SIZE]);
    patch.extend_from_slice(&0x10000u32.to_le_bytes());
    patch.extend_from_slice(&0u16.to_le_bytes());
    patch.extend_from_slice(&0u16.to_le_bytes());
    patch.extend_from_slice(&vec![0u8; BLOCK_SIZE]);

    let metadata = ApsGbaPatcher::metadata(&patch).expect("Failed to extract metadata");

    assert_eq!(metadata.target_size, Some(1024));
    assert!(
        metadata
            .extra
            .iter()
            .any(|(k, v)| k == "Record Count" && v == "2")
    );
}

#[test]
fn test_metadata_invalid_patch() {
    let invalid = b"NOTAPS";
    let result = ApsGbaPatcher::metadata(invalid);
    assert!(result.is_err());
}
