# references/downloads — Resource Archive

Automatically fetched by `tools/fetch_resources.py`. Tracks all items in `references/resources.md`.

## Directory Layout

```
references/downloads/
  s1-primary-scans/       §1  Lontar scans (Internet Archive, Wikimedia Commons)
  s2-transcriptions/      §2  Wariga Gemet transcriptions (komangputra, BasaBali)
  s3-related-lontar/      §3  Search leads only (no confirmed public URLs)
  s4-academic/            §4  Peer-reviewed papers
  s5-historical/          §5  Historia.id, British Library blog, Leiden KITLV
  s6-encyclopedias/       §6  Wikipedia articles + wikitext, Calendopedia
  s7-computational/       §7  kalenderbali.info/org, BabadBali
  s8-practitioner/        §8  Tribun Bali, tatkala.co, Sastra Bali, etc.
  s9-print/               §9  README only — print books, acquire manually
  s10-archival/           §10 KITLV/ANRI landing snapshots + search leads
  MANIFEST.json           Machine-readable record of all fetch results
```

## Re-running the Script

```bash
# Full run (idempotent — skips already-downloaded files)
uv run --with requests --with beautifulsoup4 --with lxml python tools/fetch_resources.py

# Single section only
uv run --with requests --with beautifulsoup4 --with lxml python tools/fetch_resources.py --section 4

# Dry run (no network, no writes — prints plan only)
uv run --with requests --with beautifulsoup4 --with lxml python tools/fetch_resources.py --dry-run
```

## MANIFEST.json Fields

| Field | Meaning |
|---|---|
| `slug` | Unique identifier matching the subdirectory name |
| `section` | Section number from `resources.md` (1–10) |
| `url` | Original canonical URL |
| `local_path` | Path relative to repo root |
| `status` | `ok` / `snapshot` / `failed` / `manual` |
| `tier` | `A` = auto-fetch, `B` = HTML snapshot only, `C` = manual |
| `sha256` | SHA-256 of the saved file (empty if not saved) |
| `fetched_at` | ISO 8601 UTC timestamp |

## Status Codes

- **`ok`** — full file saved (PDF or complete HTML)
- **`snapshot`** — HTML-only or partial (Tier B items, bot-challenged pages)
- **`failed`** — network error, 403, or 404; item needs manual attention
- **`manual`** — Tier C: print books, paywalled, or institutional-only

## Manual Acquisition Checklist (Tier C + persistent failures)

### Needs browser download (server blocks automated TLS from this IP)
- [ ] **Proudfoot 2007** — `https://scispace.com/pdf/in-search-of-lost-time-javanese-and-balinese-understandings-4hivzafwai.pdf`
  → save to `s4-academic/proudfoot-2007-in-search-of-lost-time/proudfoot-2007-in-search-of-lost-time.pdf`

### Dead URLs (no Wayback snapshot)
- [ ] **JoMEaL ethnomathematics** — `https://journal.unej.ac.id/JoMEaL/article/view/1913`
  Domain does not resolve; no archived copy found. Search Google Scholar: *"ethnomathematics balinese calendar" LCM 210 site:unej.ac.id*

### Cloudflare-protected (require JS challenge)
- [ ] **BasaBali Wiki: Lontar Wariga Gemet** — `https://dictionary.basabali.org/Lontar_Wariga_Gemet`
  → save to `s2-transcriptions/basabali-lontar-wariga-gemet/page.html`
- [ ] **ANRI** — `https://anri.go.id/`
  → save to `s10-archival/anri-arsip-nasional/page.html`

### Institutional / paywall
- [ ] **Dershowitz & Reingold, Cambridge** — `https://www.cambridge.org/core/books/calendrical-calculations/balinese-pawukon-calendar/AE3E7D55A609FFA3017A40C70A15A758` — acquire via library
- [ ] **Gale OneFile** — `https://go.gale.com/ps/i.do?p=AONE&...` — requires institutional Gale access
- [ ] **Walisongo thesis (Ramdhani 2017)** — search `http://eprints.walisongo.ac.id/`
- [ ] **Suarka 2008** — check Unud eprints / contact author

### Print books (§9)
See `s9-print/README.md` for full list with acquisition notes.
