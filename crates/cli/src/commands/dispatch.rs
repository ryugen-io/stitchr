//! Format dispatch logic for applying patches

use anyhow::Result;
use stitchr_core::{PatchFormat, PatchType};
use stitchr_formats::{
    aps::ApsPatcher, bdf::BdfPatcher, bps::BpsPatcher, ebp::EbpPatcher, ips::IpsPatcher,
    ppf::PpfPatcher, rup::RupPatcher, ups::UpsPatcher, xdelta::XdeltaPatcher,
};

/// Apply patch based on detected format
pub fn apply_patch(rom: &mut Vec<u8>, patch: &[u8], patch_type: &PatchType) -> Result<()> {
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
        PatchType::Aps => {
            let patcher = ApsPatcher;
            patcher.apply(rom, patch)?;
        }
        PatchType::Ebp => {
            let patcher = EbpPatcher;
            patcher.apply(rom, patch)?;
        }
        PatchType::Rup => {
            let patcher = RupPatcher;
            patcher.apply(rom, patch)?;
        }
        PatchType::Ppf => {
            let patcher = PpfPatcher;
            patcher.apply(rom, patch)?;
        }
        PatchType::Xdelta => {
            let patcher = XdeltaPatcher;
            patcher.apply(rom, patch)?;
        }
        PatchType::Bdf => {
            let patcher = BdfPatcher;
            patcher.apply(rom, patch)?;
        }
    }

    Ok(())
}
