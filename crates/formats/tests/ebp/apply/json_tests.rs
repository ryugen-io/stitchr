//! Tests for applying patches with JSON metadata

use rom_patcher_core::PatchFormat;
use rom_patcher_formats::ebp::EbpPatcher;

#[test]
fn test_apply_with_json_metadata() {
    let mut rom = vec![0x00; 10];

    let mut patch = Vec::new();
    patch.extend_from_slice(b"PATCH\x00\x00\x05\x00\x01\xAAEOF");
    patch.extend_from_slice(b"{\"title\":\"Test\"}");

    let patcher = EbpPatcher;
    patcher.apply(&mut rom, &patch).unwrap();

    assert_eq!(rom[5], 0xAA);
}

#[test]
fn test_apply_with_large_json() {
    let mut rom = vec![0x00; 10];

    let mut patch = Vec::new();
    patch.extend_from_slice(b"PATCH\x00\x00\x03\x00\x02\xBB\xCCEOF");
    patch.extend_from_slice(b"{\"title\":\"Test\",\"author\":\"Me\",\"description\":\"Long description here\"}");

    let patcher = EbpPatcher;
    patcher.apply(&mut rom, &patch).unwrap();

    assert_eq!(rom[3], 0xBB);
    assert_eq!(rom[4], 0xCC);
}
