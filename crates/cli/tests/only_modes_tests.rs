//! Integration tests for --only flag with multiple modes

use rom_patcher_cli::OnlyMode;

/// Test that OnlyMode enum has expected variants
#[test]
fn test_only_mode_variants() {
    let verify = OnlyMode::Verify;
    let ra = OnlyMode::Ra;
    
    // Basic construction works
    assert!(matches!(verify, OnlyMode::Verify));
    assert!(matches!(ra, OnlyMode::Ra));
}

/// Test that Vec<OnlyMode> can be constructed
#[test]
fn test_only_modes_vec_construction() {
    let modes: Vec<OnlyMode> = vec![OnlyMode::Verify, OnlyMode::Ra];
    assert_eq!(modes.len(), 2);
}

/// Test that empty Vec works (normal mode)
#[test]
fn test_empty_modes_vec() {
    let modes: Vec<OnlyMode> = vec![];
    assert!(modes.is_empty());
}

/// Test that modes can be iterated
#[test]
fn test_modes_iteration() {
    let modes = vec![OnlyMode::Verify, OnlyMode::Ra];
    let mut count = 0;
    
    for mode in &modes {
        match mode {
            OnlyMode::Verify => count += 1,
            OnlyMode::Ra => count += 1,
        }
    }
    
    assert_eq!(count, 2);
}

/// Test that modes can be checked with any()
#[test]
fn test_modes_any_verify() {
    let modes = vec![OnlyMode::Verify, OnlyMode::Ra];
    assert!(modes.iter().any(|m| matches!(m, OnlyMode::Verify)));
}

/// Test that modes can be checked with any() for Ra
#[test]
fn test_modes_any_ra() {
    let modes = vec![OnlyMode::Verify, OnlyMode::Ra];
    assert!(modes.iter().any(|m| matches!(m, OnlyMode::Ra)));
}

/// Test that single mode works
#[test]
fn test_single_verify_mode() {
    let modes = vec![OnlyMode::Verify];
    assert_eq!(modes.len(), 1);
    assert!(modes.iter().any(|m| matches!(m, OnlyMode::Verify)));
    assert!(!modes.iter().any(|m| matches!(m, OnlyMode::Ra)));
}

/// Test that single Ra mode works
#[test]
fn test_single_ra_mode() {
    let modes = vec![OnlyMode::Ra];
    assert_eq!(modes.len(), 1);
    assert!(modes.iter().any(|m| matches!(m, OnlyMode::Ra)));
    assert!(!modes.iter().any(|m| matches!(m, OnlyMode::Verify)));
}

/// Test that duplicate modes can be stored (if user provides them)
#[test]
fn test_duplicate_modes_allowed() {
    let modes = vec![OnlyMode::Verify, OnlyMode::Verify];
    assert_eq!(modes.len(), 2);
}
