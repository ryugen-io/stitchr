//! RetroAchievements check handler

use anyhow::Result;
use std::path::Path;

use crate::commands::apply::input;

/// Handle --only ra mode (RetroAchievements check)
pub fn handle_ra_mode(rom_path: &Path, quiet: bool) -> Result<()> {
    if !quiet {
        println!("Running RetroAchievements check (ROM-only mode)");
    }
    let rom = input::load_rom_with_checksum(rom_path, quiet)?;
    // We probably want RA output even in quiet mode if it's the specific task?
    // Or maybe quiet suppresses everything. Let's assume quiet suppresses standard info.
    // check_and_display prints a lot. We won't modify it for now to avoid scope creep,
    // but at least we fix the build error.
    crate::utils::retroachievements::check_and_display(&rom, rom_path);
    Ok(())
}
