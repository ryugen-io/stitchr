//! xdelta basic apply tests

use stitchr_core::{PatchError, PatchFormat};
use stitchr_formats::xdelta::XdeltaPatcher;
use std::path::PathBuf;

#[test]
fn test_xdelta_file_apply() -> Result<(), PatchError> {
    let patch_path = PathBuf::from("../../../test_files/xdelta/patch.xdelta");
    let rom_path = PathBuf::from("../../../test_files/xdelta/rom.nds");

    if !patch_path.exists() || !rom_path.exists() {
        println!("Skipping xdelta test: files not found");
        return Ok(());
    }

    let patch_data = std::fs::read(&patch_path).map_err(PatchError::Io)?;
    let mut rom_data = std::fs::read(&rom_path).map_err(PatchError::Io)?;

    let patcher = XdeltaPatcher;
    patcher.apply(&mut rom_data, &patch_data)?;

    Ok(())
}
