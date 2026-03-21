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
      - Cross-validated against kalenderbali.org (I Ketut Suwintana, Universitas Udayana)
      - 365/365 day-of-week matches · 30/30 Wuku · 12/12 Sasih · zero mismatches

### Remaining before tag
- [ ] **A2: Paringkelan spot-checks** — assert Watek (Madya & Alit) and Lintang
      output against ~30 dates from 2026 corpus. Cross-validate names against
      edysantosa/sakacalendar (LGPL-2.1) tables:
      - Watek Alit (4): Uler, Gajah, Lembu, Lintah
      - Watek Madya (5): Gajah, Watu, Buta, Suku, Wong
      - Lintang (35): Gajah through Begoong
      Source: I.B. Putra Manik Aryana, *Dasar Wariga* + *Tenung Wariga*;
      I.B. Supartha Ardana, *Pokok-Pokok Wariga* (2005).
- [ ] **A3: Pararasan validation** — uncomment assertions in `validation_2026_test.rs`.
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
      Expose both via `PancaSuda::name()` (Aryana default) and
      `PancaSuda::name_sundari_bungkah()` (Bidja variant), with doc comments
      citing the manuscript source for each.
- [ ] **A4: Gebogan Urip Tri-Pramana validation** — compare 210-entry table against
      crate's standard urip. Key finding: Tri-Pramana = f(Wuku, SaptaWara) only,
      values 12–29, incorporating SadWara. This is NOT SaptaWara.urip + PancaWara.urip.
      Flag outlier: Pahang + Soma = 29 (max value) — verify against physical source.
- [ ] **Perf: pawukon_day() dedup** — compute once in `from_jdn_unchecked`, pass to
      all subsystem constructors (currently recomputed ~15× per date construction)
- [ ] **Safety: date validation** — use `NaiveDate::from_ymd_opt` in `gregorian_to_jdn()`
      to reject impossible dates like Feb 30

---

## v0.1.3 — Infrastructure

### `serde` feature flag
- [ ] Derive `Serialize` / `Deserialize` on all public types behind `serde` feature
- [ ] Include `serde` and `serde_json` as optional dev-dependencies
- [ ] Enables JSON output for Dedauh API, GARUDA pipelines, frontend bridges

### WASM target
- [ ] `wasm32-unknown-unknown` support via `wasm-bindgen`
- [ ] JS interop layer: `from_ymd()`, `today()`, rahinan list, formatted string
- [ ] Depends on: `serde` feature (for JSON bridge to JS)
- [ ] Enables client-side Balinese calendar in Dedauh web frontend

### Astronomical sunrise
- [ ] Implement `DayBoundary::Astronomical` using the `sunrise` crate
      - Bali centroid default: lat -8.3405, lon 115.0920
      - Accept custom coordinates for non-Bali Hindu communities
      - Test against known sunrise times from BMKG
- [ ] Expose `DayBoundary` in WASM bindings

---

## v0.2.0 — Wariga Computation Layer

This release adds the traditional Wariga computation systems extracted from
I Made Bidja's 2026 calendar (Wariga Sundari Bungkah manuscript tradition)
and cross-validated against edysantosa/sakacalendar (Aryana manuscript tradition)
and peradnya/balinese-date-java-lib.

### Wariga BELOG Harmonisation (personalized day quality) — NEW
Fully extracted from JSI/STIKOM 2022. Pure modular arithmetic, no fuzzy logic.
This is the simplest Dewasa Ayu feature: personalized to birth date, computable today.

- [ ] New type: `WarigaBelog` enum — `Pati`, `Guru`, `Ratu`, `Lara`
- [ ] Algorithm: `(birth_urip + daily_urip) % 4` where urip = sapta + panca
      ```
      0 = Pati   — danger, avoid major activities
      1 = Guru   — wisdom, good for learning/spiritual practice
      2 = Ratu   — authority, good for leadership/official matters
      3 = Lara   — suffering, avoid important undertakings
      ```
