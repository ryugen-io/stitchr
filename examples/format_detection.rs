//! Example of automatic patch format detection.
//!
//! This example demonstrates:
//! - Auto-detecting patch format from file contents
//! - Format-agnostic patching
//! - Handling multiple patch types
//!
//! Run with: cargo run --example format_detection

use rom_patcher_core::{PatchFormat, PatchType};
use rom_patcher_formats::{detect_format, ips::IpsPatcher};
use std::fs;

fn apply_patch_auto(rom: &mut Vec<u8>, patch: &[u8]) -> anyhow::Result<PatchType> {
    // Auto-detect format
    let format = detect_format(patch)
        .ok_or_else(|| anyhow::anyhow!("Unknown patch format"))?;

    println!("  Detected format: {} (.{})", format.name(), format.extension());

    // Apply patch based on detected format
    match format {
        PatchType::Ips => {
            let patcher = IpsPatcher;
            patcher.apply(rom, patch)?;
        }
        PatchType::Bps => {
            // BPS not yet implemented
            anyhow::bail!("BPS format not yet implemented");
        }
        PatchType::Ups => {
            // UPS not yet implemented
            anyhow::bail!("UPS format not yet implemented");
        }
        _ => {
            anyhow::bail!("Format {} not yet implemented", format.name());
        }
    }

    Ok(format)
}

fn main() -> anyhow::Result<()> {
    println!("ROM Patcher RS - Format Detection Example");
    println!("==========================================\n");

    // Paths - update these to your actual files
    let rom_path = "path/to/your/game.smc";
    let patch_path = "path/to/your/patch.ips"; // Can be .ips, .bps, .ups, etc.
    let output_path = "path/to/output/game_patched.smc";

    println!("Loading files...");
    let mut rom = fs::read(rom_path)?;
    let patch = fs::read(patch_path)?;
    println!("  ROM size: {} bytes", rom.len());
    println!("  Patch size: {} bytes\n", patch.len());

    println!("Detecting patch format...");
    let format = apply_patch_auto(&mut rom, &patch)?;
    println!("  Patch applied using {} format!\n", format.name());

    println!("Saving patched ROM...");
    fs::write(output_path, &rom)?;
    println!("  Saved to: {}\n", output_path);

    println!("Done! Your ROM has been patched.");

    Ok(())
}
