//! RetroAchievements integration
//!
//! Provides console-specific ROM hashing for RetroAchievements compatibility.

mod types;

#[cfg(feature = "retroachievements")]
mod api;

pub use types::{Console, RaHashChecker};

#[cfg(feature = "retroachievements")]
pub use api::{game_url, lookup_game_by_hash};
