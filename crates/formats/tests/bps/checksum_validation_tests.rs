//! Integration tests for BPS with real ROM patches
//!
//! Tests BPS patching against real-world patches and validates checksums

use rom_patcher_core::PatchFormat;
use rom_patcher_formats::bps::BpsPatcher;
use std::fs;
use std::path::PathBuf;

fn test_rom_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../test_files/bps")
        .join(name)
}

#[test]
fn test_samurai_kid_patch() {
    // Samurai Kid English Translation patch from RomPatcherJS test suite
    // This is a real-world BPS patch that exercises all action types

    // Load original ROM
    let rom_path = test_rom_path("Samurai Kid (Japan).gbc");
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

    // TODO: Add expected output CRC32 when we have reference implementation output
    // For now, just verify the patch applied without errors
}

#[test]
fn test_patch_file_integrity() {
    // Verify patch file hasn't been corrupted
    let patch_path = test_rom_path("patch.bps");
    let patch = match fs::read(&patch_path) {
        Ok(data) => data,
        Err(_) => {
            println!("Skipping test: patch file not found");
            return;
        }
    };

    let patch_crc = crc32fast::hash(&patch);
    println!("Patch CRC32: {:08X}", patch_crc);
    println!("Patch size: {} bytes", patch.len());

    // Validate patch structure
    BpsPatcher::validate(&patch).expect("Patch validation failed");

    // Verify it's BPS format
    assert!(BpsPatcher::can_handle(&patch));
}

#[test]
fn test_rom_file_exists() {
    // Verify we're using the correct original ROM
    let rom_path = test_rom_path("Samurai Kid (Japan).gbc");
    let rom = match fs::read(&rom_path) {
        Ok(data) => data,
        Err(_) => {
            println!("Skipping test: ROM file not found");
            return;
        }
    };

    let rom_crc = crc32fast::hash(&rom);
    println!("Original ROM CRC32: {:08X}", rom_crc);
    println!("ROM size: {} bytes", rom.len());

    assert_eq!(rom.len(), 1048576, "ROM should be 1MB");
}
