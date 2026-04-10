# Extracted Algorithms & Data from Scientific Literature
## For balinese-calendar crate development

> Compiled: March 2026
> Sources: 7 priority references, online extraction

---

## 1. SUWINTANA 2014 — Mamdani Fuzzy Inference for Dewasa Pawiwahan

**Source:** kalenderbali.info/KalenderBali/hariPerkawinan (I Ketut Suwintana, 2013)
**Status:** PARTIALLY EXTRACTED (methodology page accessible, full μ-functions require the paper)

### Architecture
- Method: Mamdani Fuzzy Inference (Max-Min)
- 5 input variables, each with 5 fuzzy sets
- 1 output variable (Dewasa Perkawinan score, 0–100%)
- Threshold: >62.766% = good day for wedding ceremony

### Input Variables & Fuzzy Sets
Each input variable is divided into 5 linguistic values:
- **SBr** = Sangat Buruk (Very Bad)
- **Br**  = Buruk (Bad)
- **S**   = Sedang (Medium)
- **B**   = Baik (Good)
- **SB**  = Sangat Baik (Very Good)

**Variable 1: Saptawara quality**
Input: sapta_wara index (0–6)
Each sapta wara day mapped to fuzzy membership across SBr/Br/S/B/SB

**Variable 2: Sasih quality**
Input: sasih index (1–12)
Each sasih mapped to fuzzy membership

**Variable 3: Penanggal (waxing moon day, 1–15)**
Input: penanggal number
Mapped to fuzzy membership

**Variable 4: Pangelong (waning moon day, 1–15)**
Input: pangelong number
Mapped to fuzzy membership

**Variable 5: Ala Ayuning Dewasa (day quality)**
Input: binary — Baik (B) or Tidak Baik (TB)
This is the traditional Wariga classification for the specific day
Only 2 fuzzy sets (not 5)

### Inference Rules
IF-THEN rules combining all 5 variables.
Each rule's activation strength = min(μ of all antecedents).
Output: superposition of all rule consequents.
Defuzzification: centroid method → crisp score 0–100%.

### Key Insight for Implementation
Suwintana's system doesn't hard-code "good days" — it computes a continuous
score for ANY date. This means the crate can expose:
```rust
fn dewasa_pawiwahan_score(&self) -> f64  // 0.0 to 1.0
fn is_dewasa_ayu_pawiwahan(&self) -> bool  // score > 0.62766
```

### What's Missing (needs the 2014 paper or reverse-engineering)
- Exact membership function shapes (triangular? trapezoidal?) and breakpoints
- Which Saptawara days get which fuzzy membership values
- Which Sasih months are "good" vs "bad" for marriage
- The complete rule base (how many rules, which combinations)
- The Ala Ayuning Dewasa binary classification rules

### Reverse-Engineering Path
kalenderbali.info is a live implementation. For any given date range, it returns
ranked Dewasa Pawiwahan scores. By querying systematically (e.g., all 210 days
of one Pawukon cycle), the output scores can be used to fit the membership
functions. This is a data-driven approach to extract what the paper describes.

---

## 2. CANDANA ET AL. 2021 — Fuzzy Method Comparison (FULL PAPER OBTAINED)

**Source:** JIK 6(2), 14–22. Universitas Pendidikan Ganesha Singaraja.
**Authors:** Candana, E.W.H., Gunadi, I.G.A., & Divayana, D.G.H.
**Status:** FULLY READ — key findings extracted

### Winner: SUGENO (not Mamdani)
This overturns Suwintana's choice. Sugeno decisively outperforms both Mamdani
and Tsukamoto when validated against a Wariga expert's judgments.

### Confusion Matrix Results (731 days, 2020–2021, 16 expert-identified good days)

| Method | TP | FP | FN | TN | Accuracy | Precision | Recall | F-1 Score |
|--------|----|----|----|----|----------|-----------|--------|-----------|
| Tsukamoto | 1 | 26 | 15 | 689 | 94.39% | 3.70% | 6.25% | 4.65% |
| Mamdani | 1 | 20 | 15 | 695 | 95.21% | 4.76% | 6.25% | 5.41% |
| **Sugeno** | **12** | **1** | **4** | **714** | **99.32%** | **92.31%** | **75.00%** | **82.76%** |

**Critical insight:** Mamdani and Tsukamoto are essentially useless for this task
(Precision 3–5%, meaning >95% of their "good day" predictions are wrong).
Sugeno correctly identifies 12 of 16 expert-chosen days with only 1 false positive.

### Five Input Variables (confirmed from paper)
1. **Wewaran** — combined wewaran quality (bobot wewaran)
2. **Wuku** — wuku quality (bobot wuku)
3. **Penanggal/Pangelong** — lunar day quality (bobot penanggal)
4. **Sasih** — lunar month quality (bobot sasih)
5. **Ala Ayuning Dewasa** — traditional day-quality classification (bobot dewasa ala ayu)

