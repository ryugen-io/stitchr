//! EBP patch application

use crate::ips::IpsPatcher;
use rom_patcher_core::{PatchFormat, Result};

/// Apply EBP patch to ROM
/// Delegates to IPS implementation since EBP is IPS with metadata
pub fn apply(rom: &mut Vec<u8>, patch: &[u8]) -> Result<()> {
    // EBP is IPS-compatible, just apply as IPS
    // The JSON metadata doesn't affect the patching process
    let patcher = IpsPatcher;
    patcher.apply(rom, patch)
}
