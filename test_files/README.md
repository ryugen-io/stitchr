# Test ROM Patches

Test patches for validating rom-patcher-rs implementation across all supported formats.

## Test Suite Sources

Based on RomPatcher.js test suite, using real-world ROM hacks:

### IPS Format
- **Patch**: Super Mario Land 2 DX v1.8.1
- **Console**: Game Boy
- **Source**: https://www.romhacking.net/hacks/3784/
- **Base ROM**: Super Mario Land 2 - 6 Golden Coins (UE) (V1.0)
- **Base SHA1**: bba408539ecbf8d322324956d859bc86e2a9977b

### BPS Format
- **Patch**: Samurai Kid English Translation
- **Console**: Game Boy Color
- **Source**: https://www.romhacking.net/translations/6297/
- **Base ROM**: Samurai Kid (Japan)

### UPS Format
- **Patch**: Mother 3 English Translation v1.3
- **Console**: Game Boy Advance
- **Source**: https://www.romhacking.net/translations/1333/
- **Alt Source**: https://mother3.fobby.net/
- **Base ROM**: Mother 3 (Japan)
- **Base Size**: 32 MB

### APS Format (N64)
- **Patch**: The Legend of Zelda: Ocarina of Time Spanish Translation
- **Console**: Nintendo 64
- **Source**: https://www.romhacking.net/translations/1054/
- **Base ROM**: The Legend of Zelda: Ocarina of Time (USA)

### APS Format (GBA)
- **Patch**: Final Fantasy Tactics Advance X
- **Console**: Game Boy Advance
- **Source**: Search romhacking.net for FFTA hacks
- **Base ROM**: Final Fantasy Tactics Advance (USA/Europe)

### RUP Format (NINJA2)
- **Patch**: Uchuu no Kishi: Tekkaman Blade English Translation
- **Console**: Super Nintendo
- **Source**: https://www.romhacking.net/translations/843
- **Format**: NINJA format (RUP)
- **Base ROM**: Uchuu no Kishi: Tekkaman Blade (Japan)

### PPF Format
- **Patch**: Vagrant Story Retranslation
- **Console**: PlayStation
- **Source**: https://www.romhacking.net/translations/5411/
- **Base ROM**: Vagrant Story (USA)
- **Format**: PPF3

### xdelta Format (VCDIFF)
- **Patch**: New Super Mario Bros. - Domain Infusion
- **Console**: Nintendo DS
- **Source**: Search for NSMB ROM hacks
- **Format**: xdelta/VCDIFF

## Usage

1. Download base ROMs (you must own legitimate copies)
2. Download patch files from sources above
3. Place in respective format directories
4. Run integration tests: `cargo test --test integration_test`

## Legal Notice

These patches are free and legal to download. However, you must provide your own legitimate ROM files to apply patches to. Do not distribute patched ROMs.

## Testing Methodology

Tests validate:
- Patch format detection (magic bytes)
- Successful patch application
- CRC32 checksum validation of patched output
- Round-trip patch creation and application
