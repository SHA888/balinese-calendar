# TODO — balinese-calendar

> The first native Rust implementation of the Balinese Saka Calendar.
> Tracking what's done, what's next, and what the community can help with.

---

## v0.1.2 — Stabilise Core

### Done
- [x] Add `DayBoundary` enum (`Midnight`, `FixedSunrise`, `Astronomical` stub)
- [x] `today_with_boundary()` — fix 00:00–06:00 ambiguous window
- [x] Feature-gate `DayBoundary::Astronomical` behind `#[cfg(feature = "astronomical")]`
- [x] Replace hardcoded `NAMPIH_YEARS` with algorithmic Metonic cycle detection
- [x] Rewrite sasih as walk-forward algorithm from peradnya pivot points
- [x] Validate Hari Raya Nyepi (March 19, 2026 = Tahun Baru Saka 1948)
- [x] Fix `PAWUKON_EPOCH_JDN` (corrected to 2440976 from peradnya pivots)
- [x] Fix Pancawara, Caturwara, Astawara computations to match peradnya reference
- [x] Refactor `PancaSuda`, `Pararasan`, `Rakam` to take pre-computed wewaran refs
- [x] Flatten `SasihDayInfo::Ngunaratri` to non-recursive `TithiPhase`; `SasihDayInfo` is `Copy`
- [x] Add `HariBhataraSri` (Buda Wage) detection in `Rahinan::detect()`
- [x] Add `impl fmt::Display for SasihDayInfo`
- [x] Fix silent Saraswati test — converted to unconditional `assert_eq!`
- [x] Add Ngunaratri edge case tests (specific dates + 63-day cycle integrity)
- [x] Add Astawara and Sangawara spot-check tests
- [x] Resolve contradictory sasih assertions for March 6, 2026
- [x] Remove unused `LUNATION_DAYS` and `SASIH_EPOCH_JDN` constants

### Validation corpus (DONE)
- [x] Generate 2026 full-year validation corpus from printed Balinese calendar
      (I Made Bidja Alm., IBI Cabang Kab. Badung — 50+ lontar Wariga sources)
      - `tests/fixtures/balinese_calendar_2026_corpus.json` — 365 days, all fields
      - `tests/fixtures/gebogan_urip_tri_pramana.json` — 210-entry Wuku × Sapta Wara lookup
      - `tests/validation_2026_test.rs` — integration tests covering pawukon, sasih
        boundaries, saka year, ingkel, urip, rahinan, pararasan, cycle integrity
      - Cross-validated against kalenderbali.org (I Wayan Nuarsa, Universitas Udayana)
      - 365/365 day-of-week matches · 30/30 Wuku · 12/12 Sasih · zero mismatches

### Remaining before tag
- [x] **A2: Paringkelan spot-checks** — assert Watek (Madya & Alit) and Lintang
      output against ~30 dates from 2026 corpus. Cross-validate names against
      edysantosa/sakacalendar (LGPL-2.1) tables:
      - Watek Alit (4): Uler, Gajah, Lembu, Lintah
      - Watek Madya (5): Gajah, Watu, Buta, Suku, Wong
      - Lintang (35): Gajah through Pucang
      Source: I.B. Putra Manik Aryana, *Dasar Wariga* + *Tenung Wariga*;
      I.B. Supartha Ardana, *Pokok-Pokok Wariga* (2005).
- [x] **A3: Pararasan validation** — uncomment assertions in `validation_2026_test.rs`.
      Two naming traditions must be supported:
      | Aryana (edysantosa) | Bidja (OCR corpus) | Status |
      |---|---|---|
      | Wisesa Segara | Wisesa Segara | Match |
      | Tunggak Semi | Tunggak Semi | Match |
      | Satria Wibhawa | Satria Wibawa | Spelling variant |
      | Sumur Sinaba | Sumer Sinuhe | **Different tradition** |
      | Bumi Kapetak | Bumi Kapetak | Match |
      | Satria Wirang | Satria Wirang | Match |
      | Lebu Katiup Angin | Lelu Kalung Angis | **Different tradition** |
      Exposed both via `PancaSuda::name()` (Aryana default) and
      `PancaSuda::name_sundari_bungkah()` (Bidja variant), with doc comments
      citing the manuscript source for each.
