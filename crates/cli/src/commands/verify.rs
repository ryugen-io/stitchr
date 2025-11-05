//! Checksum verification for patches

use anyhow::Result;
use rom_patcher_core::{PatchFormat, PatchType};
use rom_patcher_formats::{bps::BpsPatcher, ips::IpsPatcher};

/// Dispatch verify() calls to appropriate format
fn dispatch_verify(rom: &[u8], patch: &[u8], patch_type: &PatchType, target: Option<&[u8]>) -> Result<()> {
    match patch_type {
        PatchType::Ips => IpsPatcher::verify(rom, patch, target)?,
        PatchType::Bps => BpsPatcher::verify(rom, patch, target)?,
        _ => anyhow::bail!("Format {} does not support verification", patch_type.name()),
    }
    Ok(())
}

/// Dispatch validate() calls to appropriate format
fn dispatch_validate(patch: &[u8], patch_type: &PatchType) -> Result<()> {
    match patch_type {
        PatchType::Ips => IpsPatcher::validate(patch)?,
        PatchType::Bps => BpsPatcher::validate(patch)?,
        _ => anyhow::bail!("Format {} does not support verification", patch_type.name()),
    }
    Ok(())
}

/// Verify source ROM checksum against patch
pub fn verify_source(rom: &[u8], patch: &[u8], patch_type: &PatchType) -> Result<()> {
    println!("Validating patch integrity...");
    dispatch_validate(patch, patch_type)?;
    println!("Patch integrity verified!");

    println!("Verifying source ROM checksum...");
    dispatch_verify(rom, patch, patch_type, None)?;
    println!("Source ROM checksum verified!");
    Ok(())
}

/// Verify target ROM checksum against patch
pub fn verify_target(rom: &[u8], patch: &[u8], patch_type: &PatchType) -> Result<()> {
    println!("Verifying target ROM checksum...");
    dispatch_verify(rom, patch, patch_type, Some(rom))?;
    println!("Target ROM checksum verified!");
    Ok(())
}
