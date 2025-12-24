# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`summon-keepass` is a Rust-based provider for [summon](https://cyberark.github.io/summon/) that enables reading secrets from KeePass (.kdbx) database files. It's a single-binary CLI tool that integrates with summon's secrets management workflow.

## Build and Development Commands

### Building
```bash
# Development build
cargo build --verbose

# Release build (optimized)
cargo build --verbose --release

# Release build with stripped symbols (production-ready)
cargo build --verbose --release
strip target/release/summon-keepass
```

### Testing
Tests are currently minimal (see Todo in README). The tests directory exists but is empty.

### Running
```bash
# Run the binary directly (requires configuration)
./target/release/summon-keepass "path/to/entry|field"

# Or from debug build
./target/debug/summon-keepass "path/to/entry|field"
```

## Architecture

### Single-File Application
The entire application logic is contained in `src/main.rs` (~65 lines). This is a straightforward CLI tool with no modular architecture.

### Core Flow (src/main.rs)
1. **Configuration Loading** (lines 28-33): Reads `~/.summon-keepass.ini` for KeePass database path and password
2. **Database Access** (lines 35-36): Opens the KeePass database using the `keepass` crate
3. **Secret Path Parsing** (lines 38-53): Parses the secret path format `[group/subgroup/]entry[|field]`
   - Default field is "Password" if not specified
   - Uses `|` as field separator
4. **Entry Retrieval** (lines 54-59): Navigates the KeePass database tree structure using `/` separators
5. **Output** (line 56): Writes the field value to stdout with DOS/Windows line endings converted to Unix (using `dos2unix`)

### Dependencies (Cargo.toml)
- `keepass` (0.4.9): KeePass database parsing
- `rust-ini` (0.9.4): Configuration file parsing
- `newline-converter` (0.2.0): DOS to Unix line ending conversion (addresses line ending issues from keepass library)

### Secret Path Format
The tool uses a specific syntax for identifying secrets:
- **Entry path**: `group/subgroup/entry` (uses `/` as separator)
- **Field specification**: `entry|field` (uses `|` as separator)
- **Examples**:
  - `aws/iam/user/robot/access_key_id` - Password field of nested entry
  - `account|UserName` - UserName field of root-level entry
  - `ssh/some server|priv_key` - Custom field from grouped entry

### Configuration
Requires `~/.summon-keepass.ini` with format:
```ini
[keepass_db]
path=/path/to/your/keepass_database_file.kdbx
pass=password to your keepass database
```

### Exit Codes
- `0`: Success
- `1`: No variable provided OR entry could not be retrieved
- `2`: Invalid secret path format

## Release Process

The GitHub Actions workflow (`.github/workflows/rust.yml`) automatically:
1. Builds the release binary
2. Strips symbols for smaller binary size
3. Packages as `summon-keepass-linux-amd64.tar.gz`
4. Creates GitHub releases for version tags:
   - `v1.2.3` format creates a full release
   - `v1.2.3-beta.1` format creates a pre-release

## Known Issues and Todo Items

From README.md:
- Tests need to be added
- KeePass DB password should be read from environment variable (preferred over config file)
- Key file authentication not yet supported
- Error handling for incorrect config/KeePass DB file paths needs improvement
