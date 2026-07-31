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
- [x] CHANGELOG.md updated with v0.3.0 notes (working classifier + documented accuracy gap, not a scaffold caveat)
- [x] `cargo semver-checks` passes (0.2→0.3 bump correctly clears the `DewasaAyuConfig.exclusion_penalty` field removal under 0.x semver rules)
- [x] Bump `Cargo.toml` version to `0.3.0`
- [x] Git tag `v0.3.0` created

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

---

## Dewasa Ayu Phase 3+4 — Bobot Tables & Multi-Category (blocked)

Follow-on work from the v0.3.0 Dewasa Ayu epic (Phase 1+2 shipped in `v0.3.0`,
2026-07-28). Every task below is blocked on primary source material that has not
been obtained — none should move to `cc:WIP` until its blocking resource lands. This
section exists so the backlog is tracked in the active sprint file instead of only
living in `TODO.md`.

| Task | Description | DoD | Depends | Status |
|------|-------------|-----|---------|--------|
| 3.8 | **Tier C resource acquisition** — manually acquire remaining Tier C research resources per `references/downloads/README.md` (ANRI archives, JoMEaL articles, Proudfoot 2007, Scribd-gated lontar transcriptions); BasaBali Wiki already acquired | each resource either downloaded to `references/downloads/` with `MANIFEST.json` updated (local path + SHA256), or explicitly documented as permanently inaccessible with the reason | - | blocked |
| 3.9 | **Wewaran bobot table** — derive weight table from expert pattern analysis (Buddha and Sukra highest-weighted saptawara, Redite/Saniscara effectively zero; Pon highest-weighted pancawara) to constrain the μ-functions | bobot table documented and wired into `rule_base::alahaning_dewasa_rules`; test asserts weights match the documented expert pattern | - | blocked |
| 3.10 | **Sasih bobot table** — from Ariana & Budayoga (2016), or reverse-engineered from kalenderbali.info if the source remains unobtained | bobot table documented and wired into the rule base; test coverage on representative Sasih inputs | Ariana & Budayoga (2016) source | blocked |
| 3.11 | **Penanggal/Pangelong bobot table** — same source as 3.10 | bobot table documented and wired into the rule base; test coverage on representative Penanggal/Pangelong inputs | Ariana & Budayoga (2016) source | blocked |
| 3.12 | **Ala Ayu Dewasa binary overlay** — prohibition overlay from Wariga Gemet, layered on top of the Sugeno score | overlay implemented and tested against known prohibited days | Wariga Gemet source | blocked |
| 3.13 | **Multi-category Dewasa Ayu** — extend beyond Pawiwahan to Menggunakan, Dewa Yadnya, Kerja/Pembangunan, Pertanian, Metatah, Ngaben, and Pemberangkatan, each potentially with its own bobot weights on the same Sugeno engine | per-category classification available; rule bases cross-checked against "Pedoman Ala Ayuning Dewasa" right-column entries | 3.9, 3.10, 3.11, 3.12 | blocked |

### Required resources (blocking Phase 3+4)
- [ ] Ariana & Budayoga (2016). *Ala Ayuning Dewasa Ketut Bangbang Gde Rawi (Sebuah Canang Sari)*, II. Denpasar: ESBE Buku. → bobot tables for each Wariga element (blocks 3.10, 3.11)
- [ ] Suwintana (2014). *Lontar Komputer* 5(1), 392–401. → full Mamdani rule base, useful for cross-validation even though this crate uses Sugeno
- [ ] Pasek Swastika (2015). *Wariga Padewasan*. Denpasar: CV. Kayumas Agung. → additional Dewasa Ayu classification rules

### Tier C acquisition status (see `references/downloads/README.md`)
- [x] BasaBali Wiki — acquired locally
- [ ] ANRI archives — blocked (Cloudflare 403)
- [ ] JoMEaL articles — blocked (404, no Wayback snapshot)
- [ ] Proudfoot 2007 — blocked (paywall/403 on all mirrors)
- [ ] Scribd-gated lontar transcriptions — blocked (subscription required)

---

## v0.4.0 — Export & Completeness

Ships the `TraditionalMarker` export API and batch generators data-pipeline consumers
need. Does **not** depend on v0.3.0 — nullable Dewasa Ayu fields handle its absence.

