//! RetroAchievements API client

use super::parser::parse_game_id_response;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const RA_API_BASE: &str = "https://retroachievements.org";
const RA_USER_AGENT: &str = "rompatcherrs/0.1";
const RATE_LIMIT_MS: u64 = 500; // 500ms between requests

/// Get path to rate limit file in XDG cache directory
fn get_rate_limit_file() -> Result<PathBuf, String> {
    let cache_dir = if let Ok(xdg_cache) = env::var("XDG_CACHE_HOME") {
        PathBuf::from(xdg_cache)
    } else {
        let home = env::var("HOME").map_err(|_| "HOME environment variable not set")?;
        PathBuf::from(home).join(".cache")
    };

    let app_cache = cache_dir.join("rompatcherrs");
    fs::create_dir_all(&app_cache)
        .map_err(|e| format!("Failed to create cache directory: {}", e))?;

    Ok(app_cache.join("ra-last-request"))
}

/// File-based rate limiter (works across process invocations)
fn wait_if_needed() -> Result<(), String> {
    let rate_file = get_rate_limit_file()?;

    // Read last request timestamp if file exists
    if let Ok(contents) = fs::read_to_string(&rate_file)
        && let Ok(last_ts) = contents.trim().parse::<u64>()
    {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let elapsed = now.saturating_sub(last_ts);

        if elapsed < RATE_LIMIT_MS {
            std::thread::sleep(Duration::from_millis(RATE_LIMIT_MS - elapsed));
        }
    }

    // Write new timestamp
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    fs::write(&rate_file, now.to_string())
        .map_err(|e| format!("Failed to write rate limit file: {}", e))?;

    Ok(())
}

/// Look up game ID by MD5 hash
pub fn lookup_game_by_hash(md5_hash: &str) -> Result<Option<u32>, String> {
    // Rate limit (file-based, works across process invocations)
    wait_if_needed()?;

    // Build URL
    let url = format!("{}/dorequest.php?r=gameid&m={}", RA_API_BASE, md5_hash);

    // Make request
    let response = minreq::get(&url)
        .with_header("User-Agent", RA_USER_AGENT)
        .send()
        .map_err(|e| format!("API request failed: {}", e))?;

    // Read response as string (no allocation - just borrow)
    let json = response
        .as_str()
        .map_err(|e| format!("Failed to read response: {}", e))?;

    // Parse manually (no serde needed!)
    parse_game_id_response(json)
}

/// Get RetroAchievements game URL
pub fn game_url(game_id: u32) -> String {
    format!("{}/game/{}", RA_API_BASE, game_id)
}
