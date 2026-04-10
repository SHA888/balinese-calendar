# Citation Provenance & Audit Trail

> This file records the provenance of every non-trivial citation in the
> `balinese-calendar` crate documentation. Its purpose is to prevent
> citation regression across AI-assisted research sessions and to give
> future contributors a single source of truth for "where did this claim
> come from?" questions.
>
> **Rule:** Any new citation added to this crate's docs must be verifiable
> against a primary source listed here, or added here with a primary-source
> reference, before it ships.

---

## Section 1 — Corrected Citations (Regression Guards)

The following citations were incorrect in earlier drafts of this crate's
documentation. They are recorded here so future AI-assisted sessions do not
re-introduce the same errors.

### 1.1 Suwintana (2014) — fuzzy Dewasa Pawiwahan journal article

**Correct citation:**

> Suwintana, I.K. (2014). 'Penentuan Hari Baik Perkawinan di Bali Berbasis
> Logika Fuzzy'. *Lontar Komputer*, 5(1), 392–401. ISSN: 2088-1541.
> Politeknik Negeri Bali.

**Errors previously present:**

| Field | Incorrect | Correct |
|---|---|---|
| Year | 2015 | **2014** |
| Pages | 392–403 | **392–401** |
| Affiliation | Universitas Udayana | **Politeknik Negeri Bali** |

**Source of error:** Candana et al. (2021) reference [3] miscites this paper
as "vol. 5, no. 1, h. 392–403, 2015". The original *Lontar Komputer* 2014
issue is the authoritative version. Any future citation matching the Candana
[3] wording should be treated as propagation of this error.

**Verification sources used:**
- User-supplied authoritative bibliographic entry (this conversation)
- Cross-check: *Lontar Komputer* is the Universitas Udayana Computer Science
  journal, but Suwintana's institutional affiliation is Politeknik Negeri Bali
  (confirmed by his email domain at the time of publication)

### 1.2 Suwintana & Prihatini (2014) — Android app seminar paper

This is a **separate publication** by the same first author, not to be
conflated with §1.1:

> Suwintana, I.K. & Prihatini, P.M. (2014). 'Perancangan Aplikasi Kalender
> Bali Pada Smartphone Berbasis Android'. In: *Seminar Nasional Sains dan
> Teknologi*, pp. 837–843.

**Topic:** Android app architecture for a Balinese calendar. Does not
contain the fuzzy-logic methodology — that is in §1.1 (the journal article).

**Error previously present:** These two 2014 publications were conflated in
earlier BIBLIOGRAPHY.md drafts. They must remain as separate entries.

### 1.3 kalenderbali.org vs kalenderbali.info attribution

These are **two different websites by two different researchers at two
different institutions**. Earlier drafts attributed both to I Ketut Suwintana,
which is incorrect.

| Site | Operator | Institution | Methodology |
|---|---|---|---|
| **kalenderbali.info** | I Ketut Suwintana | Politeknik Negeri Bali | Mamdani fuzzy inference for Dewasa Pawiwahan (§1.1) |
| **kalenderbali.org** | I Wayan Nuarsa | Universitas Udayana | Day-by-day calendar reference, used by this crate for validation |

**Regression guard pattern:** Any text matching the regex
`kalenderbali\.org.*Suwintana` or `Suwintana.*Universitas Udayana` is an error.

### 1.4 Candana et al. (2021) — author-splitting bug

**Correct citation:**

> Candana, E.W.H., Gunadi, I.G.A., & Divayana, D.G.H. (2021). 'Perbandingan
> Fuzzy Tsukamoto, Mamdani dan Sugeno dalam Penentuan Hari Baik Pernikahan
> Berdasarkan Wariga Menggunakan Confusion Matrix'. *Jurnal Ilmu Komputer
> Indonesia (JIK)*, 6(2), 14–22. p-ISSN: 2615-2703, e-ISSN: 2615-2711.
> Universitas Pendidikan Ganesha Singaraja.

**Error previously present:** Earlier drafts rendered the author list as
"Candana, E., Widastra, H., Gunadi, I.G.A. & Divayana, D.G.H." — splitting
the first author **E.W. Hary Candana** into two people. "Widastra" is the
email prefix of the first author (`eka.widastra@undiksha.ac.id`), not a
separate co-author.

**Correct author count:** 3 (not 4).

**Regression guard pattern:** Any text matching `Candana.*Widastra` is an error.

### 1.5 Hallucinated title (Candana 2021)

At one point in an earlier session, the Candana 2021 paper was given a
fabricated English title "*Fuzzy Inference System for Pawiwahan Good Day
Classification*". **This title does not exist.** The correct title is the
Indonesian one in §1.4 ("Perbandingan Fuzzy Tsukamoto, Mamdani dan Sugeno...").

Any occurrence of the fabricated title is a hallucination and must be removed.

---

## Section 2 — Divergent Manuscript Traditions (Not Errors)

Unlike §1, the divergences in this section are **not** errors. They
represent legitimate differences between Wariga manuscript traditions
that the crate supports explicitly via dual-naming API.

### 2.1 PancaSuda naming: Aryana vs Bidja traditions

Two authoritative Wariga manuscript traditions use different names for
the same PancaSuda positions.

