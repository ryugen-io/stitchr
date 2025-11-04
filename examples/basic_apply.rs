//! Basic example of applying an IPS patch to a ROM file.
//!
//! This example demonstrates the simplest use case: loading a ROM and patch,
//! applying the patch, and saving the result.
//!
//! Run with: cargo run --example basic_apply

use rom_patcher_core::PatchFormat;
use rom_patcher_formats::ips::IpsPatcher;
use std::fs;

fn main() -> anyhow::Result<()> {
    // Paths - update these to your actual files
    let rom_path = "path/to/your/game.smc";
    let patch_path = "path/to/your/patch.ips";
    let output_path = "path/to/output/game_patched.smc";

    println!("ROM Patcher RS - Basic Example");
    println!("================================\n");

    // Load ROM into memory
    println!("Loading ROM: {}", rom_path);
    let mut rom = fs::read(rom_path)?;
    println!("  ROM size: {} bytes", rom.len());

    // Load patch into memory
    println!("Loading patch: {}", patch_path);
    let patch = fs::read(patch_path)?;
    println!("  Patch size: {} bytes", patch.len());

    // Apply patch
    println!("\nApplying patch...");
    let patcher = IpsPatcher;
    patcher.apply(&mut rom, &patch)?;
    println!("  Patch applied successfully!");
    println!("  Patched ROM size: {} bytes", rom.len());

    // Save patched ROM
    println!("\nSaving patched ROM: {}", output_path);
    fs::write(output_path, &rom)?;
    println!("  Done!");

    Ok(())
}
