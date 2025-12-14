use rom_patcher_core::PatchFormat;
use rom_patcher_formats::bdf::{BdfPatcher, constants::BDF_MAGIC};

#[test]
fn test_can_handle() {
    let mut patch = Vec::new();
    patch.extend_from_slice(BDF_MAGIC);
    assert!(BdfPatcher::can_handle(&patch));

    let invalid = b"INVALID_MAGIC";
    assert!(!BdfPatcher::can_handle(invalid));
}

#[test]
fn test_validate_truncated_header() {
    let patch = b"BSDIFF40";
    assert!(BdfPatcher::validate(patch).is_err());
}

#[test]
fn test_validate_checks_size() {
    let mut patch = Vec::new();
    patch.extend_from_slice(BDF_MAGIC);
    patch.extend_from_slice(&10u64.to_le_bytes()); // control
    patch.extend_from_slice(&10u64.to_le_bytes()); // diff
    patch.extend_from_slice(&100u64.to_le_bytes()); // patched size
    // Total needed: 32 + 10 + 10 = 52 bytes.
    // We only provide header (32 bytes).
    assert!(BdfPatcher::validate(&patch).is_err());
}

#[test]
fn test_validate_valid_minimal() {
    let mut patch = Vec::new();
    patch.extend_from_slice(BDF_MAGIC);
    patch.extend_from_slice(&0u64.to_le_bytes()); // control
    patch.extend_from_slice(&0u64.to_le_bytes()); // diff
    patch.extend_from_slice(&0u64.to_le_bytes()); // patched size
    // Total needed: 32 + 0 + 0 = 32 bytes.
    assert!(BdfPatcher::validate(&patch).is_ok());
}

#[test]
fn test_validate_magic_typo() {
    let mut patch = Vec::new();
    patch.extend_from_slice(b"BSDIFF41"); // 41 instead of 40
    patch.extend_from_slice(&0u64.to_le_bytes());
    patch.extend_from_slice(&0u64.to_le_bytes());
    patch.extend_from_slice(&0u64.to_le_bytes());
    assert!(!BdfPatcher::can_handle(&patch));
    assert!(BdfPatcher::validate(&patch).is_err());
}

#[test]
fn test_validate_header_boundary() {
    // 31 bytes
    let patch = vec![0u8; 31];
    assert!(BdfPatcher::validate(&patch).is_err());
}

#[test]
fn test_validate_huge_sizes_overflow() {
    let mut patch = Vec::new();
    patch.extend_from_slice(BDF_MAGIC);
    patch.extend_from_slice(&u64::MAX.to_le_bytes()); // control
    patch.extend_from_slice(&u64::MAX.to_le_bytes()); // diff
    patch.extend_from_slice(&0u64.to_le_bytes());
    // This should fail size check gracefully without panic
    assert!(BdfPatcher::validate(&patch).is_err());
}
