//! RetroAchievements types and hash checker

use stitchr_core::Result;

/// Supported console types for RetroAchievements
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Console {
    Nes,
    Snes,
    N64,
    Gb,
    Gbc,
    Gba,
    Nds,
    N3ds,
    Genesis,
    MasterSystem,
    GameGear,
    Psx,
    Ps2,
    Psp,
}

/// RetroAchievements hash checker
pub struct RaHashChecker;

impl RaHashChecker {
    /// Create a new RA hash checker
    pub fn new() -> Self {
        Self
    }

    /// Compute RetroAchievements hash for a ROM
    pub fn compute_hash(&self, _rom: &[u8], _console: Console) -> Result<String> {
        // TODO: Implement console-specific hashing
        // Each console has different hashing requirements
        Ok("NOT_IMPLEMENTED".to_string())
    }
}

impl Default for RaHashChecker {
    fn default() -> Self {
        Self::new()
    }
}
