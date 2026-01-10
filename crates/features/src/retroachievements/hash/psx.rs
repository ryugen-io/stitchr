//! PlayStation (PSX) hash algorithm for RetroAchievements
//!
//! Algorithm:
//! 1. Load and parse SYSTEM.CNF from disc
//! 2. Find BOOT= line to identify primary executable
//! 3. Write executable path to buffer
//! 4. Append executable contents to buffer
//! 5. MD5 hash the buffer

use std::path::Path;

/// PSX CD sector size (Mode 2 Form 1)
const SECTOR_SIZE_RAW: usize = 2352;
/// Data offset within raw sector (sync + header + subheader)
const DATA_OFFSET: usize = 24;
/// User data size per sector
const DATA_SIZE: usize = 2048;
/// Primary Volume Descriptor location
const PVD_SECTOR: usize = 16;

/// Compute RetroAchievements hash for a PSX disc image
///
/// Accepts either a .cue file path (will find associated .bin) or direct .bin
/// path
pub fn compute_psx_hash(path: &Path) -> Result<String, String> {
    // Get the BIN file path
    let bin_path = resolve_bin_path(path)?;

    // Read the entire BIN file
    let bin_data =
        std::fs::read(&bin_path).map_err(|e| format!("Failed to read BIN file: {}", e))?;

    // Read SYSTEM.CNF
    let system_cnf = read_file_from_iso(&bin_data, "SYSTEM.CNF")
        .or_else(|_| read_file_from_iso(&bin_data, "SYSTEM.CNF;1"))?;

    // Parse BOOT= line
    let boot_path = parse_boot_path(&system_cnf)?;

    // Extract just the filename for the hash (e.g., "SLES_039.36" from
    // "cdrom:\SLES_039.36;1")
    let exe_name = extract_exe_name(&boot_path)?;

    // Read the executable
    let exe_data = read_file_from_iso(&bin_data, &exe_name)
        .or_else(|_| read_file_from_iso(&bin_data, &format!("{};1", exe_name)))?;

    // Build hash buffer: exe_name + exe_contents
    let mut buffer = Vec::with_capacity(exe_name.len() + exe_data.len());
    buffer.extend_from_slice(exe_name.as_bytes());
    buffer.extend_from_slice(&exe_data);

    // Compute MD5
    let digest = md5::compute(&buffer);
    Ok(format!("{:x}", digest))
}

/// Resolve .cue to .bin path, or return .bin path directly
fn resolve_bin_path(path: &Path) -> Result<std::path::PathBuf, String> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
        .unwrap_or_default();

    if ext == "bin" {
        return Ok(path.to_path_buf());
    }

    if ext == "cue" {
        // Parse CUE to find BIN file
        let cue_content =
            std::fs::read_to_string(path).map_err(|e| format!("Failed to read CUE file: {}", e))?;

        // Find FILE "xxx.bin" line
        for line in cue_content.lines() {
            let line = line.trim();
            if line.to_uppercase().starts_with("FILE") {
                // Extract filename between quotes
                if let Some(start) = line.find('"')
                    && let Some(end) = line[start + 1..].find('"')
                {
                    let bin_name = &line[start + 1..start + 1 + end];
                    let bin_path = path.parent().unwrap_or(Path::new(".")).join(bin_name);
                    return Ok(bin_path);
                }
            }
        }
        return Err("No BIN file found in CUE".to_string());
    }

    Err(format!("Unsupported file type: {}", ext))
}

