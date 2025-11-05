//! Apply patch command with transactional safety

mod input;
mod output;

use anyhow::{Context, Result};
use rom_patcher_formats::detect_format;
use std::path::PathBuf;

/// Apply a patch to a ROM file with transactional safety
///
/// Safety features:
/// - Clones ROM data before patching (rollback on error)
/// - Validates input != output paths
/// - Writes to temp file first, then atomic rename
/// - Always shows CRC32 checksums for verification
/// - Optional source/target checksum verification (--verify flag)
pub fn execute(
    rom_path: PathBuf,
    patch_path: PathBuf,
    output_path: Option<PathBuf>,
    verify: bool,
) -> Result<()> {
    // Generate default output path if not specified
    let output_path = match output_path {
        Some(path) => path,
        None => crate::utils::paths::generate_default_output(&rom_path)?,
    };

    // Safety check: prevent overwriting input
    if rom_path == output_path {
        anyhow::bail!(
            "Input and output paths cannot be the same. Use a different output path to preserve the original ROM."
        );
    }

    // Load ROM and patch with checksum display
    let original_rom = input::load_rom_with_checksum(&rom_path)?;
    let patch_data = input::load_patch_with_checksum(&patch_path)?;

    // Auto-detect patch format
    let patch_type =
        detect_format(&patch_data).context("Could not detect patch format from file header")?;

    println!(
        "Detected format: {} ({})",
        patch_type.name(),
        patch_type.extension()
    );

    // Verify source checksum if requested
    if verify {
        super::verify::verify_source(&original_rom, &patch_data, &patch_type)
            .context("Source ROM checksum verification failed")?;
    }

    // Clone ROM data for transactional patching (rollback on error)
    let mut patched_rom = original_rom.clone();

    // Apply patch with format-specific handler
    super::dispatch::apply_patch(&mut patched_rom, &patch_data, &patch_type)
        .context("Failed to apply patch")?;

    // Verify target checksum if requested
    if verify {
        super::verify::verify_target(&patched_rom, &patch_data, &patch_type)
            .context("Target ROM checksum verification failed")?;
    }

    // Write output with checksum display
    let original_size = original_rom.len();
    output::write_patched_rom(&patched_rom, original_size, &output_path)?;

    Ok(())
}