- [ ] API: `wariga_belog(birth: &BalineseDate, query: &BalineseDate) -> WarigaBelog`
- [ ] Source: Wariga BELOG manuscript (Gianyar tradition), via T.I.P. Nyoman (2014)
      *Guide Book Buku Pedoman Wariga Belog*, Koleksi Griya Cebaang Giri Kesuma.

### Gebogan Urip Tri-Pramana (public API)
The Tri-Pramana system assigns a composite urip value and fourfold quality
classification to each of the 210 Wuku-day positions.

- [ ] New type: `TriPramana { urip: u8, quality: PramanaQuality }`
- [ ] `PramanaQuality` enum with 4 variants:
      - `LungguhSakti` — auspicious for crafting, practical work
      - `UtamaAsih` — excellent for all good works
      - `PugeranBakti` — favourable for worship, devotion
      - `MuktiPapa` — inauspicious, risk of danger
- [ ] Embed 210-entry lookup from `tests/fixtures/gebogan_urip_tri_pramana.json`
- [ ] API: `BalineseDate::tri_pramana() -> TriPramana`
- [ ] Source: Wariga Sundari Bungkah via I Made Bidja (complete table extracted)
- [ ] Document clearly: this differs from standard `sapta_wara.urip() + panca_wara.urip()`

### Pawiwahan (marriage compatibility)
The single most-consulted Wariga table in Balinese culture.

- [ ] `pawiwahan_compatibility(a: &BalineseDate, b: &BalineseDate) -> PawiwahanResult`
- [ ] `PawiwahanResult { combined_urip: u8, remainder: u8, quality: PawiwahanQuality }`
- [ ] 16-point quality scale from Wariga Sundari Bungkah:
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
- [ ] Full 30×7 base lookup table already extracted from OCR
- [ ] Cross-validate against einvite.id and kalenderbali.info

### Dauh Sukaranti (time-slot quality)
Traditional system for best time of day, based on combined urip.

- [ ] `dauh_sukaranti(urip: u8) -> [DauhQuality; 5]`
- [ ] 5 time periods: Dauh I (05:30–07:55), II (07:55–10:25), III (10:20–12:45),
      IV (12:45–15:10), V (15:10–17:30) WITA
- [ ] Quality values: Kelara · Pali · Sume · Krta · Peta
- [ ] Complete 12×5 lookup table extracted from OCR
- [ ] Source: Wariga Sundari Bungkah via I Made Bidja

### Tenung Patemuan Adan (name compatibility)
- [ ] `name_compatibility(a: &str, b: &str) -> PatemuanResult`
- [ ] Letter → urip mapping via directional chart (18 consonant groups)
- [ ] Source: Lontar Joyoboyo

### Otonan calculator
The otonan (Balinese birthday) falls every 210 days. Second most-requested
feature after Dewasa Ayu.

- [ ] `otonan_dates(birth: NaiveDate, count: usize) -> Vec<NaiveDate>`
- [ ] `next_otonan(birth: NaiveDate) -> NaiveDate`
- [ ] `next_otonan_from(birth: NaiveDate, after: NaiveDate) -> NaiveDate`

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
- [ ] Add `tests/fixtures/candana_2021_dewasa.json` with 77 prediction dates:
      - 16 expert (Pakar Wariga) days with scores 70–80
      - 13 Sugeno predictions with scores 70–76 (12 TP, 1 FP)
      - 27 Tsukamoto predictions (1 TP, 26 FP)
      - 21 Mamdani predictions (1 TP, 20 FP)
- [ ] Add wewaran cross-reference for all 16 expert dates (already computed):
      Saptawara distribution: Buddha 5, Wraspati 4, Sukra 4, Soma 2, Anggara 1
      Pancawara distribution: Pon 6, Kliwon 4, Paing 3, Wage 2, Umanis 1
      Score-80 days: exclusively Buddha or Sukra
      Expert NEVER selects Redite or Saniscara (despite Saniscara having highest urip)
