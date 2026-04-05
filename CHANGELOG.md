# Changelog

All notable changes to this project will be documented in this file.

## [0.1.1] - Unreleased

### Added
- HDLC comprehensive tests: 45 integration tests covering CRC, byte stuffing, frame boundaries, noise resilience, parser recovery
- Property-based tests using `proptest` for HDLC (10 tests) and AXDR (10 tests)
- Fuzz target for HDLC parser (no-panic guarantee)
- Bug fix: U-frame encoding in `frame.rs` — fixed bit collision between modifier and poll_final bit
- Test framework: `fuzz/` directory with `cargo-fuzz` integration
- Documentation: Updated README with test statistics, quick start examples, and feature overview

### Fixed
- U-frame `from_control`: Changed from 3-bit modifier to 2-bit modifier (bits 3:2) to match standard HDLC and avoid bit collision
- Workspace Cargo.toml: Moved `proptest` from `[workspace.dev-dependencies]` to `[workspace.dependencies]`

### Changed
- Updated test count from 1,026 to 531 lib tests
