//! Output file writing with atomic rename and checksum display

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// Write patched ROM to output path with atomic rename
///
/// Safety: Writes to temp file first, then atomically renames
pub fn write_patched_rom(
    patched_rom: &[u8],
    original_size: usize,
    output_path: &Path,
) -> Result<()> {
    // Write to temp file first, then atomic rename
    let temp_path = output_path.with_extension("tmp");
    fs::write(&temp_path, patched_rom).context("Failed to write temporary output file")?;
    fs::rename(&temp_path, output_path).context("Failed to finalize output file")?;

    println!("Successfully patched: {}", output_path.display());
    println!(
        "ROM size: {} → {} bytes",
        original_size,
        patched_rom.len()
    );

    // Always show output checksum
    #[cfg(feature = "validation")]
    {
        let crc = crate::utils::validation::compute_crc32(patched_rom);
        println!(
            "Output ROM CRC32: {}",
            crate::utils::validation::format_crc32(crc)
        );
    }

    // RetroAchievements hash check (if enabled)
    #[cfg(feature = "retroachievements")]
    {
        crate::utils::retroachievements::check_and_display(patched_rom, output_path);
    }

    Ok(())
}
