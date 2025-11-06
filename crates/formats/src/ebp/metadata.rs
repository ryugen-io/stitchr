//! EBP metadata extraction

use super::helpers;

/// EBP metadata structure
#[derive(Debug, Clone, PartialEq)]
pub struct EbpMetadata {
    pub title: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
    pub version: Option<String>,
}

impl EbpMetadata {
    /// Extract metadata from EBP patch
    pub fn from_patch(patch: &[u8]) -> Self {
        let json_str = helpers::extract_json(patch);

        if json_str.is_empty() {
            return Self::empty();
        }

        Self {
            title: extract_string_field(json_str, "title"),
            author: extract_string_field(json_str, "author"),
            description: extract_string_field(json_str, "description"),
            version: extract_string_field(json_str, "version"),
        }
    }

    /// Create empty metadata
    fn empty() -> Self {
        Self {
            title: None,
            author: None,
            description: None,
            version: None,
        }
    }
}

/// Extract a string field from JSON manually
fn extract_string_field(json: &str, field: &str) -> Option<String> {
    let pattern = format!("\"{}\"", field);
    let start = json.find(&pattern)?;
    let after_key = &json[start + pattern.len()..];

    let colon = after_key.find(':')?;
    let after_colon = &after_key[colon + 1..].trim_start();

    if !after_colon.starts_with('"') {
        return None;
    }

    let value_start = 1;
    let mut value_end = value_start;
    let chars: Vec<char> = after_colon.chars().collect();

    while value_end < chars.len() {
        if chars[value_end] == '\\' && value_end + 1 < chars.len() {
            value_end += 2;
            continue;
        }
        if chars[value_end] == '"' {
            break;
        }
        value_end += 1;
    }

    if value_end >= chars.len() {
        return None;
    }

    let value: String = chars[value_start..value_end].iter().collect();
    Some(unescape_json_string(&value))
}

/// Unescape JSON string escapes
fn unescape_json_string(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars();

    while let Some(ch) = chars.next() {
        if ch == '\\' {
            if let Some(next) = chars.next() {
                match next {
                    'n' => result.push('\n'),
                    't' => result.push('\t'),
                    'r' => result.push('\r'),
                    '\\' => result.push('\\'),
                    '"' => result.push('"'),
                    _ => {
                        result.push('\\');
                        result.push(next);
                    }
                }
            }
        } else {
            result.push(ch);
        }
    }

    result
}