| Position | Aryana tradition (default) | Bidja / Wariga Sundari Bungkah |
|---|---|---|
| 1 | Wisesa Segara | Wisesa Segara |
| 2 | Tunggak Semi | Tunggak Semi |
| 3 | Satria Wibhawa | Satria Wibawa |
| 4 | **Sumur Sinaba** | **Sumer Sinuhe** |
| 5 | Bumi Kapetak | Bumi Kapetak |
| 6 | Satria Wirang | Satria Wirang |
| 7 | **Lebu Katiup Angin** | **Lelu Kalung Angis** |

**Sources:**
- **Aryana tradition:** I.B. Putra Manik Aryana, *Dasar Wariga* and
  *Tenung Wariga* (via edysantosa/sakacalendar LGPL-2.1 source code).
- **Bidja tradition:** I Made Bidja, *Kalender Bali 2026* (IBI Cabang
  Kab. Badung), derived from Wariga Sundari Bungkah manuscript.

**Crate policy:** The Aryana names are the default (`PancaSuda::name()`)
because they are more widely cited in published Wariga reference works.
The Bidja variants are exposed via `PancaSuda::name_sundari_bungkah()`
with doc comments citing the manuscript source.

Neither tradition is "wrong" — they reflect different authoritative
lineages. Future contributors should not "correct" one to match the other.

### 2.2 Satria Wibawa vs Satria Wibhawa (spelling variant)

The aspirated form ("Wibhawa") is closer to the Sanskrit etymology.
The unaspirated form ("Wibawa") is the contemporary Balinese vernacular.
Both are acceptable; this crate uses the unaspirated form for consistency
with modern Indonesian-Balinese orthography.

---

## Section 3 — Verification Checklist for New Citations

Before any new citation is added to this crate's documentation, the
contributor (human or AI-assisted) must confirm all five items:

1. **Primary source obtained.** The cited paper, book, or web resource has
   been directly accessed. Citations derived purely from a secondary
   source's reference list must be marked `[unverified]` in the bibliography
   and treated with suspicion until independently confirmed.

2. **Author list verified.** Each author has been confirmed as a distinct
   person. Check that initials are not being mistaken for separate authors
   (see §1.4 for the E.W. Hary Candana error pattern).

3. **Year, volume, and pages match the primary source.** Do not trust
   secondary-source reference lists for these fields. Candana 2021's
   reference [3] is a documented example of a published paper containing
   a miscitation (§1.1).

4. **Affiliation matches the author's email domain or ORCID.** Indonesian
   academic affiliations in particular can be easily confused when a
   researcher's work is hosted on a journal published by a different
   institution (see §1.3 for the Suwintana / Politeknik Negeri Bali vs
   Lontar Komputer / Universitas Udayana confusion).

5. **Title is the actual title.** Do not paraphrase titles into English
   unless the primary source provides an official English translation.
   Do not generate plausible-sounding titles (see §1.5 for the
   hallucination example).

---

## Section 4 — Grep Patterns for Regression Testing

Add these to CI or pre-commit hooks to prevent regression of the §1 errors.
All five patterns must return zero matches across all `*.md` and `*.rs`
files in the repository:

```bash
# §1.1 — Suwintana year and pages errors
rg -n 'Suwintana.*2015' .
rg -n '392[–-]403' .

# §1.3 — kalenderbali.org / Suwintana conflation
rg -n 'kalenderbali\.org.*Suwintana' .
rg -n 'Suwintana.*Universitas Udayana' .

# §1.4 — Candana author-splitting
rg -n 'Candana.*Widastra' .

# §1.5 — Hallucinated Candana title
rg -n 'Fuzzy Inference System for Pawiwahan' .
```

Positive confirmation patterns — these should return matches in at least
the bibliography files:

```bash
rg -n 'Suwintana.*2014.*Lontar Komputer' .
rg -n '392[–-]401' .
rg -n 'Politeknik Negeri Bali' .
rg -n 'Candana.*E\.W\.H\.' .
```

---

## Section 5 — Correction History

| Date | Corrections applied | Files affected | Lines changed |
|---|---|---|---|
| 2026-03 | §1.1 Suwintana year/pages/affiliation, §1.3 kalenderbali.org attribution, §1.4 Candana author split, §1.5 hallucinated title removal, §1.2 added missing seminar paper | TODO.md, README.md, references/BIBLIOGRAPHY.md, references/EXTRACTED_ALGORITHMS.md, tests/validation_2026_test.rs | 16 |

Future corrections must append to this table with the date, the sections
of this file that document the error pattern, and the files/lines affected.

---

## Section 6 — How This File Should Be Used by AI-Assisted Sessions

When an AI coding agent or research assistant is working on this crate's
documentation, it should be given this file as primary context alongside
any research task involving citations. The agent should:

1. Read §1 before generating any new citation to avoid re-introducing
   documented errors.
2. Read §2 before "correcting" any PancaSuda name — the divergence is
   intentional and the crate's dual-naming API depends on preserving both.
3. Run §4's grep patterns after making any documentation changes, and
   report zero-match confirmation in its completion summary.
4. Append new corrections to §5 rather than silently overwriting history.

This file is the single source of truth for citation provenance in this
crate. Disagreements between this file and other documentation should be
resolved in favor of this file.
