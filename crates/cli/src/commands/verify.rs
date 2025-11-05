//! Checksum verification for patches

use anyhow::Result;
use rom_patcher_core::{PatchFormat, PatchType};
use rom_patcher_formats::{bps::BpsPatcher, ips::IpsPatcher};

/// Verify source ROM checksum against patch
pub fn verify_source(rom: &[u8], patch: &[u8], patch_type: &PatchType) -> Result<()> {
    // First validate patch integrity (patch CRC32)
    println!("Validating patch integrity...");
    match patch_type {
        PatchType::Ips => {
            IpsPatcher::validate(patch)?;
        }
        PatchType::Bps => {
            BpsPatcher::validate(patch)?;
        }
        _ => {
            anyhow::bail!("Format {} does not support verification", patch_type.name());
        }
    }
    println!("Patch integrity verified!");

    // Then verify source ROM checksum
    println!("Verifying source ROM checksum...");
    match patch_type {
        PatchType::Ips => {
            IpsPatcher::verify(rom, patch, None)?;
        }
        PatchType::Bps => {
            BpsPatcher::verify(rom, patch, None)?;
        }
        _ => {
            anyhow::bail!("Format {} does not support verification", patch_type.name());
        }
    }

    println!("Source ROM checksum verified!");
    Ok(())
}

/// Verify target ROM checksum against patch
pub fn verify_target(rom: &[u8], patch: &[u8], patch_type: &PatchType) -> Result<()> {
    println!("Verifying target ROM checksum...");

    match patch_type {
        PatchType::Ips => {
            IpsPatcher::verify(rom, patch, Some(rom))?;
        }
        PatchType::Bps => {
            BpsPatcher::verify(rom, patch, Some(rom))?;
        }
        _ => {
            anyhow::bail!("Format {} does not support verification", patch_type.name());
        }
    }

    println!("Target ROM checksum verified!");
    Ok(())
}
