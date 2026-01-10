//! SNES hash algorithm for RetroAchievements
//!
//! If the ROM size is 512 bytes more than a multiple of 8KB (32768 bytes),
//! skip the first 512 bytes (SMC/SWC copier header).
//! Otherwise, hash the entire file.

/// SMC/SWC copier header size
const COPIER_HEADER_SIZE: usize = 512;
/// SNES ROM block size (8KB)
const BLOCK_SIZE: usize = 8 * 1024;

/// Compute RetroAchievements hash for a SNES ROM
pub fn compute_snes_hash(rom: &[u8]) -> String {
    let data = if has_copier_header(rom.len()) {
        // Skip copier header
        &rom[COPIER_HEADER_SIZE..]
    } else {
        rom
    };

    let digest = md5::compute(data);
    format!("{:x}", digest)
}

/// Check if ROM has a copier header (512 bytes over a multiple of 8KB)
#[inline]
fn has_copier_header(size: usize) -> bool {
    size > COPIER_HEADER_SIZE && size % BLOCK_SIZE == COPIER_HEADER_SIZE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snes_with_copier_header() {
        // ROM with 512-byte copier header + 8KB of data
        let mut rom = vec![0x00; COPIER_HEADER_SIZE]; // Copier header
        rom.extend(vec![0xAB; BLOCK_SIZE]); // 8KB ROM data

        assert!(has_copier_header(rom.len()));

        let hash = compute_snes_hash(&rom);
        let expected = format!("{:x}", md5::compute(vec![0xAB; BLOCK_SIZE]));
        assert_eq!(hash, expected);
    }

    #[test]
    fn test_snes_without_header() {
        // Clean ROM (exact multiple of 8KB)
        let rom = vec![0xCD; BLOCK_SIZE * 2]; // 16KB

        assert!(!has_copier_header(rom.len()));

        let hash = compute_snes_hash(&rom);
        let expected = format!("{:x}", md5::compute(&rom));
        assert_eq!(hash, expected);
    }

    #[test]
    fn test_has_copier_header() {
        assert!(has_copier_header(512 + 8192)); // 512 + 8KB
        assert!(has_copier_header(512 + 8192 * 4)); // 512 + 32KB
        assert!(!has_copier_header(8192)); // Exact 8KB
        assert!(!has_copier_header(8192 * 2)); // Exact 16KB
        assert!(!has_copier_header(1000)); // Random size
    }
}
