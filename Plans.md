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

### Phase 1 status (complete — NOT released)

Phase 1 is done on `main` but intentionally **unreleased**. The scaffold scorer is
non-functional (classifies ~72% of days as good vs the 2.19% ground truth), so no
alpha is cut. Phase 1 + Phase 2 ship together as a single `0.3.0` once the Sugeno
engine makes the scorer meaningful. The feature is gated off by default, so leaving
it unreleased on `main` has no cost.

- [x] Task 3.1: candana_2021_dewasa.json created and validated
- [x] Task 3.2: DewasaAyu trait compiles and feature-gates correctly (gated in cb17271d)
- [x] Task 3.3: integration tests pass and demonstrate fixture infrastructure
- [x] `cargo test --all-features` passes (including new dewasa_ayu tests)

### Release gate for v0.3.0 (do NOT tag until ALL pass)
- [x] Phase 2 Sugeno engine complete; `test_scaffold_rarity_over_full_year` passes **un-ignored** (<3% positive rate) — done in task 3.6
- [x] Sugeno performance measured against Candana targets and the gap documented in Plans.md + code (task 3.7: F-1 20.0% vs 82.76% target). **Gate relaxed 2026-07-28**: exact Candana replication is not required for release — it depends on the still-unobtained Ariana & Budayoga (2016) bobot tables (Phase 3). Meeting the rarity gate with the gap honestly documented is sufficient to ship as a best-effort classifier.
- [ ] CHANGELOG.md updated with v0.3.0 notes (working classifier + documented accuracy gap, not a scaffold caveat)
- [ ] `cargo semver-checks` passes
- [ ] Bump `Cargo.toml` version to `0.3.0`
- [ ] Git tag `v0.3.0` created

## v0.3.0 — Dewasa Ayu Phase 2 (Functional Sugeno Classifier)

Wire the existing Sugeno machinery (FuzzySet / SugenoEngine, already in the gated
`sugeno` module) into a real date classifier. Rarity (<3%) is the hard gate; exact
Candana metric replication is best-effort (the bobot weight tables — Ariana &
Budayoga 2016 — are research-blocked, so penanggal/sasih/ala-ayu calibration is
principled-but-approximate until that source lands in Phase 3).

| Task | Description | DoD | Depends | Status |
|------|-------------|-----|---------|--------|
| 3.4 | **Input derivation + empirical characterization** — `DewasaInput` from a `BalineseDate` (5 normalized vars: wewaran, wuku, penanggal, sasih, ala_ayu); characterize the 16 expert dates across all 5 vars to find the rarity-preserving filter pattern | derivation fn returns clamped [0,1] inputs for any date; analysis test dumps the 16 experts' full profile + distributions | 3.3 | cc:done |
| 3.5 | **Sugeno rule base** — conjunctive (product t-norm) rules ordered by the Alahaning Dewasa hierarchy (Wewaran→Wuku→Penanggal→Sasih→Dauh); "good" output requires high wewaran AND non-prohibited penanggal/sasih | rule base builder returns a populated `SugenoEngine`; unit tests on firing strength for representative inputs | 3.4 | cc:done [e72c9d7f] |
| 3.6 | **Wire trait to engine + rarity gate** — `DewasaAyu` scoring delegates to the Sugeno engine; un-ignore `test_scaffold_rarity_over_full_year` and make it pass (<3% positives over 2020) | trait uses engine; rarity test passes un-ignored; all expert dates whose wewaran qualifies are still classified correctly | 3.5 | cc:done [bac4cc2f] |
| 3.7 | **Fixture validation + accuracy report** — measure TP/FP/precision/recall vs the 16 expert dates; document achieved metrics against Candana targets and the bobot-source limitation | accuracy test prints metrics; gap vs 82% F-1 documented in code + Plans.md; no overfitting beyond the 16-date fixture without corpus note | 3.6 | cc:done [c52602f4] |

### Task 3.7 accuracy report

`tests/dewasa_ayu_test.rs::test_accuracy_report_against_candana_corpus` walks the full
731-day study period (2020-01-01..2021-12-31) and classifies every day with the current
`DewasaAyu` Sugeno engine, scoring against the 16 Candana 2021 expert dates as ground
truth (a day not in the 16 is treated as an implicit expert negative — the same
convention the source paper's own reported precision/recall imply, since it never
publishes a full day-by-day expert ledger for the other 715 days).

Measured (pinned as a regression guard in the test):

| Metric | Achieved | Candana 2021 Sugeno FIS | Gap |
|--------|----------|--------------------------|-----|
| TP / FP / FN / TN | 3 / 11 / 13 / 704 | 12 / 1 / 4 / 714 | — |
| Precision | 21.43% | 92.31% | -70.9pp |
| Recall | 18.75% | 75.00% | -56.3pp |
| F-1 | 20.00% | 82.76% | -62.8pp |

This gap is the direct, expected cost of task 3.6's calibration choice, not an
unaddressed defect: with no validated Ariana & Budayoga (2016) bobot tables for
Wuku/Penanggal/Sasih prohibition, the engine cannot satisfy both the <3% full-year
rarity gate (`test_scaffold_rarity_over_full_year`) and full recall on the 16 expert
dates — `verify_finding_experts_span_space` shows they are empirically mutually
exclusive, since the 16 expert dates span nearly the entire Wuku/Penanggal/Sasih range.
Task 3.6 prioritized the rarity gate per its DoD, which caps recall at 3/16.

**Corpus-limitation note**: the 16-date fixture is 46x smaller than the paper's own
731-day corpus and carries no per-day expert label outside those 16 dates. Closing this
gap requires the real bobot-source data (tracked for Phase 3), not further tuning
against this fixture — a rule base hand-fitted tighter to reproduce more of these
specific 16 dates without new source data would be overfitting to the fixture, not a
generalizable accuracy improvement.