- [ ] New trait: `DewasaAyu` with method `score(&self) -> f64` (0.0–1.0)
- [ ] Threshold: `is_dewasa_ayu(&self) -> bool` where score > configurable threshold
- [ ] Test: reproduce Sugeno's 12 TP matches against expert ground truth

#### Phase 2: Five-variable Sugeno inference engine
- [ ] Implement zero-order Sugeno fuzzy inference (constant consequents):
      ```rust
      struct SugenoEngine {
          rules: Vec<SugenoRule>,
      }
      struct SugenoRule {
          // Antecedents: membership degrees for 5 variables
          wewaran_set: FuzzySet,
          wuku_set: FuzzySet,
          penanggal_set: FuzzySet,
          sasih_set: FuzzySet,
          ala_ayu_set: FuzzySet,
          // Consequent: constant output value
          output: f64,
      }
      // Defuzzification: weighted average of fired rules
      fn infer(&self, input: &DewasaInput) -> f64 {
          let fired: Vec<(f64, f64)> = self.rules.iter()
              .map(|r| (r.firing_strength(input), r.output))
              .filter(|(strength, _)| *strength > 0.0)
              .collect();
          fired.iter().map(|(w,z)| w*z).sum::<f64>()
              / fired.iter().map(|(w,_)| *w).sum::<f64>()
      }
      ```
- [ ] Membership functions: triangular/trapezoidal for each variable
      - 5 linguistic values: SBr (Sangat Buruk), Br (Buruk), S (Sedang),
        B (Baik), SB (Sangat Baik)
      - Breakpoints TBD: derive from Ariana & Budayoga (2016) *Ala Ayuning
        Dewasa Ketut Bangbang Gde Rawi* bobot tables, or reverse-engineer
        from kalenderbali.info output by querying all 210 Pawukon days
- [ ] Feature-gate behind `#[cfg(feature = "dewasa-ayu")]` (pulls in `f64` ops)

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
- [ ] Suwintana (2015). *Lontar Komputer* 5(1), 392–403.
      → Full Mamdani rule base (useful for cross-validation even though we use Sugeno)
- [ ] Pasek Swastika (2015). *Wariga Padewasan*. Denpasar: CV. Kayumas Agung.
      → Additional Dewasa Ayu classification rules

---

## v0.4.0 — Completeness & Depth

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
- [ ] `Ingkel::ecological_domain() -> &'static str`
- [ ] Wong (human affairs), Sato (animals), Mina (fish/maritime), Manuk (birds),
      Taru (trees/forestry), Buku (bamboo/reeds)

### Candra Praleka (observational Sasih verification)
- [ ] `candra_praleka(sasih: Sasih) -> CandraPosition`
- [ ] 12 stellar diagrams (Pleiades/Orion positions) extracted from OCR
- [ ] Connects to `astronomical` feature flag

### Multi-year Sasih transition table
- [ ] Pre-compute Sasih transitions 2020–2035 for O(1) lookup
- [ ] Must account for Nampih Sasih (PHDI overrides need annual verification)

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
- **kalenderbali.org** — I Ketut Suwintana (Universitas Udayana)
- **dictionary.basabali.org** — BASAbali Wiki
- **kebudayaan.kemdikbud.go.id/bpnbbali** — BPNB Bali
- **babadbali.com** — Yayasan Bali Galang

### Algorithm reference
- edysantosa/sakacalendar (LGPL-2.1) — Java, complete paringkelan tables
- peradnya/balinese-date-java-lib (Apache-2.0) — Java/JS, Pawukon + Sasih
- Candana et al. (2021) JIK 6(2) — Sugeno vs Mamdani comparison, 16-date ground truth
- Suwintana (2014/2015) — Mamdani fuzzy Dewasa Pawiwahan
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
