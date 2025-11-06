//! Tests for EBP metadata extraction (JSON parsing)

use rom_patcher_core::PatchFormat;
use rom_patcher_formats::ebp::EbpPatcher;

#[test]
fn test_metadata_no_json() {
    let patch = b"PATCHEOF";
    let metadata = EbpPatcher::metadata(patch).unwrap();
    assert!(metadata.extra.is_empty());
}

#[test]
fn test_metadata_with_title() {
    let mut patch = Vec::new();
    patch.extend_from_slice(b"PATCHEOF{\"title\":\"Test\"}");

    let metadata = EbpPatcher::metadata(&patch).unwrap();
    let title = metadata.extra.iter().find(|(k, _)| k == "title");
    assert_eq!(title.unwrap().1, "Test");
}

#[test]
fn test_metadata_full_json() {
    let mut patch = Vec::new();
    patch.extend_from_slice(b"PATCHEOF{\"title\":\"My Patch\",\"author\":\"Me\"}");

    let metadata = EbpPatcher::metadata(&patch).unwrap();
    assert_eq!(metadata.extra.len(), 2);
}

#[test]
fn test_metadata_escaped_chars() {
    let mut patch = Vec::new();
    patch.extend_from_slice(b"PATCHEOF{\"title\":\"L1\\nL2\"}");

    let metadata = EbpPatcher::metadata(&patch).unwrap();
    let title = metadata.extra.iter().find(|(k, _)| k == "title");
    assert_eq!(title.unwrap().1, "L1\nL2");
}

#[test]
fn test_metadata_invalid_json() {
    let mut patch = Vec::new();
    patch.extend_from_slice(b"PATCHEOF{invalid}");

    let metadata = EbpPatcher::metadata(&patch).unwrap();
    assert!(metadata.extra.is_empty());
}
