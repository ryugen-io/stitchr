//! APS N64 error handling tests

use stitchr_core::PatchFormat;
use stitchr_formats::aps::n64::ApsN64Patcher;

#[test]
fn test_apply_invalid_patch() {
    let mut rom = vec![0u8; 256];
    let result = ApsN64Patcher.apply(&mut rom, b"INVALID");
    assert!(result.is_err());
}

#[test]
fn test_apply_truncated_patch() {
    let mut rom = vec![0u8; 256];
    let result = ApsN64Patcher.apply(&mut rom, b"APS10\x01");
    assert!(result.is_err());
}
