//! Basic EBP application tests (IPS compatibility)

use rom_patcher_core::PatchFormat;
use rom_patcher_formats::ebp::EbpPatcher;

#[test]
fn test_can_handle() {
    assert!(EbpPatcher::can_handle(b"PATCH"));
    assert!(EbpPatcher::can_handle(b"PATCHEOF"));
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

    let mut patch = Vec::new();
    patch.extend_from_slice(b"PATCH");
    patch.extend_from_slice(&[0x00, 0x00, 0x05, 0x00, 0x01, 0xAA]);
    patch.extend_from_slice(&[0x00, 0x00, 0x0A, 0x00, 0x01, 0xBB]);
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

    assert_eq!(rom, vec![0x12, 0x34, 0x56]);
}
