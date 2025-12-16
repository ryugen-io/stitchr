//! Validation types and enums

/// Hash algorithm types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashAlgorithm {
    /// CRC32 checksum
    Crc32,
    /// Adler32 checksum
    Adler32,
    /// MD5 hash
    Md5,
    /// SHA-1 hash
    Sha1,
    /// SHA-256 hash
    Sha256,
}