- [x] **A4: Gebogan Urip Tri-Pramana validation** — compare 210-entry table against
      crate's standard urip. Key finding: Tri-Pramana = f(Wuku, SaptaWara) only,
      values 12–29, incorporating SadWara. This is NOT SaptaWara.urip + PancaWara.urip.
      Flag outlier: Pahang + Soma = 29 (max value) — verify against physical source.
- [x] **Perf: pawukon_day() dedup** — compute once in `from_jdn_unchecked`, pass to
      all subsystem constructors (currently recomputed ~15× per date construction)
- [x] **Safety: date validation** — use `NaiveDate::from_ymd_opt` in `gregorian_to_jdn()`
      to reject impossible dates like Feb 30
- [x] **Code Review Fixes** — fixed critical Pararasan enum design flaw, completed
      performance optimization for all pawukon-dependent components, improved test
      robustness, and removed tautology assertions

---

## v0.1.3 — Infrastructure

### `serde` feature flag
- [x] Derive `Serialize` / `Deserialize` on all public types behind `serde` feature
- [x] Include `serde` and `serde_json` as optional dev-dependencies
- [x] Enables JSON output for any API, pipelines, frontend bridges

### WASM target
- [x] `wasm32-unknown-unknown` support via `wasm-bindgen`
- [x] JS interop layer: `from_ymd()`, `today()`, rahinan list, formatted string
- [x] Depends on: `serde` feature (for JSON bridge to JS)
- [x] Enables client-side Balinese calendar in any web frontend

### Astronomical sunrise
- [x] Implement `DayBoundary::Astronomical` using the `sunrise` crate
      - Bali centroid default: lat -8.3405, lon 115.0920
      - Accept custom coordinates for non-Bali Hindu communities
      - Test against known sunrise times from BMKG
- [x] Expose `DayBoundary` in WASM bindings

---

## v0.2.0 — Wariga Computation Layer

This release adds the traditional Wariga computation systems extracted from
I Made Bidja's 2026 calendar (Wariga Sundari Bungkah manuscript tradition)
and cross-validated against edysantosa/sakacalendar (Aryana manuscript tradition)
and peradnya/balinese-date-java-lib.

### Wariga BELOG Harmonisation (personalized day quality) — NEW
Fully extracted from JSI/STIKOM 2022. Pure modular arithmetic, no fuzzy logic.
This is the simplest Dewasa Ayu feature: personalized to birth date, computable today.

- [x] New type: `WarigaBelog` enum — `Pati`, `Guru`, `Ratu`, `Lara`
- [x] Algorithm: `(birth_urip + daily_urip) % 4` where urip = sapta + panca
      ```
      0 = Pati   — danger, avoid major activities
      1 = Guru   — wisdom, good for learning/spiritual practice
      2 = Ratu   — authority, good for leadership/official matters
      3 = Lara   — suffering, avoid important undertakings
      ```
- [x] API: `wariga_belog(birth: &BalineseDate, query: &BalineseDate) -> WarigaBelog`
- [x] Source: Wariga BELOG manuscript (Gianyar tradition), via T.I.P. Nyoman (2014)
      *Guide Book Buku Pedoman Wariga Belog*, Koleksi Griya Cebaang Giri Kesuma.

### Gebogan Urip Tri-Pramana (public API)
The Tri-Pramana system assigns a composite urip value and fourfold quality
classification to each of the 210 Wuku-day positions.

- [x] New type: `TriPramana { urip: u8, quality: PramanaQuality }`
- [x] `PramanaQuality` enum with 4 variants:
      - `LungguhSakti` — auspicious for crafting, practical work
      - `UtamaAsih` — excellent for all good works
      - `PugeranBakti` — favourable for worship, devotion
      - `MuktiPapa` — inauspicious, risk of danger
- [x] JSON loading from `tests/fixtures/gebogan_urip_tri_pramana.json`
- [x] API: `BalineseDate::tri_pramana() -> Option<TriPramana>`
- [x] Source: Wariga Sundari Bungkah via I Made Bidja (complete table extracted)
- [x] Document clearly: this differs from standard `sapta_wara.urip() + panca_wara.urip()`
- [x] Bounds checking: returns None for invalid pawukon_day (≥210)

### Pawiwahan (marriage compatibility)
The single most-consulted Wariga table in Balinese culture.

