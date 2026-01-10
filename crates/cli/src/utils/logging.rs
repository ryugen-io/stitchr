//! Logging initialization utilities

use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

/// Initialize logger based on verbose count
///
/// - 0: Warn level (errors and warnings only)
/// - 1 (-v): Info level (normal operation details)
/// - 2 (-vv): Debug level (detailed internals)
/// - 3+ (-vvv): Trace level (everything)
pub fn init(verbose: u8) {
    let level = match verbose {
        0 => LevelFilter::Warn,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    Builder::new()
        // Set default to off for all modules
        .filter_level(LevelFilter::Off)
        // Only enable our crates
        .filter_module("stitchr", level)
        .filter_module("stitchr_cli", level)
        .filter_module("stitchr_core", level)
        .filter_module("stitchr_features", level)
        .filter_module("stitchr_formats", level)
        .format(|buf, record| writeln!(buf, "[{}] {}", record.level(), record.args()))
        .init();
}
