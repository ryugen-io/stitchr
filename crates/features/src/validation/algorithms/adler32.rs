//! Adler32 checksum implementation

const MOD_ADLER: u32 = 65521;

/// Adler32 hasher with streaming support
pub struct Adler32Hasher {
    a: u32,
    b: u32,
}

impl Adler32Hasher {
    /// Create a new Adler32 hasher
    pub fn new() -> Self {
        Self { a: 1, b: 0 }
    }

    /// Update hash with data
    pub fn update(&mut self, data: &[u8]) {
        for byte in data {
            self.a = (self.a + *byte as u32) % MOD_ADLER;
            self.b = (self.b + self.a) % MOD_ADLER;
        }
    }

    /// Finalize and return Adler32 checksum
    pub fn finalize(self) -> u32 {
        (self.b << 16) | self.a
    }
}

impl Default for Adler32Hasher {
    fn default() -> Self {
        Self::new()
    }
}

/// Compute Adler32 checksum of data
pub fn compute(data: &[u8]) -> u32 {
    let mut hasher = Adler32Hasher::new();
    hasher.update(data);
    hasher.finalize()
}
