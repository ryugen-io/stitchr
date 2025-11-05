//! Error types for ROM patching operations

use thiserror::Error;

/// Result type alias for patch operations
pub type Result<T> = std::result::Result<T, PatchError>;

/// Errors that can occur during patching operations
#[derive(Error, Debug)]
pub enum PatchError {
    #[error("Invalid patch format: {0}")]
    InvalidFormat(String),

    #[error("Patch data is corrupted or incomplete")]
    CorruptedData,

    #[error("Checksum verification failed: expected {expected:#x}, got {actual:#x}")]
    ChecksumMismatch { expected: u32, actual: u32 },

    #[error("ROM size mismatch: expected {expected}, got {actual}")]
    SizeMismatch { expected: usize, actual: usize },

    #[error("Patch offset {offset:#x} is out of bounds (ROM size: {rom_size:#x})")]
    OutOfBounds { offset: usize, rom_size: usize },

    #[error("Unsupported patch version: {0}")]
    UnsupportedVersion(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid magic bytes: expected {expected:?}, got {actual:?}")]
    InvalidMagic { expected: Vec<u8>, actual: Vec<u8> },

    #[error("Unexpected end of file: {0}")]
    UnexpectedEof(String),

    #[error("Generic error: {0}")]
    Other(String),
}