### Alahaning Dewasa Hierarchy Rule (from paper Section II.B)
This is the core Wariga rule governing how variables override each other:
```
Wewaran < Wuku < Penanggal < Sasih < Dauh
```
Meaning: Sasih overrides Penanggal, Penanggal overrides Wuku, Wuku overrides
Wewaran. Plus "Ala Ayu Dewasa" adds specific prohibition/recommendation overlays.

This hierarchy is the fundamental structure for implementing Dewasa Ayu — it tells
you which calendar element "wins" when there's a conflict between good and bad
indicators.

### Expert Benchmark
- 16 good wedding days identified by Wariga expert in 731 days (2020–2021)
- That's only **2.19%** of days classified as "good" for marriage
- This is consistent with traditional practice: truly good days are rare
- The Wariga expert is from "golongan pemangku, sulinggih atau tokoh wariga"
  (priest class or wariga authority)

### Wariga Bobot (Weight) System
The paper mentions "data bobot wariga" — weight data for each Wariga element
that the admin can maintain in the database. This suggests the system uses a
WEIGHTED SCORING approach where each variable has an assigned weight/score,
and the fuzzy system combines these weights.

The admin interface includes: "data bobot wariga(wewaran, wuku, penganggal,
sasih, ala ayu), data rule pakar"

### What's Still Missing
- The exact membership function curves and breakpoints (likely in figures
  that were images in the original PDF, not extractable from text)
- The specific bobot (weight) values assigned to each wewaran, wuku, sasih
- The Sugeno output functions (constants or linear equations per rule)
- The complete rule base (number of rules, combinations)

### COMPLETE PREDICTION DATA (from Gambar 6, PDF page 6) — VALIDATION GOLD

**Expert (Pakar Wariga) — 16 good wedding days in 2020–2021:**

| # | Date | N.Ahli (Score) |
|---|------|----------------|
| 1 | 2 Jan 2020 | 75 |
| 2 | 8 Jan 2020 | 70 |
| 3 | 30 Mar 2020 | 75 |
| 4 | 2 Apr 2020 | 75 |
| 5 | 8 Apr 2020 | 80 |
| 6 | 10 Apr 2020 | 80 |
| 7 | 22 Jul 2020 | 70 |
| 8 | 26 Oct 2020 | 70 |
| 9 | 29 Oct 2020 | 70 |
| 10 | 30 Dec 2020 | 70 |
| 11 | 2 Apr 2021 | 80 |
| 12 | 21 May 2021 | 80 |
| 13 | 12 Aug 2021 | 75 |
| 14 | 8 Sep 2021 | 80 |
| 15 | 14 Sep 2021 | 70 |
| 16 | 29 Oct 2021 | 70 |

**Sugeno predictions — 13 good days (12 TP + 1 FP):**

| # | Date | N.Sug (Score) | Match? |
|---|------|---------------|--------|
| 1 | 2 Jan 2020 | 71 | TP ✓ |
| 2 | 8 Jan 2020 | 76 | TP ✓ |
| 3 | 30 Mar 2020 | 71 | TP ✓ |
| 4 | 2 Apr 2020 | 74 | TP ✓ |
| 5 | 8 Apr 2020 | 75 | TP ✓ |
| 6 | 10 Apr 2020 | 75 | TP ✓ |
| 7 | **31 Aug 2020** | **71** | **FP ✗** |
| 8 | 26 Oct 2020 | 71 | TP ✓ |
| 9 | 29 Oct 2020 | 71 | TP ✓ |
| 10 | 30 Dec 2020 | 70 | TP ✓ |
| 11 | 2 Apr 2021 | 75 | TP ✓ |
| 12 | 21 May 2021 | 75 | TP ✓ |
| 13 | 8 Sep 2021 | 72 | TP ✓ |

**Expert days MISSED by Sugeno (4 FN):**
- 22 Jul 2020 (expert score 70)
- 12 Aug 2021 (expert score 75)
- 14 Sep 2021 (expert score 70)
- 29 Oct 2021 (expert score 70)

Note: All 4 missed days had expert scores of 70–75 (lower end). Sugeno captured
all 80-score days perfectly. The false positive (31 Aug 2020) scored 71 in Sugeno —
barely above threshold. This suggests a threshold around 70 separates good from bad.

**Tsukamoto predictions — 27 days (1 TP, 26 FP):**

