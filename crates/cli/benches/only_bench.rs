use rom_patcher_cli::OnlyMode;

fn main() {
    divan::main();
}

/// Benchmark creating a Vec<OnlyMode> with single mode
#[divan::bench]
fn single_mode_verify() -> Vec<OnlyMode> {
    vec![divan::black_box(OnlyMode::Verify)]
}

/// Benchmark creating a Vec<OnlyMode> with multiple modes
#[divan::bench]
fn multiple_modes_verify_ra() -> Vec<OnlyMode> {
    vec![
        divan::black_box(OnlyMode::Verify),
        divan::black_box(OnlyMode::Ra),
    ]
}

/// Benchmark checking if Vec contains a mode with any()
#[divan::bench]
fn mode_check_any_verify() -> bool {
    let modes = vec![OnlyMode::Verify, OnlyMode::Ra];
    divan::black_box(modes.iter().any(|m| matches!(m, OnlyMode::Verify)))
}

/// Benchmark iterating over modes
#[divan::bench]
fn mode_iteration() {
    let modes = vec![OnlyMode::Verify, OnlyMode::Ra];
    for mode in divan::black_box(&modes) {
        divan::black_box(mode);
    }
}

/// Benchmark empty Vec check (true case)
#[divan::bench]
fn empty_check_true() -> bool {
    let empty_modes: Vec<OnlyMode> = vec![];
    divan::black_box(empty_modes.is_empty())
}

/// Benchmark empty Vec check (false case)
#[divan::bench]
fn empty_check_false() -> bool {
    let non_empty_modes = vec![OnlyMode::Verify];
    divan::black_box(non_empty_modes.is_empty())
}
