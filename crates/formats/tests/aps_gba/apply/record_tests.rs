//! APS GBA multiple record tests

use super::helpers::*;
use stitchr_core::PatchFormat;
use stitchr_formats::aps::gba::ApsGbaPatcher;

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
