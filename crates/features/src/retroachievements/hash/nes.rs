//! NES hash algorithm for RetroAchievements
//!
//! If the ROM starts with "NES\x1a" (iNES header), skip the first 16 bytes.
//! Otherwise, hash the entire file.

/// NES iNES header magic bytes
const INES_MAGIC: &[u8] = b"NES\x1a";
/// iNES header size
const INES_HEADER_SIZE: usize = 16;

/// Compute RetroAchievements hash for a NES ROM
pub fn compute_nes_hash(rom: &[u8]) -> String {
    let data = if rom.starts_with(INES_MAGIC) {
        // Skip iNES header
        &rom[INES_HEADER_SIZE..]
    } else {
        rom
    };

    let digest = md5::compute(data);
    format!("{:x}", digest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nes_with_header() {
        // NES ROM with iNES header
        let mut rom = vec![0x4E, 0x45, 0x53, 0x1A]; // "NES\x1a"
        rom.extend(vec![0; 12]); // Rest of 16-byte header
        rom.extend(vec![0xAB; 32]); // ROM data

        let hash = compute_nes_hash(&rom);
        // Should hash only the 32 bytes of ROM data (0xAB repeated)
        let expected = format!("{:x}", md5::compute(vec![0xAB; 32]));
        assert_eq!(hash, expected);
    }

    #[test]
    fn test_nes_without_header() {
        // Raw NES ROM without header
        let rom = vec![0xCD; 64];
        let hash = compute_nes_hash(&rom);
        let expected = format!("{:x}", md5::compute(&rom));
        assert_eq!(hash, expected);
    }
}
