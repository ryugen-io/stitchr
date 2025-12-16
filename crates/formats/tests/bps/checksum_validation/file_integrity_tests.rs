//! File integrity verification tests

use super::helpers::*;
use stitchr_core::PatchFormat;
use stitchr_formats::bps::BpsPatcher;
use std::fs;

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
    let rom_path = test_rom_path("test.rom.gbc");
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
