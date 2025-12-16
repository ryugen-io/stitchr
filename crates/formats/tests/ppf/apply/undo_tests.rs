//! PPF undo data skipping tests

use stitchr_core::PatchFormat;
use stitchr_formats::ppf::PpfPatcher;

#[test]
fn test_apply_with_undo_data() {
    let mut patch_data = Vec::new();
    patch_data.extend_from_slice(b"PPF30");
    patch_data.push(0x02);
    patch_data.extend_from_slice(&[0u8; 50]);
    patch_data.push(0x00);
    patch_data.push(0x00); // Block check
    patch_data.push(0x01); // Undo data = TRUE
    patch_data.push(0x00); // Dummy

    // Record: Offset 0, Len 1, Data 0xBB, Undo 0xAA
    patch_data.extend_from_slice(&0u64.to_le_bytes());
    patch_data.push(0x01);
    patch_data.push(0xBB);
    patch_data.push(0xAA); // Undo data (should be skipped)

    // Another record: Offset 2, Len 1, Data 0xCC, Undo 0xDD
    patch_data.extend_from_slice(&2u64.to_le_bytes());
    patch_data.push(0x01);
    patch_data.push(0xCC);
    patch_data.push(0xDD); // Undo data

    let mut rom_data = vec![0xAA, 0xAA, 0xAA, 0xAA];
    let patcher = PpfPatcher;
    patcher.apply(&mut rom_data, &patch_data).unwrap();

    assert_eq!(rom_data, vec![0xBB, 0xAA, 0xCC, 0xAA]);
}
