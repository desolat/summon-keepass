# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/desolat/summon-keepass/compare/v0.3.1-rc.1...HEAD
[0.3.1-rc.1]: https://github.com/desolat/summon-keepass/compare/v0.3.0-beta.1...v0.3.1-rc.1
[0.3.0-beta.1]: https://github.com/desolat/summon-keepass/compare/v0.2.0...v0.3.0-beta.1
[0.2.0]: https://github.com/desolat/summon-keepass/releases/tag/v0.2.0
