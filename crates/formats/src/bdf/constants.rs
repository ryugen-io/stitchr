//! BDF constants

/// BDF magic header ("BSDIFF40")
pub const BDF_MAGIC: &[u8] = b"BSDIFF40";

/// Size of the BDF/// Header size in bytes
pub const HEADER_SIZE: usize = 32;

/// Maximum allowed patched size (1GB) to prevent DoS allocations
pub const MAX_PATCHED_SIZE: usize = 1024 * 1024 * 1024;
