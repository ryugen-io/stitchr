//! Apply tests for APS N64 format

use rom_patcher_core::PatchFormat;
use rom_patcher_formats::aps::n64::ApsN64Patcher;

fn make_header(output_size: u32) -> Vec<u8> {
    let mut patch = Vec::new();
    patch.extend_from_slice(b"APS10");
    patch.push(0x01);
    patch.push(0x00);
    patch.extend_from_slice(&[0u8; 50]);
    patch.push(0x01);
    patch.extend_from_slice(b"TST");
    patch.extend_from_slice(&[0u8; 8]);
    patch.extend_from_slice(&[0u8; 5]);
    patch.extend_from_slice(&output_size.to_le_bytes());
    patch
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

#[test]
fn test_apply_rle_record() {
    let mut patch = make_header(1024);
    patch.extend_from_slice(&0x200u32.to_le_bytes());
    patch.push(0x00);
    patch.push(0xFF);
    patch.push(10);

    let mut rom = vec![0u8; 256];
    ApsN64Patcher.apply(&mut rom, &patch).unwrap();

    assert_eq!(rom.len(), 1024);
    assert_eq!(&rom[0x200..0x20A], &[0xFF; 10]);
}

#[test]
fn test_apply_mixed_records() {
    let mut patch = make_header(1024);
    patch.extend_from_slice(&0x10u32.to_le_bytes());
    patch.push(3);
    patch.extend_from_slice(&[0xAA, 0xBB, 0xCC]);
    patch.extend_from_slice(&0x20u32.to_le_bytes());
    patch.push(0x00);
    patch.push(0x55);
    patch.push(5);

    let mut rom = vec![0u8; 128];
    ApsN64Patcher.apply(&mut rom, &patch).unwrap();

    assert_eq!(&rom[0x10..0x13], &[0xAA, 0xBB, 0xCC]);
    assert_eq!(&rom[0x20..0x25], &[0x55; 5]);
}

#[test]
fn test_apply_empty_rom() {
    let patch = make_header(256);
    let mut rom = vec![];
    ApsN64Patcher.apply(&mut rom, &patch).unwrap();
    assert_eq!(rom.len(), 256);
}

#[test]
fn test_apply_rom_resize() {
    let patch = make_header(1024);
    let mut rom = vec![0xFF; 512];
    ApsN64Patcher.apply(&mut rom, &patch).unwrap();
    assert_eq!(rom.len(), 1024);
    assert_eq!(&rom[0..512], &[0xFF; 512]);
    assert_eq!(&rom[512..1024], &[0x00; 512]);
}

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