- [x] `pawiwahan_compatibility(a: &BalineseDate, b: &BalineseDate) -> PawiwahanResult`
- [x] `PawiwahanResult { combined_urip: u8, remainder: u8, quality: PawiwahanQuality }`
- [x] 16-point quality scale from Wariga Sundari Bungkah:
      ```
       1  Madya (Suka-Duka) — mixed fortune
       2  Kawon (Lara, Miskin) — hardship, poverty
       3  Kawon (Lara, Warang) — strife, frequent quarrels
       4  Kawon (Panake Mati) — danger to children
       5  Becik Pisan (Sudha Nulus) — excellent, harmonious
       6  Kawon (Sengsara) — suffering, frequent illness
       7  Madya (Suka-Duka) — mixed fortune
       8  Kawon (Lara, Kenapali) — persistent hardship
       9  Kawon Pisan (Baya Pati) — worst, risk of death
      10  Becik (Bikiga Ratuna) — good, influential
      11  Becik (Kapardyaniyah) — good, prosperous livelihood
      12  Becik (Kedrping Hari) — good, harmonious
      13  Becik (Tan Kirang) — wealthy, abundant
      14  Kawon (Tan Polih Keselamatan) — persistent misfortune
      15  Becik (Bokung) — good but childless
      16  Becik (Nyama Braya Asih) — beloved by family/community
      ```
- [x] Full 30×7 base lookup table already extracted from OCR
- [x] Cross-validate against einvite.id and kalenderbali.info

### Dauh Sukaranti (time-slot quality)
Traditional system for best time of day, based on combined urip.

- [x] `dauh_sukaranti(urip: u8) -> [DauhQuality; 5]`
- [x] 5 time periods: Dauh I (05:30–07:55), II (07:55–10:25), III (10:20–12:45),
      IV (12:45–15:10), V (15:10–17:30) WITA
- [x] Quality values: Kelara · Pali · Sume · Krta · Peta
- [ ] Complete 12×5 lookup table extracted from OCR (currently using placeholder algorithm)
- [x] Source: Wariga Sundari Bungkah via I Made Bidja

### Tenung Patemuan Adan (name compatibility)
- [x] `name_compatibility(a: &str, b: &str) -> PatemuanResult`
- [ ] Letter → urip mapping via directional chart (18 consonant groups) (currently using placeholder algorithm)
- [x] Source: Lontar Joyoboyo

### Otonan calculator
The otonan (Balinese birthday) falls every 210 days. Second most-requested
feature after Dewasa Ayu.

- [x] `otonan_dates(birth: NaiveDate, count: usize) -> Vec<NaiveDate>`
- [x] `next_otonan(birth: NaiveDate) -> NaiveDate`
- [x] `next_otonan_from(birth: NaiveDate, after: NaiveDate) -> NaiveDate`

### Ingkel ecological domain accessors
Pulled forward from v0.4.0 — belongs thematically in the Wariga computation layer.

- [ ] `Ingkel::ecological_domain() -> &'static str` — English snake_case label (stable for data-pipeline columns)
      | Variant | Return value |
      |---|---|
      | Wong  | `human_affairs`  |
      | Sato  | `animals`        |
      | Mina  | `fish_maritime`  |
      | Manuk | `birds`          |
      | Taru  | `trees_forestry` |
      | Buku  | `bamboo_reeds`   |
- [ ] `Ingkel::ecological_domain_id() -> &'static str` — Balinese manuscript term (`wong`, `sato`, `mina`, `manuk`, `taru`, `buku`)
- [ ] Tests: exhaustive coverage for all 6 variants on both methods
- [ ] Doc comment citing: I.B.S. Ardhana, *Pokok-Pokok Wariga* (2005); I Made Bidja bibliography

---

## v0.3.0 — Dewasa Ayu (Auspicious Day Classification)

This is the highest-impact user-facing feature. The implementation strategy is
grounded in two peer-reviewed studies and validated against a Wariga expert's
16-date ground truth for 2020–2021.

### Architectural decisions (based on evidence)

**Method: Sugeno fuzzy inference (not Mamdani)**
Candana et al. (2021) conclusively demonstrated Sugeno outperforms Mamdani:
- Sugeno F-1 = 82.76% · Precision = 92.31% · Recall = 75%
- Mamdani F-1 = 5.41% · Precision = 4.76% · Recall = 6.25%
- Tsukamoto F-1 = 4.65% · Precision = 3.70% · Recall = 6.25%
Sugeno found 12/16 expert days with 1 false positive. Mamdani found 1/16 with
20 false positives. Source: JIK 6(2), 14–22 (Universitas Pendidikan Ganesha).

