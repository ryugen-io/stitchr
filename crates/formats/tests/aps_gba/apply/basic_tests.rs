//! Basic APS GBA application tests

use super::helpers::*;
use rom_patcher_core::PatchFormat;
use rom_patcher_formats::aps::gba::ApsGbaPatcher;

#[test]
fn test_can_handle() {
    let valid_patch = make_header(1024, 1024);

    assert!(ApsGbaPatcher::can_handle(&valid_patch));
    assert!(!ApsGbaPatcher::can_handle(b"NOTAPS"));
    assert!(!ApsGbaPatcher::can_handle(b"APS"));
    assert!(!ApsGbaPatcher::can_handle(b""));
}

#[test]
fn test_apply_simple_xor() {
    let mut patch = make_header(512, 512);
    patch.extend_from_slice(&0x10u32.to_le_bytes());
    patch.extend_from_slice(&0u16.to_le_bytes());
    patch.extend_from_slice(&0u16.to_le_bytes());
    let mut xor_data = vec![0xFF, 0xFF, 0xFF, 0xFF];
    xor_data.resize(BLOCK_SIZE, 0);
    patch.extend_from_slice(&xor_data);

    let mut rom = vec![0xAA; 512];
    ApsGbaPatcher.apply(&mut rom, &patch).unwrap();

    assert_eq!(rom.len(), 512);
    assert_eq!(&rom[0x10..0x14], &[0x55, 0x55, 0x55, 0x55]);
}

#[test]
fn test_apply_zero_xor() {
    let mut patch = make_header(512, 512);
    patch.extend_from_slice(&0u32.to_le_bytes());
    patch.extend_from_slice(&0u16.to_le_bytes());
    patch.extend_from_slice(&0u16.to_le_bytes());
    patch.extend_from_slice(&vec![0u8; BLOCK_SIZE]);

    let mut rom = vec![0xAA; 512];
    let original = rom.clone();
    ApsGbaPatcher.apply(&mut rom, &patch).unwrap();

    assert_eq!(rom, original);
}
