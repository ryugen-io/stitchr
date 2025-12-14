//! Common types for ROM patching

/// Identifies the type of patch format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PatchType {
    /// International Patching System
    Ips,
    /// Beat Patching System (byuu)
    Bps,
    /// Universal Patching System
    Ups,
    /// Nintendo 64 APS format
    Aps,
    /// EBP format
    Ebp,
    /// Rupture patches
    Rup,
    /// PlayStation Patch Format
    Ppf,
    /// xdelta binary diff format
    Xdelta,
    /// Binary Diff Format (bsdiff)
    Bdf,
}

impl PatchType {
    /// Returns the common file extension for this patch type
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Ips => "ips",
            Self::Bps => "bps",
            Self::Ups => "ups",
            Self::Aps => "aps",
            Self::Ebp => "ebp",
            Self::Rup => "rup",
            Self::Ppf => "ppf",
            Self::Xdelta => "xdelta",
            Self::Bdf => "bdf",
        }
    }

    /// Returns the human-readable name of this patch format
    pub fn name(&self) -> &'static str {
        match self {
            Self::Ips => "International Patching System",
            Self::Bps => "Beat Patching System",
            Self::Ups => "Universal Patching System",
            Self::Aps => "Nintendo 64 APS Format",
            Self::Ebp => "Extended Binary Patch",
            Self::Rup => "Rupture Patches",
            Self::Ppf => "PlayStation Patch Format",
            Self::Xdelta => "xdelta Binary Diff",
            Self::Bdf => "Binary Diff Format",
        }
    }
}

/// Metadata extracted from a patch file
#[derive(Debug, Clone)]
pub struct PatchMetadata {
    /// The patch format type
    pub patch_type: PatchType,
    /// Expected source ROM size (if available)
    pub source_size: Option<usize>,
    /// Expected target ROM size (if available)
    pub target_size: Option<usize>,
    /// Source ROM checksum (if available)
    pub source_checksum: Option<Vec<u8>>,
    /// Target ROM checksum (if available)
    pub target_checksum: Option<Vec<u8>>,
    /// Additional format-specific metadata
    pub extra: Vec<(String, String)>,
}

impl PatchMetadata {
    /// Create new metadata with just the patch type
    pub fn new(patch_type: PatchType) -> Self {
        Self {
            patch_type,
            source_size: None,
            target_size: None,
            source_checksum: None,
            target_checksum: None,
            extra: Vec::new(),
        }
    }

    /// Add a custom metadata entry
    pub fn with_extra(mut self, key: String, value: String) -> Self {
        self.extra.push((key, value));
        self
    }
}
