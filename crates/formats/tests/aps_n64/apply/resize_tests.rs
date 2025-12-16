//! APS N64 ROM resize tests

use super::helpers::*;
use stitchr_core::PatchFormat;
use stitchr_formats::aps::n64::ApsN64Patcher;

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
