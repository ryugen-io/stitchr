//! PPF version specific tests

use stitchr_core::PatchFormat;
use stitchr_formats::ppf::PpfPatcher;

#[test]
fn test_apply_ppf1_structure() {
    // PPF1: Magic "PPF10", Encoding, Description. No extra flags.
    let mut patch = Vec::new();
    patch.extend_from_slice(b"PPF10");
    patch.push(0x00); // Encoding
    patch.extend_from_slice(&[0u8; 50]); // Description (consumed but ignored)

    // Record: Offset 0 (u32), Len 1, Data 0xAA
    patch.extend_from_slice(&0u32.to_le_bytes());
    patch.push(1);
    patch.push(0xAA);

    let mut rom = vec![0x00];
    // Note: Our implementation handles PPF1 by checking magic and skipping flags
    // reading If it works, it writes 0xAA.
    PpfPatcher.apply(&mut rom, &patch).unwrap();
    assert_eq!(rom[0], 0xAA);
}

#[test]
fn test_apply_ppf2_structure() {
    // PPF2: Magic "PPF20", Encoding, Description, InputSize(u32), BlockCheck(1024
    // bytes)
    let mut patch = Vec::new();
    patch.extend_from_slice(b"PPF20");
    patch.push(0x01); // Encoding
    patch.extend_from_slice(&[0u8; 50]); // Description

    // Input Size (u32) - implicitly enables block check logic, but applying doesn't
    // enforce size check in our implementation yet (maybe it should?)
    // The decoder reads it.
    patch.extend_from_slice(&1024u32.to_le_bytes());

    // Block Check Data (1024 bytes) - skipped
    patch.extend_from_slice(&[0xFF; 1024]);

    // Record: Offset 0 (u32), Len 1, Data 0xBB
    patch.extend_from_slice(&0u32.to_le_bytes());
    patch.push(1);
    patch.push(0xBB);

    let mut rom = vec![0x00];
    PpfPatcher.apply(&mut rom, &patch).unwrap();
    assert_eq!(rom[0], 0xBB);
}
