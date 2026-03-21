# TODO — balinese-calendar

> The first native Rust implementation of the Balinese Saka Calendar.
> Tracking what's done, what's next, and what the community can help with.

---

## v0.1.2 (stabilise core)

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
- [ ] Paringkelan spot-checks: assert Watek (Madya & Alit) and Lintang output against
      ~30 dates from 2026 corpus — these subsystems currently have the least test coverage
- [ ] Pararasan validation: uncomment pararasan assertions in `validation_2026_test.rs`,
      verify enum variants match the canonical 7 names from Wariga Sundari Bungkah:
      Bumi Kapetak · Sumer Sinuhe · Satria Wirang · Wisesa Segara ·
      Tunggak Semi · Satria Wibawa · Lelu Kalung Angis
- [ ] Gebogan Urip Tri-Pramana: compare the 210-entry table against the crate's urip
      computation — key finding: Tri-Pramana uses Wuku + Sapta Wara only (values 12–29),
      which is a different formula from the standard Sapta + Panca Wara urip. Clarify
      which system the crate exposes and document the distinction.
      Flag: Pahang + Soma = 29 is the max value — verify against physical source.
- [ ] Compute `pawukon_day()` once in `from_jdn_unchecked` and pass to all subsystem
      constructors (currently recomputed ~15 times per date construction)
- [ ] Validate dates via `NaiveDate::from_ymd_opt` in `gregorian_to_jdn()` — current
      check accepts impossible dates like Feb 30

---

## v0.1.3 (infrastructure)

### `serde` feature flag
- [ ] Derive `Serialize` / `Deserialize` on all public types behind `serde` feature
- [ ] Include `serde` and `serde_json` as optional dev-dependencies
- [ ] Enables JSON output for web APIs, data pipelines, and frontend bridges

### WASM target
- [ ] `wasm32-unknown-unknown` support via `wasm-bindgen`
- [ ] JS interop layer: `from_ymd()`, `today()`, rahinan list, formatted string
- [ ] Depends on: `serde` feature (for JSON bridge to JS)
- [ ] Enables client-side Balinese calendar in any web application

### Astronomical sunrise
- [ ] Implement `DayBoundary::Astronomical` using the `sunrise` crate
      - Bali centroid default: lat -8.3405, lon 115.0920
      - Accept custom coordinates for non-Bali Hindu communities
      - Test against known sunrise times from BMKG
- [ ] Expose `DayBoundary` in WASM bindings

---

## v0.2.0 (Wariga expansion)

The OCR extraction of I Made Bidja's 2026 calendar revealed several traditional Wariga
computation systems that are standard in every printed Balinese calendar but not yet
implemented in this crate. These are not obscure — they're used daily by millions of
Balinese Hindus and are documented in the Wariga Sundari Bungkah manuscript tradition.

### Gebogan Urip Tri-Pramana (public API)
The Tri-Pramana system assigns a composite urip value and a fourfold quality
classification to each of the 210 Wuku-day positions. Unlike the standard urip
(Sapta Wara + Panca Wara), Tri-Pramana is a function of Wuku × Sapta Wara only,
incorporating the Sad Wara component. Its values range from 12 to 29 and carry
explicit quality meanings used in birth reading, ceremony timing, and daily guidance.

- [ ] New type: `TriPramana { urip: u8, quality: PramanaQuality }`
- [ ] `PramanaQuality` enum with 4 variants:
      - `LungguhSakti` — auspicious for crafting tools, weapons, practical work
      - `UtamaAsih` — excellent for all good works and positive undertakings
      - `PugeranBakti` — favourable for worship, devotion, and spiritual practice
      - `MuktiPapa` — inauspicious, risk of danger; avoid major undertakings
- [ ] Embed 210-entry lookup table (30 Wuku × 7 Sapta Wara → urip + quality)
- [ ] Expose via `BalineseDate::tri_pramana() -> TriPramana`
- [ ] Source: Wariga Sundari Bungkah (complete table extracted and validated)
- [ ] Document clearly how this differs from standard `sapta_wara.urip() + panca_wara.urip()`

### Marriage compatibility (Pawiwahan)
Marriage compatibility (Pawiwahan) is the single most-consulted Wariga table in
Balinese culture. Every family checks this before a wedding. The algorithm sums
the Tri-Pramana urip of both partners based on their birth Wuku and Sapta Wara,
then reduces mod 16 to yield a quality verdict on a 16-point scale.

