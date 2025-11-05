//! Path utilities for output file generation

use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Generate default output path: {rom_dir}/patched/{stem}.patched.{ext}
pub fn generate_default_output(rom_path: &Path) -> Result<PathBuf> {
    let rom_dir = rom_path
        .parent()
        .context("ROM file has no parent directory")?;

    let patched_dir = rom_dir.join("patched");
    fs::create_dir_all(&patched_dir).context("Failed to create patched/ directory")?;

    let file_stem = rom_path
        .file_stem()
        .and_then(|s| s.to_str())
        .context("ROM file has invalid filename")?;

    let extension = rom_path.extension().and_then(|s| s.to_str()).unwrap_or("");

    let output_filename = if extension.is_empty() {
        format!("{}.patched", file_stem)
    } else {
        format!("{}.patched.{}", file_stem, extension)
    };

    Ok(patched_dir.join(output_filename))
}
