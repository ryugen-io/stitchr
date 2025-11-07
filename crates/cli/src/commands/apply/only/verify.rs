//! Verify-only mode handler

use anyhow::{Context, Result};
use rom_patcher_core::PatchType;

/// Handle --only verify mode
pub fn handle_verify_mode(
    original_rom: &[u8],
    patch_data: &[u8],
    patch_type: &PatchType,
) -> Result<()> {
    println!("Running in verify-only mode (no patching will be performed)");

    // IPS format has no embedded checksums - skip verification
    if patch_type.name() == "International Patching System" {
        println!("Note: IPS format does not support checksum verification (no embedded checksums)");
        return Ok(());
    }

    // Verify source checksum
    crate::commands::verify::verify_source(original_rom, patch_data, patch_type)
        .context("Source ROM checksum verification failed")?;

    println!("Verification completed successfully!");
    Ok(())
}
