# Download Instructions for Test Patches

## Goal: Match RomPatcher.js Test Suite Exactly

We use the same test patches as RomPatcher.js to ensure compatibility.

## Download Links (RomPatcher.js Test Suite)

### 1. IPS Format
**Super Mario Land 2 DX**
- Source: https://www.romhacking.net/hacks/3784/
- Download patch file
- Place as: `test_roms/ips/sml2dx.ips`
- Base ROM: Super Mario Land 2 - 6 Golden Coins (UE) (V1.0)
- Base SHA1: bba408539ecbf8d322324956d859bc86e2a9977b

### 2. BPS Format
**Samurai Kid Translation**
- Source: https://www.romhacking.net/translations/6297/
- Download BPS patch
- Place as: `test_roms/bps/samurai_kid.bps`
- Base ROM: Samurai Kid (Japan) GBC

### 3. UPS Format
**Mother 3 Translation**
- Source: https://www.romhacking.net/translations/1333/
- Alternative: https://mother3.fobby.net/
- Place as: `test_roms/ups/mother3.ups`
- Base ROM: Mother 3 (Japan) GBA (32 MB)

### 4. APS Format (N64)
**Zelda OoT Spanish Translation**
- Source: https://www.romhacking.net/translations/1054/
- Download APS patch
- Place as: `test_roms/aps_n64/zelda_oot_spanish.aps`
- Base ROM: The Legend of Zelda: Ocarina of Time (USA)

### 5. APS Format (GBA)
**Final Fantasy Tactics Advance X**
- Source: Search romhacking.net for "FFTA X" or similar FFTA APS patch
- Place as: `test_roms/aps_gba/ffta_x.aps`
- Base ROM: Final Fantasy Tactics Advance

### 6. RUP Format (NINJA2)
**Tekkaman Blade Translation**
- Source: https://www.romhacking.net/translations/843
- Download NINJA/RUP format patch
- Place as: `test_roms/rup/tekkaman_blade.rup`
- Base ROM: Uchuu no Kishi: Tekkaman Blade (Japan) SNES

### 7. PPF Format
**Any PlayStation Translation with PPF**
- Suggested: Vagrant Story - https://www.romhacking.net/translations/5411/
- Or use another PS1 PPF patch
- Place as: `test_roms/ppf/[game].ppf`

### 8. xdelta/VCDIFF Format
**NSMB Hack Domain Infusion**
- Source: Search for "New Super Mario Bros Domain Infusion"
- Place as: `test_roms/xdelta/nsmb_domain_infusion.xdelta`
- Base ROM: New Super Mario Bros (NDS)

## Quick Start

```bash
cd /data/code/devel/rom-patcher-rs/test_roms

# Download each patch from the links above
# Place in respective directories with the filenames shown

# Verify structure:
tree -L 2
```

## Why These Specific Patches?

These are the exact patches used by RomPatcher.js in their test suite. Using the same patches allows us to:

1. Validate CRC32 checksums match their expected outputs
2. Ensure format compatibility with their implementation
3. Test against real-world ROM hacks that are known to work

## Testing Without ROMs

You can still test patch parsing and validation without base ROMs by:
- Reading the patch files
- Validating format/magic bytes
- Parsing metadata
- Checking patch structure

Full integration tests require legitimate base ROM files.
