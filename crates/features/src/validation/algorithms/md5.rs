//! MD5 hash implementation

/// MD5 hasher
pub struct Md5Hasher {
    context: md5::Context,
}

impl Md5Hasher {
    /// Create a new MD5 hasher
    pub fn new() -> Self {
        Self {
            context: md5::Context::new(),
        }
    }

    /// Update hash with data
    pub fn update(&mut self, data: &[u8]) {
        self.context.consume(data);
    }

    /// Finalize and return MD5 hash as hex string
    pub fn finalize(self) -> String {
        let digest = self.context.compute();
        format!("{:x}", digest)
    }
}

impl Default for Md5Hasher {
    fn default() -> Self {
        Self::new()
    }
}

/// Compute MD5 hash of data
pub fn compute(data: &[u8]) -> String {
    let mut hasher = Md5Hasher::new();
    hasher.update(data);
    hasher.finalize()
}