**Alahaning Dewasa hierarchy (override priority)**
Confirmed by Candana 2021 Section II.B and Wariga Sundari Bungkah:
```
Priority (low → high): Wewaran → Wuku → Penanggal → Sasih → Dauh
```
A bad Sasih overrides a good Wuku. Ala Ayu Dewasa adds binary prohibition overlays.

**Rarity constraint**
Only 16/731 days (2.19%) classified as "good" for Pawiwahan by the expert.
Any implementation producing >3% good days is likely too permissive.

### Implementation phases

#### Phase 1: Validation fixture + scoring scaffold
- [x] Add `tests/fixtures/candana_2021_dewasa.json` with 79 prediction dates:
      - 16 expert (Pakar Wariga) days with scores 70–80
      - 13 Sugeno predictions with scores 70–76 (11 TP, 1 FP with fixture dates)
      - 27 Tsukamoto predictions (1 TP, 26 FP)
      - 21 Mamdani predictions (1 TP, 20 FP)
      - **Note:** Wewaran values corrected to match balinese-calendar library calculations
- [x] Add wewaran cross-reference for all 16 expert dates:
      Saptawara distribution: Wraspati 12, Sukra 4 (actual from fixture dates)
      Pancawara distribution: Wage 4, Kliwon 4, Pon 3, Paing 3, Umanis 2
      Score-80 days: Wraspati, Sukra (2 entries at score 80)
      Expert NEVER selects Redite or Saniscara (verified ✓)
- [x] New trait: `DewasaAyu` with method `score(&self) -> f64` (0.0–1.0)
- [x] Threshold: `is_dewasa_ayu(&self) -> bool` where score > configurable threshold
- [x] Test: reproduce Sugeno TP matches against expert ground truth (11 TP with fixture dates)

#### Phase 2: Five-variable Sugeno inference engine
- [x] Implement zero-order Sugeno fuzzy inference (constant consequents):
      ```rust
      struct SugenoEngine { rules: Vec<SugenoRule> }
      struct SugenoRule {
          wewaran_set: FuzzySet, wuku_set: FuzzySet, penanggal_set: FuzzySet,
          sasih_set: FuzzySet, ala_ayu_set: FuzzySet, output: f64,
      }
      fn infer(&self, input: &DewasaInput) -> f64 {
          let fired: Vec<(f64, f64)> = self.rules.iter()
              .map(|r| (r.firing_strength(input), r.output))
              .filter(|(strength, _)| *strength > 0.0).collect();
          fired.iter().map(|(w,z)| w*z).sum::<f64>()
              / fired.iter().map(|(w,_)| *w).sum::<f64>()
      }
      ```
      Implemented in `src/dewasa_ayu.rs` with `SugenoEngine::infer()` method.
- [x] Membership functions: triangular/trapezoidal for each variable
      - 5 linguistic values: SBr (0.1), Br (0.3), S (0.5), B (0.75), SB (0.9)
      - `FuzzySet::triangular(a, b, c)` and `FuzzySet::trapezoidal(a, b, c, d)`
      - Standard preset sets: `standard_sets::triangular_five()`, `trapezoidal_five()`
- [x] Feature-gate behind `#[cfg(feature = "dewasa-ayu")]` — enables f64 ops and Sugeno types
      Exported: `DewasaInput`, `FuzzySet`, `LinguisticValue`, `MembershipShape`, `SugenoEngine`, `SugenoRule`

#### Research Resources Acquisition (completed)
Supporting materials for Phase 3–4 bobot derivation and rule extraction.

- [x] Curated source checklist at `references/resources.md` — 60+ sources organized in 10 sections (Tier A/B/C)
- [x] Local archive at `references/downloads/` with subdirectories `s1-primary-scans/` through `s10-government/`
- [x] Large PDFs tracked via Git LFS (wariga-gemet.pdf 508MB, wariga-gede-gemet.pdf 75MB)
- [x] Idempotent fetcher `tools/fetch_resources.py` + `tools/requirements.txt` with SSL fallback, Wayback Machine integration
- [x] `MANIFEST.json` with per-resource URL, local path, SHA256 hash, status codes
- [ ] Manual acquisition pending for Tier C items — see `references/downloads/README.md`:
  - BasaBali Wiki (Cloudflare 403)
  - ANRI archives (Cloudflare 403)
  - JoMEaL articles (404, no Wayback snapshot)
  - Proudfoot 2007 (paywall/403 on all mirrors)
  - Scribd-gated lontar transcriptions (subscription required)

