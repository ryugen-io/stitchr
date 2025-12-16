//! SHA hash implementations

use sha1::{Digest, Sha1};
use sha2::Sha256;

/// SHA-1 hasher
pub struct Sha1Hasher {
    hasher: Sha1,
}

impl Sha1Hasher {
    /// Create a new SHA-1 hasher
    pub fn new() -> Self {
        Self {
            hasher: Sha1::new(),
        }
    }

    /// Update hash with data
    pub fn update(&mut self, data: &[u8]) {
        self.hasher.update(data);
    }

    /// Finalize and return SHA-1 hash as hex string
    pub fn finalize(self) -> String {
        let digest = self.hasher.finalize();
        format!("{:x}", digest)
    }
}

impl Default for Sha1Hasher {
    fn default() -> Self {
        Self::new()
    }
}

/// Compute SHA-1 hash of data
pub fn compute_sha1(data: &[u8]) -> String {
    let mut hasher = Sha1Hasher::new();
    hasher.update(data);
    hasher.finalize()
}

/// SHA-256 hasher
pub struct Sha256Hasher {
    hasher: Sha256,
}

impl Sha256Hasher {
    /// Create a new SHA-256 hasher
    pub fn new() -> Self {
        Self {
            hasher: Sha256::new(),
        }
    }

    /// Update hash with data
    pub fn update(&mut self, data: &[u8]) {
        self.hasher.update(data);
    }

    /// Finalize and return SHA-256 hash as hex string
    pub fn finalize(self) -> String {
        let digest = self.hasher.finalize();
        format!("{:x}", digest)
    }
}

impl Default for Sha256Hasher {
    fn default() -> Self {
        Self::new()
    }
}

/// Compute SHA-256 hash of data
pub fn compute_sha256(data: &[u8]) -> String {
    let mut hasher = Sha256Hasher::new();
    hasher.update(data);
    hasher.finalize()
}
