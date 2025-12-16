use stitchr_core::PatchFormat;
use stitchr_formats::bdf::{BdfPatcher, constants::BDF_MAGIC};

#[test]
fn test_metadata_simple() {
    let mut patch = Vec::new();
    patch.extend_from_slice(BDF_MAGIC);
    patch.extend_from_slice(&100u64.to_le_bytes()); // control_size
    patch.extend_from_slice(&200u64.to_le_bytes()); // diff_size
    patch.extend_from_slice(&1024u64.to_le_bytes()); // patched_size

    let metadata = BdfPatcher::metadata(&patch).unwrap();
    assert_eq!(metadata.patch_type, stitchr_core::PatchType::Bdf);

    // extra is Vec<(String, String)>
    let find_extra = |key: &str| -> Option<String> {
        metadata
            .extra
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.clone())
    };

    assert_eq!(find_extra("control_size"), Some("100".to_string()));
    assert_eq!(find_extra("diff_size"), Some("200".to_string()));
    assert_eq!(find_extra("patched_size"), Some("1024".to_string()));
}

#[test]
fn test_metadata_max_values() {
    let mut patch = Vec::new();
    patch.extend_from_slice(BDF_MAGIC);
    patch.extend_from_slice(&u64::MAX.to_le_bytes());
    patch.extend_from_slice(&u64::MAX.to_le_bytes());
    patch.extend_from_slice(&u64::MAX.to_le_bytes());

    let metadata = BdfPatcher::metadata(&patch).unwrap();
    // Should extract strings properly
    let find_extra = |key: &str| -> Option<String> {
        metadata
            .extra
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.clone())
    };
    assert_eq!(find_extra("control_size"), Some(u64::MAX.to_string()));
}

#[test]
fn test_metadata_partial_read() {
    // Header ok, but stream ends immediately
    let mut patch = Vec::new();
    patch.extend_from_slice(BDF_MAGIC);
    // Missing size fields
    assert!(BdfPatcher::metadata(&patch).is_err());
}

#[test]
fn test_metadata_zeros() {
    let mut patch = Vec::new();
    patch.extend_from_slice(BDF_MAGIC);
    patch.extend_from_slice(&0u64.to_le_bytes());
    patch.extend_from_slice(&0u64.to_le_bytes());
    patch.extend_from_slice(&0u64.to_le_bytes());
    let metadata = BdfPatcher::metadata(&patch).unwrap();
    let find_extra = |key: &str| -> Option<String> {
        metadata
            .extra
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.clone())
    };
    assert_eq!(find_extra("patched_size"), Some("0".to_string()));
}

#[test]
fn test_metadata_invalid_patch() {
    let patch = b"INVALID";
    assert!(BdfPatcher::metadata(patch).is_err());
}