#### Phase 3: Bobot (weight) tables
- [ ] Wewaran bobot: from expert pattern analysis, Buddha and Sukra are highest-
      weighted saptawara for Pawiwahan; Redite and Saniscara are effectively zero.
      Pon is highest-weighted pancawara. These patterns constrain the μ-functions.
- [ ] Sasih bobot: from Ariana & Budayoga (2016) or reverse-engineer from
      kalenderbali.info
- [ ] Penanggal/Pangelong bobot: from same source
- [ ] Ala Ayu Dewasa classification: binary overlay from Wariga Gemet

#### Phase 4: Multi-category Dewasa Ayu
- [ ] Extend beyond Pawiwahan to other ceremony types:
      - Menggunakan (general auspicious activities)
      - Dewa Yadnya (worship, temple ceremonies)
      - Kerja / Pembangunan (work, construction)
      - Pertanian (agriculture)
      - Metatah (tooth-filing ceremony)
      - Ngaben (cremation ceremony)
      - Pemberangkatan (travel, departure)
- [ ] Each category may have different bobot weights — same Sugeno engine,
      different rule bases
- [ ] Source: printed calendar "Pedoman Ala Ayuning Dewasa" right-column entries

### Required resources (not yet obtained)
- [ ] Ariana & Budayoga (2016). *Ala Ayuning Dewasa Ketut Bangbang Gde Rawi
      (Sebuah Canang Sari)*, II. Denpasar: ESBE Buku.
      → Contains bobot tables for each Wariga element
- [ ] Suwintana (2014). *Lontar Komputer* 5(1), 392–401.
      → Full Mamdani rule base (useful for cross-validation even though we use Sugeno)
- [ ] Pasek Swastika (2015). *Wariga Padewasan*. Denpasar: CV. Kayumas Agung.
      → Additional Dewasa Ayu classification rules

---

## v0.4.0 — Export & Completeness

This release ships the TraditionalMarker export API and batch generators that GARUDA
and other downstream consumers need to populate Saka-aware data pipeline tables.
Does **not** depend on v0.3.0 — nullable fields handle absence of Dewasa Ayu cleanly.

### `SakaSeason` enum (new module: `src/marker.rs`)

Deterministic, O(1) classification derived from `Sasih` position.

- [ ] `SakaSeason` enum with three variants:
      ```rust
      pub enum SakaSeason { MusimHujan, MusimKemarau, Pancaroba }
      ```
- [ ] `const fn SakaSeason::from_sasih(s: Sasih) -> SakaSeason`
      Mapping (consistent with existing `Sasih::season_tag()`):
      - `Pancaroba`: Kalima, Kanem
      - `MusimHujan`: Kapitu, Kawolu, Kasanga, Kadasa
      - `MusimKemarau`: Jyesta, Sadha, Kasa, Karo, Katiga, Kapat
- [ ] `const fn SakaSeason::name() -> &'static str` — snake_case: `pancaroba`, `musim_hujan`, `musim_kemarau`
- [ ] `#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]`
- [ ] Module doc note: single-direction pancaroba (dry→wet) per validated v0.1.2 work;
      if wet→dry pancaroba is later validated, add `Sasih::pancaroba_direction()` (additive, non-breaking)
- [ ] Module doc note: Nampih Sasih does not change classification — it duplicates the
      underlying Sasih which already carries its season tag

### `TraditionalMarker` struct

Structured representation of traditional Saka seasonal knowledge for a date,
suitable for ingestion into data pipelines. All fields deterministically computed.

- [ ] New public struct in `src/marker.rs`:
      ```rust
      pub struct TraditionalMarker {
          pub gregorian_date: NaiveDate,
          pub jdn: i64,                              // matches BalineseDate::jdn
          pub saka_year: i32,                        // matches BalineseDate::saka_year
          pub saka_sasih: Sasih,
          pub sasih_day: SasihDayInfo,
          pub is_nampih: bool,
          pub pawukon_day: u16,                      // 0–209, matches BalineseDate::pawukon_day
          pub wuku: Wuku,
          pub traditional_season: SakaSeason,
          pub is_pancaroba: bool,
          pub ingkel: Ingkel,
          pub ingkel_domain: &'static str,           // = ingkel.ecological_domain()
          pub ingkel_domain_id: &'static str,        // = ingkel.ecological_domain_id()
          pub tri_pramana: Option<TriPramana>,
          pub dewasa_pertanian: Option<f64>,         // None until v0.3.0 Phase 4
          pub agricultural_guidance: Option<&'static str>, // None until Pedoman / Phase 4
          pub saptawara: Saptawara,
          pub pancawara: Pancawara,
          pub combined_urip: u8,                     // saptawara.urip() + pancawara.urip()
          pub rahinan: Vec<&'static str>,            // Rahinan::name() values — serde-clean
      }
      ```
