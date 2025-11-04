//! Example of applying a patch with validation and checksum verification.
//!
//! This example demonstrates best practices:
//! - Pre-validation of patch file
//! - CRC32 checksum computation
//! - Error handling
//! - Transactional safety (ROM cloned before patching)
//!
//! Run with: cargo run --example with_validation

use rom_patcher_core::PatchFormat;
use rom_patcher_features::validation::algorithms::crc32;
use rom_patcher_formats::ips::IpsPatcher;
use std::fs;

fn main() -> anyhow::Result<()> {
    // Paths - update these to your actual files
    let rom_path = "path/to/your/game.smc";
    let patch_path = "path/to/your/patch.ips";
    let output_path = "path/to/output/game_patched.smc";

    println!("ROM Patcher RS - Validation Example");
    println!("====================================\n");

    // Load files
    println!("Loading files...");
    let rom = fs::read(rom_path)?;
    let patch = fs::read(patch_path)?;
    println!("  ROM size: {} bytes", rom.len());
    println!("  Patch size: {} bytes\n", patch.len());

    // Compute input checksums
    println!("Computing checksums...");
    let rom_crc = crc32::compute(&rom);
    let patch_crc = crc32::compute(&patch);
    println!("  ROM CRC32:   {:08X}", rom_crc);
    println!("  Patch CRC32: {:08X}\n", patch_crc);

    // Validate patch before applying
    println!("Validating patch structure...");
    IpsPatcher::validate(&patch)?;
    println!("  Patch is valid!\n");

    // Extract metadata
    println!("Extracting patch metadata...");
    let metadata = IpsPatcher::metadata(&patch)?;
    if let Some(target_size) = metadata.target_size {
        println!("  Expected output size: {} bytes", target_size);
    }
    println!();

    // Clone ROM for transactional safety
    println!("Applying patch...");
    let mut patched_rom = rom.clone(); // Original ROM preserved
    let patcher = IpsPatcher;
    patcher.apply(&mut patched_rom, &patch)?;
    println!("  Patch applied successfully!");
    println!("  Output size: {} bytes\n", patched_rom.len());

    // Compute output checksum
    let output_crc = crc32::compute(&patched_rom);
    println!("Output CRC32: {:08X}\n", output_crc);

    // Save result
    println!("Saving patched ROM...");
    fs::write(output_path, &patched_rom)?;
    println!("  Saved to: {}\n", output_path);

    println!("Summary:");
    println!("  Input CRC32:  {:08X}", rom_crc);
    println!("  Output CRC32: {:08X}", output_crc);
    println!("  Size change:  {} → {} bytes", rom.len(), patched_rom.len());

    Ok(())
}
