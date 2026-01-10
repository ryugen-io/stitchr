//! RetroAchievements hash checking utilities

use log::{debug, error, info, trace, warn};
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

/// Get human-readable hash algorithm description
fn hash_algorithm_description(console: Console) -> &'static str {
    match console {
        Console::Nes => "iNES header skip + MD5",
        Console::Snes => "Copier header skip + MD5",
        Console::N64 => "Byte-order conversion + MD5",
        Console::Nds => "Header + ARM9 + ARM7 + Icon/Title MD5",
        Console::Psx => "SYSTEM.CNF BOOT= exe MD5",
        Console::Ps2 => "SYSTEM.CNF BOOT2= exe MD5",
        Console::Psp => "PARAM.SFO + EBOOT.BIN MD5",
        Console::Gb | Console::Gbc | Console::Gba => "Full ROM MD5",
        _ => "Full ROM MD5",
    }
}

/// Compute console-specific hash for RetroAchievements
fn compute_ra_hash(rom: &[u8], path: &Path, console: Console) -> Result<String, String> {
    info!("Algorithm: {}", hash_algorithm_description(console));

    match console {
        Console::Nes => {
            let has_header = rom.len() >= 4 && &rom[0..4] == b"NES\x1a";
            debug!(
                "iNES header: {}",
                if has_header {
                    "detected (16 bytes)"
                } else {
                    "not present"
                }
            );
            trace!(
                "Hashing: {} bytes",
                if has_header {
                    rom.len() - 16
                } else {
                    rom.len()
                }
            );
            Ok(compute_nes_hash(rom))
        }
        Console::Snes => {
            let has_header = rom.len() > 512 && rom.len() % 8192 == 512;
            debug!(
                "Copier header: {}",
                if has_header {
                    "detected (512 bytes)"
                } else {
                    "not present"
                }
            );
            trace!(
                "Hashing: {} bytes",
                if has_header {
                    rom.len() - 512
                } else {
                    rom.len()
                }
            );
            Ok(compute_snes_hash(rom))
        }
        Console::N64 => {
            let format = if rom.len() >= 4 {
                match &rom[0..4] {
                    [0x80, 0x37, 0x12, 0x40] => "Big Endian (.z64)",
                    [0x40, 0x12, 0x37, 0x80] => "Little Endian (.n64)",
                    [0x37, 0x80, 0x40, 0x12] => "Byte-swapped (.v64)",
                    _ => "Unknown",
                }
            } else {
                "Unknown"
            };
            debug!("ROM format: {}", format);
            trace!("Converting to Big Endian for hash");
            Ok(compute_n64_hash(rom))
        }
        Console::Nds => {
            if rom.len() >= 0x160 {
                let arm9_offset = u32::from_le_bytes([rom[0x20], rom[0x21], rom[0x22], rom[0x23]]);
                let arm9_size = u32::from_le_bytes([rom[0x2C], rom[0x2D], rom[0x2E], rom[0x2F]]);
                let arm7_offset = u32::from_le_bytes([rom[0x30], rom[0x31], rom[0x32], rom[0x33]]);
                let arm7_size = u32::from_le_bytes([rom[0x3C], rom[0x3D], rom[0x3E], rom[0x3F]]);
                let icon_offset = u32::from_le_bytes([rom[0x68], rom[0x69], rom[0x6A], rom[0x6B]]);
                debug!("Header: 0x160 bytes");
                debug!("ARM9: offset=0x{:X}, size={} bytes", arm9_offset, arm9_size);
                debug!("ARM7: offset=0x{:X}, size={} bytes", arm7_offset, arm7_size);
                trace!("Icon/Title: offset=0x{:X}, size=0xA00 bytes", icon_offset);
            }
            compute_nds_hash(rom)
        }
        Console::Psx => {
            debug!("Parsing ISO9660 filesystem...");
            trace!("Reading SYSTEM.CNF for BOOT= executable");
            compute_psx_hash(path)
        }
        Console::Ps2 => {
            debug!("Parsing ISO9660 filesystem...");
            trace!("Reading SYSTEM.CNF for BOOT2= executable");
            compute_ps2_hash(path)
        }
        Console::Psp => {
            debug!("Parsing ISO9660 filesystem...");
            trace!("Reading PSP_GAME/PARAM.SFO");
            trace!("Reading PSP_GAME/SYSDIR/EBOOT.BIN");
            compute_psp_hash(path)
        }
        // For cartridge-based systems without special handling, just MD5 the whole ROM
        _ => {
            trace!("Hashing full ROM: {} bytes", rom.len());
            Ok(md5::compute(rom))
        }
    }
}

/// Compute and display RetroAchievements info for patched ROM
pub fn check_and_display(rom: &[u8], output_path: &Path) {
    // Only check for supported consoles
    let Some(mut console) = detect_console(output_path) else {
        warn!("RetroAchievements: Unsupported file extension");
        return;
    };

    info!("Detected console from extension: {:?}", console);

    // For ISO files, distinguish between PS2 and PSP by checking content
    if console == Console::Ps2 {
        debug!("Checking ISO content for PSP signature...");
        if is_psp_iso(rom) {
            info!("PSP_GAME found - reclassifying as PSP");
            console = Console::Psp;
        } else {
            trace!("No PSP signature - keeping as PS2");
        }
    }

    // Compute console-specific hash
    info!("Computing RetroAchievements hash...");
    let md5_hash = match compute_ra_hash(rom, output_path, console) {
        Ok(hash) => hash,
        Err(e) => {
            println!("\nRetroAchievements compatibility:");
            println!("  Console: {:?}", console);
            error!("Hash computation failed: {}", e);
            return;
        }
    };

    println!("\nRetroAchievements compatibility:");
    println!("  Console: {:?}", console);
    println!("  MD5: {}", md5_hash);

    // Look up game on RA
    debug!("Querying RetroAchievements API...");
    match lookup_game_by_hash(&md5_hash) {
        Ok(Some(game_id)) => {
            println!("  Status: Recognized by RetroAchievements");
            println!("  Game: {}", game_url(game_id));
            debug!("Game ID: {}", game_id);
        }
        Ok(None) => {
            println!("  Status: Not found in RetroAchievements database");
            println!("  Note: This ROM may not have achievements");
        }
        Err(e) => {
            error!("API lookup failed: {}", e);
        }
    }
}
