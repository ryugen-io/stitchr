//! RetroAchievements integration
//!
//! Provides console-specific ROM hashing for RetroAchievements compatibility.

mod types;

#[cfg(feature = "retroachievements")]
mod api;

#[cfg(feature = "retroachievements")]
pub mod parser;

#[cfg(feature = "retroachievements")]
pub mod hash;

pub use types::{Console, RaHashChecker};

#[cfg(feature = "retroachievements")]
pub use api::{game_url, lookup_game_by_hash};

#[cfg(feature = "retroachievements")]
pub use hash::{
    compute_n64_hash, compute_nds_hash, compute_nes_hash, compute_ps2_hash, compute_psp_hash,
    compute_psx_hash, compute_snes_hash, is_psp_iso,
};
