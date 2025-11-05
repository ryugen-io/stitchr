//! Apply tests for APS GBA format

use rom_patcher_core::PatchFormat;
use rom_patcher_formats::aps::gba::ApsGbaPatcher;

const BLOCK_SIZE: usize = 0x10000; // 64KB

fn make_header(source_size: u32, target_size: u32) -> Vec<u8> {
    let mut patch = Vec::new();
    patch.extend_from_slice(b"APS1");
    patch.extend_from_slice(&source_size.to_le_bytes());
    patch.extend_from_slice(&target_size.to_le_bytes());
    patch
}

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
fn test_apply_multiple_records() {
    let mut patch = make_header(1024, 1024);

    // First record
    patch.extend_from_slice(&0x10u32.to_le_bytes());
    patch.extend_from_slice(&0u16.to_le_bytes());
    patch.extend_from_slice(&0u16.to_le_bytes());
    let mut xor_data1 = vec![0xFF; 4];
    xor_data1.resize(BLOCK_SIZE, 0);
    patch.extend_from_slice(&xor_data1);

    // Second record
    patch.extend_from_slice(&0x20u32.to_le_bytes());
    patch.extend_from_slice(&0u16.to_le_bytes());
    patch.extend_from_slice(&0u16.to_le_bytes());
    let mut xor_data2 = vec![0x55; 4];
    xor_data2.resize(BLOCK_SIZE, 0);
    patch.extend_from_slice(&xor_data2);

    let mut rom = vec![0xAA; 1024];
    ApsGbaPatcher.apply(&mut rom, &patch).unwrap();

    assert_eq!(&rom[0x10..0x14], &[0x55, 0x55, 0x55, 0x55]);
    assert_eq!(&rom[0x20..0x24], &[0xFF, 0xFF, 0xFF, 0xFF]);
}

#[test]
fn test_apply_rom_resize() {
    let patch = make_header(512, 1024);
    let mut rom = vec![0xFF; 512];
    ApsGbaPatcher.apply(&mut rom, &patch).unwrap();
    assert_eq!(rom.len(), 1024);
    assert_eq!(&rom[0..512], &[0xFF; 512]);
    assert_eq!(&rom[512..1024], &[0x00; 512]);
}

#[test]
fn test_apply_empty_rom() {
    let patch = make_header(0, 256);
    let mut rom = vec![];
    ApsGbaPatcher.apply(&mut rom, &patch).unwrap();
    assert_eq!(rom.len(), 256);
}

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
