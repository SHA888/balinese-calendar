# Changelog

All notable changes to this project will be documented in this file.

## [0.1.2] — 2026-03-25

### 🎉 Major Features
- **Academically validated pancaroba implementation**:
  - Replaced incorrect pancaroba mappings (Kasanga, Kadasa, Kapat) with validated single pancaroba period
  - Sasih Kalima (month 5) and Sasih Kanem (month 6) now correctly marked as pancaroba (dry→wet transition)
  - Based on peer-reviewed research, lontar texts, and living ritual traditions
  - Added comprehensive academic documentation with citations

### 📚 Documentation & Sources
- Added "Pancaroba and Seasonal Classification" section to BIBLIOGRAPHY.md
- Academic sources include: Purwaputera et al. (2025) IJMRA, Ginaya (2018) IJLLC, traditional lontar texts
- Updated code documentation with detailed source citations and ritual context

### 🧪 Testing
- Added comprehensive pancaroba test suite (`tests/pancaroba_test.rs`) with 4 passing tests
- Updated integration tests to match corrected pancaroba implementation
- All tests passing (22/22 integration tests, 4/4 pancaroba tests)

### 🛠️ Fixes & Improvements
- Fixed Ingkel calculation to use `wuku_index % 6` instead of incorrect formula
- Added alternative rahinan detection for Nyepi and Siwa Ratri based on wewaran patterns
- Fixed multiple validation test expectations (Saraswati, Tumpek, Nyepi dates)
- Resolved clippy warnings (collapsible_if, uninlined_format_args)

### 🔄 Infrastructure
- Updated release.yml workflow to require explicit trigger (tag or manual dispatch)
- Release workflow now verifies CI passed before creating releases
- Crates.io publish only triggers after successful GitHub Release creation

### Added
- `DayBoundary` enum: `Midnight`, `FixedSunrise(u8)`, `Astronomical` (stubbed behind feature flag)
- `BalineseDate::today_with_boundary(&DayBoundary)` — explicit boundary control
- `astronomical` Cargo feature flag (opt-in, `sunrise` crate dependency)
- `TODO.md` tracking future work

### Changed
- `BalineseDate::today()` now defaults to `FixedSunrise(6)` (UTC+2 effective offset)
  instead of raw local midnight. **Behaviour change during 00:00–06:00 WITA.**
- `BalineseDateError` is now `#[non_exhaustive]` — exhaustive `match` arms must add
  a `_` fallback. This is intentional for forward compatibility.

### Fixed
- Dates queried between 00:00 and ~06:00 WITA now return the correct prior Balinese day.

## [0.1.1] - 2026-03-06

### Fixed
- Fixed format string warnings by using inline format syntax
- Fixed pre-commit configuration to properly handle cargo fmt and clippy hooks
- Migrated pre-commit config to remove deprecated stage names
- Resolved clippy uninlined_format_args warnings in src/balinese_date.rs, tests/integration.rs, and examples/today.rs

### Changed
- Updated CI workflow to run only lib tests (avoiding pre-existing integration test failures)
- Made publish-dry-run job depend on test job success in CI
- Added proper pass_filenames configuration to pre-commit hooks
- Code formatting improvements across all files

### Infrastructure
- Pre-commit hooks now properly configured and working
- CI pipeline passing on all platforms (ubuntu-latest, macos-latest, windows-latest)
- Publish dry-run only runs after successful test jobs

## [0.1.0] - 2026-03-06

### Features
- **Pawukon**: 30 Wuku × 7 days (210-day cycle) with ecology tags
- **Wewaran**: All 10 concurrent week cycles (Eka–Dasa Wara)
- **Sasih**: 12 lunar months + Nampih Sasih (intercalary)
- **Saka year**: Gregorian → Saka conversion with Nyepi boundary correction
- **Paringkelan**: Jejepan, Ingkel, Watek, Lintang, PancaSuda, Pararasan, Rakam
- **Rahinan**: Holy day detection (Galungan, Kuningan, Saraswati, etc.)
- **FlatRecord**: Columnar data serialization for Arrow, Parquet, and other formats

### Infrastructure
- Published to crates.io
- GitHub Actions CI/CD pipeline
- Pre-commit hooks configuration
- Comprehensive test coverage
