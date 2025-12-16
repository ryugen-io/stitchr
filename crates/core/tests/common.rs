// Common test utilities for stitchr crates

/// Generate a test ROM of specified size with repeating pattern
pub fn generate_test_rom(size: usize, pattern: u8) -> Vec<u8> {
    vec![pattern; size]
}

/// Generate a test ROM with specific pattern
pub fn generate_patterned_rom(size: usize) -> Vec<u8> {
    (0..size).map(|i| (i % 256) as u8).collect()
}

/// Apply modifications to a ROM at specific offsets
pub fn apply_modifications(rom: &mut [u8], modifications: &[(usize, u8)]) {
    for &(offset, value) in modifications {
        if offset < rom.len() {
            rom[offset] = value;
        }
    }
}

/// Create two ROMs that differ at specific offsets
pub fn create_rom_pair(size: usize, differences: &[(usize, u8, u8)]) -> (Vec<u8>, Vec<u8>) {
    let original = generate_patterned_rom(size);
    let mut modified = original.clone();

    for &(offset, _original_val, new_val) in differences {
        if offset < size {
            modified[offset] = new_val;
        }
    }

    (original, modified)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_test_rom() {
        let rom = generate_test_rom(100, 0x42);
        assert_eq!(rom.len(), 100);
        assert!(rom.iter().all(|&b| b == 0x42));
    }

    #[test]
    fn test_generate_patterned_rom() {
        let rom = generate_patterned_rom(300);
        assert_eq!(rom.len(), 300);
        assert_eq!(rom[0], 0);
        assert_eq!(rom[255], 255);
        assert_eq!(rom[256], 0);
    }

    #[test]
    fn test_apply_modifications() {
        let mut rom = vec![0u8; 100];
        apply_modifications(&mut rom, &[(10, 0xFF), (50, 0xAA)]);
        assert_eq!(rom[10], 0xFF);
        assert_eq!(rom[50], 0xAA);
        assert_eq!(rom[0], 0);
    }

    #[test]
    fn test_create_rom_pair() {
        let (original, modified) = create_rom_pair(100, &[(10, 10, 0xFF), (20, 20, 0xAA)]);
        assert_eq!(original.len(), 100);
        assert_eq!(modified.len(), 100);
        assert_eq!(modified[10], 0xFF);
        assert_eq!(modified[20], 0xAA);
        assert_eq!(original[10], 10);
        assert_eq!(original[20], 20);
    }
}
