mod common;

use common::*;

// ===== Version Flag Tests =====

#[test]
fn test_version_flag_short() {
    let (stdout, stderr, exit_code) = run_summon_keepass(&["-V"]);
    assert_success(&stdout, &stderr, exit_code);
    assert!(stdout.contains("0.3.1-rc.1"), "Version output should contain '0.3.0', got: {}", stdout);
}

#[test]
fn test_version_flag_long() {
    let (stdout, stderr, exit_code) = run_summon_keepass(&["--version"]);
    assert_success(&stdout, &stderr, exit_code);
    assert!(stdout.contains("0.3.1-rc.1"), "Version output should contain '0.3.0', got: {}", stdout);
}

// ===== Basic Retrieval Tests =====

#[test]
fn test_no_arguments() {
    let (stdout, stderr, exit_code) = run_summon_keepass(&[]);
    assert_failure(exit_code, 1, &stdout, &stderr);
    assert!(stderr.contains("no variable was provided"),
        "Expected 'no variable was provided' error, got: {}", stderr);
}

#[test]
fn test_simple_password_retrieval() {
    let (stdout, stderr, exit_code) = run_summon_keepass(&["simple-entry"]);
    assert_success(&stdout, &stderr, exit_code);
    assert_eq!(stdout.trim(), "simple-password",
        "Expected 'simple-password', got: '{}'", stdout);
}

#[test]
fn test_nested_group_path() {
    let (stdout, stderr, exit_code) = run_summon_keepass(&["aws/iam/user/robot"]);
    assert_success(&stdout, &stderr, exit_code);
    assert_eq!(stdout.trim(), "aws-secret-key",
        "Expected 'aws-secret-key', got: '{}'", stdout);
}

#[test]
fn test_deeply_nested_path() {
    let (stdout, stderr, exit_code) = run_summon_keepass(&["test-group/sub-group/nested-entry"]);
    assert_success(&stdout, &stderr, exit_code);
    assert_eq!(stdout.trim(), "nested-password",
        "Expected 'nested-password', got: '{}'", stdout);
}

// ===== Field Access Tests =====

#[test]
fn test_custom_field_access() {
    let (stdout, stderr, exit_code) = run_summon_keepass(&["aws/iam/user/robot|access_key_id"]);
    assert_success(&stdout, &stderr, exit_code);
    assert_eq!(stdout.trim(), "AKIAIOSFODNN7EXAMPLE",
        "Expected 'AKIAIOSFODNN7EXAMPLE', got: '{}'", stdout);
}

#[test]
fn test_username_field() {
    let (stdout, stderr, exit_code) = run_summon_keepass(&["simple-entry|UserName"]);
    assert_success(&stdout, &stderr, exit_code);
    assert_eq!(stdout.trim(), "simple-user",
        "Expected 'simple-user', got: '{}'", stdout);
}

#[test]
fn test_url_field() {
    let (stdout, stderr, exit_code) = run_summon_keepass(&["special-chars|URL"]);
    assert_success(&stdout, &stderr, exit_code);
    assert_eq!(stdout.trim(), "https://example.com",
        "Expected 'https://example.com', got: '{}'", stdout);
}

#[test]
fn test_multiline_field() {
    let (stdout, stderr, exit_code) = run_summon_keepass(&["ssh/some-server|priv_key"]);
    assert_success(&stdout, &stderr, exit_code);
    assert!(stdout.contains("BEGIN RSA PRIVATE KEY"),
        "Expected private key to contain 'BEGIN RSA PRIVATE KEY', got: {}", stdout);
}

#[test]
fn test_custom_field_in_nested_entry() {
    let (stdout, stderr, exit_code) = run_summon_keepass(&["test-group/sub-group/nested-entry|custom-field"]);
    assert_success(&stdout, &stderr, exit_code);
    assert_eq!(stdout.trim(), "custom-value",
        "Expected 'custom-value', got: '{}'", stdout);
}

// ===== Line Ending Conversion Test =====

#[test]
fn test_dos_line_ending_conversion() {
    let (stdout, stderr, exit_code) = run_summon_keepass(&["test-group/multiline"]);
    assert_success(&stdout, &stderr, exit_code);

    // Should not contain \r (CR), only \n (LF)
    assert!(!stdout.contains('\r'),
        "Output should not contain CR characters (DOS line endings)");
    assert!(stdout.contains("line1\nline2\nline3"),
        "Expected Unix line endings, got: {:?}", stdout);
}

// ===== Error Handling Tests =====

#[test]
fn test_nonexistent_entry() {
    let (stdout, stderr, exit_code) = run_summon_keepass(&["nonexistent/entry"]);
    assert_failure(exit_code, 1, &stdout, &stderr);
    assert!(stderr.contains("could not be retrieved"),
        "Expected 'could not be retrieved' error, got: {}", stderr);
}

#[test]
fn test_nonexistent_field() {
    let (stdout, stderr, exit_code) = run_summon_keepass(&["simple-entry|nonexistent-field"]);
    assert_failure(exit_code, 1, &stdout, &stderr);
    // Field doesn't exist, should fail
}

#[test]
fn test_invalid_path_multiple_pipes() {
    let (stdout, stderr, exit_code) = run_summon_keepass(&["entry|field|extra"]);
    assert_failure(exit_code, 2, &stdout, &stderr);
    assert!(stderr.contains("is no valid secret path"),
        "Expected 'is no valid secret path' error, got: {}", stderr);
}

// ===== Special Characters Tests =====

#[test]
fn test_special_characters_in_password() {
    let (stdout, stderr, exit_code) = run_summon_keepass(&["special-chars"]);
    assert_success(&stdout, &stderr, exit_code);
    assert_eq!(stdout.trim(), "p@$$w0rd!#%",
        "Expected 'p@$$w0rd!#%', got: '{}'", stdout);
}

#[test]
fn test_special_characters_in_username() {
    let (stdout, stderr, exit_code) = run_summon_keepass(&["special-chars|UserName"]);
    assert_success(&stdout, &stderr, exit_code);
    assert_eq!(stdout.trim(), "user@example.com",
        "Expected 'user@example.com', got: '{}'", stdout);
}
