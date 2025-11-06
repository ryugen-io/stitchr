//! EBP helper functions

use super::constants::*;

/// Find the start of JSON metadata in patch data
/// Returns None if no JSON found
pub fn find_json_start(patch: &[u8]) -> Option<usize> {
    // Look for EOF marker first
    let eof_pos = patch
        .windows(EOF_MARKER.len())
        .position(|w| w == EOF_MARKER)?;

    // JSON should start after EOF marker
    let search_start = eof_pos + EOF_MARKER.len();

    // Find opening brace
    patch[search_start..]
        .iter()
        .position(|&b| b == JSON_START_MARKER)
        .map(|pos| search_start + pos)
}

/// Extract JSON string from patch data
/// Returns empty string if no valid JSON found
pub fn extract_json(patch: &[u8]) -> &str {
    let start = match find_json_start(patch) {
        Some(pos) => pos,
        None => return "",
    };

    // Find matching closing brace
    let end = match patch[start..].iter().rposition(|&b| b == JSON_END_MARKER) {
        Some(pos) => start + pos + 1,
        None => return "",
    };

    // Convert to UTF-8 string
    std::str::from_utf8(&patch[start..end]).unwrap_or("")
}
