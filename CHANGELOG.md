# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0] - 2025-12-27

### Added
- Environment variable configuration support (`SUMMON_KEEPASS_DB_PATH` and `SUMMON_KEEPASS_DB_PASS`)
- Configuration precedence: environment variables override `~/.summon-keepass.ini`
- Support for mixed configuration sources (e.g., path from env var, password from config file)
- Comprehensive error messages showing all checked configuration sources and helpful setup instructions
- Help flag (`-h` / `--help`) with detailed usage, configuration options, examples, and exit codes

### Changed
- Updated to Rust 2024 edition using cargo migration tooling
- Updated Dockerfile.test to use Rust 1.85 for compatibility with latest dependencies
- Configuration loading now tries environment variables before falling back to INI file

### Fixed
- No longer panics when HOME environment variable is not set
- Graceful handling of missing or invalid configuration files with descriptive error messages
- install.sh no longer leaves behind JSON files when checking release versions

## [0.3.0] - 2025-12-26

### Added
- Version flag support: `-V` and `--version` to display current version
- Comprehensive integration test suite with 17 tests covering all functionality
- Docker-based testing infrastructure with multi-stage builds
- GitHub Actions CI pipeline with containerized tests running before builds
- Rust 2021 edition support
- cargo-release configuration for automated version management and releases
- CI validation to ensure tag versions match Cargo.toml
- GitHub workflow for creating releases directly from GitHub UI
- Support for retrieving custom fields from KeePass entries
- Multi-line value support with DOS to Unix line ending conversion

### Fixed
- Nonexistent field handling now gracefully exits with code 1 instead of panicking
- Proper error messages when requesting fields that don't exist in KeePass entries
- Various bug fixes and stability improvements

### Changed
- Tests now run automatically in CI before building releases
- Updated project to Rust 2021 edition for better ergonomics
- Release process now uses cargo-release for consistency and automation

## [0.2.0] - Earlier release

### Added
- Basic KeePass database reading functionality
- Entry retrieval by path
- Password field access by default
- Support for nested groups using `/` separator
- Custom field access using `|` separator

[Unreleased]: https://github.com/desolat/summon-keepass/compare/v0.4.0...HEAD
[0.4.0]: https://github.com/desolat/summon-keepass/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/desolat/summon-keepass/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/desolat/summon-keepass/releases/tag/v0.2.0
