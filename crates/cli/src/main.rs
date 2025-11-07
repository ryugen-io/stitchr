//! ROM Patcher CLI
//!
//! A minimal CLI for applying ROM patches with automatic validation.

use anyhow::Result;
use clap::{Parser, ValueEnum};
use std::path::PathBuf;

mod commands;
mod utils;

use rom_patcher_cli::OnlyMode as OnlyModeLib;

/// Operation mode for --only flag
#[derive(ValueEnum, Clone, Debug)]
enum OnlyMode {
    /// Only verify checksums without applying patch
    Verify,
    /// Check ROM against RetroAchievements database
    Ra,
}

impl From<OnlyMode> for OnlyModeLib {
    fn from(mode: OnlyMode) -> Self {
        match mode {
            OnlyMode::Verify => OnlyModeLib::Verify,
            OnlyMode::Ra => OnlyModeLib::Ra,
        }
    }
}

/// ROM Patcher - Apply patches to ROM files
#[derive(Parser, Debug)]
#[command(name = "rompatchrs")]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the ROM file
    rom: PathBuf,

    /// Path to the patch file (not required for --only ra)
    patch: Option<PathBuf>,

    /// Output path (optional, defaults to {rom_dir}/patched/{rom}.patched.{ext})
    output: Option<PathBuf>,

    /// Verify source/target checksums (slower, safer)
    #[arg(long)]
    verify: bool,

    /// Only perform specific operation without applying patch
    #[arg(long, value_enum)]
    only: Option<OnlyMode>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let only_mode = cli.only.map(|m| m.into());

    // Validate: patch is required unless --only ra
    if cli.patch.is_none() && !matches!(only_mode, Some(OnlyModeLib::Ra)) {
        anyhow::bail!("Patch file is required (unless using --only ra)");
    }

    commands::apply::execute(cli.rom, cli.patch, cli.output, cli.verify, only_mode)
}
