//! Apply patch command with transactional safety

use anyhow::{Context, Result};
use rom_patcher_formats::detect_format;
use std::fs;
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

    println!("Loading ROM: {}", rom_path.display());
    let original_rom = fs::read(&rom_path).context("Failed to read ROM file")?;

    // Always show input ROM checksum
    #[cfg(feature = "validation")]
    {
        let input_crc = crate::utils::validation::compute_crc32(&original_rom);
        println!(
            "Input ROM CRC32: {}",
            crate::utils::validation::format_crc32(input_crc)
        );
    }

    println!("Loading patch: {}", patch_path.display());
    let patch_data = fs::read(&patch_path).context("Failed to read patch file")?;

    // Always show patch checksum
    #[cfg(feature = "validation")]
    {
        let patch_crc = crate::utils::validation::compute_crc32(&patch_data);
        println!(
            "Patch CRC32: {}",
            crate::utils::validation::format_crc32(patch_crc)
        );
    }

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

    // Write to temp file first, then atomic rename
    let temp_path = output_path.with_extension("tmp");
    fs::write(&temp_path, &patched_rom).context("Failed to write temporary output file")?;

    fs::rename(&temp_path, &output_path).context("Failed to finalize output file")?;

    println!("Successfully patched: {}", output_path.display());
    println!(
        "ROM size: {} → {} bytes",
        original_rom.len(),
        patched_rom.len()
    );

    // Always show output checksum
    #[cfg(feature = "validation")]
    {
        let output_crc = crate::utils::validation::compute_crc32(&patched_rom);
        println!(
            "Output ROM CRC32: {}",
            crate::utils::validation::format_crc32(output_crc)
        );
    }

    // RetroAchievements hash check (if enabled)
    #[cfg(feature = "retroachievements")]
    {
        crate::utils::retroachievements::check_and_display(&patched_rom, &output_path);
    }

    Ok(())
}
