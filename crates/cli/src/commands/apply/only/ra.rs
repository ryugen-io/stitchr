//! RetroAchievements check handler

use anyhow::Result;
use log::info;
use std::path::Path;

use crate::commands::apply::input;

/// Handle --only ra mode (RetroAchievements check)
pub fn handle_ra_mode(rom_path: &Path) -> Result<()> {
    info!("Running RetroAchievements check (ROM-only mode)");
    let rom = input::load_rom_with_checksum(rom_path)?;
    crate::utils::retroachievements::check_and_display(&rom, rom_path);
    Ok(())
}
