# Test Patch Files

Test patches for validating stitchr implementation across all supported formats.

All patches are stored as generic filenames (patch.ips, patch.bps, etc.) in their respective format directories.

## Required Base ROMs

To use these test patches, you need the exact base ROM files listed below. The patches in this repository are:

- **Legally redistributable** - The patches themselves are free to distribute
- **Format-specific** - Each patch validates a different patch format implementation
- **Checksum-validated** - Tests verify output matches expected checksums

### IPS Format (`ips/patch.ips`)
- **Patch**: Super Mario Land 2 DX v1.8.1
- **Patch Source**: https://www.romhacking.net/hacks/3784/
- **Required Base ROM**: Super Mario Land 2 - 6 Golden Coins (UE) (V1.0) [!].gb
- **ROM Region**: USA/Europe
- **ROM Size**: 512 KB (524,288 bytes)
- **ROM CRC32**: 0xd5ec24e4
- **ROM SHA1**: bba408539ecbf8d322324956d859bc86e2a9977b
- **Expected Output CRC32**: 0xf0799017 (validated against RomPatcherJS)

### BPS Format (`bps/patch.bps`)
- **Patch**: Samurai Kid English Translation
- **Patch Source**: https://www.romhacking.net/translations/6297/
- **Required Base ROM**: Samurai Kid (Japan).gbc
- **ROM Region**: Japan
- **Console**: Game Boy Color
- **Note**: BPS format validation only (not yet implemented)

### UPS Format (`ups/patch.ups`)
- **Patch**: Mother 3 English Translation v1.3
- **Patch Source**: https://www.romhacking.net/translations/1333/ or https://mother3.fobby.net/
- **Required Base ROM**: Mother 3 (Japan).gba
- **ROM Region**: Japan
- **ROM Size**: 32 MB
- **Console**: Game Boy Advance
- **Note**: UPS format validation only (not yet implemented)

### APS Format - N64 (`aps_n64/patch.aps`)
- **Patch**: The Legend of Zelda: Ocarina of Time Spanish Translation
- **Patch Source**: https://www.romhacking.net/translations/1054/
- **Required Base ROM**: The Legend of Zelda: Ocarina of Time (USA).z64
- **ROM Region**: USA
- **Console**: Nintendo 64
- **Note**: APS N64 format validation only (not yet implemented)

### PPF Format (`ppf/patch.ppf`)
- **Patch**: Vagrant Story Retranslation
- **Patch Source**: https://www.romhacking.net/translations/5411/
- **Required Base ROM**: Vagrant Story (USA).bin
- **ROM Region**: USA
- **Console**: PlayStation
- **Format**: PPF3
- **Note**: PPF format validation only (not yet implemented)

### RUP Format (`rup/patch.rup`)
- **Patch**: Uchuu no Kishi: Tekkaman Blade English Translation
- **Patch Source**: https://www.romhacking.net/translations/843
- **Required Base ROM**: Uchuu no Kishi: Tekkaman Blade (Japan).smc
- **ROM Region**: Japan
- **Console**: Super Nintendo
- **Format**: NINJA2/RUP format
- **Note**: RUP format validation only (not yet implemented)

### xdelta Format (`xdelta/patch.xdelta`)
- **Patch**: New Super Mario Bros. - Domain Infusion
- **Patch Source**: Search romhacking.net for "NSMB Domain Infusion"
- **Required Base ROM**: New Super Mario Bros (USA).nds
- **ROM Region**: USA
- **Console**: Nintendo DS
- **Format**: xdelta/VCDIFF
- **Note**: xdelta format validation only (not yet implemented)

## Usage Instructions

### Step 1: Obtain Base ROMs

You must provide your own legally obtained ROM files. The required ROMs are listed above with their exact filenames and checksums.

**For the IPS test specifically:**
- Place `Super Mario Land 2 - 6 Golden Coins (UE) (V1.0) [!].gb` in `test_files/ips/`
- Verify CRC32 checksum: `0xd5ec24e4`
- File size must be exactly 524,288 bytes (512 KB)

### Step 2: Directory Structure

```
test_files/
├── ips/
│   ├── patch.ips                    (included in repo)
│   └── Super Mario Land 2 (...).gb  (you provide)
├── bps/
│   ├── patch.bps                    (included in repo)
│   └── Samurai Kid (Japan).gbc      (you provide)
├── ups/
│   ├── patch.ups                    (included in repo)
│   └── Mother 3 (Japan).gba         (you provide)
└── ... (other formats)
```

### Step 3: Run Tests

```bash
# Run all IPS integration tests (including checksum validation)
cargo test --test ips_integration

# Run specific checksum validation test
cargo test test_sml2dx_patch_checksum
```

## Legal Notice

These patches are free and legal to download. However, you must provide your own legitimate ROM files to apply patches to. Do not distribute patched ROMs.

## Testing Methodology

Tests validate:
- Patch format detection (magic bytes)
- Successful patch application
- CRC32 checksum validation of patched output
- Round-trip patch creation and application
