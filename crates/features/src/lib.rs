//! Extended features for ROM patching
//!
//! This crate provides optional features that extend the basic patching
//! functionality:
//! - Validation: Checksum verification and integrity checks
//! - RetroAchievements: Hash checking against RetroAchievements database

#[cfg(feature = "validation")]
pub mod validation;

#[cfg(feature = "retroachievements")]
pub mod retroachievements;

pub use stitchr_core::*;
