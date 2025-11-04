# CLI Usage Guide

Complete guide to using the `rompatch` command-line tool.

---

## Table of Contents

- [Installation](#installation)
- [Quick Start](#quick-start)
- [Commands](#commands)
  - [Apply Command](#apply-command)
- [Options and Flags](#options-and-flags)
- [Usage Examples](#usage-examples)
- [Output Behavior](#output-behavior)
- [Error Handling](#error-handling)
- [Advanced Usage](#advanced-usage)

---

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/yourusername/rom-patcher-rs.git
cd rom-patcher-rs

# Build release binary
cargo build --release

# Binary will be at target/release/rompatch
./target/release/rompatch --help
```

### Using `cargo install`

```bash
cargo install --path crates/cli
```

### Using `just`

If you have [just](https://github.com/casey/just) installed:

```bash
just build
./target/release/rompatch --help
```

---

## Quick Start

Apply a patch to a ROM:

```bash
rompatch original.smc translation.ips
```

This creates `patched/original.patched.smc` in the same directory.

Specify custom output location:

```bash
rompatch original.smc translation.ips -o translated.smc
```

---

## Commands

### Apply Command

Apply a patch to a ROM file.

#### Syntax

```bash
rompatch <ROM_PATH> <PATCH_PATH> [OPTIONS]
```

#### Arguments

##### `<ROM_PATH>` (required)

Path to the original ROM file.

**Accepted formats**: Any file can be used (no extension restrictions)

**Common extensions**:
- `.smc`, `.sfc` - Super Nintendo
- `.nes` - Nintendo Entertainment System
- `.gb`, `.gbc` - Game Boy / Game Boy Color
- `.gba` - Game Boy Advance
- `.n64`, `.z64`, `.v64` - Nintendo 64
- `.md`, `.bin` - Sega Genesis
- `.iso` - CD-based games

**Examples**:
```bash
rompatch game.smc patch.ips
rompatch ~/roms/zelda.nes translation.ips
rompatch "/path/with spaces/game.gba" hack.ips
```

##### `<PATCH_PATH>` (required)

Path to the patch file.

**Supported formats** (current):
- `.ips` - International Patching System  Fully implemented

**Supported formats** (planned):
- `.bps` - Beat Patching System
- `.ups` - Universal Patching System
- `.aps` - Nintendo 64 APS
- `.rup` - Rupture Patches
- `.ppf` - PlayStation Patch Format
- `.xdelta` - xdelta binary diff

**Format detection**: Automatic based on file contents (not extension)

#### Options

##### `-o, --output <PATH>`

Specify output path for patched ROM.

**Default behavior** (if not specified):
```
<rom_directory>/patched/<rom_name>.patched.<rom_extension>
```

**Examples**:
```bash
# Default output: patched/game.patched.smc
rompatch game.smc patch.ips

# Custom output path
rompatch game.smc patch.ips -o translated.smc

# Custom directory
rompatch game.smc patch.ips -o ~/patched_roms/game.smc

# Different extension
rompatch game.smc patch.ips -o game.sfc
```

**Safety**: The tool prevents overwriting the input ROM:

```bash
# This will error
rompatch game.smc patch.ips -o game.smc
# Error: Output path cannot be the same as input ROM
```

---

## Options and Flags

### Global Options

#### `--help`

Display help information.

```bash
rompatch --help
```

#### `--version`

Display version information.

```bash
rompatch --version
```

---

## Usage Examples

### Basic Patching

```bash
# Apply IPS patch to SNES ROM
rompatch super_metroid.smc project_base.ips

# Result: patched/super_metroid.patched.smc
```

### Custom Output Location

```bash
# Specify output file
rompatch earthbound.smc translation.ips -o earthbound_translated.smc

# Specify output directory (must exist)
mkdir translated_roms
rompatch game.smc patch.ips -o translated_roms/game.smc
```

### Multiple Patches

Apply multiple patches sequentially:

```bash
# Apply base hack first
rompatch original.smc base_hack.ips -o temp.smc

# Then apply graphics patch
rompatch temp.smc graphics.ips -o final.smc

# Clean up
rm temp.smc
```

Or use a script:

```bash
#!/bin/bash
ROM="original.smc"
for patch in patches/*.ips; do
    echo "Applying $(basename "$patch")..."
    rompatch "$ROM" "$patch" -o temp.smc
    ROM="temp.smc"
done
mv temp.smc final.smc
```

### Batch Patching

Patch multiple ROMs with the same patch:

```bash
#!/bin/bash
PATCH="universal_fix.ips"

for rom in roms/*.smc; do
    basename=$(basename "$rom" .smc)
    echo "Patching $basename..."
    rompatch "$rom" "$PATCH" -o "patched/${basename}.smc"
done
```

### Using with Pipes

Check ROM before patching:

```bash
# Calculate original ROM checksum
crc32 original.smc

# Apply patch
rompatch original.smc patch.ips

# Verify patched ROM
crc32 patched/original.patched.smc
```

---

## Output Behavior

### Default Output Path

When no output path is specified, the tool creates:

```
<rom_directory>/patched/<rom_name>.patched.<rom_extension>
```

**Examples**:

| Input ROM | Output ROM |
|-----------|------------|
| `game.smc` | `patched/game.patched.smc` |
| `~/roms/zelda.nes` | `~/roms/patched/zelda.patched.nes` |
| `./test.gba` | `./patched/test.patched.gba` |

The `patched/` directory is created automatically if it doesn't exist.

### Output Information

The tool always displays:

1. **Input ROM checksum** (CRC32)
2. **Patch checksum** (CRC32)
3. **Output ROM checksum** (CRC32)
4. **Output file path**

**Example output**:

```
Patching ROM with IPS format...
Input ROM CRC32:  A1B2C3D4
Patch CRC32:      E5F6A7B8
Output ROM CRC32: C9D0E1F2
Patched ROM saved to: patched/game.patched.smc
```

### Transactional Safety

The patching process is **transactional**:

1. Original ROM is **never modified** (loaded into memory)
2. Patch is applied to an **in-memory copy**
3. If patching **fails**, no output file is created
4. Output is written to a **temporary file** first
5. Temporary file is **atomically renamed** to final path

This ensures:
-  No data loss if patching fails
-  No partial/corrupted output files
-  Original ROM remains untouched

**Example error handling**:

```bash
# Invalid patch
rompatch game.smc corrupted.ips
# Error: Patch is corrupted
# (no output file created, game.smc unchanged)

# Success
rompatch game.smc valid.ips
# (patched/game.patched.smc created, game.smc unchanged)
```

---

## Error Handling

### Common Errors

#### "File not found"

```
Error: No such file or directory (os error 2)
```

**Cause**: ROM or patch file doesn't exist

**Solution**: Check file paths

```bash
# Check if files exist
ls -l game.smc patch.ips

# Use absolute paths
rompatch "$(pwd)/game.smc" "$(pwd)/patch.ips"
```

#### "Invalid patch format"

```
Error: Invalid magic bytes, expected [50, 41, 54, 43, 48] (PATCH), got [49, 50, 51, 52, 53]
```

**Cause**: Patch file is not a valid IPS patch (or wrong format)

**Solution**: Verify patch file

```bash
# Check patch header (should start with "PATCH")
xxd patch.ips | head -n 1

# Try detecting format
file patch.ips
```

#### "Patch is corrupted"

```
Error: Patch data is corrupted
```

**Cause**: Patch file is incomplete or damaged

**Solution**: Re-download patch file

#### "Output path cannot be the same as input ROM"

```
Error: Output path cannot be the same as input ROM
```

**Cause**: Trying to overwrite original ROM

**Solution**: Use different output path

```bash
# Wrong
rompatch game.smc patch.ips -o game.smc

# Correct
rompatch game.smc patch.ips -o game_patched.smc
```

#### "Permission denied"

```
Error: Permission denied (os error 13)
```

**Cause**: No write permission in output directory

**Solution**: Check directory permissions

```bash
# Check permissions
ls -ld patched/

# Create directory with correct permissions
mkdir -p patched
chmod 755 patched

# Or specify different output directory
rompatch game.smc patch.ips -o ~/Documents/game.smc
```

### Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | General error (invalid arguments, file not found, etc.) |
| `2` | Patch format error (invalid/corrupted patch) |

**Usage in scripts**:

```bash
#!/bin/bash
if rompatch game.smc patch.ips; then
    echo "Patching succeeded!"
else
    echo "Patching failed with code $?"
    exit 1
fi
```

---

## Advanced Usage

### Checksum Verification

The tool automatically displays CRC32 checksums for all files:

```bash
rompatch game.smc patch.ips

# Output includes:
# Input ROM CRC32:  12345678
# Patch CRC32:      ABCDEF01
# Output ROM CRC32: 9ABCDEF0
```

**Verify against known good checksums**:

```bash
# Expected checksums (from patch readme)
EXPECTED_INPUT="12345678"
EXPECTED_OUTPUT="9ABCDEF0"

# Apply patch and capture output
OUTPUT=$(rompatch game.smc patch.ips 2>&1)

# Extract checksums
INPUT_CRC=$(echo "$OUTPUT" | grep "Input ROM CRC32" | awk '{print $4}')
OUTPUT_CRC=$(echo "$OUTPUT" | grep "Output ROM CRC32" | awk '{print $4}')

# Verify
if [ "$INPUT_CRC" != "$EXPECTED_INPUT" ]; then
    echo "Wrong ROM version!"
    exit 1
fi

if [ "$OUTPUT_CRC" != "$EXPECTED_OUTPUT" ]; then
    echo "Patching produced unexpected result!"
    exit 1
fi

echo "Checksums verified successfully"
```

### Integration with Version Control

```bash
#!/bin/bash
# Script to apply patches and track in git

ROM="$1"
PATCH="$2"
OUTPUT="patched/$(basename "$ROM")"

# Apply patch
rompatch "$ROM" "$PATCH" -o "$OUTPUT"

# Calculate checksums for documentation
INPUT_CRC=$(crc32 "$ROM")
OUTPUT_CRC=$(crc32 "$OUTPUT")

# Commit to git
git add "$OUTPUT"
git commit -m "Apply $(basename "$PATCH")

Original ROM: $INPUT_CRC
Patched ROM:  $OUTPUT_CRC"
```

### ROM Testing Workflow

```bash
#!/bin/bash
# Test ROM before and after patching in an emulator

ROM="original.smc"
PATCH="test.ips"
OUTPUT="test_patched.smc"

# Apply patch
if ! rompatch "$ROM" "$PATCH" -o "$OUTPUT"; then
    echo "Patching failed"
    exit 1
fi

# Launch in emulator
snes9x "$OUTPUT"

# Ask user if patch is good
read -p "Keep patched ROM? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    mv "$OUTPUT" "patched/$(basename "$ROM")"
    echo "Saved to patched/"
else
    rm "$OUTPUT"
    echo "Discarded"
fi
```

### Parallel Batch Patching

Using GNU Parallel:

```bash
# Install GNU Parallel
# Ubuntu/Debian: sudo apt install parallel
# macOS: brew install parallel

# Patch multiple ROMs in parallel
parallel rompatch {} patch.ips ::: roms/*.smc

# With custom output directory
parallel rompatch {} patch.ips -o output/{/} ::: roms/*.smc
```

Using xargs:

```bash
# Patch all SNES ROMs in parallel (4 jobs)
find roms/ -name "*.smc" -print0 | \
    xargs -0 -P 4 -I {} rompatch {} patch.ips
```

### Creating a ROM Patcher GUI Script

Simple `zenity` GUI wrapper:

```bash
#!/bin/bash
# gui-patcher.sh - Simple GUI for rompatch

# Select ROM
ROM=$(zenity --file-selection --title="Select ROM file")
[ -z "$ROM" ] && exit

# Select patch
PATCH=$(zenity --file-selection --title="Select patch file")
[ -z "$PATCH" ] && exit

# Select output location
OUTPUT=$(zenity --file-selection --save --title="Save patched ROM as")
[ -z "$OUTPUT" ] && exit

# Apply patch with progress dialog
(
    echo "10" ; echo "# Loading ROM..."
    echo "30" ; echo "# Loading patch..."
    echo "50" ; echo "# Applying patch..."

    if rompatch "$ROM" "$PATCH" -o "$OUTPUT" 2>&1; then
        echo "100" ; echo "# Done!"
    else
        zenity --error --text="Patching failed!"
        exit 1
    fi
) | zenity --progress --title="Patching ROM" --percentage=0

# Success message
zenity --info --text="ROM patched successfully!\n\nOutput: $OUTPUT"
```

---

## Performance Tips

### Large ROM Files

The tool is optimized for performance:

- **1 MB ROM**: ~16 µs patching time
- **4 MB ROM**: ~50 µs patching time
- **16 MB ROM**: ~200 µs patching time

**Memory usage**: ROM size × 2 (original + patched copy)

### SSD vs HDD

Most time is spent on file I/O:

| Storage | 4 MB ROM | 16 MB ROM |
|---------|----------|-----------|
| SSD | ~10 ms total | ~30 ms total |
| HDD | ~50 ms total | ~150 ms total |

**Recommendation**: Use SSD for batch operations

### Network Filesystems

Avoid patching directly on network drives:

```bash
# Slow (network I/O for every operation)
rompatch /mnt/nas/game.smc patch.ips -o /mnt/nas/patched.smc

# Fast (local I/O, then copy)
rompatch /mnt/nas/game.smc patch.ips -o /tmp/patched.smc
cp /tmp/patched.smc /mnt/nas/patched.smc
```

---

## Compatibility

### Emulator Testing

After patching, test in emulators:

**SNES**:
- [snes9x](https://github.com/snes9xgit/snes9x)
- [bsnes](https://github.com/bsnes-emu/bsnes)

**NES**:
- [Mesen](https://github.com/SourMesen/Mesen2)
- [FCEUX](http://fceux.com/)

**Game Boy**:
- [SameBoy](https://github.com/LIJI32/SameBoy)
- [mGBA](https://mgba.io/)

### Flashcart Compatibility

Patched ROMs work on flashcarts:
- **EverDrive** series
- **SD2SNES** (now **FXPak Pro**)
- **SuperCard** series

---

## Troubleshooting

### "Patch doesn't work in emulator"

1. **Verify checksums** match expected values
2. **Try different emulator** (some are more accurate)
3. **Check ROM header** (some patches require headerless/headered ROMs)
4. **Re-download patch** (may be corrupted)

### "Wrong ROM version" errors

Some patches require specific ROM versions:

```bash
# Check your ROM's CRC32
crc32 game.smc
# Compare with patch documentation
```

### "Patched ROM is wrong size"

IPS patches can change ROM size:

```bash
# Check sizes
ls -lh game.smc patched/game.patched.smc

# This is normal - patches may:
# - Expand ROM (add new data)
# - Truncate ROM (remove data)
```

---

## See Also

- [API Documentation](API.md) - Library usage
- [Developer Guide](DEVELOPER_GUIDE.md) - Contributing to the project
- [Troubleshooting Guide](TROUBLESHOOTING.md) - Detailed error solutions

---

## Getting Help

- **Issues**: https://github.com/yourusername/rom-patcher-rs/issues
- **Discussions**: https://github.com/yourusername/rom-patcher-rs/discussions
- **ROM hacking forums**: https://romhacking.net

---

## Examples Repository

Additional examples and scripts available at:
https://github.com/yourusername/rom-patcher-rs/tree/main/examples
