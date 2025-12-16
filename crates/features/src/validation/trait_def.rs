//! Validation feature trait

use stitchr_core::Result;

use super::types::HashAlgorithm;

/// Validation feature trait for ROM and patch integrity checking
pub trait ValidationFeature {
    /// Validate checksum of data against expected value
    fn validate_checksum(
        &self,
        data: &[u8],
        expected: &[u8],
        algorithm: HashAlgorithm,
    ) -> Result<()>;

    /// Compute hash of data using specified algorithm
    fn compute_hash(&self, data: &[u8], algorithm: HashAlgorithm) -> Vec<u8>;

    /// Verify that a patch can be applied to a ROM
    fn verify_compatibility(&self, rom: &[u8], patch: &[u8]) -> Result<()>;
}
