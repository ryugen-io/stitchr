//! Patch format implementations
//!
//! This crate provides implementations for various ROM patch formats:
//! - IPS (International Patching System)
//! - BPS (Beat Patching System)
//! - UPS (Universal Patching System)
//! - APS (Nintendo 64 APS Format)
//! - EBP (Extended Binary Patch)
//! - RUP (Rupture Patches)
//! - PPF (PlayStation Patch Format)
//! - xdelta (Generic binary diff)

use rom_patcher_core::{PatchFormat, PatchType};

#[cfg(feature = "ips")]
pub mod ips;

#[cfg(feature = "bps")]
pub mod bps;

#[cfg(feature = "ups")]
pub mod ups;

#[cfg(feature = "aps")]
pub mod aps;

#[cfg(feature = "ebp")]
pub mod ebp;

#[cfg(feature = "rup")]
pub mod rup;

#[cfg(feature = "ppf")]
pub mod ppf;

#[cfg(feature = "xdelta")]
pub mod xdelta;

#[cfg(feature = "bdf")]
pub mod bdf;

/// Auto-detect patch format from file data
pub fn detect_format(data: &[u8]) -> Option<PatchType> {
    // EBP must be checked before IPS (both use PATCH magic)
    #[cfg(feature = "ebp")]
    if ebp::EbpPatcher::can_handle(data) {
        return Some(PatchType::Ebp);
    }

    #[cfg(feature = "ips")]
    if ips::IpsPatcher::can_handle(data) {
        return Some(PatchType::Ips);
    }

    #[cfg(feature = "bps")]
    if bps::BpsPatcher::can_handle(data) {
        return Some(PatchType::Bps);
    }

    #[cfg(feature = "ups")]
    if ups::UpsPatcher::can_handle(data) {
        return Some(PatchType::Ups);
    }

    #[cfg(feature = "aps")]
    if aps::ApsPatcher::can_handle(data) {
        return Some(PatchType::Aps);
    }

    #[cfg(feature = "rup")]
    if rup::RupPatcher::can_handle(data) {
        return Some(PatchType::Rup);
    }

    #[cfg(feature = "ppf")]
    if ppf::PpfPatcher::can_handle(data) {
        return Some(PatchType::Ppf);
    }

    #[cfg(feature = "xdelta")]
    if xdelta::XdeltaPatcher::can_handle(data) {
        return Some(PatchType::Xdelta);
    }

    #[cfg(feature = "bdf")]
    if bdf::BdfPatcher::can_handle(data) {
        return Some(PatchType::Bdf);
    }

    None
}