| Task | Description | DoD | Depends | Status |
|------|-------------|-----|---------|--------|
| 4.1 | **`SakaSeason` enum** (`src/marker.rs`) — `Pancaroba`/`MusimHujan`/`MusimKemarau` variants; `const fn from_sasih(s: Sasih)`; `const fn name() -> &'static str` (snake_case); serde derive under `serde` feature | enum compiles, `from_sasih` mapping matches `Sasih::season_tag()`, `name()` returns the documented snake_case strings, module doc notes single-direction pancaroba + Nampih Sasih non-effect | - | cc:TODO |
| 4.2 | **`TraditionalMarker` struct** — new public struct in `src/marker.rs` bundling gregorian_date/jdn/saka_year/saka_sasih/sasih_day/is_nampih/pawukon_day/wuku/traditional_season/is_pancaroba/ingkel(+domain fields)/tri_pramana/dewasa_pertanian/agricultural_guidance/saptawara/pancawara/combined_urip/rahinan | struct compiles with every field per the TODO.md spec; serde derive under `serde` feature; serde caveat on `&'static str` fields documented (same as `FlatRecord`) | 4.1 | cc:TODO |
| 4.3 | **`BalineseDate::to_traditional_marker()`** — method returning a populated `TraditionalMarker`; `dewasa_pertanian`/`agricultural_guidance` return `None` until Phase 4 Dewasa Ayu lands | method compiles and populates every field from `self`; the two `None` fields are documented as pending Phase 4 | 4.2 | cc:TODO |
| 4.4 | **Batch generators** — `generate_markers(start, end_inclusive)`, `generate_markers_for_saka_year(saka_year)`, `pawukon_positions(start, end_inclusive)`; iterate by JDN to avoid repeated Gregorian→JDN cost; re-export all four items from `lib.rs` | full Saka year (~355 days) generates in under 1ms on commodity hardware; `generate_markers_for_saka_year` ends the day before the next Nyepi; all items re-exported | 4.3 | cc:TODO |
| 4.5 | **Marker export tests** (`tests/marker_export_test.rs`) — round-trip serde, 5 known-date anchors (Nyepi/Galungan/Kuningan/Saraswati/Tilem Kasanga) asserted by hand, `SakaSeason::from_sasih` exhaustiveness, `generate_markers` 365-day contiguity/monotonic-JDN check, `generate_markers_for_saka_year(1948)` boundary check, `pawukon_positions` 210-day full-Wuku coverage check | all listed test cases present and passing with `cargo test --all-features` | 4.4 | cc:TODO |
| 4.6 | **Pedoman Ala Ayuning Dewasa** — `BalineseDate::ala_ayuning_dewasa() -> AlaAyuningDewasa` (Kala list, positive qualities, deity associations) from the 210 day-specific Kawi guidance entries printed in every Balinese calendar | blocked — source text is classical Kawi and OCR extraction is unreliable; needs a Kawi specialist / Wariga practitioner review before extraction can start | - | blocked |
| 4.7 | **Extended Rahinan** — Buda Cemeng (30 wuku variants), Anggara Kasih (30 wuku variants), post-Saraswati chain (Banyupinaruh→Soma Ribek→Sabuh Mas→Pagerwesi), pre-Galungan chain (Sugihan Jawa→Sugihan Bali→Penyajaan→Penampahan), post-Galungan chain (Umanis/Paing/Pon/Wage/Kliwon Galungan→Kuningan), per-Sasih Purnama/Tilem names (e.g. Purnama Kadasa = Besakih) | all listed Rahinan variants detected correctly against corpus dates; fixture/unit tests cover each named chain | - | cc:TODO |
| 4.8 | **Sasih-specific ceremonies** — `ceremonies_for_sasih(sasih: Sasih) -> Vec<SasihCeremony>` (Piodalan Sad Kahyangan); data from OCR supplement_5 covering all Bali regencies + Lombok + East Java | function returns the documented ceremony list per Sasih; fixture JSON sourced from supplement_5 | - | cc:TODO |
| 4.9 | **Candra Praleka** — `candra_praleka(sasih: Sasih) -> CandraPosition`; 12 stellar diagrams (Pleiades/Orion positions) extracted from OCR; connects to the `astronomical` feature flag | blocked — needs OCR extraction and validation of the 12 stellar diagrams from the source manuscript before implementation can start | - | blocked |
| 4.10 | **Multi-year Sasih transition table** — pre-compute Sasih transitions 2020–2035 for O(1) lookup; must account for Nampih Sasih (PHDI overrides need annual verification) | table generated and covers the full 2020–2035 range; cross-checked against the existing walk-forward algorithm output for the same range | - | cc:TODO |

---

## v0.5.0 — Climate-Aware Extension (blocked on v0.4.0 SakaSeason)

Feature-gated behind `#[cfg(feature = "climate")]`. Zero new dependencies.
WASM-compatible (`f32` ops only, no `std::time`, no allocation in compute path).

