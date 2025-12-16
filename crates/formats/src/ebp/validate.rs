//! EBP validation

use super::constants::*;
use stitchr_core::Result;

/// Validate EBP patch format
/// EBP is IPS-compatible, so we delegate to IPS validation
pub fn validate(patch: &[u8]) -> Result<()> {
    // EBP uses same validation as IPS (magic + EOF + records)
    use crate::ips::IpsPatcher;
    use stitchr_core::PatchFormat;
    IpsPatcher::validate(patch)
}

/// Check if patch can be handled
/// EBP = IPS (PATCH magic) + JSON metadata after EOF
pub fn can_handle(patch: &[u8]) -> bool {
    if patch.len() < MAGIC_SIZE || &patch[..MAGIC_SIZE] != MAGIC {
        return false;
    }

    // Check if JSON metadata is present (distinguishes EBP from plain IPS)
    use super::helpers;
    helpers::find_json_start(patch).is_some()
}
