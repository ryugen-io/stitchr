//! Format dispatch logic for applying patches

use anyhow::Result;
use rom_patcher_core::{PatchFormat, PatchType};
use rom_patcher_formats::{bps::BpsPatcher, ips::IpsPatcher, ups::UpsPatcher};

/// Apply patch based on detected format
pub fn apply_patch(
    rom: &mut Vec<u8>,
    patch: &[u8],
    patch_type: &PatchType,
) -> Result<()> {
    match patch_type {
        PatchType::Ips => {
            let patcher = IpsPatcher;
            patcher.apply(rom, patch)?;
        }
        PatchType::Bps => {
            let patcher = BpsPatcher;
            patcher.apply(rom, patch)?;
        }
        PatchType::Ups => {
            let patcher = UpsPatcher;
            patcher.apply(rom, patch)?;
        }
        _ => {
            anyhow::bail!("Format {} is not yet implemented", patch_type.name());
        }
    }

    Ok(())
}
