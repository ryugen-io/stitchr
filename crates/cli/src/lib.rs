//! ROM Patcher CLI library
//!
//! This library exposes CLI utilities for testing purposes.

pub mod utils;

/// Operation mode for --only flag
#[derive(Clone, Debug)]
pub enum OnlyMode {
    /// Only verify checksums without applying patch
    Verify,
}