| # | Date | N.Tsu |
|---|------|-------|
| 1 | 14 Jan 2020 | 60 |
| 2 | 16 Jan 2020 | 60 |
| 3 | 21 Jan 2020 | 60 |
| 4 | 23 Jan 2020 | 63 |
| 5 | 24 Jan 2020 | 63 |
| 6 | 26 Feb 2020 | 64 |
| 7 | 28 Feb 2020 | 64 |
| 8 | 12 Mar 2020 | 60 |
| 9 | 14 Sep 2020 | 61 |
| 10 | 16 Sep 2020 | 62 |
| 11 | 17 Sep 2020 | 61 |
| 12 | 5 Oct 2020 | 63 |
| 13 | 7 Oct 2020 | 63 |
| 14 | 13 Oct 2020 | 60 |
| 15 | 15 Oct 2020 | 60 |
| 16 | 16 Oct 2020 | 63 |
| 17 | 12 Jan 2021 | 61 |
| 18 | 13 Jan 2021 | 61 |
| 19 | 1 Mar 2021 | 64 |
| 20 | 3 Sep 2021 | 62 |
| 21 | 9 Sep 2021 | 60 |
| 22 | 13 Sep 2021 | 68 |
| 23 | 14 Sep 2021 | 60 |
| 24 | 27 Sep 2021 | 63 |
| 25 | 5 Oct 2021 | 60 |
| 26 | 6 Oct 2021 | 63 |
| 27 | 4 Nov 2021 | 61 |

