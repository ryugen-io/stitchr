//! Example of extracting and displaying patch metadata.
//!
//! This example demonstrates:
//! - Extracting metadata without applying the patch
//! - Displaying patch information
//! - Pre-flight size checks
//!
//! Run with: cargo run --example metadata_extraction

use rom_patcher_core::PatchFormat;
use rom_patcher_formats::ips::IpsPatcher;
use std::fs;

fn main() -> anyhow::Result<()> {
    println!("ROM Patcher RS - Metadata Extraction Example");
    println!("=============================================\n");

    // Paths - update these to your actual files
    let patch_path = "path/to/your/patch.ips";

    println!("Loading patch: {}", patch_path);
    let patch = fs::read(patch_path)?;
    println!("  File size: {} bytes\n", patch.len());

    // Check if it's a valid IPS patch
    println!("Checking patch format...");
    if !IpsPatcher::can_handle(&patch) {
        anyhow::bail!("Not a valid IPS patch file");
    }
    println!("  Format: IPS (International Patching System)\n");

    // Validate structure
    println!("Validating patch structure...");
    match IpsPatcher::validate(&patch) {
        Ok(_) => println!("  Patch structure is valid"),
        Err(e) => {
            println!("  Patch validation failed: {}", e);
            return Err(e.into());
        }
    }
    println!();

    // Extract metadata
    println!("Extracting metadata...");
    let metadata = IpsPatcher::metadata(&patch)?;

    println!("\nPatch Metadata:");
    println!("===============");
    println!("Format:        {}", metadata.patch_type.name());
    println!("Extension:     .{}", metadata.patch_type.extension());

    if let Some(source_size) = metadata.source_size {
        println!("Source size:   {} bytes ({:.2} KB)", source_size, source_size as f64 / 1024.0);
    } else {
        println!("Source size:   Not specified");
    }

    if let Some(target_size) = metadata.target_size {
        println!("Target size:   {} bytes ({:.2} KB)", target_size, target_size as f64 / 1024.0);

        // Calculate size difference
        if let Some(source_size) = metadata.source_size {
            let diff = target_size as i64 - source_size as i64;
            if diff > 0 {
                println!("Size change:   +{} bytes (ROM will grow)", diff);
            } else if diff < 0 {
                println!("Size change:   {} bytes (ROM will shrink)", diff);
            } else {
                println!("Size change:   No change");
            }
        }
    } else {
        println!("Target size:   Not specified (will be determined during patching)");
    }

    if let Some(ref source_checksum) = metadata.source_checksum {
        println!("Source CRC:    {:02X?}", source_checksum);
    } else {
        println!("Source CRC:    Not included (no verification available)");
    }

    if let Some(ref target_checksum) = metadata.target_checksum {
        println!("Target CRC:    {:02X?}", target_checksum);
    }

    if !metadata.extra.is_empty() {
        println!("\nAdditional Information:");
        for (key, value) in &metadata.extra {
            println!("  {}: {}", key, value);
        }
    }

    println!("\nNote: IPS patches typically don't include checksum information.");
    println!("      Consider using BPS or UPS formats for patches with validation.");

    Ok(())
}
