//! Basic APS N64 application tests

use super::helpers::*;
use stitchr_core::PatchFormat;
use stitchr_formats::aps::n64::ApsN64Patcher;

#[test]
fn test_can_handle() {
    let valid_patch = make_header(1024);

    assert!(ApsN64Patcher::can_handle(&valid_patch));
    assert!(!ApsN64Patcher::can_handle(b"NOTAPS"));
    assert!(!ApsN64Patcher::can_handle(b"APS"));
    assert!(!ApsN64Patcher::can_handle(b""));
}

#[test]
fn test_apply_simple_record() {
    let mut patch = make_header(512);
    patch.extend_from_slice(&0x100u32.to_le_bytes());
    patch.push(4);
    patch.extend_from_slice(&[0xDE, 0xAD, 0xBE, 0xEF]);

    let mut rom = vec![0u8; 256];
    ApsN64Patcher.apply(&mut rom, &patch).unwrap();

    assert_eq!(rom.len(), 512);
    assert_eq!(&rom[0x100..0x104], &[0xDE, 0xAD, 0xBE, 0xEF]);
}