/// Read a file from ISO9660 filesystem in a raw BIN image
fn read_file_from_iso(bin_data: &[u8], filename: &str) -> Result<Vec<u8>, String> {
    // Read Primary Volume Descriptor (sector 16)
    let pvd = read_sector(bin_data, PVD_SECTOR)?;

    // Verify this is a PVD (type 1, "CD001")
    if pvd[0] != 1 || &pvd[1..6] != b"CD001" {
        return Err("Invalid Primary Volume Descriptor".to_string());
    }

    // Root directory record is at offset 156, 34 bytes
    let root_dir_record = &pvd[156..190];
    let root_lba = u32::from_le_bytes([
        root_dir_record[2],
        root_dir_record[3],
        root_dir_record[4],
        root_dir_record[5],
    ]) as usize;
    let root_size = u32::from_le_bytes([
        root_dir_record[10],
        root_dir_record[11],
        root_dir_record[12],
        root_dir_record[13],
    ]) as usize;

    // Read root directory
    let root_dir = read_sectors(bin_data, root_lba, root_size.div_ceil(DATA_SIZE))?;

    // Search for file in directory
    let filename_upper = filename.to_uppercase();
    let filename_no_version = filename_upper.trim_end_matches(";1");

    let mut offset = 0;
    while offset < root_dir.len() {
        let record_len = root_dir[offset] as usize;
        if record_len == 0 {
            // Move to next sector boundary
            let next_sector = ((offset / DATA_SIZE) + 1) * DATA_SIZE;
            if next_sector >= root_dir.len() {
                break;
            }
            offset = next_sector;
            continue;
        }

        if offset + record_len > root_dir.len() {
            break;
        }

        let name_len = root_dir[offset + 32] as usize;
        if name_len > 0 && offset + 33 + name_len <= root_dir.len() {
            let name_bytes = &root_dir[offset + 33..offset + 33 + name_len];
            let name = String::from_utf8_lossy(name_bytes).to_uppercase();
            let name_no_version = name.trim_end_matches(";1");

            if name_no_version == filename_no_version || name == filename_upper {
                // Found it! Get LBA and size
                let file_lba = u32::from_le_bytes([
                    root_dir[offset + 2],
                    root_dir[offset + 3],
                    root_dir[offset + 4],
                    root_dir[offset + 5],
                ]) as usize;
                let file_size = u32::from_le_bytes([
                    root_dir[offset + 10],
                    root_dir[offset + 11],
                    root_dir[offset + 12],
                    root_dir[offset + 13],
                ]) as usize;

                // Read file data
                let sectors_needed = file_size.div_ceil(DATA_SIZE);
                let mut data = read_sectors(bin_data, file_lba, sectors_needed)?;
                data.truncate(file_size);
                return Ok(data);
            }
        }

        offset += record_len;
    }

    Err(format!("File not found: {}", filename))
}

/// Read a single sector from raw BIN data
fn read_sector(bin_data: &[u8], sector: usize) -> Result<Vec<u8>, String> {
    let offset = sector * SECTOR_SIZE_RAW + DATA_OFFSET;
    if offset + DATA_SIZE > bin_data.len() {
        return Err(format!("Sector {} out of bounds", sector));
    }
    Ok(bin_data[offset..offset + DATA_SIZE].to_vec())
}

/// Read multiple consecutive sectors
fn read_sectors(bin_data: &[u8], start_sector: usize, count: usize) -> Result<Vec<u8>, String> {
    let mut data = Vec::with_capacity(count * DATA_SIZE);
    for i in 0..count {
        data.extend(read_sector(bin_data, start_sector + i)?);
    }
    Ok(data)
}

/// Parse BOOT= line from SYSTEM.CNF content
fn parse_boot_path(system_cnf: &[u8]) -> Result<String, String> {
    let content = String::from_utf8_lossy(system_cnf);

    for line in content.lines() {
        let line = line.trim();
        // Handle both "BOOT=" and "BOOT =" formats
        if let Some(pos) = line.to_uppercase().find("BOOT") {
            let rest = &line[pos + 4..].trim_start();
            if let Some(stripped) = rest.strip_prefix('=') {
                let path = stripped.trim();
                return Ok(path.to_string());
            }
        }
    }

    Err("BOOT= line not found in SYSTEM.CNF".to_string())
}

/// Extract executable name from boot path
/// e.g., "cdrom:\SLES_039.36;1" -> "SLES_039.36"
fn extract_exe_name(boot_path: &str) -> Result<String, String> {
    // Remove cdrom:\ or cdrom0:\ prefix
    let path = boot_path
        .trim_start_matches("cdrom:")
        .trim_start_matches("cdrom0:")
        .trim_start_matches('\\')
        .trim_start_matches('/');

    // Remove ;1 version suffix
    let name = path.trim_end_matches(";1");

    if name.is_empty() {
        return Err("Empty executable name".to_string());
    }

    Ok(name.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_exe_name() {
        assert_eq!(
            extract_exe_name("cdrom:\\SLES_039.36;1").unwrap(),
            "SLES_039.36"
        );
        assert_eq!(
            extract_exe_name("cdrom0:\\SLUS_012.34").unwrap(),
            "SLUS_012.34"
        );
    }

    #[test]
    fn test_parse_boot_path() {
        let cnf = b"BOOT = cdrom:\\SLES_039.36;1\r\nTCB = 4\r\n";
        assert_eq!(parse_boot_path(cnf).unwrap(), "cdrom:\\SLES_039.36;1");
    }
}
