//! Tests for RetroAchievements JSON parser

use stitchr_features::retroachievements::parser::parse_game_id_response;

#[test]
fn test_parse_valid_response() {
    let json = r#"{"Success":true,"GameID":12345}"#;
    assert_eq!(parse_game_id_response(json).unwrap(), Some(12345));
}

#[test]
fn test_parse_with_whitespace() {
    let json = r#"{"Success":true, "GameID": 999}"#;
    assert_eq!(parse_game_id_response(json).unwrap(), Some(999));
}

#[test]
fn test_parse_failure_response() {
    let json = r#"{"Success":false,"GameID":0}"#;
    assert_eq!(parse_game_id_response(json).unwrap(), None);
}

#[test]
fn test_parse_zero_game_id() {
    let json = r#"{"Success":true,"GameID":0}"#;
    assert_eq!(parse_game_id_response(json).unwrap(), None);
}
