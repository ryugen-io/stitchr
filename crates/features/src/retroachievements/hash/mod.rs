//! Console-specific hash algorithms for RetroAchievements
//!
//! Each console has specific hashing requirements per RA documentation.

mod iso9660;
mod n64;
mod nds;
mod nes;
mod ps2;
mod psp;
mod psx;
mod snes;

pub use iso9660::is_psp_iso;
pub use n64::compute_n64_hash;
pub use nds::compute_nds_hash;
pub use nes::compute_nes_hash;
pub use ps2::compute_ps2_hash;
pub use psp::compute_psp_hash;
pub use psx::compute_psx_hash;
pub use snes::compute_snes_hash;
