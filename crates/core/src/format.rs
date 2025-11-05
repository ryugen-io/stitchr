//! Core trait for patch format implementations

use crate::{PatchMetadata, Result};

/// Trait that all patch formats must implement
///
/// This trait provides the core functionality for detecting and applying patches.
pub trait PatchFormat: Send + Sync {
    /// Check if this format can handle the given patch data
    ///
    /// This typically checks magic bytes or file signatures.
    fn can_handle(data: &[u8]) -> bool
    where
        Self: Sized;

    /// Apply a patch to a ROM in-place
    ///
    /// # Arguments
    /// * `rom` - Mutable slice of ROM data to be patched
    /// * `patch` - Slice containing patch data
    ///
    /// # Errors
    /// Returns an error if the patch is invalid, corrupted, or cannot be applied
    fn apply(&self, rom: &mut Vec<u8>, patch: &[u8]) -> Result<()>;

    /// Extract metadata from a patch file
    ///
    /// # Arguments
    /// * `patch` - Patch data to analyze
    ///
    /// # Returns
    /// Metadata about the patch including checksums and size information
    fn metadata(patch: &[u8]) -> Result<PatchMetadata>
    where
        Self: Sized;

    /// Validate patch integrity without applying it
    ///
    /// # Arguments
    /// * `patch` - Patch data to validate
    ///
    /// # Errors
    /// Returns an error if the patch is corrupted or invalid
    fn validate(patch: &[u8]) -> Result<()>
    where
        Self: Sized;

    /// Verify checksums (source ROM and optionally target ROM)
    ///
    /// # Arguments
    /// * `rom` - Source ROM data to verify
    /// * `patch` - Patch data containing checksums
    /// * `target` - Optional target ROM data to verify (after patching)
    ///
    /// # Errors
    /// Returns an error if checksums don't match
    ///
    /// # Default Implementation
    /// No-op for formats without checksum support (e.g., IPS)
    fn verify(rom: &[u8], patch: &[u8], target: Option<&[u8]>) -> Result<()>
    where
        Self: Sized,
    {
        let _ = (rom, patch, target);
        Ok(())
    }
}

/// Helper trait for auto-detecting patch format from data
pub trait FormatDetector {
    /// Detect the patch format from raw patch data
    ///
    /// Returns the PatchType if detected, None otherwise
    fn detect(data: &[u8]) -> Option<crate::PatchType>;
}