- [ ] `pawiwahan_compatibility(a: &BalineseDate, b: &BalineseDate) -> PawiwahanResult`
- [ ] `PawiwahanResult { combined_urip: u8, remainder: u8, quality: PawiwahanQuality, description: &'static str }`
- [ ] 16-point quality scale from Wariga Sundari Bungkah:
      ```
       1 = Madya (Suka-Duka) — mixed fortune
       2 = Kawon (Lara, Miskin) — hardship, poverty
       3 = Kawon (Lara, Warang) — strife, frequent quarrels
       4 = Kawon (Panake Mati) — danger to children
       5 = Becik Pisan (Sudha Nulus) — excellent, harmonious in all things
       6 = Kawon (Sengsara) — suffering, frequent illness
       7 = Madya (Suka-Duka) — mixed fortune
       8 = Kawon (Lara, Kenapali) — persistent hardship
       9 = Kawon Pisan (Baya Pati) — worst, risk of death
      10 = Becik (Bikiga Ratuna) — good, influential, prosperous
      11 = Becik (Kapardyaniyah) — good, influential livelihood
      12 = Becik (Kedrping Hari) — good, harmonious
      13 = Becik (Tan Kirang) — wealthy, abundant
      14 = Kawon (Tan Polih Keselamatan) — persistent misfortune
      15 = Becik (Bokung) — good but childless
      16 = Becik (Nyama Braya Asih) — beloved by family and community
      ```
- [ ] Full 30×7 base lookup table extracted and validated from printed calendar
- [ ] Reference implementations exist at einvite.id and kalenderbali.info

### Dauh Sukaranti (time-slot quality)
Traditional system for determining the best time of day to undertake important
activities, based on the combined urip of Sapta Wara + Panca Wara.

- [ ] `dauh_sukaranti(urip: u8) -> [DauhQuality; 5]`
- [ ] Maps combined urip (7–18) to 5 time periods across the Balinese working day:
      ```
      Dauh I   — 05:30–07:55 WITA
      Dauh II  — 07:55–10:25 WITA
      Dauh III — 10:20–12:45 WITA
      Dauh IV  — 12:45–15:10 WITA
      Dauh V   — 15:10–17:30 WITA
      ```
- [ ] Quality values: `Kelara` · `Pali` · `Sume` · `Krta` · `Peta`
- [ ] Complete 12×5 lookup table extracted from printed calendar
- [ ] Source: Wariga Sundari Bungkah via I Made Bidja

### Name compatibility (Tenung Patemuan Adan)
Complementary to Pawiwahan (which uses birth date), this system checks couple
compatibility by the first letter of each partner's name. Each letter maps to a
directional urip value; the sum mod 7 yields a verdict.

- [ ] `name_compatibility(name_a: &str, name_b: &str) -> PatemuanResult`
- [ ] Letter → urip mapping via directional chart:
      MA=6, GA=1, BA=1, TA=3, NGA=1, NYA=8, NA=4, YA=10,
      CA=3, JA=3, DA=1, KA=3, LA=5, WA=7, SA=2, TA=3, DA=4
- [ ] Source: Lontar Joyoboyo

### Otonan calculator
The otonan (Balinese birthday ceremony) falls every 210 days from birth. This is
the second most-requested feature after Dewasa Ayu — parents and families need to
know upcoming otonan dates.

- [ ] `otonan_dates(birth_date: NaiveDate, count: usize) -> Vec<NaiveDate>`
- [ ] `next_otonan(birth_date: NaiveDate) -> NaiveDate`
- [ ] `next_otonan_from(birth_date: NaiveDate, after: NaiveDate) -> NaiveDate`
- [ ] Simple 210-day cycle addition, but the convenience API matters for end users

### Dewasa Ayu (auspicious day classification)
This is the highest-impact feature for end users — "is today a good day for X?"
is the primary reason people consult a Balinese calendar. The system classifies
each day by its suitability for specific categories of activity.

- [ ] `BalineseDate::dewasa_ayu() -> Vec<DewasaAyu>` — applicable day classifications
- [ ] Categories from traditional printed calendars:
      - Menggunakan / Purung Karma (general auspicious activities)
      - Pembersihan (purification / cleansing ceremonies)
      - Dewa Naqa / Dewa Yadnya (worship, temple ceremonies)
      - Pura Yatma (temple visits, pilgrimages)
      - Kerja / Pembangunan (work, construction projects)
      - Muat Bangunan (building construction starts)
      - Memasang Pintu (door/gate installation)
      - Pertanian (agriculture: planting, harvesting)
      - Metatah / Potong Gigi (tooth-filing ceremony)
      - Pawiwahan (wedding ceremony timing)
      - Ngaben (cremation ceremony timing)
      - Pemberangkatan (travel, departure)
- [ ] Implementation approach TBD:
      - Option A: encode compound Wewaran rules from Wariga manuscripts (general, complex)
      - Option B: static lookup tables from published calendars (simpler, annual update)
      - Option C: community-contributed rule definitions via builder pattern
