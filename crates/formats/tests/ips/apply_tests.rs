//! Tests for IPS patch application

use rom_patcher_core::PatchFormat;
use rom_patcher_formats::ips::IpsPatcher;

#[test]
fn test_can_handle() {
    assert!(IpsPatcher::can_handle(b"PATCH"));
    assert!(IpsPatcher::can_handle(b"PATCH\x00\x00\x00EOF"));
    assert!(!IpsPatcher::can_handle(b"NOTIPS"));
    assert!(!IpsPatcher::can_handle(b"PAT"));
}

#[test]
fn test_apply_simple_patch() {
    let mut rom = vec![0x00; 10];
    let patch = b"PATCH\x00\x00\x05\x00\x01\xFFEOF";

    let patcher = IpsPatcher;
    patcher.apply(&mut rom, patch).unwrap();

    assert_eq!(rom[5], 0xFF);
}

#[test]
fn test_apply_multiple_changes() {
    let mut rom = vec![0x00; 20];

    // Patch: change byte at offset 5 to 0xAA, offset 10 to 0xBB
    let mut patch = Vec::new();
    patch.extend_from_slice(b"PATCH");
    patch.extend_from_slice(&[0x00, 0x00, 0x05]); // offset 5
    patch.extend_from_slice(&[0x00, 0x01]); // size 1
    patch.push(0xAA);
    patch.extend_from_slice(&[0x00, 0x00, 0x0A]); // offset 10
    patch.extend_from_slice(&[0x00, 0x01]); // size 1
    patch.push(0xBB);
    patch.extend_from_slice(b"EOF");

    let patcher = IpsPatcher;
    patcher.apply(&mut rom, &patch).unwrap();

    assert_eq!(rom[5], 0xAA);
    assert_eq!(rom[10], 0xBB);
}

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

    let patcher = IpsPatcher;
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

    let patcher = IpsPatcher;
    patcher.apply(&mut rom, &patch).unwrap();

    assert!(rom.len() >= 16);
    assert_eq!(rom[15], 0xFF);
}

#[test]
fn test_empty_patch() {
    let mut rom = vec![0x12, 0x34, 0x56];
    let patch = b"PATCHEOF";

    let patcher = IpsPatcher;
    patcher.apply(&mut rom, patch).unwrap();

    // ROM should be unchanged
    assert_eq!(rom, vec![0x12, 0x34, 0x56]);
}

#[test]
fn test_apply_invalid_patch() {
    let mut rom = vec![0x00; 10];
    let patch = b"NOTVALID";

    let patcher = IpsPatcher;
    assert!(patcher.apply(&mut rom, patch).is_err());
}

#[test]
fn test_apply_truncated_patch() {
    let mut rom = vec![0x00; 10];
    // Incomplete record
    let patch = b"PATCH\x00\x00\x05\x00";

    let patcher = IpsPatcher;
    assert!(patcher.apply(&mut rom, patch).is_err());
}
