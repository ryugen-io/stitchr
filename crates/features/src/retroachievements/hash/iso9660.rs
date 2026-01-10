//! ISO9660 filesystem reader
//!
//! Supports both standard ISOs (2048-byte sectors) and raw CD images (2352-byte
//! sectors).

use std::path::Path;

/// Standard ISO sector size
const SECTOR_SIZE_ISO: usize = 2048;
/// Raw CD sector size (Mode 2)
const SECTOR_SIZE_RAW: usize = 2352;
/// Data offset in raw sector (sync + header + subheader)
const RAW_DATA_OFFSET: usize = 24;
/// Primary Volume Descriptor location
const PVD_SECTOR: usize = 16;

/// ISO image format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsoFormat {
    /// Standard ISO (2048-byte sectors)
    Standard,
    /// Raw CD image (2352-byte sectors, Mode 2 Form 1)
    Raw,
}

/// ISO9660 filesystem reader
pub struct Iso9660Reader<'a> {
    data: &'a [u8],
    format: IsoFormat,
}

impl<'a> Iso9660Reader<'a> {
    /// Create a new ISO9660 reader, auto-detecting format
    pub fn new(data: &'a [u8]) -> Result<Self, String> {
        let format = Self::detect_format(data)?;
        Ok(Self { data, format })
    }

    /// Detect ISO format by checking for CD001 signature
    fn detect_format(data: &[u8]) -> Result<IsoFormat, String> {
        // Try standard ISO first (sector 16 at 16 * 2048)
        let std_offset = PVD_SECTOR * SECTOR_SIZE_ISO;
        if data.len() > std_offset + 6 && &data[std_offset + 1..std_offset + 6] == b"CD001" {
            return Ok(IsoFormat::Standard);
        }

        // Try raw CD image (sector 16 at 16 * 2352 + 24)
        let raw_offset = PVD_SECTOR * SECTOR_SIZE_RAW + RAW_DATA_OFFSET;
        if data.len() > raw_offset + 6 && &data[raw_offset + 1..raw_offset + 6] == b"CD001" {
            return Ok(IsoFormat::Raw);
        }

        Err("Cannot detect ISO format: CD001 signature not found".to_string())
    }

    /// Read a single sector
    fn read_sector(&self, sector: usize) -> Result<&[u8], String> {
        let (offset, size) = match self.format {
            IsoFormat::Standard => (sector * SECTOR_SIZE_ISO, SECTOR_SIZE_ISO),
            IsoFormat::Raw => (sector * SECTOR_SIZE_RAW + RAW_DATA_OFFSET, SECTOR_SIZE_ISO),
        };

        if offset + size > self.data.len() {
            return Err(format!("Sector {} out of bounds", sector));
        }

        Ok(&self.data[offset..offset + size])
    }

    /// Read multiple consecutive sectors
    fn read_sectors(&self, start_sector: usize, count: usize) -> Result<Vec<u8>, String> {
        let mut data = Vec::with_capacity(count * SECTOR_SIZE_ISO);
        for i in 0..count {
            data.extend_from_slice(self.read_sector(start_sector + i)?);
        }
        Ok(data)
    }

    /// Read a file from the root directory
    pub fn read_file(&self, filename: &str) -> Result<Vec<u8>, String> {
        // Read Primary Volume Descriptor
        let pvd = self.read_sector(PVD_SECTOR)?;

        // Verify PVD
        if pvd[0] != 1 || &pvd[1..6] != b"CD001" {
            return Err("Invalid Primary Volume Descriptor".to_string());
        }

        // Root directory record at offset 156
        let root_lba =
            u32::from_le_bytes([pvd[156 + 2], pvd[156 + 3], pvd[156 + 4], pvd[156 + 5]]) as usize;
        let root_size =
            u32::from_le_bytes([pvd[156 + 10], pvd[156 + 11], pvd[156 + 12], pvd[156 + 13]])
                as usize;

        // Read root directory
        let root_dir = self.read_sectors(root_lba, root_size.div_ceil(SECTOR_SIZE_ISO))?;

        // Search for file
        self.find_file_in_directory(&root_dir, filename)
    }