- [ ] Reference: kalenderbali.org uses fuzzy-logic inference (I Ketut Suwintana);
      printed calendars list dates as flat sequences per category
- [ ] **Community help wanted:** this is the most valuable contribution anyone can
      make to this crate — see Contributing section below

---

## v0.3.0 (completeness & depth)

### Pedoman Ala Ayuning Dewasa
Every printed Balinese calendar includes a right-column section with 210 day-specific
guidance entries in Kawi (Old Javanese). Each entry lists applicable Kala (inauspicious
forces), Padewasaan (day qualities), deity associations, and activity recommendations.

- [ ] `BalineseDate::ala_ayuning_dewasa() -> AlaAyuningDewasa`
- [ ] Struct containing: Kala list, positive qualities, deity associations
- [ ] Challenge: source text is in classical Kawi — automated extraction is unreliable;
      requires expert transliteration and translation
- [ ] The 2026 OCR corpus includes all 210 entries but they are the most error-prone
      section due to the archaic script and dense abbreviations
- [ ] **Community help wanted:** Kawi specialists and Wariga practitioners

### Extended Rahinan
The crate detects major ceremonies but several commonly observed ones are missing:

- [ ] Buda Cemeng — Buda Kliwon of each Wuku (30 occurrences per Pawukon cycle):
      Buda Cemeng Sinta, Buda Cemeng Ukir, Buda Cemeng Merakih, etc.
- [ ] Anggara Kasih — Anggara Kliwon of each Wuku (30 occurrences):
      Anggara Kasih Kulantir, Anggara Kasih Tambir, Anggara Kasih Julungwangi, etc.
- [ ] Post-Saraswati sequence: Banyupinaruh → Soma Ribek → Sabuh Mas → Pagerwesi
      (4 consecutive days following Saraswati, each with distinct observances)
- [ ] Pre-Galungan sequence: Sugihan Jawa → Sugihan Bali → Penyajaan → Penampahan
      (the days leading up to Galungan, each with specific preparations)
- [ ] Post-Galungan sequence: Umanis/Paing/Pon/Wage/Kliwon Galungan → Kuningan
      (10-day period with daily observance names)
- [ ] Purnama / Tilem per-Sasih names:
      Purnama Sasih Kasa, Purnama Sasih Karo, etc. — each carries Sasih-specific
      ceremony associations (e.g., Purnama Kadasa = Besakih major ceremony)

### Sasih-specific ceremonies (Piodalan Sad Kahyangan)
Major temple ceremonies are tied to specific Sasih moments. A complete table was
extracted from the OCR supplementary pages covering the six great temples (Sad
Kahyangan), Dhang Kahyangan, and major temples across all Bali regencies plus
Lombok and East Java.

- [ ] `ceremonies_for_sasih(sasih: Sasih) -> Vec<SasihCeremony>`
- [ ] Data source: supplement_5 (piodalan listing, fully extracted)
- [ ] Notable entries:
      - Purnama Kadasa: Besakih (Bhatara Turun Kabeh), Lempuyang, Batur
      - Purnama Karo: Aci Pangenteg Jagat di Pr. Gelap Besakih
      - Purnama Kalima: Ngusaba Siram di Pr. Batu Madeg Besakih
      - Specific Wuku-triggered piodalan (e.g., Tumpek Wayang = Pr. Alas Purwo)

### Ingkel ecology metadata
- [ ] Expand Ingkel from bare name to include traditional ecological associations:
      ```
      Wong   — human affairs, social activities
      Sato   — animals, livestock care
      Mina   — fish, water creatures, maritime activities
      Manuk  — birds, poultry
      Taru   — trees, wood, forestry, plant cultivation
      Buku   — bamboo, reeds, node-plants
      ```
- [ ] Add `Ingkel::ecological_domain() -> &'static str`
- [ ] Relevant for agricultural and ecological applications

### Candra Praleka (observational Sasih verification)
The traditional astronomical method for determining Sasih by observing stellar positions.
Complete 12-diagram dataset extracted from OCR source.

