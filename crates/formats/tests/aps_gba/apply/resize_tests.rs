//! APS GBA ROM resize tests

use super::helpers::*;
use stitchr_core::PatchFormat;
use stitchr_formats::aps::gba::ApsGbaPatcher;

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
