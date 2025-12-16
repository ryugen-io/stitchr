//! APS N64 RLE record tests

use super::helpers::*;
use stitchr_core::PatchFormat;
use stitchr_formats::aps::n64::ApsN64Patcher;

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
