//! RetroAchievements hash checking utilities

use rom_patcher_features::retroachievements::{Console, game_url, lookup_game_by_hash};
use rom_patcher_features::validation::algorithms::md5;
use std::path::Path;

/// Detect console type from file extension
pub fn detect_console(path: &Path) -> Option<Console> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())?;

    match ext.as_str() {
        "nes" => Some(Console::Nes),
        "smc" | "sfc" => Some(Console::Snes),
        "z64" | "n64" => Some(Console::N64),
        "gb" => Some(Console::Gb),
        "gbc" => Some(Console::Gbc),
        "gba" => Some(Console::Gba),
        "gen" | "md" | "smd" => Some(Console::Genesis),
        "sms" => Some(Console::MasterSystem),
        "gg" => Some(Console::GameGear),
        "bin" | "cue" => Some(Console::Psx),
        "iso" => Some(Console::Ps2),
        _ => None,
    }
}

/// Compute and display RetroAchievements info for patched ROM
pub fn check_and_display(rom: &[u8], output_path: &Path) {
    // Only check for supported consoles
    let Some(console) = detect_console(output_path) else {
        return;
    };

    // Compute MD5 hash
    let md5_hash = md5::compute(rom);

    println!("\nRetroAchievements compatibility:");
    println!("  Console: {:?}", console);
    println!("  MD5: {}", md5_hash);

    // Look up game on RA
    match lookup_game_by_hash(&md5_hash) {
        Ok(Some(game_id)) => {
            println!("  Status: Recognized by RetroAchievements");
            println!("  Game: {}", game_url(game_id));
        }
        Ok(None) => {
            println!("  Status: Not found in RetroAchievements database");
            println!("  Note: This ROM may not have achievements");
        }
        Err(e) => {
            println!("  Status: API lookup failed");
            println!("  Error: {}", e);
        }
    }
}
