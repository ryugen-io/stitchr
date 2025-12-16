//! Integration tests for APS GBA with real ROM patches

use stitchr_core::PatchFormat;
use stitchr_formats::aps::gba::ApsGbaPatcher;
use std::fs;
use std::path::PathBuf;

fn test_rom_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../test_files/aps_gba")
        .join(name)
}

#[test]
fn test_patch_file_integrity() {
    let patch_path = test_rom_path("patch.aps");
    if let Ok(patch) = fs::read(&patch_path) {
        let patch_crc = crc32fast::hash(&patch);
        println!("Patch CRC32: {:08X}", patch_crc);
        println!("Patch size: {} bytes", patch.len());

        ApsGbaPatcher::validate(&patch).expect("Patch validation failed");
    } else {
        println!(
            "Skipping test: APS GBA patch file not found at {:?}",
            patch_path
        );
    }
}

#[test]
fn test_rom_file_exists() {
    let rom_path = test_rom_path("test.rom.gba");
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

#[test]
fn test_gba_translation_patch() {
    let rom_path = test_rom_path("test.rom.gba");
    let mut rom = match fs::read(&rom_path) {
        Ok(data) => data,
        Err(_) => {
            println!("Skipping test: ROM file not found at {:?}", rom_path);
            return;
        }
    };

    println!(
        "Loaded ROM: {} bytes ({} MB)",
        rom.len(),
        rom.len() / 1024 / 1024
    );

    let patch_path = test_rom_path("patch.aps");
    if !patch_path.exists() {
        println!("Skipping test: patch.aps not found");
        return;
    }
    let patch = fs::read(&patch_path).expect("Failed to read APS patch");

    println!("Loaded patch: {} bytes", patch.len());

    ApsGbaPatcher::validate(&patch).expect("Patch validation failed");

    let metadata = ApsGbaPatcher::metadata(&patch).expect("Failed to extract metadata");
    println!("Source size: {:?}", metadata.source_size);
    println!("Target size: {:?}", metadata.target_size);

    let patcher = ApsGbaPatcher;
    patcher
        .apply(&mut rom, &patch)
        .expect("Failed to apply patch");

    println!("Successfully patched ROM: {} bytes", rom.len());
    assert!(!rom.is_empty(), "Output ROM should not be empty");
}
