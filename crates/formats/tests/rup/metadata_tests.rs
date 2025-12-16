//! Tests for RUP metadata extraction

use stitchr_core::PatchFormat;
use stitchr_formats::rup::RupPatcher;
use std::fs;

#[test]
fn test_metadata_from_real_patch() {
    let patch_path = std::path::Path::new("../../test_files/rup/test.rup");
    if !patch_path.exists() {
        return;
    }
    let patch = fs::read(patch_path).expect("Failed to read RUP patch");
    let metadata = RupPatcher::metadata(&patch).expect("Failed to extract metadata");

    assert_eq!(metadata.patch_type, stitchr_core::PatchType::Rup);
    assert!(metadata.source_size.is_some());
    assert!(metadata.target_size.is_some());
}

#[test]
fn test_metadata_has_extra_fields() {
    let patch_path = std::path::Path::new("../../test_files/rup/test.rup");
    if !patch_path.exists() {
        return;
    }
    let patch = fs::read(patch_path).expect("Failed to read RUP patch");
    let metadata = RupPatcher::metadata(&patch).expect("Failed to extract metadata");

    // RUP patches can have author, title, version, etc.
    assert!(!metadata.extra.is_empty(), "RUP should have extra metadata");
}

#[test]
fn test_metadata_invalid_patch() {
    let invalid_patch = b"NOTRUP";
    let result = RupPatcher::metadata(invalid_patch);

    assert!(result.is_err());
}

#[test]
fn test_metadata_truncated_patch() {
    let patch = b"NINJA2";
    let result = RupPatcher::metadata(patch);

    assert!(result.is_err());
}
