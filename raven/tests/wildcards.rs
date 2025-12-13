use raven::handler::match_wildcard;

#[test]
fn exact_match_no_params() {
    let result = match_wildcard("user::profile", "user::profile");
    assert!(result.is_some());
    assert!(result.unwrap().is_empty());
}

#[test]
fn wildcard_single_param() {
    let params = match_wildcard("user::42", "user::{id}").unwrap();
    assert_eq!(params.get("id"), Some(&"42".to_string()));
}

#[test]
fn multiple_params() {
    let params = match_wildcard("game::1::level::3", "game::{game_id}::level::{lvl}").unwrap();

    assert_eq!(params["game_id"], "1");
    assert_eq!(params["lvl"], "3");
}

#[test]
fn mismatch_literal() {
    assert!(match_wildcard("user::42", "admin::{id}").is_none());
}