- [ ] `candra_praleka(sasih: Sasih) -> CandraPosition`
- [ ] Describes relative positions of:
      - Bintang Kartika (Pleiades) — angle from zenith, cardinal direction
      - Bintang Wuluku (Orion's Belt) — rise time, position
- [ ] Use case: validate computational Sasih against observational data;
      educational/cultural reference
- [ ] Connects to the `astronomical` feature flag

### Multi-year Sasih transition table
- [ ] Pre-compute Sasih transition dates for a range of years (e.g., 2020–2035)
- [ ] Enables O(1) lookup without walk-forward computation
- [ ] Must account for Nampih Sasih — PHDI overrides need annual verification

### Historical date support
- [ ] Support dates before the current epoch for research and inscription dating
- [ ] Useful for scholars working with lontar manuscripts and prasasti (stone inscriptions)

---

## Backlog

### Maintenance
- [ ] PHDI Nampih Sasih automation: structured data file or scraping approach
- [ ] Annual validation: generate new corpus from each year's printed calendar

### Locale & script
- [ ] Aksara Bali (Unicode Balinese script) output for all names
- [ ] Indonesian language strings alongside English
- [ ] Important for cultural authenticity — Latin transliteration loses tonal/register
      information present in the original script

### Platform targets
- [ ] `no_std` support — remove `chrono` for `from_jdn`/`to_jdn` paths; keep behind
      `std` feature flag. Enables embedded use (IoT, temple display boards).
- [ ] C FFI bindings via `cbindgen` — interop with existing calendar software
- [ ] Python bindings via `pyo3`/`maturin` (`balinese-calendar-py`) — data science,
      cultural research, web backend integration
- [ ] Swift/Kotlin wrappers — mobile app development

### Data quality
- [ ] Add validation corpora from additional years and additional calendar publishers
      to reduce single-source bias
- [ ] Cross-validate against peradnya/balinese-calendar-js-lib for multi-year ranges
- [ ] Identify and document cases where different calendar authorities disagree
      (these exist — e.g., Ngunaratri boundary interpretation varies between sources)

---

## Validation sources

### Primary (2026 corpus)
- **I Made Bidja Alm.** / I Md Agus Putra Wijaya — *Kalender Bali 2026*
  Published by Ikatan Bidan Indonesia Cabang Kabupaten Badung.
  Compiled from 50+ lontar Wariga manuscripts and 13 Kawi/Sanskrit/Balinese dictionaries.
  Full bibliography in `tests/fixtures/` (extracted from source).

### Cross-validation
- **kalenderbali.org** — I Ketut Suwintana (Universitas Udayana)
- **dictionary.basabali.org** — BASAbali Wiki (Balinese dictionary)
- **kebudayaan.kemdikbud.go.id/bpnbbali** — Balai Pelestarian Nilai Budaya Bali
- **babadbali.com** — Yayasan Bali Galang

### Algorithm reference
- Ardhana, I.B.S. (2005). *Pokok-Pokok Wariga*. Surabaya: Paramita.
- Pendit, N.S. (2001). *Nyepi: kebangkitan, toleransi, dan kerukunan*. Gramedia.
- [peradnya/balinese-calendar-js-lib](https://github.com/peradnya/balinese-calendar-js-lib) (Apache-2.0)

### Key lontar sources (via I Made Bidja bibliography)
Manuscripts underpinning the computation systems in this crate:

| Manuscript | Systems derived |
|---|---|
| Wariga Sundari Bungkah | Gebogan Urip Tri-Pramana, Pawiwahan, Pararasan, Dauh Sukaranti |
| Wariga Gemet | Ala-Ayuning Dewasa (day quality classification) |
| Lontar Joyoboyo | Tenung Patemuan Adan (name compatibility) |
| Wariga Candra Praleka | Stellar observation method for Sasih determination |
| Wariga Pawukon | 30-Wuku cycle, Bhatara associations, ecology tags |
| Wariga Pratiti Samutpada | Dependent origination calendar correlations |
| Wariga Pareresian | Pararasan birth character system |
| Wariga Pabintangan | Lintang (asterism) assignments |

Plus ~42 additional Wariga manuscripts (see `tests/fixtures/bibliography.md` for the full list).

---

## Contributing

Contributions are especially welcome for:

1. **Dewasa Ayu rules** — If you understand the compound Wewaran conditions that
   determine good/bad days for specific activities, this is the single most impactful
   contribution. The date sequences are extracted; what's needed is the *rules* that
   generate them. Open an issue describing what you know, even partially.

2. **Validation data from other years** — If you have a printed Balinese calendar
   from any year, spot-checking our output against it helps everyone. File issues for
   any mismatches you find, however minor.

3. **Aksara Bali** — Unicode Balinese script output for all calendar terms. Requires
   knowledge of Balinese orthography.

4. **Kawi expertise** — The Pedoman Ala Ayuning Dewasa section contains 210
   day-specific entries in classical Kawi that need expert review and translation.

5. **PHDI Nampih Sasih** — Annual intercalary month placements need verification.
   If you have access to official PHDI declarations, help us keep the table current.

6. **Ecological knowledge** — The Ingkel cycle connects to Bali's traditional ecological
   calendar. If you know the agricultural/ecological associations of specific Wuku or
   Ingkel periods, this knowledge is valuable for the crate's ecology metadata.

See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup and PR guidelines.
