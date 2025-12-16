//! UPS error handling tests

use stitchr_core::PatchFormat;
use stitchr_formats::ups::UpsPatcher;

#[test]
fn test_apply_invalid_patch() {
    let mut rom = vec![0u8; 10];
    let invalid_patch = vec![0xFF; 20]; // Invalid patch data

    let patcher = UpsPatcher;
    assert!(patcher.apply(&mut rom, &invalid_patch).is_err());
}

#[test]
fn test_apply_truncated_patch() {
    let mut rom = vec![0u8; 10];
    let patch = b"UPS1\x8A\x8A"; // Incomplete header

    let patcher = UpsPatcher;
    assert!(patcher.apply(&mut rom, patch).is_err());
}
