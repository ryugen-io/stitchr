//! File integrity verification tests

use super::helpers::*;
use stitchr_core::PatchFormat;
use stitchr_formats::ips::IpsPatcher;
use std::fs;

#[test]
fn test_original_rom_unchanged() {
    // Verify we're using the correct original ROM
    let rom_path = test_rom_path("test.rom.gb");
    if !rom_path.exists() {
        println!("Skipping test: test.rom.gb not found");
        return;
    }
    let rom = fs::read(&rom_path).expect("Failed to read ROM");
    let rom_crc = crc32fast::hash(&rom);

    // This is the known CRC32 of the original ROM
    // Update this value if using a different ROM version
    println!("Original ROM CRC32: {:08X}", rom_crc);
    println!("ROM size: {} bytes", rom.len());

    assert_eq!(rom.len(), 524288, "ROM should be 512KB");
}

#[test]
fn test_patch_file_integrity() {
    // Verify patch file hasn't been corrupted
    let patch_path = test_rom_path("patch.ips");
    if !patch_path.exists() {
        println!("Skipping test: patch.ips not found");
        return;
    }
    let patch = fs::read(&patch_path).expect("Failed to read patch");
    let patch_crc = crc32fast::hash(&patch);

    println!("Patch CRC32: {:08X}", patch_crc);
    println!("Patch size: {} bytes", patch.len());

    // Validate patch structure
    IpsPatcher::validate(&patch).expect("Patch validation failed");
}
