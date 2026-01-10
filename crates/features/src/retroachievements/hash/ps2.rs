//! PlayStation 2 hash algorithm for RetroAchievements
//!
//! Algorithm:
//! 1. Load and parse SYSTEM.CNF from disc
//! 2. Find BOOT2= line to identify primary executable
//! 3. Write executable path to buffer
//! 4. Append executable contents to buffer
//! 5. MD5 hash the buffer

use std::path::Path;

use super::iso9660::{Iso9660Reader, resolve_image_path};

/// Compute RetroAchievements hash for a PS2 disc image
pub fn compute_ps2_hash(path: &Path) -> Result<String, String> {
    let image_path = resolve_image_path(path)?;
    let image_data =
        std::fs::read(&image_path).map_err(|e| format!("Failed to read image: {}", e))?;

    let iso = Iso9660Reader::new(&image_data)?;

    // Read SYSTEM.CNF
    let system_cnf = iso
        .read_file("SYSTEM.CNF")
        .or_else(|_| iso.read_file("SYSTEM.CNF;1"))?;

    // Parse BOOT2= line (PS2 uses BOOT2, not BOOT)
    let boot_path = parse_boot2_path(&system_cnf)?;

    // Extract executable name
    let exe_name = extract_exe_name(&boot_path)?;

    // Read the executable
    let exe_data = iso
        .read_file(&exe_name)
        .or_else(|_| iso.read_file(&format!("{};1", exe_name)))?;

    // Build hash buffer: exe_name + exe_contents
    let mut buffer = Vec::with_capacity(exe_name.len() + exe_data.len());
    buffer.extend_from_slice(exe_name.as_bytes());
    buffer.extend_from_slice(&exe_data);

    let digest = md5::compute(&buffer);
    Ok(format!("{:x}", digest))
}

/// Parse BOOT2= line from SYSTEM.CNF content
fn parse_boot2_path(system_cnf: &[u8]) -> Result<String, String> {
    let content = String::from_utf8_lossy(system_cnf);

    for line in content.lines() {
        let line = line.trim();
        // Handle both "BOOT2=" and "BOOT2 =" formats
        if let Some(pos) = line.to_uppercase().find("BOOT2") {
            let rest = &line[pos + 5..].trim_start();
            if let Some(stripped) = rest.strip_prefix('=') {
                let path = stripped.trim();
                return Ok(path.to_string());
            }
        }
    }

    Err("BOOT2= line not found in SYSTEM.CNF".to_string())
}

/// Extract executable name from boot path
/// e.g., "cdrom0:\SLUS_123.45;1" -> "SLUS_123.45"
fn extract_exe_name(boot_path: &str) -> Result<String, String> {
    let path = boot_path
        .trim_start_matches("cdrom:")
        .trim_start_matches("cdrom0:")
        .trim_start_matches('\\')
        .trim_start_matches('/');

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
    fn test_parse_boot2_path() {
        let cnf = b"BOOT2 = cdrom0:\\SLUS_123.45;1\r\nVER = 1.00\r\n";
        assert_eq!(parse_boot2_path(cnf).unwrap(), "cdrom0:\\SLUS_123.45;1");
    }

    #[test]
    fn test_extract_exe_name() {
        assert_eq!(
            extract_exe_name("cdrom0:\\SLUS_123.45;1").unwrap(),
            "SLUS_123.45"
        );
        assert_eq!(
            extract_exe_name("cdrom0:\\SLES_567.89").unwrap(),
            "SLES_567.89"
        );
    }
}
