//! RetroAchievements CLI utilities tests

#[cfg(feature = "retroachievements")]
use rom_patcher_cli::utils::retroachievements::detect_console;
#[cfg(feature = "retroachievements")]
use rom_patcher_features::retroachievements::Console;
#[cfg(feature = "retroachievements")]
use std::path::Path;

#[test]
#[cfg(feature = "retroachievements")]
fn test_detect_console_gameboy() {
    assert_eq!(detect_console(Path::new("game.gb")), Some(Console::Gb));
    assert_eq!(detect_console(Path::new("game.gbc")), Some(Console::Gbc));
    assert_eq!(detect_console(Path::new("game.gba")), Some(Console::Gba));
}

#[test]
#[cfg(feature = "retroachievements")]
fn test_detect_console_nintendo() {
    assert_eq!(detect_console(Path::new("game.nes")), Some(Console::Nes));
    assert_eq!(detect_console(Path::new("game.smc")), Some(Console::Snes));
    assert_eq!(detect_console(Path::new("game.sfc")), Some(Console::Snes));
    assert_eq!(detect_console(Path::new("game.n64")), Some(Console::N64));
    assert_eq!(detect_console(Path::new("game.z64")), Some(Console::N64));
}

#[test]
#[cfg(feature = "retroachievements")]
fn test_detect_console_sega() {
    assert_eq!(
        detect_console(Path::new("game.gen")),
        Some(Console::Genesis)
    );
    assert_eq!(detect_console(Path::new("game.md")), Some(Console::Genesis));
    assert_eq!(
        detect_console(Path::new("game.smd")),
        Some(Console::Genesis)
    );
    assert_eq!(
        detect_console(Path::new("game.sms")),
        Some(Console::MasterSystem)
    );
    assert_eq!(
        detect_console(Path::new("game.gg")),
        Some(Console::GameGear)
    );
}

#[test]
#[cfg(feature = "retroachievements")]
fn test_detect_console_playstation() {
    assert_eq!(detect_console(Path::new("game.bin")), Some(Console::Psx));
    assert_eq!(detect_console(Path::new("game.cue")), Some(Console::Psx));
    assert_eq!(detect_console(Path::new("game.iso")), Some(Console::Ps2));
}

#[test]
#[cfg(feature = "retroachievements")]
fn test_detect_console_unsupported() {
    assert_eq!(detect_console(Path::new("game.txt")), None);
    assert_eq!(detect_console(Path::new("game.zip")), None);
    assert_eq!(detect_console(Path::new("game.exe")), None);
    assert_eq!(detect_console(Path::new("game")), None);
}

#[test]
#[cfg(feature = "retroachievements")]
fn test_detect_console_case_insensitive() {
    assert_eq!(detect_console(Path::new("game.GB")), Some(Console::Gb));
    assert_eq!(detect_console(Path::new("game.GBA")), Some(Console::Gba));
    assert_eq!(detect_console(Path::new("game.NES")), Some(Console::Nes));
}

#[test]
#[cfg(feature = "retroachievements")]
fn test_detect_console_with_path() {
    assert_eq!(
        detect_console(Path::new("/path/to/game.gb")),
        Some(Console::Gb)
    );
    assert_eq!(
        detect_console(Path::new("../relative/path/game.nes")),
        Some(Console::Nes)
    );
}
