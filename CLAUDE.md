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

This project uses [cargo-release](https://github.com/crate-ci/cargo-release) to automate version management and releases.

### GitHub Actions Workflows

**`.github/workflows/release.yml`** - Automated Release Creation (Recommended)
- **Trigger:** Manual via GitHub UI ("Run workflow" button)
- **Purpose:** Complete end-to-end release process
- **Steps:**
  1. Runs cargo-release (version bump, CHANGELOG update, tag creation)
  2. Builds release binary with optimizations
  3. Strips symbols for smaller binary size
  4. Packages as `summon-keepass-linux-amd64.tar.gz`
  5. Pushes commit and tag to repository
  6. Extracts changelog from `CHANGELOG.md` for the release version
  7. Creates GitHub Release with binary artifact
  8. Automatically determines release vs pre-release based on version format

**`.github/workflows/rust.yml`** - Continuous Integration + Fallback
- **Trigger:** Automatic on every push and pull request
- **Purpose:**
  1. **CI Testing:** Runs integration tests in Docker on all pushes/PRs
  2. **Fallback Release:** Creates releases for manually pushed tags
- **When it creates releases:** Only when a tag matching `v*` is manually pushed via git
- **Note:** The release.yml workflow is preferred for creating releases

### Creating a Release

There are two ways to create a release:

#### Method 1: GitHub UI (Recommended)

1. **Ensure all changes are committed and documented in CHANGELOG.md under `[Unreleased]`**

2. **Navigate to Actions → Create Release:**
   - Go to https://github.com/desolat/summon-keepass/actions/workflows/release.yml
   - Click "Run workflow"
   - **Select release type from dropdown:**
     - **release** - Finalize pre-release (0.3.0-rc.2 → 0.3.0) or patch bump (0.3.0 → 0.3.1)
     - **minor** - New features (0.3.0 → 0.4.0)
     - **major** - Breaking changes (0.3.0 → 1.0.0)
     - **alpha** - Create alpha pre-release (0.3.0 → 0.3.0-alpha.1)
     - **beta** - Create beta pre-release (0.3.0 → 0.3.0-beta.1 or 0.3.0-alpha.1 → 0.3.0-beta.1)
     - **rc** - Create release candidate (0.3.0 → 0.3.0-rc.1 or 0.3.0-beta.1 → 0.3.0-rc.1)
     - **custom** - Specify exact version in the custom_version field
   - Click "Run workflow"

3. **The workflow will automatically:**
   - Run cargo-release with dry-run first (for validation)
   - Update version in `Cargo.toml` and `CHANGELOG.md`
   - Create commit: "chore: release X.Y.Z"
   - Create tag `vX.Y.Z`
   - Build the release binary
   - Package it as `summon-keepass-linux-amd64.tar.gz`
   - Push commit and tag to GitHub
   - Create GitHub Release with changelog and binary artifact
   - Mark as pre-release if version contains `-alpha`, `-beta`, or `-rc`

4. **Done!** The release is published at: https://github.com/desolat/summon-keepass/releases

**Typical Pre-release to Stable Workflow:**
```
0.3.0 (current stable)
  ↓ Select: "rc"
0.3.0-rc.1 (test it)
  ↓ Select: "rc" (increments to rc.2)
0.3.0-rc.2 (more testing)
  ↓ Select: "release" (finalizes)
0.3.0 (final stable release)
  ↓ Select: "minor"
0.4.0 (next version)
```

#### Method 2: Local cargo-release

**Prerequisites:**
```bash
cargo install cargo-release
```

**Release Process:**

1. **Ensure all changes are committed and documented in CHANGELOG.md under `[Unreleased]`**

2. **Perform a dry-run to preview changes:**
   ```bash
   # For patch release (0.3.0 -> 0.3.1)
   cargo release patch --dry-run

   # For minor release (0.3.0 -> 0.4.0)
   cargo release minor --dry-run

   # For major release (0.3.0 -> 1.0.0)
   cargo release major --dry-run

   # For specific version
   cargo release 0.4.0 --dry-run

   # For alpha pre-release (0.3.0 -> 0.3.1-alpha.1)
   cargo release alpha --dry-run

   # For beta pre-release (0.3.1-alpha.1 -> 0.3.1-beta.1)
   cargo release beta --dry-run

   # For release candidate (0.3.1-beta.1 -> 0.3.1-rc.1)
   cargo release rc --dry-run
   ```

3. **Review the dry-run output. cargo-release will:**
   - Update version in `Cargo.toml`
   - Update `CHANGELOG.md` (move Unreleased to versioned section with date)
   - Update changelog comparison links
   - Create a commit with message "chore: release X.Y.Z"
   - Create a git tag `vX.Y.Z`

4. **Execute the release (without --dry-run):**
   ```bash
   cargo release minor
   ```

5. **Review the commit and tag that were created:**
   ```bash
   git log -1
   git tag -l
   ```

6. **Push the commit and tag to trigger CI:**
   ```bash
   git push
   git push --tags
   ```

7. **GitHub Actions will automatically build, test, and create the GitHub release with changelog**

### Manual Release (Not Recommended)

If you need to create a release manually without cargo-release:

1. Update version in `Cargo.toml`
2. Move unreleased changes in `CHANGELOG.md` to new version header
3. Update changelog comparison links
4. Commit: `git commit -m "chore: release X.Y.Z"`
5. Tag: `git tag vX.Y.Z`
6. Push: `git push && git push origin vX.Y.Z`

### cargo-release Configuration Notes

**Important configuration details in Cargo.toml:**

1. **tag-prefix must be empty string:**
   - cargo-release's default tag-name template is `"{{prefix}}v{{version}}"`
   - Setting `tag-prefix = "v"` creates tags like `vv0.3.1-rc.1` (double "vv")
   - Correct configuration: `tag-prefix = ""` produces tags like `v0.3.1-rc.1`

2. **pre-release-replacements require prerelease = true:**
   - By default, `pre-release-replacements` only run for standard releases
   - For replacements to run during alpha/beta/rc releases, add `prerelease = true`
   - Example: `{file="...", search="...", replace="...", prerelease=true}`
   - Without this, CHANGELOG won't update during pre-releases

3. **Pre-release levels work on the current version:**
   - `cargo release alpha` from `0.3.0` → `0.3.0-alpha.1` (NOT 0.3.1-alpha.1)
   - `cargo release beta` from `0.3.0-alpha.1` → `0.3.0-beta.1` (promotes alpha to beta)
   - `cargo release rc` from `0.3.0-beta.1` → `0.3.0-rc.1` (promotes beta to rc)
   - Subsequent runs increment: `0.3.0-rc.1` → `0.3.0-rc.2`
   - To release a pre-release of a NEW version, first bump version in Cargo.toml manually, then run pre-release

4. **Finalizing pre-releases uses patch (or "release"):**
   - `cargo release patch` from `0.3.0-rc.2` → `0.3.0` (removes pre-release suffix)
   - This is confusing because "patch" doesn't bump the patch version, it finalizes the current version
   - The GitHub workflow includes a "release" option which is clearer (maps to patch internally)
   - To bump to next version after pre-release, use minor/major: `0.3.0-rc.2` → `cargo release minor` → `0.4.0`

### cargo-release Configuration

The release behavior is configured in `Cargo.toml` under `[package.metadata.release]`:
- **No automatic push**: You must manually push after reviewing
- **No crates.io publish**: This is a binary-only project
- **Automatic file updates**: CHANGELOG updated with version and date
- **Consistent commit messages**: "chore: release X.Y.Z"

## Known Issues and Todo Items

From README.md:
- Tests need to be added
- KeePass DB password should be read from environment variable (preferred over config file)
- Key file authentication not yet supported
- Error handling for incorrect config/KeePass DB file paths needs improvement