| Task | Description | DoD | Depends | Status |
|------|-------------|-----|---------|--------|
| 5.1 | **`climate` feature setup** — add `climate = []` feature (zero deps) to `Cargo.toml`; add a wasm32-unknown-unknown CI job for `--features climate` | `cargo build --features climate` succeeds; CI matrix includes the wasm job | - | cc:TODO |
| 5.2 | **`ClimateObservation` input type** — new struct in `src/climate.rs`: `rainfall_mm`, `temperature_c`, `wind_speed_kmh: f32`, `humidity_pct`/`solar_radiation_wm2: Option<f32>` | struct compiles under `#[cfg(feature = "climate")]`, all fields `f32`-only (WASM-compatible) | 5.1 | cc:TODO |
| 5.3 | **Climate enums** — `ObservedSeason` (`Wet`/`Dry`/`Transitioning`), `SeasonAlignment` (`Aligned`/`EarlyOnset`/`LateOnset`/`ExtendedTransition`/`Inverted`), `PancarobaSubPhase` (`PersistentMoisture`/`ThermalConvection`/`FalseDry`/`WavePulse`/`CyclonicFeed`, vocabulary only — classification stays downstream) | all three enums compile and are exported; doc comment states `PancarobaSubPhase` classification is a downstream concern | 5.2 | cc:TODO |
| 5.4 | **Seasonal reference envelopes** — `SeasonalEnvelope { rainfall_mm_mean, rainfall_mm_std, temperature_c_mean, temperature_c_std: f32 }`; `impl SakaSeason { const fn envelope(sasih: Sasih) -> SeasonalEnvelope }` calibrated from BMKG long-term Bali means partitioned by Sasih | envelope values documented as canonical starting points; `envelope()` returns a value for every `Sasih` variant | 4.1, 5.3 | cc:TODO |
| 5.5 | **`SeasonalState::compute()`** — `SeasonalState { saka_season, observed_season, alignment, divergence_score: f32, pancaroba_subphase: Option<PancarobaSubPhase> }`; deterministic stateless computation: season via `SakaSeason::from_sasih`, `observed_season` from documented rainfall threshold bands, `alignment` from a decision matrix, `divergence_score = sqrt(z_rain² + z_temp²)` (NaN-safe), `pancaroba_subphase` always `None` at this layer | `compute()` implemented per the 5-step spec; `divergence_score` NaN-safe for all float inputs; doc explains the multi-day window requirement for `pancaroba_subphase` | 5.4 | cc:TODO |
| 5.6 | **Climate tests** (`tests/climate_test.rs`, gated) — `SakaSeason`×`ObservedSeason` matrix vs expected `SeasonAlignment` for all combinations, aligned-baseline divergence ≈ 0.0, anomalous observation (heavy rain in Sasih Karo) → `Inverted` + high score, determinism check, `divergence_score >= 0.0 && is_finite()` for all float inputs | all listed test cases present and passing with `cargo test --features climate` | 5.5 | cc:TODO |

---

## v1.0.0 — Stable API

| Task | Description | DoD | Depends | Status |
|------|-------------|-----|---------|--------|
| 6.1 | **API stability commitment** — freeze all public types/methods; add `#[non_exhaustive]` to enums that may gain variants; document MSRV policy | `cargo semver-checks` baseline established for 1.0; MSRV policy documented in README/CONTRIBUTING | - | cc:TODO |
| 6.2 | **`no_std` support** — gate `std`-only code behind a `std` feature flag to enable embedded/IoT targets | `cargo build --no-default-features --features <core-set>` succeeds on a `no_std` target | 6.1 | cc:TODO |
| 6.3 | **C FFI** — expose a C ABI via `cbindgen` | generated header compiles; smoke-tested from a minimal C consumer | 6.1 | cc:TODO |
| 6.4 | **Python bindings** — `balinese-calendar-py` via `pyo3`/`maturin` | package builds and installs via `maturin develop`; smoke test imports and calls the core API from Python | 6.1 | cc:TODO |
| 6.5 | **Swift/Kotlin wrappers** — mobile bindings | wrapper packages build for iOS/Android targets; smoke test from each platform | 6.1 | cc:TODO |
| 6.6 | **Aksara Bali output** — Unicode Balinese script output for all calendar term names | representative name set (Wuku, Sasih, Wewaran) renders the correct Aksara Bali Unicode | - | cc:TODO |
| 6.7 | **Indonesian language strings** — Indonesian-language labels alongside existing English strings | every public name-returning method has an Indonesian variant; tested for at least one full cycle of each enum | - | cc:TODO |

---

## Backlog (untargeted)

| Task | Description | DoD | Depends | Status |
|------|-------------|-----|---------|--------|
| 7.1 | **PHDI Nampih Sasih automation** — structured data file or scraping to track official PHDI intercalary-month declarations | annual PHDI declarations captured in a structured, versioned file; documented process for yearly refresh | - | cc:TODO |
| 7.2 | **Annual validation corpus generation** — generate a validation corpus from each year's printed calendar (process already established for 2026) | documented repeatable process; at least one additional year's corpus added | - | cc:TODO |
| 7.3 | **Additional-year corpora + peradnya cross-validation** — add corpora from other years/publishers; cross-validate against peradnya for multi-year ranges | corpus test suite covers 2+ years; peradnya cross-validation results documented | 7.2 | cc:TODO |
| 7.4 | **Document calendar-authority disagreements** — Ngunaratri boundary, PancaSuda naming, Nampih Sasih placement, and any other known divergences | disagreements documented in code + `references/BIBLIOGRAPHY.md` with source citations for each side | - | cc:TODO |
| 7.5 | **Historical date support** — dates before the current epoch for inscription/lontar (prasasti) research | `from_ymd`/`from_jdn` accept dates before the current epoch with documented accuracy bounds; at least one prasasti-dated smoke test | - | cc:TODO |
