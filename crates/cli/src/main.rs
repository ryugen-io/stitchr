//! ROM Patcher CLI
//!
//! A minimal CLI for applying ROM patches with automatic validation.

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

mod commands;
mod utils;

/// ROM Patcher - Apply patches to ROM files
#[derive(Parser, Debug)]
#[command(name = "rompatchrs")]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the ROM file
    rom: PathBuf,

    /// Path to the patch file
    patch: PathBuf,

    /// Output path (optional, defaults to {rom_dir}/patched/{rom}.patched.{ext})
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    commands::apply::execute(cli.rom, cli.patch, cli.output)
}
