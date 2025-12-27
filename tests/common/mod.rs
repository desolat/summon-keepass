use assert_cmd::Command;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

/// Set up a test environment with HOME directory containing test config
pub fn setup_test_env() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let config_path = temp_dir.path().join(".summon-keepass.ini");

    // Get absolute path to test database
    let test_db_path = get_test_db_path();

    // Create config file in temp HOME directory
    let config_content = format!(
        "[keepass_db]\npath={}\npass=test123\n",
        test_db_path.display()
    );

    fs::write(&config_path, config_content).expect("Failed to write test config");

    temp_dir
}

/// Get the absolute path to the test database
pub fn get_test_db_path() -> PathBuf {
    let mut path = std::env::current_dir().expect("Failed to get current directory");
    path.push("tests/fixtures/test-database.kdbx");
    path
}

/// Run summon-keepass with the given arguments and test environment
pub fn run_summon_keepass(args: &[&str]) -> (String, String, i32) {
    let temp_home = setup_test_env();

    let mut cmd = Command::cargo_bin("summon-keepass").expect("Failed to find binary");
    cmd.env("HOME", temp_home.path());
    cmd.args(args);

    let output = cmd.output().expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let exit_code = output.status.code().unwrap_or(-1);

    (stdout, stderr, exit_code)
}

/// Assert command succeeded
pub fn assert_success(stdout: &str, stderr: &str, exit_code: i32) {
    assert_eq!(exit_code, 0,
        "Expected exit code 0, got {}.\nStdout: {}\nStderr: {}",
        exit_code, stdout, stderr);
}

/// Assert command failed with expected exit code
pub fn assert_failure(exit_code: i32, expected_code: i32, stdout: &str, stderr: &str) {
    assert_eq!(exit_code, expected_code,
        "Expected exit code {}, got {}.\nStdout: {}\nStderr: {}",
        expected_code, exit_code, stdout, stderr);
}

/// Run summon-keepass with environment variables for configuration (no config file)
pub fn run_with_env_only(args: &[&str]) -> (String, String, i32) {
    let test_db_path = get_test_db_path();

    let mut cmd = Command::cargo_bin("summon-keepass").expect("Failed to find binary");

    // Set environment variables for configuration
    cmd.env("SUMMON_KEEPASS_DB_PATH", test_db_path.to_str().unwrap());
    cmd.env("SUMMON_KEEPASS_DB_PASS", "test123");

    // Set HOME to a nonexistent directory to ensure no config file is used
    cmd.env("HOME", "/tmp/nonexistent-summon-keepass-test");

    cmd.args(args);

    let output = cmd.output().expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let exit_code = output.status.code().unwrap_or(-1);

    (stdout, stderr, exit_code)
}

/// Run summon-keepass with only config file (no environment variables)
pub fn run_with_config_only(args: &[&str]) -> (String, String, i32) {
    // This is the same as run_summon_keepass, but explicitly named for clarity
    run_summon_keepass(args)
}

/// Run summon-keepass with environment variables that override config file
pub fn run_with_env_override(args: &[&str], env_path: Option<&str>, env_pass: Option<&str>) -> (String, String, i32) {
    let temp_home = setup_test_env();

    let mut cmd = Command::cargo_bin("summon-keepass").expect("Failed to find binary");

    // Set HOME to use config file
    cmd.env("HOME", temp_home.path());

    // Optionally override with environment variables
    if let Some(path) = env_path {
        cmd.env("SUMMON_KEEPASS_DB_PATH", path);
    }
    if let Some(pass) = env_pass {
        cmd.env("SUMMON_KEEPASS_DB_PASS", pass);
    }

    cmd.args(args);

    let output = cmd.output().expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let exit_code = output.status.code().unwrap_or(-1);

    (stdout, stderr, exit_code)
}
