//! Tests for RUP patch validation

use stitchr_core::PatchFormat;
use stitchr_formats::rup::RupPatcher;

#[test]
fn test_can_handle() {
    assert!(RupPatcher::can_handle(b"NINJA2"));
    let mut valid = vec![0u8; 0x800];
    valid[..6].copy_from_slice(b"NINJA2");
    assert!(RupPatcher::can_handle(&valid));
    assert!(!RupPatcher::can_handle(b"PATCH"));
    assert!(!RupPatcher::can_handle(b"NINJA"));
    assert!(!RupPatcher::can_handle(b""));
}

#[test]
fn test_validate_checks_magic() {
    let mut patch = vec![0u8; 0x800 + 1];
    patch[..6].copy_from_slice(b"WRONG!");
    assert!(RupPatcher::validate(&patch).is_err());
}

#[test]
fn test_validate_checks_size() {
    let too_small = b"NINJA2";
    assert!(RupPatcher::validate(too_small).is_err());
}

#[test]
fn test_validate_valid_header() {
    let mut patch = vec![0u8; 0x801];
    patch[..6].copy_from_slice(b"NINJA2");
    patch[0x800] = 0x00; // END command
    assert!(RupPatcher::validate(&patch).is_ok());
}

#[test]
fn test_validate_no_data_section() {
    let patch = vec![0u8; 0x800];
    assert!(RupPatcher::validate(&patch).is_err());
}
