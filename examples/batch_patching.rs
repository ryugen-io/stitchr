//! Example of batch patching multiple ROMs with the same patch.
//!
//! This example demonstrates:
//! - Processing multiple files
//! - Error handling for batch operations
//! - Progress reporting
//!
//! Run with: cargo run --example batch_patching

use stitchr_core::PatchFormat;
use stitchr_features::validation::algorithms::crc32;
use stitchr_formats::ips::IpsPatcher;
use std::fs;
use std::path::Path;

struct PatchResult {
    rom_name: String,
    input_crc: u32,
    output_crc: u32,
    success: bool,
    error: Option<String>,
}

fn patch_single_rom(
    rom_path: &Path,
    patch: &[u8],
    output_dir: &Path,
) -> anyhow::Result<PatchResult> {
    let rom_name = rom_path
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();

    // Load ROM
    let mut rom = fs::read(rom_path)?;
    let input_crc = crc32::compute(&rom);

    // Apply patch
    let patcher = IpsPatcher;
    patcher.apply(&mut rom, patch)?;
    let output_crc = crc32::compute(&rom);

    // Save output
    let output_path = output_dir.join(format!("patched_{}", rom_name));
    fs::write(output_path, &rom)?;

    Ok(PatchResult {
        rom_name,
        input_crc,
        output_crc,
        success: true,
        error: None,
    })
}

fn main() -> anyhow::Result<()> {
    println!("ROM Patcher RS - Batch Patching Example");
    println!("========================================\n");

    // Configuration
    let patch_path = "path/to/your/patch.ips";
    let rom_dir = "path/to/roms";
    let output_dir = "path/to/output";

    // Create output directory
    fs::create_dir_all(output_dir)?;

    // Load patch once (reused for all ROMs)
    println!("Loading patch: {}", patch_path);
    let patch = fs::read(patch_path)?;
    let patch_crc = crc32::compute(&patch);
    println!("  Patch CRC32: {:08X}", patch_crc);
    println!("  Patch size: {} bytes\n", patch.len());

    // Validate patch once
    println!("Validating patch...");
    IpsPatcher::validate(&patch)?;
    println!("  Patch is valid!\n");

    // Find all ROM files (adjust extensions as needed)
    let rom_extensions = vec!["smc", "sfc", "nes", "gba", "gb", "gbc"];
    let mut rom_files = Vec::new();

    for entry in fs::read_dir(rom_dir)? {
        let entry = entry?;
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if rom_extensions.contains(&ext.to_string_lossy().to_lowercase().as_ref()) {
                rom_files.push(path);
            }
        }
    }

    println!("Found {} ROM file(s) to patch\n", rom_files.len());

    // Patch each ROM
    let mut results = Vec::new();
    for (i, rom_path) in rom_files.iter().enumerate() {
        let rom_name = rom_path.file_name().unwrap().to_string_lossy();
        print!("[{}/{}] Patching {}... ", i + 1, rom_files.len(), rom_name);

        match patch_single_rom(rom_path, &patch, Path::new(output_dir)) {
            Ok(result) => {
                println!("OK (CRC32: {:08X})", result.output_crc);
                results.push(result);
            }
            Err(e) => {
                println!("FAILED: {}", e);
                results.push(PatchResult {
                    rom_name: rom_name.to_string(),
                    input_crc: 0,
                    output_crc: 0,
                    success: false,
                    error: Some(e.to_string()),
                });
            }
        }
    }

    // Summary
    println!("\n========================================");
    println!("Batch Patching Complete");
    println!("========================================\n");

    let successful = results.iter().filter(|r| r.success).count();
    let failed = results.iter().filter(|r| !r.success).count();

    println!("Total:      {}", results.len());
    println!("Successful: {}", successful);
    println!("Failed:     {}\n", failed);

    if failed > 0 {
        println!("Failed ROMs:");
        for result in results.iter().filter(|r| !r.success) {
            println!("  - {}: {}", result.rom_name, result.error.as_ref().unwrap());
        }
    }

    Ok(())
}
