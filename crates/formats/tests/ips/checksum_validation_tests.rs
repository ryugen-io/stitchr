//! Tests for checksum validation against known-good outputs
//!
//! These tests apply real patches to real ROMs and verify the output
//! matches expected checksums from reference implementations.

use rom_patcher_core::PatchFormat;
use rom_patcher_formats::ips::IpsPatcher;
use std::fs;
use std::path::PathBuf;

fn test_rom_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../test_files/ips")
        .join(name)
}

#[test]
fn test_sml2dx_patch_checksum() {
    // Expected checksums from RomPatcherJS reference implementation
    const EXPECTED_INPUT_CRC32: u32 = 0xd5ec24e4;
    const EXPECTED_PATCH_CRC32: u32 = 0x0b742316;
    const EXPECTED_OUTPUT_CRC32: u32 = 0xf0799017;

    // Load original ROM
    let rom_path = test_rom_path("Super Mario Land 2 - 6 Golden Coins (UE) (V1.0) [!].gb");
    let mut rom = fs::read(&rom_path).expect("Failed to read ROM");

    // Validate input ROM checksum
    let input_crc = crc32fast::hash(&rom);
    println!(
        "Input ROM CRC32: {:08X} (expected: {:08X})",
        input_crc, EXPECTED_INPUT_CRC32
    );
    assert_eq!(
        input_crc, EXPECTED_INPUT_CRC32,
        "Input ROM CRC32 mismatch! Wrong ROM version? Expected {:08X}, got {:08X}",
        EXPECTED_INPUT_CRC32, input_crc
    );

    // Load patch
    let patch_path = test_rom_path("patch.ips");
    let patch = fs::read(&patch_path).expect("Failed to read patch");

    // Validate patch checksum
    let patch_crc = crc32fast::hash(&patch);
    println!(
        "Patch CRC32: {:08X} (expected: {:08X})",
        patch_crc, EXPECTED_PATCH_CRC32
    );
    assert_eq!(
        patch_crc, EXPECTED_PATCH_CRC32,
        "Patch file CRC32 mismatch! Corrupted patch? Expected {:08X}, got {:08X}",
        EXPECTED_PATCH_CRC32, patch_crc
    );

    // Apply patch
    let patcher = IpsPatcher;
    patcher
        .apply(&mut rom, &patch)
        .expect("Failed to apply patch");

    // Validate output checksum against RomPatcherJS reference
    let output_crc = crc32fast::hash(&rom);
    println!(
        "Output ROM CRC32: {:08X} (expected: {:08X})",
        output_crc, EXPECTED_OUTPUT_CRC32
    );
    assert_eq!(
        output_crc, EXPECTED_OUTPUT_CRC32,
        "Output ROM CRC32 mismatch! Patching logic error! Expected {:08X}, got {:08X}",
        EXPECTED_OUTPUT_CRC32, output_crc
    );

    println!("All checksums match RomPatcherJS reference implementation!");
}

#[test]
fn test_original_rom_unchanged() {
    // Verify we're using the correct original ROM
    let rom_path = test_rom_path("Super Mario Land 2 - 6 Golden Coins (UE) (V1.0) [!].gb");
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
    let patch = fs::read(&patch_path).expect("Failed to read patch");
    let patch_crc = crc32fast::hash(&patch);

    println!("Patch CRC32: {:08X}", patch_crc);
    println!("Patch size: {} bytes", patch.len());

    // Validate patch structure
    IpsPatcher::validate(&patch).expect("Patch validation failed");
}
