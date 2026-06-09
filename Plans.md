# Plans — balinese-calendar

## v0.2.2 — Wariga Lookup Completeness

Patch release: Complete two placeholder-algorithm Wariga lookup tables from OCR sources. No API changes, pure data extraction.

| Task | Description | DoD | Depends | Status |
|------|-------------|-----|---------|--------|
| 2.1 | **Dauh Sukaranti** — Extract 12×5 lookup table from Wariga Sundari Bungkah | fixture JSON created, tests passing, impl replaces placeholder | - | cc:done |
| 2.2 | **Tenung Patemuan Adan** — Extract letter→urip mapping (18 consonant groups) from Lontar Joyoboyo | fixture JSON created, tests passing, impl replaces placeholder | - | cc:done |
| 2.3 | Verify tests pass & prepare release | all tests passing, CHANGELOG.md updated with v0.2.2 notes | 2.1, 2.2 | cc:done |

### Release checklist (trigger for v0.2.2)
- [x] Both 2.1 and 2.2 complete and tested
- [x] `cargo semver-checks` passes
- [x] Fixture tests cover all 12 urip values (Dauh) and representative letters (Tenung)
- [x] CHANGELOG.md updated
- [x] Git tag `v0.2.2` created

---

## v0.3.0 — Dewasa Ayu Phase 1 (Validation Fixture & Scoring Scaffold)

Foundational implementation: extract expert-ground-truth dataset from Candana et al. (2021), scaffold the DewasaAyu trait, and validate Sugeno inference mechanism against 11 true-positive predictions.

| Task | Description | DoD | Depends | Status |
|------|-------------|-----|---------|--------|
| 3.1 | **Candana fixture extraction** — Create `tests/fixtures/candana_2021_dewasa.json` with 79 prediction dates (16 expert, 13 Sugeno, 27 Tsukamoto, 21 Mamdani) + wewaran cross-reference for all 16 expert dates | fixture JSON created, all 16 expert dates have wewaran fields populated, saptawara/pancawara distributions documented | - | cc:done [c520eae0] |
| 3.2 | **DewasaAyu trait & scaffold** — New trait with `score(&self) -> f64` (0.0–1.0) and `is_dewasa_ayu(threshold: f64) -> bool` methods; feature-gate behind `#[cfg(feature = "dewasa-ayu")]`; placeholder scoring (returns constant for now) | trait compiles, methods are callable, feature gate works with `cargo build --features dewasa-ayu`, placeholder implementation returns valid scores in [0.0, 1.0] | 3.1 | cc:done [c520eae0] |
| 3.3 | **Phase 1 integration tests** — Tests verifying fixture loading, scoring reproducibility, and Sugeno TP match count (target: 11/16 expert dates produce scores ≥ 0.7) | `tests/dewasa_ayu_test.rs` created, fixture loads without error, can reproduce 11 expert matches (allow ±1 for numerical stability), test passes with `cargo test --features dewasa-ayu` | 3.2 | cc:done [c520eae0] |

### Phase 1 Scope Notes
- **Placeholder scoring:** Return a hardcoded constant (e.g., 0.5) or naive average of wewaran + wuku urip. This is NOT the Sugeno engine yet — Phase 2 will replace it.
- **Fixture data source:** Candana et al. (2021) JIK 6(2), Table 3 (79 dates). Wewaran values must be recomputed using this library's actual algorithms to match balinese-calendar (not just copied from the paper — the paper's wewaran had errors).
- **Expert ground truth:** 16 dates scored 70–80 by Wariga expert; task 3.3 verifies we can load these and run inference (even with placeholder scoring, we establish the test infrastructure).
- **No algorithm change:** Phase 1 is pure fixture extraction + trait scaffolding. The Sugeno engine itself is Phase 2.

### Release checklist (trigger for v0.3.0-alpha)
- [ ] Task 3.1 complete: candana_2021_dewasa.json created and validated
- [ ] Task 3.2 complete: DewasaAyu trait compiles and feature-gates correctly
- [ ] Task 3.3 complete: integration tests pass and demonstrate fixture infrastructure
- [ ] CHANGELOG.md updated with v0.3.0-alpha notes
- [ ] `cargo test --all-features` passes (including new dewasa_ayu tests)
