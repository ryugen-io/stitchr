//! CRC32 checksum implementation

/// CRC32 hasher with streaming support
pub struct Crc32Hasher {
    hasher: crc32fast::Hasher,
}

impl Crc32Hasher {
    /// Create a new CRC32 hasher
    pub fn new() -> Self {
        Self {
            hasher: crc32fast::Hasher::new(),
        }
    }

    /// Update hash with data
    pub fn update(&mut self, data: &[u8]) {
        self.hasher.update(data);
    }

    /// Finalize and return CRC32 checksum
    pub fn finalize(self) -> u32 {
        self.hasher.finalize()
    }
}

impl Default for Crc32Hasher {
    fn default() -> Self {
        Self::new()
    }
}

/// Compute CRC32 checksum of data
pub fn compute(data: &[u8]) -> u32 {
    crc32fast::hash(data)
}
