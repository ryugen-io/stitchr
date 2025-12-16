//! PPF error handling tests

use stitchr_core::{PatchError, PatchFormat};
use stitchr_formats::ppf::PpfPatcher;

#[test]
fn test_apply_too_small_rom() {
    let mut patch_data = Vec::new();
    patch_data.extend_from_slice(b"PPF30");
    patch_data.push(0x02);
    patch_data.extend_from_slice(&[0u8; 50]);
    patch_data.push(0x00); // Image type
    patch_data.push(0x00); // Block check
    patch_data.push(0x00); // Undo data
    patch_data.push(0x00); // Dummy

    // Offset 1, Length 1 -> End 2
    patch_data.extend_from_slice(&1u64.to_le_bytes());
    patch_data.push(0x01);
    patch_data.push(0xBB);

    let mut rom_data = vec![0xAA]; // Len 1

    let patcher = PpfPatcher;
    let result = patcher.apply(&mut rom_data, &patch_data);

    assert!(matches!(result, Err(PatchError::OutOfBounds { .. })));
}

#[test]
fn test_apply_truncated_header() {
    let patch_data = b"PPF3"; // Too short
    let mut rom_data = vec![0u8; 10];
    let patcher = PpfPatcher;
    // can_handle checks length, so apply returns InvalidFormat or similar
    // Actually apply calls can_handle first.
    let result = patcher.apply(&mut rom_data, patch_data);
    assert!(matches!(result, Err(PatchError::InvalidFormat(_))));
}

#[test]
fn test_apply_corrupted_record() {
    let mut patch_data = Vec::new();
    patch_data.extend_from_slice(b"PPF30");
    patch_data.push(0x02);
    patch_data.extend_from_slice(&[0u8; 50]);
    patch_data.push(0x00);
    patch_data.push(0x00);
    patch_data.push(0x00);
    patch_data.push(0x00);

    // Truncated record (only offset, no length)
    patch_data.extend_from_slice(&0u64.to_le_bytes());

    let mut rom_data = vec![0u8; 10];
    let patcher = PpfPatcher;
    let result = patcher.apply(&mut rom_data, &patch_data);
    assert!(matches!(result, Err(PatchError::CorruptedData)));
}
