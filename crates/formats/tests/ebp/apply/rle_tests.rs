//! IPS RLE compression tests

use stitchr_core::PatchFormat;
use stitchr_formats::ebp::EbpPatcher;

#[test]
fn test_apply_rle_compression() {
    let mut rom = vec![0x00; 100];

    // RLE: fill 50 bytes at offset 10 with 0xFF
    let mut patch = Vec::new();
    patch.extend_from_slice(b"PATCH");
    patch.extend_from_slice(&[0x00, 0x00, 0x0A]); // offset 10
    patch.extend_from_slice(&[0x00, 0x00]); // size 0 (RLE marker)
    patch.extend_from_slice(&[0x00, 0x32]); // RLE size: 50
    patch.push(0xFF);
    patch.extend_from_slice(b"EOF");

    let patcher = EbpPatcher;
    patcher.apply(&mut rom, &patch).unwrap();

    // Check RLE applied correctly
    for (i, &byte) in rom.iter().enumerate().skip(10).take(50) {
        assert_eq!(byte, 0xFF, "Byte at {} should be 0xFF", i);
    }
    assert_eq!(rom[9], 0x00);
    assert_eq!(rom[60], 0x00);
}

#[test]
fn test_expand_rom() {
    let mut rom = vec![0x00; 10];

    // Patch extends ROM to 20 bytes
    let mut patch = Vec::new();
    patch.extend_from_slice(b"PATCH");
    patch.extend_from_slice(&[0x00, 0x00, 0x0F]); // offset 15
    patch.extend_from_slice(&[0x00, 0x01]); // size 1
    patch.push(0xFF);
    patch.extend_from_slice(b"EOF");

    let patcher = EbpPatcher;
    patcher.apply(&mut rom, &patch).unwrap();

    assert!(rom.len() >= 16);
    assert_eq!(rom[15], 0xFF);
}
