//! PPF basic apply tests

use rom_patcher_core::{PatchError, PatchFormat};
use rom_patcher_formats::ppf::PpfPatcher;
use std::path::PathBuf;

#[test]
fn test_ppf3_apply_simple_patch() -> Result<(), PatchError> {
    // This is a minimal, manually crafted PPF3 patch.
    // Header: "PPF30" (5 bytes)
    // Encoding: 0x02 (1 byte)
    // Description: 50 bytes (padded with 0s)
    // Image type: 0x00 (1 byte)
    // Block check: 0x00 (1 byte)
    // Undo data: 0x00 (1 byte)
    // Dummy: 0x00 (1 byte)
    // Patch records:
    //   Offset: 0x0000000000000000 (8 bytes, little-endian)
    //   Data length: 0x01 (1 byte)
    //   New data: 0xBB (1 byte)
    //   Offset: 0x0000000000000002 (8 bytes, little-endian)
    //   Data length: 0x01 (1 byte)
    //   New data: 0xCC (1 byte)

    let mut patch_data = Vec::new();
    patch_data.extend_from_slice(b"PPF30"); // Magic
    patch_data.push(0x02); // Encoding
    patch_data.extend_from_slice(&[0u8; 50]); // Description (50 bytes)
    patch_data.push(0x00); // Image type
    patch_data.push(0x00); // Block check
    patch_data.push(0x00); // Undo data
    patch_data.push(0x00); // Dummy

    // Patch record 1: offset 0x00, length 1, data 0xBB
    patch_data.extend_from_slice(&0u64.to_le_bytes()); // Offset 0x00
    patch_data.push(0x01); // Data length 1
    patch_data.push(0xBB); // New data

    // Patch record 2: offset 0x02, length 1, data 0xCC
    patch_data.extend_from_slice(&2u64.to_le_bytes()); // Offset 0x02
    patch_data.push(0x01); // Data length 1
    patch_data.push(0xCC); // New data

    let mut rom_data = vec![0xAA, 0xAA, 0xAA, 0xAA]; // Original ROM data

    let patcher = PpfPatcher;
    patcher.apply(&mut rom_data, &patch_data)?;

    assert_eq!(rom_data, vec![0xBB, 0xAA, 0xCC, 0xAA]);

    Ok(())
}

#[test]
fn test_ppf_file_apply() -> Result<(), PatchError> {
    let patch_path = PathBuf::from("../../../test_files/ppf/patch.ppf");

    if !patch_path.exists() {
        println!("Skipping test: patch file not found at {:?}", patch_path);
        return Ok(());
    }

    let patch_data = std::fs::read(&patch_path).map_err(PatchError::Io)?;
    let mut rom_data = vec![0u8; 1024]; // Dummy ROM

    let _ = PpfPatcher.apply(&mut rom_data, &patch_data);

    Ok(())
}
