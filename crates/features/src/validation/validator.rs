//! Validator implementation

use rom_patcher_core::{PatchError, Result};

use super::algorithms::{md5, sha};
use super::trait_def::ValidationFeature;
use super::types::HashAlgorithm;

/// Default validation implementation
pub struct Validator;

impl Validator {
    /// Create a new validator
    pub fn new() -> Self {
        Self
    }

    /// Compute CRC32 checksum
    pub fn crc32(data: &[u8]) -> u32 {
        crc32fast::hash(data)
    }

    /// Compute checksum/hash based on algorithm
    pub fn compute(data: &[u8], algorithm: HashAlgorithm) -> Vec<u8> {
        match algorithm {
            HashAlgorithm::Crc32 => {
                let crc = Self::crc32(data);
                crc.to_be_bytes().to_vec()
            }
            HashAlgorithm::Md5 => {
                let hash = md5::compute(data);
                hash.as_bytes().to_vec()
            }
            HashAlgorithm::Sha1 => {
                let hash = sha::compute_sha1(data);
                hash.as_bytes().to_vec()
            }
            HashAlgorithm::Sha256 => {
                let hash = sha::compute_sha256(data);
                hash.as_bytes().to_vec()
            }
        }
    }
}

impl Default for Validator {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidationFeature for Validator {
    fn validate_checksum(
        &self,
        data: &[u8],
        expected: &[u8],
        algorithm: HashAlgorithm,
    ) -> Result<()> {
        let computed = self.compute_hash(data, algorithm);
        if computed == expected {
            Ok(())
        } else {
            Err(PatchError::InvalidFormat(format!(
                "Checksum mismatch: expected {:?}, got {:?}",
                expected, computed
            )))
        }
    }

    fn compute_hash(&self, data: &[u8], algorithm: HashAlgorithm) -> Vec<u8> {
        Self::compute(data, algorithm)
    }

    fn verify_compatibility(&self, _rom: &[u8], _patch: &[u8]) -> Result<()> {
        // TODO: Implement patch compatibility verification
        Ok(())
    }
}