- [ ] `#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]`
- [ ] Serde caveat documented: `&'static str` fields serialize but cannot round-trip
      deserialize (same constraint as existing `FlatRecord`)

### `BalineseDate::to_traditional_marker()`

- [ ] Method on `BalineseDate` returning `TraditionalMarker`
- [ ] Populates all fields from `self`; `dewasa_pertanian` and `agricultural_guidance` return `None`

### Batch generators

- [ ] `pub fn generate_markers(start: NaiveDate, end_inclusive: NaiveDate) -> Vec<TraditionalMarker>`
      Iterates by JDN to avoid repeated Gregorian→JDN cost.
      Performance target: full Saka year (~355 days) under 1 ms on commodity hardware.
- [ ] `pub fn generate_markers_for_saka_year(saka_year: i32) -> Vec<TraditionalMarker>`
      Walks Gregorian days from Nyepi of `saka_year` to the day before Nyepi of `saka_year + 1`.
      Thin wrapper over `generate_markers` using existing sasih walk-forward.
- [ ] `pub fn pawukon_positions(start: NaiveDate, end_inclusive: NaiveDate) -> Vec<(NaiveDate, u16, Wuku)>`
      Strict subset projection for callers needing only Pawukon position
      (e.g. Pawukon-MJO correlation analysis) without full marker allocation cost.
- [ ] Re-export all four items from `lib.rs`

### Tests (`tests/marker_export_test.rs`)

- [ ] Round-trip serde: `serde_json::to_string(&marker)` → validate field count and key spelling
- [ ] Known-date verification: 5 anchor days from 2026 corpus (Nyepi, Galungan, Kuningan,
      Saraswati, Tilem Kasanga) — assert every field by hand
- [ ] `SakaSeason::from_sasih` exhaustiveness against all 12 Sasih
- [ ] `generate_markers` for full Gregorian 2026 returns 365 markers, contiguous dates, monotonic JDN
- [ ] `generate_markers_for_saka_year(1948)` ends the day before Nyepi 2027
- [ ] `pawukon_positions` for a 210-day window covers all 30 Wuku exactly once at wuku-day-0 positions

### Pedoman Ala Ayuning Dewasa
Every printed Balinese calendar includes 210 day-specific guidance entries in Kawi.

- [ ] `BalineseDate::ala_ayuning_dewasa() -> AlaAyuningDewasa`
- [ ] Struct: Kala list, positive qualities, deity associations
- [ ] Challenge: source text in classical Kawi — OCR extraction unreliable
- [ ] **Community help wanted:** Kawi specialists and Wariga practitioners

### Extended Rahinan
- [ ] Buda Cemeng (Buda Kliwon per wuku) — 30 named variants
- [ ] Anggara Kasih (Anggara Kliwon per wuku) — 30 named variants
- [ ] Post-Saraswati: Banyupinaruh → Soma Ribek → Sabuh Mas → Pagerwesi
- [ ] Pre-Galungan: Sugihan Jawa → Sugihan Bali → Penyajaan → Penampahan
- [ ] Post-Galungan: Umanis/Paing/Pon/Wage/Kliwon Galungan → Kuningan
- [ ] Purnama/Tilem per-Sasih names (e.g. Purnama Kadasa = Besakih ceremony)

### Sasih-specific ceremonies (Piodalan Sad Kahyangan)
- [ ] `ceremonies_for_sasih(sasih: Sasih) -> Vec<SasihCeremony>`
- [ ] Data: supplement_5 from OCR (all Bali regencies + Lombok + East Java)

### Ingkel ecology metadata
- [x] `Ingkel::ecological_domain() -> &'static str` — pulled forward to v0.2.0
- [x] `Ingkel::ecological_domain_id() -> &'static str` — Balinese term — pulled forward to v0.2.0

### Candra Praleka (observational Sasih verification)
- [ ] `candra_praleka(sasih: Sasih) -> CandraPosition`
- [ ] 12 stellar diagrams (Pleiades/Orion positions) extracted from OCR
- [ ] Connects to `astronomical` feature flag

