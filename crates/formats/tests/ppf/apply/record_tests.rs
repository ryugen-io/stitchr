//! PPF record application tests

use stitchr_core::PatchFormat;
use stitchr_formats::ppf::PpfPatcher;

#[test]
fn test_apply_overlapping_records() {
    // Two records writing to the same location. Last one should win.
    let mut patch = Vec::new();
    patch.extend_from_slice(b"PPF30");
    patch.push(0x02); // Encoding
    patch.extend_from_slice(&[0u8; 50]);
    patch.push(0x00);
    patch.push(0x00);
    patch.push(0x00);
    patch.push(0x00); // Flags

    // Record 1: Offset 0, Data 0xAA
    patch.extend_from_slice(&0u64.to_le_bytes());
    patch.push(1);
    patch.push(0xAA);

    // Record 2: Offset 0, Data 0xBB (Should overwrite)
    patch.extend_from_slice(&0u64.to_le_bytes());
    patch.push(1);
    patch.push(0xBB);

    let mut rom = vec![0x00];
    PpfPatcher.apply(&mut rom, &patch).unwrap();
    assert_eq!(rom[0], 0xBB);
}

#[test]
fn test_apply_contiguous_records() {
    // Records that touch but don't overlap
    let mut patch = Vec::new();
    patch.extend_from_slice(b"PPF30");
    patch.push(0x02);
    patch.extend_from_slice(&[0u8; 50]);
    patch.push(0x00);
    patch.push(0x00);
    patch.push(0x00);
    patch.push(0x00);

    // Record 1: Offset 0, Len 1, Data 0x11
    patch.extend_from_slice(&0u64.to_le_bytes());
    patch.push(1);
    patch.push(0x11);

    // Record 2: Offset 1, Len 1, Data 0x22
    patch.extend_from_slice(&1u64.to_le_bytes());
    patch.push(1);
    patch.push(0x22);

    let mut rom = vec![0x00, 0x00];
    PpfPatcher.apply(&mut rom, &patch).unwrap();
    assert_eq!(rom, vec![0x11, 0x22]);
}

#[test]
fn test_apply_zero_length_record() {
    let mut patch = Vec::new();
    patch.extend_from_slice(b"PPF30");
    patch.push(0x02);
    patch.extend_from_slice(&[0u8; 50]);
    patch.push(0x00);
    patch.push(0x00);
    patch.push(0x00);
    patch.push(0x00);

    // Record: Offset 0, Len 0, Data (empty)
    patch.extend_from_slice(&0u64.to_le_bytes());
    patch.push(0);
    // No data bytes

    let mut rom = vec![0xAA];
    PpfPatcher.apply(&mut rom, &patch).unwrap();
    assert_eq!(rom[0], 0xAA);
}
