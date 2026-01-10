//! N64 hash algorithm for RetroAchievements
//!
//! N64 ROMs can be in different byte orders:
//! - Big Endian (.z64): Native format, hash directly
//! - Little Endian (.n64): Swap every 4 bytes
//! - Byte-swapped (.v64): Swap every 2 bytes
//!
//! RA expects Big Endian format for hashing.

/// N64 ROM format based on magic bytes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum N64Format {
    /// Big Endian (.z64) - native, no conversion needed
    BigEndian,
    /// Little Endian (.n64) - swap every 4 bytes
    LittleEndian,
    /// Byte-swapped (.v64) - swap every 2 bytes
    ByteSwapped,
    /// Unknown format
    Unknown,
}

/// Magic bytes for each format (first 4 bytes of ROM)
const MAGIC_BIG_ENDIAN: [u8; 4] = [0x80, 0x37, 0x12, 0x40];
const MAGIC_LITTLE_ENDIAN: [u8; 4] = [0x40, 0x12, 0x37, 0x80];
const MAGIC_BYTE_SWAPPED: [u8; 4] = [0x37, 0x80, 0x40, 0x12];

/// Detect N64 ROM format from magic bytes
fn detect_format(rom: &[u8]) -> N64Format {
    if rom.len() < 4 {
        return N64Format::Unknown;
    }

    let magic = &rom[0..4];
    if magic == MAGIC_BIG_ENDIAN {
        N64Format::BigEndian
    } else if magic == MAGIC_LITTLE_ENDIAN {
        N64Format::LittleEndian
    } else if magic == MAGIC_BYTE_SWAPPED {
        N64Format::ByteSwapped
    } else {
        N64Format::Unknown
    }
}

/// Convert ROM to Big Endian format
fn to_big_endian(rom: &[u8], format: N64Format) -> Vec<u8> {
    match format {
        N64Format::BigEndian | N64Format::Unknown => rom.to_vec(),
        N64Format::LittleEndian => {
            // Swap every 4 bytes: ABCD -> DCBA
            let mut result = rom.to_vec();
            for chunk in result.chunks_exact_mut(4) {
                chunk.reverse();
            }
            result
        }
        N64Format::ByteSwapped => {
            // Swap every 2 bytes: ABCD -> BADC
            let mut result = rom.to_vec();
            for chunk in result.chunks_exact_mut(2) {
                chunk.swap(0, 1);
            }
            result
        }
    }
}

/// Compute RetroAchievements hash for a N64 ROM
pub fn compute_n64_hash(rom: &[u8]) -> String {
    let format = detect_format(rom);
    let data = to_big_endian(rom, format);

    let digest = md5::compute(&data);
    format!("{:x}", digest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_format_big_endian() {
        let rom = [0x80, 0x37, 0x12, 0x40, 0x00, 0x00, 0x00, 0x00];
        assert_eq!(detect_format(&rom), N64Format::BigEndian);
    }

    #[test]
    fn test_detect_format_little_endian() {
        let rom = [0x40, 0x12, 0x37, 0x80, 0x00, 0x00, 0x00, 0x00];
        assert_eq!(detect_format(&rom), N64Format::LittleEndian);
    }

    #[test]
    fn test_detect_format_byte_swapped() {
        let rom = [0x37, 0x80, 0x40, 0x12, 0x00, 0x00, 0x00, 0x00];
        assert_eq!(detect_format(&rom), N64Format::ByteSwapped);
    }

    #[test]
    fn test_little_endian_conversion() {
        // Little endian: DCBA -> Big endian: ABCD
        let le_rom = [0x40, 0x12, 0x37, 0x80, 0x04, 0x03, 0x02, 0x01];
        let be_rom = to_big_endian(&le_rom, N64Format::LittleEndian);
        assert_eq!(be_rom, [0x80, 0x37, 0x12, 0x40, 0x01, 0x02, 0x03, 0x04]);
    }

    #[test]
    fn test_byte_swapped_conversion() {
        // Byte-swapped: BADC -> Big endian: ABCD
        let v64_rom = [0x37, 0x80, 0x40, 0x12, 0x02, 0x01, 0x04, 0x03];
        let be_rom = to_big_endian(&v64_rom, N64Format::ByteSwapped);
        assert_eq!(be_rom, [0x80, 0x37, 0x12, 0x40, 0x01, 0x02, 0x03, 0x04]);
    }

    #[test]
    fn test_big_endian_no_change() {
        let be_rom = [0x80, 0x37, 0x12, 0x40, 0x01, 0x02, 0x03, 0x04];
        let result = to_big_endian(&be_rom, N64Format::BigEndian);
        assert_eq!(result, be_rom);
    }
}