### Multi-year Sasih transition table
- [ ] Pre-compute Sasih transitions 2020–2035 for O(1) lookup
- [ ] Must account for Nampih Sasih (PHDI overrides need annual verification)

---

## v0.5.0 — Climate-Aware Extension

Feature-gated behind `#[cfg(feature = "climate")]`. Zero new dependencies.
WASM-compatible (`f32` ops only, no `std::time`, no allocation in compute path).
All types serde-aware via existing flag.

### Cargo.toml
- [ ] Add `climate = []` feature (zero deps)
- [ ] Add wasm32-unknown-unknown build job to CI matrix for `--features climate`

### New module: `src/climate.rs`

#### Input type
```rust
#[cfg(feature = "climate")]
pub struct ClimateObservation {
    pub rainfall_mm: f32,
    pub temperature_c: f32,
    pub wind_speed_kmh: f32,
    pub humidity_pct: Option<f32>,
    pub solar_radiation_wm2: Option<f32>,
}
```

#### Enums
- [ ] `ObservedSeason` — `Wet`, `Dry`, `Transitioning`
- [ ] `SeasonAlignment` — `Aligned`, `EarlyOnset`, `LateOnset`, `ExtendedTransition`, `Inverted`
- [ ] `PancarobaSubPhase` — `PersistentMoisture`, `ThermalConvection`, `FalseDry`, `WavePulse`, `CyclonicFeed`
      **Classification of this enum lives downstream** (requires multi-day window).
      The crate exports the vocabulary; consumers apply the classification logic.
      Motivated by: Feb–Apr 2026 Denpasar observations — 3-month chaotic pancaroba
      with five distinct observable sub-phases.

#### Sasih reference envelopes
- [ ] `pub struct SeasonalEnvelope { rainfall_mm_mean: f32, rainfall_mm_std: f32, temperature_c_mean: f32, temperature_c_std: f32 }`
- [ ] `impl SakaSeason { pub const fn envelope(sasih: Sasih) -> SeasonalEnvelope }`
      Initial calibration from BMKG long-term Bali means partitioned by Sasih.
      Doc note: these are canonical starting-point values; consumers with multi-year
      baselines should compute their own divergence against richer historical context.

#### `SeasonalState` and `compute()`
- [ ] `pub struct SeasonalState { saka_season, observed_season, alignment, divergence_score: f32, pancaroba_subphase: Option<PancarobaSubPhase> }`
- [ ] `SeasonalState::compute(date: &BalineseDate, obs: &ClimateObservation) -> SeasonalState`
      Computation (deterministic, stateless):
      1. `saka_season = SakaSeason::from_sasih(date.sasih)`
      2. `observed_season` from rainfall threshold bands (documented constants)
      3. `alignment` from decision matrix of `saka_season` × `observed_season`
      4. `divergence_score = sqrt(z_rain² + z_temp²)` against Sasih envelope; NaN-safe
      5. `pancaroba_subphase = None` (documented as downstream concern)
- [ ] `pancaroba_subphase` field always `None` at this layer — doc explains multi-day
      window requirement and directs consumers to implement classification themselves

### Tests (`tests/climate_test.rs`, gated)
- [ ] `SakaSeason` × `ObservedSeason` matrix → expected `SeasonAlignment` for all combinations
- [ ] Aligned-baseline observation → `divergence_score` ≈ 0.0
- [ ] Anomalous observation (heavy rain in Sasih Karo) → `Inverted` alignment + high score
- [ ] Determinism: identical inputs → identical outputs
- [ ] `divergence_score >= 0.0 && is_finite()` for all float inputs

### Research context (recorded in module doc)
- **Pawukon-MJO hypothesis:** 210-day cycle ≈ 3.5–7 MJO cycles (MJO: 30–60 day period).
  Wuku subdivisions may encode empirically observed MJO-modulated seasonal patterns.
  Testable with correlation analysis against BoM MJO phase data via `pawukon_positions()`.
- **Climate drift measurement:** Divergence between Saka seasonal markers and observed
  climate measures how far Indonesia's climate has moved from the stable regime the
  calendar was calibrated against over ~1,000 years.
  Working title: *"Bidirectional Encoding of Traditional Balinese Seasonal Knowledge
  and Modern Climate Data: A Computational Framework for Measuring Climate Drift
  Through Cultural Knowledge Systems."*
