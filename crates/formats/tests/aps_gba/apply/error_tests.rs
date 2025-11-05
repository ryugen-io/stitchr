//! APS GBA error handling tests

use rom_patcher_core::PatchFormat;
use rom_patcher_formats::aps::gba::ApsGbaPatcher;

#[test]
fn test_apply_invalid_patch() {
    let mut rom = vec![0u8; 256];
    let result = ApsGbaPatcher.apply(&mut rom, b"INVALID");
    assert!(result.is_err());
}

#[test]
fn test_apply_truncated_patch() {
    let mut rom = vec![0u8; 256];
    let result = ApsGbaPatcher.apply(&mut rom, b"APS1\x00\x00");
    assert!(result.is_err());
}
