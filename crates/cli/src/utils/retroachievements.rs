//! RetroAchievements hash checking utilities

use std::path::Path;
use stitchr_features::retroachievements::{
    Console, compute_n64_hash, compute_nds_hash, compute_nes_hash, compute_ps2_hash,
    compute_psp_hash, compute_psx_hash, compute_snes_hash, game_url, is_psp_iso,
    lookup_game_by_hash,
};
use stitchr_features::validation::algorithms::md5;

/// Detect console type from file extension
pub fn detect_console(path: &Path) -> Option<Console> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())?;

    match ext.as_str() {
        "nes" => Some(Console::Nes),
        "smc" | "sfc" | "swc" => Some(Console::Snes),
        "z64" | "n64" | "v64" => Some(Console::N64),
        "gb" => Some(Console::Gb),
        "gbc" => Some(Console::Gbc),
        "gba" => Some(Console::Gba),
        "nds" => Some(Console::Nds),
        "3ds" | "cci" | "cxi" => Some(Console::N3ds),
        "gen" | "md" | "smd" => Some(Console::Genesis),
        "sms" => Some(Console::MasterSystem),
        "gg" => Some(Console::GameGear),
        "bin" | "cue" => Some(Console::Psx),
        "iso" => Some(Console::Ps2),
        "cso" | "pbp" => Some(Console::Psp),
        _ => None,
    }
}

/// Compute console-specific hash for RetroAchievements
fn compute_ra_hash(rom: &[u8], path: &Path, console: Console) -> Result<String, String> {
    match console {
        Console::Nes => Ok(compute_nes_hash(rom)),
        Console::Snes => Ok(compute_snes_hash(rom)),
        Console::N64 => Ok(compute_n64_hash(rom)),
        Console::Nds => compute_nds_hash(rom),
        Console::Psx => compute_psx_hash(path),
        Console::Ps2 => compute_ps2_hash(path),
        Console::Psp => compute_psp_hash(path),
        // For cartridge-based systems without special handling, just MD5 the whole ROM
        _ => Ok(md5::compute(rom)),
    }
}

/// Compute and display RetroAchievements info for patched ROM
pub fn check_and_display(rom: &[u8], output_path: &Path) {
    // Only check for supported consoles
    let Some(mut console) = detect_console(output_path) else {
        return;
    };

    // For ISO files, distinguish between PS2 and PSP by checking content
    if console == Console::Ps2 && is_psp_iso(rom) {
        console = Console::Psp;
    }

    // Compute console-specific hash
    let md5_hash = match compute_ra_hash(rom, output_path, console) {
        Ok(hash) => hash,
        Err(e) => {
            println!("\nRetroAchievements compatibility:");
            println!("  Console: {:?}", console);
            println!("  Status: Hash computation failed");
            println!("  Error: {}", e);
            return;
        }
    };

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
