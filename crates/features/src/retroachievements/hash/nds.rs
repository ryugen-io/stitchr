//! Nintendo DS hash algorithm for RetroAchievements
//!
//! Algorithm:
//! 1. Read 0x160-byte ROM header
//! 2. Read ARM9 boot code (offset at header+0x20, size at header+0x2C)
//! 3. Read ARM7 boot code (offset at header+0x30, size at header+0x3C)
//! 4. Read Icon/Title data (0xA00 bytes at offset from header+0x68)
//! 5. MD5 hash the concatenated buffer

/// NDS header size
const HEADER_SIZE: usize = 0x160;
/// Icon/title data size
const ICON_TITLE_SIZE: usize = 0xA00;

/// Compute RetroAchievements hash for a Nintendo DS ROM
pub fn compute_nds_hash(rom: &[u8]) -> Result<String, String> {
    if rom.len() < HEADER_SIZE {
        return Err("ROM too small for NDS header".to_string());
    }

    // Read header
    let header = &rom[0..HEADER_SIZE];

    // ARM9 boot code: offset at 0x20 (4 bytes LE), size at 0x2C (4 bytes LE)
    let arm9_offset =
        u32::from_le_bytes([header[0x20], header[0x21], header[0x22], header[0x23]]) as usize;
    let arm9_size =
        u32::from_le_bytes([header[0x2C], header[0x2D], header[0x2E], header[0x2F]]) as usize;

    // ARM7 boot code: offset at 0x30 (4 bytes LE), size at 0x3C (4 bytes LE)
    let arm7_offset =
        u32::from_le_bytes([header[0x30], header[0x31], header[0x32], header[0x33]]) as usize;
    let arm7_size =
        u32::from_le_bytes([header[0x3C], header[0x3D], header[0x3E], header[0x3F]]) as usize;

    // Icon/title offset at 0x68 (4 bytes LE)
    let icon_offset =
        u32::from_le_bytes([header[0x68], header[0x69], header[0x6A], header[0x6B]]) as usize;

    // Validate offsets
    if arm9_offset + arm9_size > rom.len() {
        return Err(format!(
            "ARM9 code out of bounds: offset={}, size={}, rom_len={}",
            arm9_offset,
            arm9_size,
            rom.len()
        ));
    }
    if arm7_offset + arm7_size > rom.len() {
        return Err(format!(
            "ARM7 code out of bounds: offset={}, size={}, rom_len={}",
            arm7_offset,
            arm7_size,
            rom.len()
        ));
    }
    if icon_offset + ICON_TITLE_SIZE > rom.len() {
        return Err(format!(
            "Icon/title out of bounds: offset={}, size={}, rom_len={}",
            icon_offset,
            ICON_TITLE_SIZE,
            rom.len()
        ));
    }

    // Build hash buffer: header + ARM9 + ARM7 + icon/title
    let mut buffer = Vec::with_capacity(HEADER_SIZE + arm9_size + arm7_size + ICON_TITLE_SIZE);
    buffer.extend_from_slice(header);
    buffer.extend_from_slice(&rom[arm9_offset..arm9_offset + arm9_size]);
    buffer.extend_from_slice(&rom[arm7_offset..arm7_offset + arm7_size]);
    buffer.extend_from_slice(&rom[icon_offset..icon_offset + ICON_TITLE_SIZE]);

    let digest = md5::compute(&buffer);
    Ok(format!("{:x}", digest))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nds_hash_too_small() {
        let rom = vec![0u8; 100];
        assert!(compute_nds_hash(&rom).is_err());
    }

    #[test]
    fn test_nds_hash_invalid_offsets() {
        // Create minimal header with invalid offsets
        let mut rom = vec![0u8; HEADER_SIZE + 100];
        // Set ARM9 offset to point beyond ROM
        rom[0x20..0x24].copy_from_slice(&[0xFF, 0xFF, 0xFF, 0x7F]);
        assert!(compute_nds_hash(&rom).is_err());
    }
}
