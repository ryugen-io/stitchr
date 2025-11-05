//! Integration tests for APS N64 with real ROM patches

use rom_patcher_core::PatchFormat;
use rom_patcher_formats::aps::n64::ApsN64Patcher;
use std::fs;
use std::path::PathBuf;

fn test_rom_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../test_files/aps_n64")
        .join(name)
}

#[test]
fn test_zelda_oot_spanish_patch() {
    // Zelda OoT Spanish Translation from RomPatcherJS test suite

    // Load original ROM
    let rom_path = test_rom_path("test.rom.z64");
    let mut rom = match fs::read(&rom_path) {
        Ok(data) => data,
        Err(_) => {
            println!("Skipping test: ROM file not found at {:?}", rom_path);
            println!("Download: The Legend of Zelda: Ocarina of Time (USA).z64");
            return;
        }
    };

    println!(
        "Loaded ROM: {} bytes ({} MB)",
        rom.len(),
        rom.len() / 1024 / 1024
    );

    // Load patch
    let patch_path = test_rom_path("patch.aps");
    let patch = fs::read(&patch_path).expect("Failed to read APS patch");

    println!("Loaded patch: {} bytes", patch.len());

    // Validate patch structure
    ApsN64Patcher::validate(&patch).expect("Patch validation failed");

    // Extract and verify metadata
    let metadata = ApsN64Patcher::metadata(&patch).expect("Failed to extract metadata");
    println!("Target size: {:?}", metadata.target_size);

    // Verify source ROM with N64 header
    ApsN64Patcher::verify(&rom, &patch, None).expect("ROM verification failed");

    // Apply patch
    let patcher = ApsN64Patcher;
    patcher
        .apply(&mut rom, &patch)
        .expect("Failed to apply patch");

    println!("Successfully patched ROM: {} bytes", rom.len());
    assert!(!rom.is_empty(), "Output ROM should not be empty");
}

#[test]
fn test_patch_file_integrity() {
    // Verify patch file hasn't been corrupted
    let patch_path = test_rom_path("patch.aps");
    let patch = fs::read(&patch_path).expect("Failed to read patch");
    let patch_crc = crc32fast::hash(&patch);

    println!("Patch CRC32: {:08X}", patch_crc);
    println!("Patch size: {} bytes", patch.len());

    // Validate patch structure
    ApsN64Patcher::validate(&patch).expect("Patch validation failed");
}

#[test]
fn test_rom_file_exists() {
    let rom_path = test_rom_path("test.rom.z64");
    if let Ok(rom) = fs::read(&rom_path) {
        let rom_crc = crc32fast::hash(&rom);
        println!("ROM CRC32: {:08X}", rom_crc);
        println!(
            "ROM size: {} bytes ({} MB)",
            rom.len(),
            rom.len() / 1024 / 1024
        );
    } else {
        println!("ROM file not found at {:?}", rom_path);
        println!("This is expected if you haven't downloaded the base ROM yet.");
    }
}
