# TODO — balinese-calendar

## v0.1.2 (this release)
- [x] Add `DayBoundary` enum (`Midnight`, `FixedSunrise`, `Astronomical` stub)
- [x] `today_with_boundary()` — fix 00:00–06:00 ambiguous window
- [x] Bump to v0.1.2, add `astronomical` feature flag (stubbed)

### Architecture
- [x] Feature-gate `DayBoundary::Astronomical` variant behind `#[cfg(feature = "astronomical")]`
      so it only exists when the feature is enabled (compile-time safety over runtime error)
- [x] Add a test that fails when `NAMPIH_YEARS` table no longer covers the current Saka year
      (forcing function for annual maintenance) — replaced with algorithmic Metonic cycle detection
- [x] Cross-validate sasih epoch against kalenderbali.org for 12+ dates spanning a full year;
      fix `EPOCH_SASIH_OFFSET` and Saka year boundary logic in `src/sasih.rs`
      — rewrote sasih as walk-forward algorithm from peradnya pivot points
- [x] Validation Hari Raya Nyepi (March 19, 2026 = Tahun Baru Saka 1948) — verified
- [x] Fix PAWUKON_EPOCH_JDN (was 2232407, corrected to 2440976 from peradnya pivots)
- [x] Fix Pancawara, Caturwara, Astawara computations to match peradnya reference

### Code Quality
- [ ] Refactor `PancaSuda`, `Pararasan`, `Rakam` to take pre-computed `&Pancawara`/`&Saptawara`
      instead of calling `from_jdn()` internally (follow the `Dasawara::from_wewaran` pattern)
- [ ] Flatten `SasihDayInfo::Ngunaratri` from `Box<SasihDayInfo>` to non-recursive `TithiPhase`;
      make `SasihDayInfo` `Copy` to match the stated design goal
- [x] Add `HariBhataraSri` (Buda Wage) detection in `Rahinan::detect()` — variant exists but is never matched
- [x] Add `impl fmt::Display for SasihDayInfo` and use it in `to_balinese_string()` to remove inline tithi formatting

### Tests
- [x] Fix silent Saraswati test (`tests/integration_test.rs`) — converted to unconditional `assert_eq!`
- [ ] Add Ngunaratri edge case tests: verify specific ngunaratri dates produce `SasihDayInfo::Ngunaratri`
      with correct primary/secondary tithis, plus a cycle integrity check (every 63 days from epoch)
- [ ] Add Astawara and Sangawara spot-check tests for known dates (these have the most complex
      adjustment logic and currently zero test coverage)
- [x] Resolve contradictory sasih assertions for March 6, 2026 — both tests now agree on Kasanga

### Performance
- [ ] Compute `pawukon_day()` once in `from_jdn_unchecked` and pass to all subsystem constructors
      (currently recomputed ~15 times per construction; combine with Code Quality refactor above)
- [ ] Validate dates via `NaiveDate::from_ymd_opt` in `gregorian_to_jdn()` — current check
      accepts impossible dates like Feb 30 (chrono is already a dependency)
- [x] Remove unused `LUNATION_DAYS` and `SASIH_EPOCH_JDN` constants from `src/utils.rs`

## v0.1.3 (next)
- [ ] Implement `DayBoundary::Astronomical` using the `sunrise` crate
      - Bali centroid default: lat -8.3405, lon 115.0920
      - Test: known sunrise times from BMKG (Badan Meteorologi, Klimatologi, dan Geofisika)
- [ ] Expose `DayBoundary` in WASM bindings (if added)

## Backlog
- [ ] `BalineseDate::now_with_boundary()` — return both date AND time-of-day position
      relative to the active Balinese day (useful for UI countdowns to next day boundary)
- [ ] PHDI Nampih Sasih: automate annual update via scraping or structured data file
- [ ] Locale strings: Balinese script (Aksara Bali) output alongside Latin
- [ ] `serde` feature flag for JSON serialisation of all public types
- [ ] WASM target (`wasm32-unknown-unknown`) for use in Dedauh frontend directly
- [ ] Consider `const BALI_CENTROID: (f64, f64) = (-8.3405, 115.0920)` alongside
      `BALI_UTC_OFFSET_HOURS` for Astronomical default to avoid hardcoded coords
