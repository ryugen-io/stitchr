//! Apply patch command with transactional safety

mod input;
mod only;
mod output;

use anyhow::{Context, Result};
use stitchr_formats::detect_format;
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
    patch_path: Option<PathBuf>,
    output_path: Option<PathBuf>,
    verify: bool,
    only_modes: Vec<stitchr_cli::OnlyMode>,
    verbose: u8,
) -> Result<()> {
    // Generate default output path if not specified (not needed for only-modes)
    let output_path = if only_modes.is_empty() {
        match output_path {
            Some(path) => path,
            None => crate::utils::paths::generate_default_output(&rom_path)?,
        }
    } else {
        // Dummy path for only-modes (won't be used)
        rom_path.with_extension("dummy")
    };

    if verbose > 0 {
        println!("Output path resolved to: {}", output_path.display());
    }

    // Safety check: prevent overwriting input (skip in only-modes)
    if only_modes.is_empty() && rom_path == output_path {
        anyhow::bail!(
            "Input and output paths cannot be the same. Use a different output path to preserve \
             the original ROM."
        );
    }

    // Handle --only modes
    if !only_modes.is_empty() {
        for mode in &only_modes {
            match mode {
                stitchr_cli::OnlyMode::Ra => {
                    only::handle_ra_mode(&rom_path, verbose)?;
                }
                stitchr_cli::OnlyMode::Verify => {
                    // Verify needs patch, handled below
                }
            }
        }

        // If only Ra mode (no verify), we're done
        if !only_modes
            .iter()
            .any(|m| matches!(m, stitchr_cli::OnlyMode::Verify))
        {
            return Ok(());
        }
    }

    // For all other modes, patch is required
    let patch_path = patch_path.expect("Patch path should be validated in main");

    // Load ROM and patch with checksum display
    let original_rom = input::load_rom_with_checksum(&rom_path, verbose)?;
    let patch_data = input::load_patch_with_checksum(&patch_path, verbose)?;

    // Auto-detect patch format
    let patch_type =
        detect_format(&patch_data).context("Could not detect patch format from file header")?;

    println!(
        "Detected format: {} ({})",
        patch_type.name(),
        patch_type.extension()
    );

    if verbose > 0 {
        println!("Internal patch type: {:?}", patch_type);
    }

    // Handle --only verify mode
    if only_modes
        .iter()
        .any(|m| matches!(m, stitchr_cli::OnlyMode::Verify))
    {
        return only::handle_verify_mode(&original_rom, &patch_data, &patch_type);
    }

    // Normal mode: apply patch with optional verification

    // Verify source checksum if requested
    if verify {
        // IPS format has no embedded checksums - skip verification
        if patch_type.name() == "International Patching System" {
            println!(
                "Note: IPS format does not support checksum verification (no embedded checksums)"
            );
        } else {
            if verbose > 0 {
                println!("Verifying source ROM checksum...");
            }
            super::verify::verify_source(&original_rom, &patch_data, &patch_type)
                .context("Source ROM checksum verification failed")?;
        }
    }

    // Clone ROM data for transactional patching (rollback on error)
    let mut patched_rom = original_rom.clone();

    // Apply patch with format-specific handler
    if verbose > 0 {
        println!("Applying patch data to ROM buffer...");
    }
    super::dispatch::apply_patch(&mut patched_rom, &patch_data, &patch_type)
        .context("Failed to apply patch")?;

    // Verify target checksum if requested
    if verify {
        // IPS format has no embedded checksums - skip verification
        if patch_type.name() != "International Patching System" {
            if verbose > 0 {
                println!("Verifying target ROM checksum...");
            }
            super::verify::verify_target(&original_rom, &patched_rom, &patch_data, &patch_type)
                .context("Target ROM checksum verification failed")?;
        }
    }

    // Write output with checksum display
    let original_size = original_rom.len();
    output::write_patched_rom(&patched_rom, original_size, &output_path, verbose)?;

    Ok(())
}
