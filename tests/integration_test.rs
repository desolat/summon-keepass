mod common;

use common::*;

// ===== Help Flag Tests =====

#[test]
fn test_help_flag_short() {
    let (stdout, stderr, exit_code) = run_summon_keepass(&["-h"]);
    assert_success(&stdout, &stderr, exit_code);

    // Verify help text contains key sections
    assert!(stdout.contains("summon-keepass"), "Help should contain program name");
    assert!(stdout.contains("USAGE:"), "Help should contain USAGE section");
    assert!(stdout.contains("OPTIONS:"), "Help should contain OPTIONS section");
    assert!(stdout.contains("SECRET PATH FORMAT:"), "Help should contain SECRET PATH FORMAT section");
    assert!(stdout.contains("EXAMPLES:"), "Help should contain EXAMPLES section");
    assert!(stdout.contains("CONFIGURATION:"), "Help should contain CONFIGURATION section");
    assert!(stdout.contains("EXIT CODES:"), "Help should contain EXIT CODES section");
    assert!(stdout.contains("SUMMON_KEEPASS_DB_PATH"), "Help should mention environment variables");
    assert!(stdout.contains("~/.summon-keepass.ini"), "Help should mention config file");
}

#[test]
fn test_help_flag_long() {
    let (stdout, stderr, exit_code) = run_summon_keepass(&["--help"]);
    assert_success(&stdout, &stderr, exit_code);

    // Verify help text contains key sections
    assert!(stdout.contains("summon-keepass"), "Help should contain program name");
    assert!(stdout.contains("USAGE:"), "Help should contain USAGE section");
    assert!(stdout.contains("CONFIGURATION:"), "Help should contain CONFIGURATION section");
}

// ===== Version Flag Tests =====

#[test]
fn test_version_flag_short() {
    let (stdout, stderr, exit_code) = run_summon_keepass(&["-V"]);
    assert_success(&stdout, &stderr, exit_code);

    // Validate semver format (MAJOR.MINOR.PATCH or MAJOR.MINOR.PATCH-PRERELEASE)
    let version = stdout.trim();
    assert!(!version.is_empty(), "Version output should not be empty");

    // Check for basic semver pattern: starts with digit, contains dots
    let parts: Vec<&str> = version.split('-').next().unwrap().split('.').collect();
    assert_eq!(parts.len(), 3, "Version should have 3 parts (MAJOR.MINOR.PATCH), got: {}", version);

    // Verify each part is a number
    for part in parts {
        assert!(part.parse::<u32>().is_ok(), "Version part '{}' should be a number in: {}", part, version);
    }
}

#[test]
fn test_version_flag_long() {
    let (stdout, stderr, exit_code) = run_summon_keepass(&["--version"]);
    assert_success(&stdout, &stderr, exit_code);

    // Validate semver format (MAJOR.MINOR.PATCH or MAJOR.MINOR.PATCH-PRERELEASE)
    let version = stdout.trim();
    assert!(!version.is_empty(), "Version output should not be empty");

    // Check for basic semver pattern: starts with digit, contains dots
    let parts: Vec<&str> = version.split('-').next().unwrap().split('.').collect();
    assert_eq!(parts.len(), 3, "Version should have 3 parts (MAJOR.MINOR.PATCH), got: {}", version);

    // Verify each part is a number
    for part in parts {
        assert!(part.parse::<u32>().is_ok(), "Version part '{}' should be a number in: {}", part, version);
    }
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

// ===== Configuration Tests =====

#[test]
fn test_env_var_config_path_and_pass() {
    let (stdout, stderr, exit_code) = run_with_env_only(&["simple-entry"]);
    assert_success(&stdout, &stderr, exit_code);
    assert_eq!(stdout.trim(), "simple-password",
        "Expected 'simple-password', got: '{}'", stdout);
}

#[test]
fn test_env_var_priority_over_config_file() {
    // Set environment variable to point to test DB, config file has same path
    // Since they both point to same DB, we just verify env vars work when both are present
    let test_db_path = get_test_db_path();
    let (stdout, stderr, exit_code) = run_with_env_override(
        &["simple-entry"],
        Some(test_db_path.to_str().unwrap()),
        Some("test123")
    );
    assert_success(&stdout, &stderr, exit_code);
    assert_eq!(stdout.trim(), "simple-password",
        "Expected 'simple-password', got: '{}'", stdout);
}

#[test]
fn test_config_file_fallback_when_no_env_vars() {
    // Existing tests already cover this, but explicit test for clarity
    let (stdout, stderr, exit_code) = run_with_config_only(&["simple-entry"]);
    assert_success(&stdout, &stderr, exit_code);
    assert_eq!(stdout.trim(), "simple-password",
        "Expected 'simple-password', got: '{}'", stdout);
}

#[test]
fn test_partial_env_config_path_only() {
    // Set only SUMMON_KEEPASS_DB_PATH via env, password comes from config file
    let test_db_path = get_test_db_path();
    let (stdout, stderr, exit_code) = run_with_env_override(
        &["simple-entry"],
        Some(test_db_path.to_str().unwrap()),
        None  // No password in env, should use config file
    );
    assert_success(&stdout, &stderr, exit_code);
    assert_eq!(stdout.trim(), "simple-password",
        "Expected 'simple-password', got: '{}'", stdout);
}

#[test]
fn test_partial_env_config_pass_only() {
    // Set only SUMMON_KEEPASS_DB_PASS via env, path comes from config file
    let (stdout, stderr, exit_code) = run_with_env_override(
        &["simple-entry"],
        None,  // No path in env, should use config file
        Some("test123")
    );
    assert_success(&stdout, &stderr, exit_code);
    assert_eq!(stdout.trim(), "simple-password",
        "Expected 'simple-password', got: '{}'", stdout);
}

#[test]
fn test_missing_config_error_message() {
    use assert_cmd::Command;

    let mut cmd = Command::cargo_bin("summon-keepass").expect("Failed to find binary");

    // Don't set environment variables
    // Set HOME to nonexistent directory so config file is not found
    cmd.env("HOME", "/tmp/nonexistent-dir-for-summon-keepass-test");
    cmd.args(&["simple-entry"]);

    let output = cmd.output().expect("Failed to execute command");
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let exit_code = output.status.code().unwrap_or(-1);

    assert_eq!(exit_code, 1, "Expected exit code 1, got: {}", exit_code);
    assert!(stderr.contains("Configuration error"),
        "Expected 'Configuration error' in stderr, got: {}", stderr);
    assert!(stderr.contains("SUMMON_KEEPASS_DB_PATH"),
        "Expected 'SUMMON_KEEPASS_DB_PATH' in stderr, got: {}", stderr);
    assert!(stderr.contains("SUMMON_KEEPASS_DB_PASS"),
        "Expected 'SUMMON_KEEPASS_DB_PASS' in stderr, got: {}", stderr);
    assert!(stderr.contains(".summon-keepass.ini"),
        "Expected '.summon-keepass.ini' in stderr, got: {}", stderr);
}

#[test]
fn test_backward_compatibility_existing_users() {
    // Verify that config file still works (all existing tests use config file)
    // This test explicitly documents the backward compatibility requirement
    let (stdout, stderr, exit_code) = run_summon_keepass(&["simple-entry"]);
    assert_success(&stdout, &stderr, exit_code);
    assert_eq!(stdout.trim(), "simple-password",
        "Expected 'simple-password', got: '{}'", stdout);
}
