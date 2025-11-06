//! Basic IPS application tests

use rom_patcher_core::PatchFormat;
use rom_patcher_formats::ebp::EbpPatcher;

#[test]
fn test_can_handle() {
    // EBP requires JSON metadata after EOF
    assert!(EbpPatcher::can_handle(b"PATCHEOF{}"));
    assert!(EbpPatcher::can_handle(b"PATCH\x00\x00\x00EOF{\"title\":\"test\"}"));

    // Plain IPS (no JSON) should not be handled by EBP
    assert!(!EbpPatcher::can_handle(b"PATCH"));
    assert!(!EbpPatcher::can_handle(b"PATCHEOF"));

    // Invalid formats
    assert!(!EbpPatcher::can_handle(b"NOTIPS"));
    assert!(!EbpPatcher::can_handle(b"PAT"));
}

#[test]
fn test_apply_simple_patch() {
    let mut rom = vec![0x00; 10];
    let patch = b"PATCH\x00\x00\x05\x00\x01\xFFEOF";

    let patcher = EbpPatcher;
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

    let patcher = EbpPatcher;
    patcher.apply(&mut rom, &patch).unwrap();

    assert_eq!(rom[5], 0xAA);
    assert_eq!(rom[10], 0xBB);
}

#[test]
fn test_empty_patch() {
    let mut rom = vec![0x12, 0x34, 0x56];
    let patch = b"PATCHEOF";

    let patcher = EbpPatcher;
    patcher.apply(&mut rom, patch).unwrap();

    // ROM should be unchanged
    assert_eq!(rom, vec![0x12, 0x34, 0x56]);
}
