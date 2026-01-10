//! PlayStation Portable hash algorithm for RetroAchievements
//!
//! Algorithm:
//! 1. Read PSP_GAME/PARAM.SFO (game metadata)
//! 2. Read PSP_GAME/SYSDIR/EBOOT.BIN (executable)
//! 3. Concatenate PARAM.SFO + EBOOT.BIN
//! 4. MD5 hash the buffer

use std::path::Path;

use super::iso9660::{Iso9660Reader, resolve_image_path};

/// Compute RetroAchievements hash for a PSP disc image
pub fn compute_psp_hash(path: &Path) -> Result<String, String> {
    let image_path = resolve_image_path(path)?;
    let image_data =
        std::fs::read(&image_path).map_err(|e| format!("Failed to read image: {}", e))?;

    let iso = Iso9660Reader::new(&image_data)?;

    // Read PARAM.SFO
    let param_sfo = iso.read_file_path("PSP_GAME/PARAM.SFO")?;

    // Read EBOOT.BIN
    let eboot_bin = iso.read_file_path("PSP_GAME/SYSDIR/EBOOT.BIN")?;

    // Build hash buffer: PARAM.SFO + EBOOT.BIN
    let mut buffer = Vec::with_capacity(param_sfo.len() + eboot_bin.len());
    buffer.extend_from_slice(&param_sfo);
    buffer.extend_from_slice(&eboot_bin);

    let digest = md5::compute(&buffer);
    Ok(format!("{:x}", digest))
}

#[cfg(test)]
mod tests {
    // PSP hash tests would require actual ISO files
    // Basic structure tests can be added here if needed
}