- **Alahaning Dewasa in climate context:** `Sasih` is the dominant seasonal signal
  in the traditional hierarchy — maps naturally to `saka_sasih` as the primary
  partition key for climate baseline computation.

---

## v1.0.0 — Stable API

### API stability commitment
- [ ] All public types and methods frozen
- [ ] `#[non_exhaustive]` on all enums that may gain variants
- [ ] MSRV policy documented

### Platform targets
- [ ] `no_std` support behind `std` feature flag (enables embedded/IoT)
- [ ] C FFI via `cbindgen`
- [ ] Python bindings via `pyo3`/`maturin` (`balinese-calendar-py`)
- [ ] Swift/Kotlin wrappers for mobile

### Locale & script
- [ ] Aksara Bali (Unicode Balinese script) output for all names
- [ ] Indonesian language strings alongside English

---

## Backlog

### Maintenance
- [ ] PHDI Nampih Sasih automation: structured data file or scraping
- [ ] Annual validation: generate corpus from each year's printed calendar

### Data quality
- [ ] Add corpora from additional years and publishers
- [ ] Cross-validate against peradnya for multi-year ranges
- [ ] Document cases where calendar authorities disagree
      (known: Ngunaratri boundary, PancaSuda naming, Nampih Sasih placement)

### Historical date support
- [ ] Dates before current epoch for inscription/lontar research
- [ ] Useful for scholars working with prasasti (stone inscriptions)

---

## Validation Sources

### Primary (2026 corpus)
- **I Made Bidja Alm.** / I Md Agus Putra Wijaya — *Kalender Bali 2026*
  IBI Cabang Kab. Badung. 50+ lontar, 13 dictionaries. Full bibliography in
  `tests/fixtures/BIBLIOGRAPHY.md`.

### Cross-validation
- **kalenderbali.org** — I Wayan Nuarsa (Universitas Udayana)
- **dictionary.basabali.org** — BASAbali Wiki
- **kebudayaan.kemdikbud.go.id/bpnbbali** — BPNB Bali
- **babadbali.com** — Yayasan Bali Galang

### Algorithm reference
- edysantosa/sakacalendar (LGPL-2.1) — Java, complete paringkelan tables
- peradnya/balinese-date-java-lib (Apache-2.0) — Java/JS, Pawukon + Sasih
- Candana et al. (2021) JIK 6(2) — Sugeno vs Mamdani comparison, 16-date ground truth
- Suwintana (2014) — Mamdani fuzzy Dewasa Pawiwahan (Candana 2021 ref [3] miscites as 2015, pp. 392–403)
- JSI/STIKOM (2022) — Wariga BELOG mod-4 algorithm
- Karjanto (2020) arXiv:2012.10064 — Zeller's congruence for Pawukon

### Key lontar sources (via I Made Bidja bibliography)
| Manuscript | Systems derived |
|---|---|
| Wariga Sundari Bungkah | Tri-Pramana, Pawiwahan, Pararasan, Dauh Sukaranti |
| Wariga Gemet | Ala-Ayuning Dewasa (day quality classification) |
| Lontar Joyoboyo | Tenung Patemuan Adan (name compatibility) |
| Wariga Candra Praleka | Stellar observation for Sasih determination |
| Wariga Pawukon | 30-Wuku cycle, Bhatara associations |
| Wariga BELOG | Personal day-quality harmonisation (mod-4) |

---

## Contributing

Contributions are especially welcome for:

1. **Dewasa Ayu rules** — If you understand the compound Wewaran conditions that
   determine good/bad days for specific activities, this is the single most impactful
   contribution. We have 77 dated predictions from Candana (2021) as ground truth;
   what's needed is the bobot (weight) tables. Open an issue describing what you know.

2. **Wariga bobot tables** — If you have access to Ariana & Budayoga (2016)
   *Ala Ayuning Dewasa Ketut Bangbang Gde Rawi*, the weight values for each
   wewaran/wuku/sasih element would directly enable the Sugeno engine (v0.3.0).

3. **Validation data from other years** — If you have a printed Balinese calendar
   from any year, spot-checking our output helps everyone.

4. **Aksara Bali** — Unicode Balinese script output for all calendar terms.

5. **Kawi expertise** — The Pedoman Ala Ayuning Dewasa section contains 210
   day-specific entries in classical Kawi that need expert review.

6. **PHDI Nampih Sasih** — Annual intercalary month placements need verification.

See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup and PR guidelines.