    /// Read a file from a subdirectory path (e.g., "PSP_GAME/SYSDIR/EBOOT.BIN")
    pub fn read_file_path(&self, path: &str) -> Result<Vec<u8>, String> {
        let parts: Vec<&str> = path.split(['/', '\\']).filter(|s| !s.is_empty()).collect();

        if parts.is_empty() {
            return Err("Empty path".to_string());
        }

        // Read PVD
        let pvd = self.read_sector(PVD_SECTOR)?;
        if pvd[0] != 1 || &pvd[1..6] != b"CD001" {
            return Err("Invalid Primary Volume Descriptor".to_string());
        }

        // Start with root directory
        let mut dir_lba =
            u32::from_le_bytes([pvd[156 + 2], pvd[156 + 3], pvd[156 + 4], pvd[156 + 5]]) as usize;
        let mut dir_size =
            u32::from_le_bytes([pvd[156 + 10], pvd[156 + 11], pvd[156 + 12], pvd[156 + 13]])
                as usize;

        // Navigate through directories
        for (i, part) in parts.iter().enumerate() {
            let dir_data = self.read_sectors(dir_lba, dir_size.div_ceil(SECTOR_SIZE_ISO))?;

            if i == parts.len() - 1 {
                // Last part - this is the file
                return self.find_file_in_directory(&dir_data, part);
            } else {
                // Navigate to subdirectory
                let (lba, size) = self.find_directory_in_directory(&dir_data, part)?;
                dir_lba = lba;
                dir_size = size;
            }
        }

        Err("Path not found".to_string())
    }

    /// Find a file in a directory and return its contents
    fn find_file_in_directory(&self, dir_data: &[u8], filename: &str) -> Result<Vec<u8>, String> {
        let filename_upper = filename.to_uppercase();
        let filename_no_version = filename_upper.trim_end_matches(";1");

        let mut offset = 0;
        while offset < dir_data.len() {
            let record_len = dir_data[offset] as usize;
            if record_len == 0 {
                let next_sector = ((offset / SECTOR_SIZE_ISO) + 1) * SECTOR_SIZE_ISO;
                if next_sector >= dir_data.len() {
                    break;
                }
                offset = next_sector;
                continue;
            }

            if offset + record_len > dir_data.len() {
                break;
            }

            let name_len = dir_data[offset + 32] as usize;
            if name_len > 0 && offset + 33 + name_len <= dir_data.len() {
                let name = String::from_utf8_lossy(&dir_data[offset + 33..offset + 33 + name_len])
                    .to_uppercase();
                let name_no_version = name.trim_end_matches(";1");

                if name_no_version == filename_no_version || name == filename_upper {
                    let file_lba = u32::from_le_bytes([
                        dir_data[offset + 2],
                        dir_data[offset + 3],
                        dir_data[offset + 4],
                        dir_data[offset + 5],
                    ]) as usize;
                    let file_size = u32::from_le_bytes([
                        dir_data[offset + 10],
                        dir_data[offset + 11],
                        dir_data[offset + 12],
                        dir_data[offset + 13],
                    ]) as usize;

                    let sectors_needed = file_size.div_ceil(SECTOR_SIZE_ISO);
                    let mut data = self.read_sectors(file_lba, sectors_needed)?;
                    data.truncate(file_size);
                    return Ok(data);
                }
            }

            offset += record_len;
        }

        Err(format!("File not found: {}", filename))
    }

