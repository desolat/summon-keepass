# summon-keepass Integration Tests

This directory contains integration tests for the summon-keepass CLI tool.

## Test Structure

```
tests/
├── fixtures/
│   ├── test-database.kdbx       # Test KeePass database (password: test123)
│   └── test-config.ini           # Test configuration (for reference)
├── common/
│   └── mod.rs                    # Shared test utilities
└── integration_test.rs           # Main integration tests
```

## Test Database

The test database (`test-database.kdbx`) contains the following entries:

### Root Level Entries
- **simple-entry**
  - Password: `simple-password`
  - UserName: `simple-user`

- **special-chars**
  - Password: `p@$$w0rd!#%`
  - UserName: `user@example.com`
  - URL: `https://example.com`

### Nested Groups

#### aws/iam/user/robot
- Password: `aws-secret-key`
- UserName: `AKIAIOSFODNN7EXAMPLE`
- Notes: `test-notes`
- Custom fields:
  - `access_key_id`: `AKIAIOSFODNN7EXAMPLE`
  - `secret_access_key`: `wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY`

#### ssh/some-server
- Password: `ssh-pass`
- UserName: `root`
- Custom fields:
  - `priv_key`: Multi-line RSA private key

#### test-group/sub-group/nested-entry
- Password: `nested-password`
- Custom fields:
  - `custom-field`: `custom-value`

#### test-group/multiline
- Password: `line1\r\nline2\r\nline3` (Windows line endings)
- Notes: `Windows line endings test`

**Database Password:** `test123`

## Test Coverage

The integration tests cover:

### Version Flag (2 tests)
- Short flag: `-V`
- Long flag: `--version`

### Basic Retrieval (4 tests)
- No arguments error handling
- Simple password retrieval
- Nested group path navigation
- Deeply nested path navigation

### Field Access (5 tests)
- Custom field retrieval
- UserName field access
- URL field access
- Multi-line field handling
- Custom fields in nested entries

### Line Ending Conversion (1 test)
- DOS (CRLF) to Unix (LF) conversion

### Error Handling (3 tests)
- Nonexistent entry error
- Nonexistent field error
- Invalid path format (multiple pipes)

### Special Characters (2 tests)
- Special characters in passwords
- Special characters in usernames

**Total:** 17 integration tests

## Running Tests

### With Docker (Recommended for CI)
```bash
# Build test image
docker build -f Dockerfile.test -t summon-keepass:test .

# Run all tests
docker run --rm summon-keepass:test

# Run specific test
docker run --rm summon-keepass:test cargo test test_version_flag_short

# Shell access for debugging
docker run --rm -it summon-keepass:test /bin/bash
```

### With Rust Toolchain
```bash
# Set HOME to test fixtures directory
export HOME=$(pwd)/tests/fixtures

# Run all tests
cargo test

# Run specific test
cargo test test_version_flag_short

# Run with output
cargo test -- --nocapture
```

## Adding New Tests

1. **Add test data to database:**
   - Modify `create_test_db.py` if needed to add new entries
   - Regenerate database: `python3 create_test_db.py`

2. **Add test function:**
   - Add test function to `integration_test.rs`
   - Use helper functions from `common` module
   - Follow existing naming conventions

3. **Verify tests pass:**
   ```bash
   cargo test
   ```

## Test Utilities

The `common/mod.rs` module provides:

- `setup_test_env()` - Creates temporary HOME with test config
- `run_summon_keepass(args)` - Executes binary and returns (stdout, stderr, exit_code)
- `assert_success()` - Asserts command succeeded (exit code 0)
- `assert_failure()` - Asserts command failed with expected code

## Continuous Integration

Tests run automatically in GitHub Actions on every push and pull request. The workflow:

1. Builds Docker test image with layer caching
2. Runs all integration tests in container
3. Fails the build if any test fails
4. Only proceeds to build job if tests pass

## Notes

- Tests run serially (`--test-threads=1`) to avoid HOME directory conflicts
- Test database contains only non-sensitive test data
- Database password is documented and safe for public repository
- Docker provides consistent test environment across platforms
