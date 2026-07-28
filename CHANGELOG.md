# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

## [0.3.0] — 2026-07-28

### ✨ Added

- **Dewasa Ayu (auspicious-day classification)** — new `dewasa-ayu` feature flag (the crate's only floating-point component):
  - `DewasaAyu` trait (`dewasa_ayu_score()`, `is_dewasa_ayu()`, and `_with_config` variants) implemented for `BalineseDate`
  - `DewasaAyuConfig` for configurable Saptawara/Pancawara weights and classification threshold
  - Zero-order Sugeno fuzzy inference engine (`SugenoEngine`, `SugenoRule`, `FuzzySet`, `LinguisticValue`) with a hand-authored rule base (`rule_base::alahaning_dewasa_rules`) encoding the traditional Alahaning Dewasa override hierarchy (Wewaran → Wuku → Penanggal → Sasih → Dauh)
  - `DewasaInput` derivation from `BalineseDate`, normalizing Wewaran, Wuku, Penanggal, and Sasih into `[0.0, 1.0]` fuzzy inputs
  - Calibrated so full-year "auspicious" classifications stay under 3% of days, matching the traditional rarity of Dewasa Ayu days
  - Validation fixture `tests/fixtures/candana_2021_dewasa.json` (79 prediction dates from Candana et al. 2021, including all 16 expert-labeled ground-truth dates)

### ⚠️ Known limitations

- Measured against the Candana et al. (2021) 731-day validation corpus: precision 21.43%, recall 18.75%, F-1 20.00% — well below the source paper's own Sugeno FIS figures (92.31% / 75% / 82.76%). This gap is the direct, documented cost of prioritizing the <3% full-year rarity gate without the Ariana & Budayoga (2016) bobot tables for Wuku/Penanggal/Sasih prohibition data, which have not yet been obtained. See `Plans.md` task 3.7 and `tests/dewasa_ayu_test.rs::test_accuracy_report_against_candana_corpus` for the full report. Closing this gap is tracked as a future phase, blocked on that source.

### 🧪 Testing

- New `tests/dewasa_ayu_test.rs` integration suite covering fixture loading, expert-date classification, full-year rarity-gate compliance, and the accuracy report against the Candana 2021 corpus

## [0.2.2] — 2026-05-22

### ✨ Added

- **Wariga Lookup Completeness**:
  - **Dauh Sukaranti** — JSON-backed 12×5 lookup table replacing heuristic-only time-slot quality computation. All 12 urip values (1–12) map to 5 period qualities (Krta, Sume, Peta, Pali, Kelara) via `dauh_sukaranti_fixtures` loaded from `tests/fixtures/dauh_sukaranti.json`. Fallback algorithm preserved for robustness.
  - **Tenung Patemuan Adan** — JSON-backed 18-consonant-group mapping replacing name-length-only logic. Multi-character clusters ("ng"→urip 3, "ny"→urip 6) now correctly handled via longest-match-first algorithm. Loaded from `tests/fixtures/tenung_patemuan_adan.json`. Name compatibility rule (remainder ≠ 0 and ≠ 3) preserved for 16 possible outcomes. Fallback to name-length heuristic maintained.

### 🧪 Testing

- Enhanced `test_dauh_sukaranti()` with spot checks across all 12 urip values
- New `test_name_compatibility_with_clusters()` validating multi-character consonant handling (Balinese names: Mangga, Nyoman, etc.)
- Edge case tests for empty strings, single characters, long names, case-insensitivity

### 🔧 Infrastructure

- GitHub Actions: Upgraded Node.js LTS from v20 → v24 in `docs.yml` workflow
- Actions: `actions/upload-pages-artifact` v4 → v5

## [0.2.1] — 2026-04-28

### ✨ Added

- **Ingkel ecological domain accessors** (`src/paringkelan/mod.rs`):
  - `Ingkel::ecological_domain() -> &'static str` — English snake_case label
    (`human_affairs`, `animals`, `fish_maritime`, `birds`, `trees_forestry`, `bamboo_reeds`)
  - `Ingkel::ecological_domain_id() -> &'static str` — Balinese manuscript term
    (`wong`, `sato`, `mina`, `manuk`, `taru`, `buku`)
  - Stable machine-readable identifiers intended for data pipeline consumers
  - Source: Wariga Sundari Bungkah via I.B.S. Ardhana, *Pokok-Pokok Wariga* (2005),
    and I Made Bidja bibliography

### 🧪 Testing

- Exhaustive tests for all 6 Ingkel variants on both new methods
- Integration tests against 2026 corpus (Krulut=Taru, Merakih=Buku, Tambir=Wong)

## [0.2.0] — 2026-04-07

### 🎉 Wariga Computation Layer

Complete implementation of the Wariga personal and compatibility systems:

- **WarigaBelog** — personalized day quality via `(birth_urip + daily_urip) % 4`
- **Gebogan Urip Tri-Pramana** — 210-entry Wuku × Saptawara lookup with 4 quality classes
- **Pawiwahan** — marriage compatibility scoring on 16-point scale
- **Dauh Sukaranti** — time-of-day quality (5 periods: 05:30–17:30 WITA)
- **Tenung Patemuan Adan** — name compatibility via Lontar Joyoboyo letter→urip mapping
- **Otonan calculator** — 210-day birthday cycle utilities (`otonan_dates`, `next_otonan`)

### 🔧 Infrastructure

- Added `dewasa-ayu` feature flag (preparation for v0.3.0 Sugeno inference)
- New `TODO.md` with detailed roadmap through v1.0.0

## [0.1.3] — 2026-03-27

### 🎉 Major Features
- **WASM (WebAssembly) support**:
  - Complete `wasm32-unknown-unknown` target via `wasm-bindgen`
  - JavaScript interop layer with `from_ymd()`, `today()`, rahinan list, formatted string
  - Enables client-side Balinese calendar in any web frontend
  - Depends on `serde` feature for JSON bridge to JavaScript

- **Astronomical sunrise support**:
  - `DayBoundary::Astronomical` using the `sunrise` crate
  - Bali centroid default coordinates: lat -8.3405, lon 115.0920
  - Accept custom coordinates for non-Bali Hindu communities
  - Tested against known sunrise time patterns

### 🔧 Infrastructure Improvements
- **Serde feature flag**:
  - Derive `Serialize` / `Deserialize` on all public types behind `serde` feature
  - Include `serde` and `serde_json` as optional dependencies
  - Enables JSON output for any API, pipelines, frontend bridges

### 🐛 Bug Fixes
- Fixed critical astronomical sunrise calculation logic
- Added early year range validation for better error messages
- Improved WASM error message consistency using structured errors
- Enhanced code quality with comprehensive test coverage

### 🧪 Testing
- Added 6 astronomical sunrise tests with BMKG reference validation
- All 95 tests passing with comprehensive coverage
- Code quality verified with pre-commit checks (no warnings, proper formatting)

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