    /// Find a directory and return its LBA and size
    fn find_directory_in_directory(
        &self,
        dir_data: &[u8],
        dirname: &str,
    ) -> Result<(usize, usize), String> {
        let dirname_upper = dirname.to_uppercase();

        let mut offset = 0;
        while offset < dir_data.len() {
            let record_len = dir_data[offset] as usize;
            if record_len == 0 {
                let next_sector = ((offset / SECTOR_SIZE_ISO) + 1) * SECTOR_SIZE_ISO;
                if next_sector >= dir_data.len() {
                    break;
                }
                offset = next_sector;
                continue;
            }

            if offset + record_len > dir_data.len() {
                break;
            }

            // Check if this is a directory (file flags at offset 25, bit 1 = directory)
            let flags = dir_data[offset + 25];
            let is_dir = (flags & 0x02) != 0;

            let name_len = dir_data[offset + 32] as usize;
            if name_len > 0 && offset + 33 + name_len <= dir_data.len() && is_dir {
                let name = String::from_utf8_lossy(&dir_data[offset + 33..offset + 33 + name_len])
                    .to_uppercase();
                let name_no_version = name.trim_end_matches(";1");

                if name_no_version == dirname_upper || name == dirname_upper {
                    let dir_lba = u32::from_le_bytes([
                        dir_data[offset + 2],
                        dir_data[offset + 3],
                        dir_data[offset + 4],
                        dir_data[offset + 5],
                    ]) as usize;
                    let dir_size = u32::from_le_bytes([
                        dir_data[offset + 10],
                        dir_data[offset + 11],
                        dir_data[offset + 12],
                        dir_data[offset + 13],
                    ]) as usize;

                    return Ok((dir_lba, dir_size));
                }
            }

            offset += record_len;
        }

        Err(format!("Directory not found: {}", dirname))
    }
}

/// Resolve .cue to .bin path, or return ISO path directly
pub fn resolve_image_path(path: &Path) -> Result<std::path::PathBuf, String> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
        .unwrap_or_default();

    match ext.as_str() {
        "bin" | "iso" => Ok(path.to_path_buf()),
        "cue" => {
            let cue_content = std::fs::read_to_string(path)
                .map_err(|e| format!("Failed to read CUE file: {}", e))?;

            for line in cue_content.lines() {
                let line = line.trim();
                if line.to_uppercase().starts_with("FILE")
                    && let Some(start) = line.find('"')
                    && let Some(end) = line[start + 1..].find('"')
                {
                    let bin_name = &line[start + 1..start + 1 + end];
                    let bin_path = path.parent().unwrap_or(Path::new(".")).join(bin_name);
                    return Ok(bin_path);
                }
            }
            Err("No BIN file found in CUE".to_string())
        }
        _ => Err(format!("Unsupported file type: {}", ext)),
    }
}

/// Check if an ISO file is a PSP game (has PSP_GAME directory)
pub fn is_psp_iso(data: &[u8]) -> bool {
    let Ok(iso) = Iso9660Reader::new(data) else {
        return false;
    };
    // Try to find PSP_GAME directory
    iso.read_file_path("PSP_GAME/PARAM.SFO").is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iso_format_detection() {
        // Create minimal standard ISO PVD at sector 16
        let mut iso_data = vec![0u8; 17 * 2048 + 10];
        let pvd_offset = 16 * 2048;
        iso_data[pvd_offset] = 1; // Type
        iso_data[pvd_offset + 1..pvd_offset + 6].copy_from_slice(b"CD001");

        let format = Iso9660Reader::detect_format(&iso_data).unwrap();
        assert_eq!(format, IsoFormat::Standard);
    }

    #[test]
    fn test_raw_format_detection() {
        // Create minimal raw CD PVD at sector 16
        let mut raw_data = vec![0u8; 17 * 2352 + 30];
        let pvd_offset = 16 * 2352 + 24; // Raw sector + data offset
        raw_data[pvd_offset] = 1;
        raw_data[pvd_offset + 1..pvd_offset + 6].copy_from_slice(b"CD001");

        let format = Iso9660Reader::detect_format(&raw_data).unwrap();
        assert_eq!(format, IsoFormat::Raw);
    }
}
