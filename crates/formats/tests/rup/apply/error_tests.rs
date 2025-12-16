//! RUP error handling tests

use stitchr_core::PatchFormat;
use stitchr_formats::rup::RupPatcher;

#[test]
fn test_apply_wrong_magic() {
    let mut rom = vec![0u8; 100];
    let patch = b"NOTRUP";

    let patcher = RupPatcher;
    let result = patcher.apply(&mut rom, patch);

    assert!(result.is_err());
}

#[test]
fn test_apply_truncated_header() {
    let mut rom = vec![0u8; 100];
    let patch = b"NINJA2";

    let patcher = RupPatcher;
    let result = patcher.apply(&mut rom, patch);

    assert!(result.is_err());
}

#[test]
fn test_apply_empty_patch() {
    let mut rom = vec![0u8; 100];
    let patch = b"";

    let patcher = RupPatcher;
    let result = patcher.apply(&mut rom, patch);

    assert!(result.is_err());
}
