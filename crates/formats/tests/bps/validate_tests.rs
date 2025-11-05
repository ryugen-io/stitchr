//! Tests for BPS patch validation

use rom_patcher_core::PatchFormat;
use rom_patcher_formats::bps::BpsPatcher;

#[test]
fn test_can_handle() {
    assert!(BpsPatcher::can_handle(b"BPS1"));
    assert!(BpsPatcher::can_handle(
        b"BPS1\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"
    ));
    assert!(!BpsPatcher::can_handle(b"PATCH"));
    assert!(!BpsPatcher::can_handle(b"BPS"));
    assert!(!BpsPatcher::can_handle(b""));
}

#[test]
fn test_validate_checks_magic() {
    let bad_patch = b"XXXX\x00\x00\x00";
    assert!(BpsPatcher::validate(bad_patch).is_err());
}

#[test]
fn test_validate_checks_size() {
    let too_small = b"BPS1";
    assert!(BpsPatcher::validate(too_small).is_err());
}