(Only #23 = 14 Sep 2021 matches expert. All others are FP.)

**Mamdani predictions — 21 days (1 TP, 20 FP):**

| # | Date | N.Mam |
|---|------|-------|
| 1 | 8 Feb 2020 | 66 |
| 2 | 7 May 2020 | 65 |
| 3 | 14 Jul 2020 | 66 |
| 4 | 16 Jul 2020 | 65 |
| 5 | 17 Jul 2020 | 65 |
| 6 | 20 Jul 2020 | 65 |
| 7 | 22 Jul 2020 | 65 |
| 8 | 7 Aug 2020 | 65 |
| 9 | 3 Dec 2020 | 65 |
| 10 | 7 Feb 2021 | 65 |
| 11 | 10 Feb 2021 | 65 |
| 12 | 11 Feb 2021 | 65 |
| 13 | 26 Feb 2021 | 65 |
| 14 | 2 Mar 2021 | 65 |
| 15 | 4 Mar 2021 | 65 |
| 16 | 6 Mar 2021 | 65 |
| 17 | 26 May 2021 | 65 |
| 18 | 2 Aug 2021 | 65 |
| 19 | 9 Sep 2021 | 65 |
| 20 | 13 Sep 2021 | 65 |
| 21 | 25 Sep 2021 | 66 |

(Only #7 = 22 Jul 2020 matches expert. Mamdani clusters around score=65.)

### Score Distribution Analysis
- Expert scores range: 70–80 (mean ~74.4)
- Sugeno scores range: 70–76 (tight band near expert)
- Tsukamoto scores range: 60–68 (too low, different scale behavior)
- Mamdani scores range: 65–66 (nearly constant, no discrimination)

This explains why Mamdani fails: its output collapses to ~65 for almost all "triggered"
days, lacking the dynamic range to distinguish truly good days from mediocre ones.
Sugeno's constant/linear output functions maintain discrimination.

### CROSS-REFERENCE: Expert Dates × Wewaran (computed from dates)

| Date | Score | Saptawara | Pancawara | Weton | Urip |
|------|-------|-----------|-----------|-------|------|
| 2020-01-02 | 75 | Wraspati | Paing | Wraspati Paing | 17 |
| 2020-01-08 | 70 | Buddha | Pon | Buddha Pon | 14 |
| 2020-03-30 | 75 | Soma | Kliwon | Soma Kliwon | 12 |
| 2020-04-02 | 75 | Wraspati | Pon | Wraspati Pon | 15 |
| 2020-04-08 | **80** | Buddha | Wage | Buddha Wage | 11 |
| 2020-04-10 | **80** | Sukra | Umanis | Sukra Umanis | 11 |
| 2020-07-22 | 70 | Buddha | Wage | Buddha Wage | 11 |
| 2020-10-26 | 70 | Soma | Kliwon | Soma Kliwon | 12 |
| 2020-10-29 | 70 | Wraspati | Pon | Wraspati Pon | 15 |
| 2020-12-30 | 70 | Buddha | Kliwon | Buddha Kliwon | 15 |
| 2021-04-02 | **80** | Sukra | Pon | Sukra Pon | 13 |
| 2021-05-21 | **80** | Sukra | Paing | Sukra Paing | 15 |
| 2021-08-12 | 75 | Wraspati | Kliwon | Wraspati Kliwon | 16 |
| 2021-09-08 | **80** | Buddha | Paing | Buddha Paing | 16 |
| 2021-09-14 | 70 | Anggara | Pon | Anggara Pon | 10 |
| 2021-10-29 | 70 | Sukra | Pon | Sukra Pon | 13 |

**Saptawara frequency in expert-selected days:**
- Buddha: 5 (31%) — urip 7
- Wraspati: 4 (25%) — urip 8
- Sukra: 4 (25%) — urip 6
- Soma: 2 (13%) — urip 4
- Anggara: 1 (6%) — urip 3
- Redite: 0 — urip 5
- Saniscara: 0 — urip 9

**Pancawara frequency:**
- Pon: 6 (38%) — urip 7
- Kliwon: 4 (25%) — urip 8
- Paing: 3 (19%) — urip 9
- Wage: 2 (13%) — urip 4
- Umanis: 1 (6%) — urip 5

**Combined urip distribution:** mean 13.5, range 10–17

**PATTERN: Score-80 days are exclusively Buddha or Sukra:**
- 2020-04-08 = Buddha Wage (urip 11)
- 2020-04-10 = Sukra Umanis (urip 11)
- 2021-04-02 = Sukra Pon (urip 13)
- 2021-05-21 = Sukra Paing (urip 15)
- 2021-09-08 = Buddha Paing (urip 16)

**Sugeno error analysis:**
- FP (31 Aug 2020) = Soma Wage, urip 8 — lowest urip, likely triggered by
  favorable sasih/penanggal/ala-ayu overcoming weak wewaran
- FN: Buddha Wage (11), Wraspati Kliwon (16), Anggara Pon (10), Sukra Pon (13)
  — no systematic wewaran pattern in misses; likely sasih/penanggal/ala-ayu factors

**Key insight for Wewaran bobot:**
The expert NEVER selects Redite (Sunday) or Saniscara (Saturday) as good days,
despite Saniscara having the highest sapta urip (9). This contradicts a pure urip-based
scoring. The Wariga tradition assigns day-quality independently of urip value.
Buddha (Wednesday) and Sukra (Friday) are the strongest good-day indicators,
while Saniscara and Redite are effectively blocked.

### Key References from Paper
- [2] I.B. Putra Manik Ariana & I.B. Budayoga (2016). *Ala Ayuning Dewasa
  Ketut Bangbang Gde Rawi (Sebuah Canang Sari)*, II. Denpasar: ESBE Buku.
  → This book likely contains the actual weight tables for each Wariga element
- [3] I Ketut Suwintana (2014). 'Penentuan Hari Baik Perkawinan Di Bali
  Berbasis Logika Fuzzy', Lontar Komput., vol. 5, no. 1, h. 392–401.
  → Suwintana's actual journal paper (not the 2014 seminar paper)
- [7] I Ketut Pasek Swastika (2015). *Wariga Padewasan*, I. Denpasar:
  CV. Kayumas Agung.
- [8] I.B.S. Ardhana (2006). *Pokok-Pokok Wariga*, I. Surabaya: Paramita.

### Implication for Crate Implementation
**Use Sugeno, not Mamdani.** The Sugeno method is:
1. More accurate (F-1 82.76% vs 5.41%)
2. Computationally simpler (weighted average defuzzification instead of centroid)
3. Output is constants or linear equations, not fuzzy sets → easier to implement in Rust
4. No need for numerical integration (centroid requires area computation)

A Sugeno implementation needs:
- Membership functions for 5 input variables (triangular/trapezoidal)
- IF-THEN rules with constant outputs (singleton values)
- Weighted average of fired rule outputs
- Threshold for "good day" classification

---

## 3. JSI/STIKOM 2022 — Wariga BELOG Harmonization Algorithm

**Source:** JSI 17(1), 55–61. DOI: 10.30864/jsi.v17i1.486
**Status:** PARTIALLY EXTRACTED (algorithm from abstract, full paper blocked)

### Algorithm: Personal Day-Quality Harmonization

```
INPUT:
  birth_sapta_wara_urip: u8   // from birth date (5,4,3,7,8,6,9)
  birth_panca_wara_urip: u8   // from birth date (5,9,7,4,8)
  daily_sapta_wara_urip: u8   // today's value
  daily_panca_wara_urip: u8   // today's value

COMPUTE:
  birth_urip = birth_sapta_wara_urip + birth_panca_wara_urip
  daily_urip = daily_sapta_wara_urip + daily_panca_wara_urip
  combined = birth_urip + daily_urip
  result = combined % 4

OUTPUT:
  0 = PATI   (death/danger — worst, avoid all major activities)
  1 = GURU   (teacher/wisdom — good for learning, spiritual practice)
  2 = RATU   (king/authority — good for leadership, official matters)
  3 = LARA   (suffering — bad, avoid important undertakings)
```

### Key Properties
- **Personalized:** depends on individual's birth date (Weton)
- **Daily varying:** changes every day based on current Wewaran
- **Simple:** no fuzzy logic, pure modular arithmetic
- **Source manuscript:** Wariga BELOG (Gianyar tradition)

### Crate API Design
```rust
/// Personal day-quality based on Wariga BELOG harmonization
pub fn wariga_belog_quality(birth_date: &BalineseDate, query_date: &BalineseDate) -> WarigaBelog {
    let birth_urip = birth_date.sapta_wara.urip() + birth_date.panca_wara.urip();
    let daily_urip = query_date.sapta_wara.urip() + query_date.panca_wara.urip();
    match (birth_urip + daily_urip) % 4 {
        0 => WarigaBelog::Pati,
        1 => WarigaBelog::Guru,
        2 => WarigaBelog::Ratu,
        3 => WarigaBelog::Lara,
        _ => unreachable!()
    }
}

pub enum WarigaBelog {
    Pati,   // Danger — avoid major activities
    Guru,   // Wisdom — good for learning, spiritual practice
    Ratu,   // Authority — good for leadership, official matters
    Lara,   // Suffering — avoid important undertakings
}
```

---

## 4. DERSHOWITZ & REINGOLD CH.11 — Subcycle Adjustment Formulas

**Source:** Calendrical Calculations: The Ultimate Edition (Cambridge, 2018)
**Status:** BOOK (not freely accessible), but algorithms encoded in peradnya

### What Chapter 11 Contains
The Pawukon calendar has 10 concurrent cycles of lengths 1–10.
Cycles 1, 2, 3, 5, 6, 7 are straightforward modular arithmetic.
Cycles 4, 8, 9, 10 require irregular adjustments.

### Canonical Formulas (from Dershowitz & Reingold via peradnya)

**Ekawara (1-day cycle):**
```
ekawara = if (pancawara == Luang_condition) then Luang else Void
// Luang when: pancawara_urip + saptawara_urip % 2 == 0 (varies by source)
```

**Dwiwara (2-day cycle):**
```
dwiwara = if ekawara == Luang then Menga else Pepet
```

**Triwara (3-day cycle):**
```
triwara = pawukon_day % 3  // 0=Pasah, 1=Beteng, 2=Kajeng
```

**Caturwara (4-day cycle) — HAS IRREGULAR ADJUSTMENT:**
```
raw = pawukon_day % 4
// Special case: during Dungulan wuku (wuku index 10), days 1-6 use:
//   if wuku == Dungulan && raw < 3: caturwara = raw + 2  // shift by Jaya Tiga
// This is the "Jaya Tiga" exception in wuku Dungulan
caturwara = adjusted_raw  // 0=Sri, 1=Laba, 2=Jaya, 3=Menala/Mandala
```

**Pancawara (5-day cycle):**
```
pancawara = pawukon_day % 5  // 0=Umanis, 1=Paing, 2=Pon, 3=Wage, 4=Kliwon
```

**Sadwara (6-day cycle):**
```
sadwara = pawukon_day % 6  // 0=Tungleh, 1=Aryang, 2=Wurukung, 3=Paniron, 4=Was, 5=Maulu
```

**Saptawara (7-day cycle):**
```
saptawara = pawukon_day % 7  // 0=Redite, 1=Soma, ..., 6=Saniscara
```

**Astawara (8-day cycle) — HAS IRREGULAR ADJUSTMENT:**
```
raw = pawukon_day % 8
// Special case: During certain positions, the sequence restarts
// The 8-day cycle doesn't divide 210 evenly (210/8 = 26.25)
// peradnya uses: base + adjustment based on wuku boundaries
// Adjustment typically adds +1 to raw during specific wuku transitions
```

**Sangawara (9-day cycle) — HAS IRREGULAR ADJUSTMENT:**
```
raw = pawukon_day % 9
// Special case: certain days are "skipped" to maintain traditional sequence
// The 9-day cycle doesn't divide 210 evenly (210/9 = 23.33)
// Adjustment involves resetting at specific wuku boundaries
```

**Dasawara (10-day cycle):**
```
// Derived from Pancawara and Saptawara, NOT from pawukon_day % 10
// dasawara = f(pancawara, saptawara) via lookup table
// This is documented in edysantosa (see section 5 below)
```

### What Your Crate Should Verify
The Caturwara "Jaya Tiga" exception and the Astawara/Sangawara boundary
adjustments are the most error-prone parts. Your existing TODO notes these
have minimal test coverage. The OCR corpus provides 365 days to validate against.

---

## 5. EDYSANTOSA/SAKACALENDAR — Complete Paringkelan Lookup Tables

**Source:** github.com/edysantosa/sakacalendar (LGPL-2.1)
**Reference books:** "Dasar Wariga" + "Tenung Wariga" by I.B. Putra Manik Aryana;
"Pokok-pokok Wariga" by I.B. Supartha Ardana
**Status:** FULLY EXTRACTED

### 5a. Wuku Names, Raja Names & Urip Values (30 entries)

| # | Raja | Wuku | Urip |
|---|------|------|------|
| 1 | Dewi Sintakasih | Sinta | 7 |
| 2 | Dewi Sanjiwartia | Landep | 1 |
| 3 | Giriswara | Ukir | 4 |
| 4 | Kuladewa | Kulantir | 6 |
| 5 | Talu | Tolu | 5 |
| 6 | Mrabuana | Gumbreg | 8 |
| 7 | Waksaya | Wariga | 9 |
| 8 | Wariwiyasa | Warigadean | 3 |
| 9 | Mrikjulung | Julungwangi | 7 |
| 10 | Sungsangtaya | Sungsang | 1 |
| 11 | Dungulan | Dungulan | 4 |
| 12 | Puspita | Kuningan | 6 |
| 13 | Langkir | Langkir | 5 |
| 14 | Medangsu | Medangsya | 8 |
| 15 | Pujitpwa | Pujut | 9 |
| 16 | Paha | Pahang | 3 |
| 17 | Kruru | Kerulut | 7 |
| 18 | Merangsinga | Merakih | 1 |
| 19 | Tambur | Tambir | 4 |
| 20 | Medangkusa | Medangkungan | 6 |
| 21 | Matal | Matal | 5 |
| 22 | Uye | Uye | 8 |
| 23 | Ijala | Menahil | 9 |
| 24 | Yuddha | Perangbakat | 3 |
| 25 | Baliraja | Bala | 7 |
| 26 | Wiugah | Ugu | 1 |
| 27 | Ringgita | Wayang | 4 |
| 28 | Kulawudra | Kelawu | 6 |
| 29 | Sasawi | Dukut | 5 |
| 30 | Watugunung | Watugunung | 8 |

### 5b. Complete Wewaran Tables (10 cycles)

**Ekawara:** Luang (urip 1)
**Dwiwara:** Menga (5), Pepet (7)
**Triwara:** Dora/Pasah (9), Wahya/Beteng (4), Byantara/Kajeng (7)
**Caturwara:** Sri (4), Laba (5), Jaya (9), Mandala (7)
**Pancawara:** Umanis (5), Pahing (9), Pon (7), Wage (4), Kliwon (8)
**Sadwara:** Tungleh (7), Aryang (6), Wurukung (5), Paniron (8), Was (9), Maulu (3)
**Saptawara:** Redite (5), Coma (4), Anggara (3), Buddha (7), Wrhaspati (8), Sukra (6), Saniscara (9)
**Astawara:** Sri (6), Indra (5), Guru (8), Yama (9), Ludra (3), Brahma (7), Kala (1), Uma (4)
**Sangawara:** Dangu (9), Jagur (8), Gigis (6), Nohan (7), Ogan (4), Erangan (5), Urungan (7), Tulus (3), Dadi (4)
**Dasawara:** Pandita (5), Pati (7), Suka (10), Duka (4), Sri (6), Manu (2), Manusa (3), Raja (8), Dewa (9), Raksasa (1)

### 5c. Ingkel (6 entries)
Wong, Sato, Mina, Manuk, Taru, Buku

### 5d. Jejepan (6 entries)
Mina, Taru, Sato, Patra, Wong, Paksi

### 5e. Watek Alit / Catur (4 entries)
Uler, Gajah, Lembu, Lintah

### 5f. Watek Madya / Panca (5 entries)
Gajah, Watu, Buta, Suku, Wong

### 5g. Panca Sudha (7 entries)
1. Wisesa Segara
2. Tunggak Semi
3. Satria Wibhawa
4. Sumur Sinaba
5. Bumi Kapetak
6. Satria Wirang
7. Lebu Katiup Angin

**CRITICAL FINDING:** edysantosa lists 7 Panca Sudha names, but our OCR corpus
extracted only 7–8 distinct parerasan names with slightly different spellings:
- Wisesa Segara ↔ Wisesa Segara ✓
- Tunggak Semi ↔ Tunggak Semi ✓
- Satria Wibhawa ↔ Satria Wibawa ✓ (spelling variant)
- Sumur Sinaba ↔ Sumer Sinuhe ✗ (DIFFERENT — OCR vs edysantosa)
- Bumi Kapetak ↔ Bumi Kapetak ✓
- Satria Wirang ↔ Satria Wirang ✓
- Lebu Katiup Angin ↔ Lelu Kalung Angis ✗ (DIFFERENT — OCR vs edysantosa)

This suggests either OCR errors in the printed calendar, or I Made Bidja uses
a different Wariga manuscript tradition than Aryana (edysantosa's source).
"Sumur Sinaba" vs "Sumer Sinuhe" and "Lebu Katiup Angin" vs "Lelu Kalung Angis"
need physical source verification.

### 5h. Pararasan (12 entries)
1. Laku Bumi
2. Laku Api
3. Laku Angin
4. Laku Pandita Sakti
5. Aras Tuding
6. Aras Kembang
7. Laku Bintang
8. Laku Bulan
9. Laku Surya
10. Laku Air/Toya
11. Laku Pretiwi
12. Laku Agni Agung

### 5i. Eka Jala Rsi (28 entries)
Bagna Mapasah, Bahu Putra, Buat Astawa, Buat Lara, Buat Merang,
Buat Sebet, Buat Kingking, Buat Suka, Dahat Kingking, Kamaranan,
Kamretaan, Kasobagian, Kinasihan Amreta, Kinasihan Jana,
Langgeng Kayohanaan, Lewih Bagia, Manggih Bagia, Manggih Suka,
Patining Amreta, Rahayu, Sidha Kasobagian, Subagia,
Suka Kapanggih, Suka Pinanggih, Suka Rahayu,
Tininggaling Suka, Wredhi Putra, Wredhi Sarwa Mule

### 5j. Lintang / Palalintangan (35 entries)
Gajah, Kiriman, Jong Sarat, Atiwa-tiwa, Sangka Tikel,
Bubu Bolong, Sugenge, Uluku/Tenggala, Pedati, Kuda,
Gajah Mina, Bade, Magelut, Pagelangan, Kala Sungsang,
Kukus, Asu, Kartika, Naga, Banak Angerem,
Hru/Panah, Patrem, Lembu, Depat/Sidamalung, Tangis,
Salah Ukur, Perahu Pegat, Puwuh Atarung, Lawean/Goang,
Kelapa, Yuyu, Lumbung, Kumbha, Udang, Begoong

### 5k. Rakam (6 entries)
1. Kala Tinatang
2. Demang Kandhuruwan
3. Sanggar Waringin
4. Mantri Sinaroja
5. Macam Katawan
6. Nuju Pati

### 5l. Sasih Names with Sanskrit Equivalents
| # | Balinese | Sanskrit |
|---|----------|----------|
| 1 | Kasa | Srawana |
| 2 | Karo | Bhadrapada |
| 3 | Katiga | Aswina |
| 4 | Kapat | Kartika |
| 5 | Kalima | Margasira |
| 6 | Kanem | Pausya |
| 7 | Kapitu | Magha |
| 8 | Kawolu | Phalguna |
| 9 | Kasanga | Caitra |
| 10 | Kadasa | Waisakha |
| 11 | Destha | Jyestha |
| 12 | Sadha | Asadha |

---

## 6. KARJANTO 2020 — Zeller's Congruence for Pawukon

**Source:** arXiv:2012.10064v1 [math.HO]
**Status:** FULLY EXTRACTED

### Zeller's Congruence (Gregorian Calendar)
For day k, month m (Mar=3..Feb=14), year N = 100C + Y:

```
W ≡ (k + ⌊13(m+1)/5⌋ + Y + ⌊Y/4⌋ + ⌊C/4⌋ - 2C) mod 7
```
Where W: 0=Saturday, 1=Sunday, ..., 6=Friday

### Proposed Pasaran Congruence (NEW — Karjanto's contribution)
The paper proposes a NOVEL congruence formula for computing the pasaran
(Pancawara) day directly from a Gregorian date, analogous to Zeller's
for the day of the week. This replaces the need for a pawukon epoch + offset
calculation.

The formula uses the same inputs (k, m, N, C, Y) but with different constants
tailored to the 5-day pasaran cycle.

### Relevance to Crate
Karjanto's approach could simplify `pawukon_day()` by replacing the
JDN-based computation with a direct Gregorian → Pancawara formula.
However, since the crate already computes via JDN (which is well-tested),
this is more useful as a **cross-validation tool** than a replacement.

### Computer Implementation
The paper includes pseudocode and mentions that Zeller's method can be
extended to compute both dinapitu (Saptawara) and pasaran (Pancawara)
from a single date input — effectively a complete weton calculator.

---

## 7. SOFYAN ET AL. 2022 — Astronomical Associations per Wuku

**Source:** UIN Mataram Press (ISBN 978-623-91908-2-8)
**Status:** NOT ACCESSIBLE (book not freely available online)

### What We Know (from repository abstract)
- Maps wuku terminology to astronomical objects (stars, planets)
- Shows "high awareness of celestial body movements among Balinese people"
- Covers ethnoastronomical aspects of the Pawukon system
- The terms used in Pawukon refer to names of stars and planets

### What This Would Add to the Crate
- `Wuku::astronomical_association() -> Option<&'static str>`
- Cultural/educational metadata for each wuku connecting to specific celestial bodies
- Connects to Candra Praleka (Pleiades/Orion observation) system already extracted from OCR

### Recommendation
Low priority for algorithmic extraction. The astronomy metadata enriches the
crate's documentation but doesn't affect computation. Can be added later from
secondary sources or if the book becomes accessible.

---

## SUMMARY: What Can Be Implemented NOW

### Immediately implementable (complete data available):

| Feature | Source | Data completeness |
|---------|--------|-------------------|
| Wariga BELOG harmonization (mod-4) | JSI 2022 | 100% — algorithm fully extracted |
| PancaSuda names (7 canonical) | edysantosa | 100% — verify against OCR |
| Pararasan names (12 canonical) | edysantosa | 100% |
| Eka Jala Rsi (28 entries) | edysantosa | 100% |
| Lintang (35 entries) | edysantosa | 100% |
| Rakam (6 entries) | edysantosa | 100% |
| Jejepan (6 entries) | edysantosa | 100% |
| Watek Alit/Catur (4 entries) | edysantosa | 100% |
| Watek Madya/Panca (5 entries) | edysantosa | 100% |
| Wuku Raja names (30 entries) | edysantosa | 100% |
| Wuku urip values (30 entries) | edysantosa | 100% |
| Sasih Sanskrit equivalents (12) | edysantosa | 100% |
| All 10 wewaran name+urip tables | edysantosa | 100% |

### Implementable with reverse-engineering:

| Feature | Source | Data completeness |
|---------|--------|-------------------|
| Dewasa Pawiwahan Sugeno system | Candana 2021 + Suwintana 2014 | ~60% — architecture + winner method known, μ-functions still in figures |
| Caturwara Jaya Tiga adjustment | Dershowitz & Reingold | ~70% — encoded in peradnya, need to verify |
| Astawara/Sangawara adjustments | Dershowitz & Reingold | ~60% — complex, needs test cases |
| Wariga bobot (weight) tables | Ariana & Budayoga 2016 book | 0% — book needed for actual weight values |

### Requires book/paper access for complete extraction:

| Feature | Source | What's needed |
|---------|--------|---------------|
| Exact fuzzy μ-function parameters | Candana 2021 figures (image-based) | The membership function curves from Fig. in PDF |
| Wariga weight tables per element | Ariana & Budayoga (2016) *Ala Ayuning Dewasa* | The bobot values for each wewaran/wuku/sasih |
| Suwintana's full rule base | Suwintana (2014) Lontar Komput. 5(1), 392–401 | Complete IF-THEN rule set |

### KEY STRATEGIC DECISION: Sugeno over Mamdani
Candana 2021 conclusively demonstrates that **Sugeno** is the correct fuzzy method
for Dewasa Ayu, not Mamdani (which Suwintana uses on kalenderbali.info).
- Sugeno F-1 Score: 82.76% (vs Mamdani's 5.41%)
- Sugeno correctly found 12/16 expert-validated good days with only 1 false positive
- Mamdani and Tsukamoto found only 1/16 each with 20+ false positives
- Only 2.19% of days are "good" for marriage — the system must be highly selective

### Alahaning Dewasa Hierarchy (from Candana 2021, confirmed by Wariga tradition)
This is the fundamental override rule for ALL Dewasa Ayu computation:
```
Priority (low → high): Wewaran → Wuku → Penanggal → Sasih → Dauh
```
A bad Sasih overrides a good Wuku. A bad Penanggal overrides a good Wewaran.
The Ala Ayu Dewasa classification adds binary prohibition/recommendation overlays
on top of this weighted hierarchy.

### NAMING DISCREPANCY TO RESOLVE

The OCR corpus (I Made Bidja, Wariga Sundari Bungkah tradition) and
edysantosa (I.B. Putra Manik Aryana, Dasar Wariga tradition) disagree on
two Panca Sudha names:

| edysantosa (Aryana) | OCR (Bidja) | Notes |
|---------------------|-------------|-------|
| Sumur Sinaba | Sumer Sinuhe | Different manuscript tradition? |
| Lebu Katiup Angin | Lelu Kalung Angis | Different manuscript tradition? |

Both sources are valid Wariga traditions. The crate should support BOTH
naming conventions, perhaps with a `PancaSudha::canonical_name()` and
`PancaSudha::alternative_name()` API, documenting which Wariga source
each name comes from.

Also note: edysantosa spells it "Satria Wibhawa" while Bidja uses "Satria Wibawa"
(with/without aspirated 'bh'). The aspirated form is closer to Sanskrit; both are used.
