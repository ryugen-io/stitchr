//! Integration tests for BPS with real ROM patches

use super::helpers::*;
use stitchr_core::PatchFormat;
use stitchr_formats::bps::BpsPatcher;
use std::fs;

#[test]
fn test_samurai_kid_patch() {
    // Samurai Kid English Translation patch from RomPatcherJS test suite
    // This is a real-world BPS patch that exercises all action types

    // Load original ROM
    let rom_path = test_rom_path("test.rom.gbc");
    let mut rom = match fs::read(&rom_path) {
        Ok(data) => data,
        Err(_) => {
            println!("Skipping test: ROM file not found at {:?}", rom_path);
            return;
        }
    };

    println!("Loaded ROM: {} bytes", rom.len());

    // Load patch
    let patch_path = test_rom_path("patch.bps");
    if !patch_path.exists() {
        println!("Skipping test: patch.bps not found");
        return;
    }
    let patch = fs::read(&patch_path).expect("Failed to read BPS patch");

    println!("Loaded patch: {} bytes", patch.len());

    // Validate patch structure
    BpsPatcher::validate(&patch).expect("Patch validation failed");

    // Extract and verify metadata
    let metadata = BpsPatcher::metadata(&patch).expect("Failed to extract metadata");
    println!("Source size: {:?}", metadata.source_size);
    println!("Target size: {:?}", metadata.target_size);

    assert_eq!(metadata.source_size, Some(1048576)); // 1MB
    assert_eq!(metadata.target_size, Some(1048576)); // 1MB
    assert_eq!(rom.len(), 1048576);

    // Apply patch
    let patcher = BpsPatcher;
    patcher
        .apply(&mut rom, &patch)
        .expect("Failed to apply patch");

    // Verify output size
    assert_eq!(rom.len(), 1048576, "Output ROM size mismatch");

    println!("Successfully patched ROM");

    // Calculate output CRC32
    let output_crc = crc32fast::hash(&rom);
    println!("Output CRC32: {:08X}", output_crc);

    // TODO: Add expected output CRC32 when we have reference implementation
    // output For now, just verify the patch applied without errors
}
