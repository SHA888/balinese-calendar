#!/usr/bin/env python3
"""
Fetch Balinese calendar research resources listed in references/resources.md.

Tiers:
  A — auto-fetchable (PDF or full HTML snapshot; status → [x])
  B — HTML snapshot only / partial access          (status → [~])
  C — manual / out-of-scope                        (status stays [ ])

Run:
    pip install -r tools/requirements.txt
    python tools/fetch_resources.py [--dry-run] [--section N]

Outputs:
  references/downloads/<section-slug>/<item-slug>/   — downloaded files
  references/downloads/MANIFEST.json                 — machine-readable record
"""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import sys
import time
import urllib.robotparser
from dataclasses import asdict, dataclass, field
from datetime import datetime, timezone
from pathlib import Path
from typing import Optional
from urllib.parse import urljoin, urlparse

import requests
from bs4 import BeautifulSoup

# ---------------------------------------------------------------------------
# Config
# ---------------------------------------------------------------------------

REPO_ROOT = Path(__file__).resolve().parent.parent
DOWNLOADS_DIR = REPO_ROOT / "references" / "downloads"
MANIFEST_PATH = DOWNLOADS_DIR / "MANIFEST.json"
RESOURCES_MD = REPO_ROOT / "references" / "resources.md"

UA = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36"
DELAY = 1.1  # seconds between requests
RETRY_COUNT = 3
RETRY_BACKOFF = 2.0  # seconds; multiplied per attempt

STATUS_OK = "ok"
STATUS_SNAPSHOT = "snapshot"
STATUS_FAILED = "failed"
STATUS_SKIPPED = "skipped"
STATUS_MANUAL = "manual"

SECTION_DIRS = {
    1: "s1-primary-scans",
    2: "s2-transcriptions",
    3: "s3-related-lontar",
    4: "s4-academic",
    5: "s5-historical",
    6: "s6-encyclopedias",
    7: "s7-computational",
    8: "s8-practitioner",
    9: "s9-print",
    10: "s10-archival",
}


# ---------------------------------------------------------------------------
# Data structures
# ---------------------------------------------------------------------------

@dataclass
class FetchResult:
    section: int
    slug: str
    url: str
    local_path: str
    status: str
    note: str
    sha256: str = ""
    fetched_at: str = ""
    tier: str = "A"


# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

_session: Optional[requests.Session] = None
_robots_cache: dict[str, urllib.robotparser.RobotFileParser] = {}


def session() -> requests.Session:
    global _session
    if _session is None:
        _session = requests.Session()
        _session.headers.update({"User-Agent": UA})
    return _session


def robots_allowed(url: str) -> bool:
    parsed = urlparse(url)
    base = f"{parsed.scheme}://{parsed.netloc}"
    if base not in _robots_cache:
        rp = urllib.robotparser.RobotFileParser()
        robots_url = f"{base}/robots.txt"
        try:
            resp = session().get(robots_url, timeout=10)
            rp.parse(resp.text.splitlines())
        except Exception:
            rp.allow_all = True
        _robots_cache[base] = rp
        time.sleep(DELAY)
    return _robots_cache[base].can_fetch(UA, url)


def get(url: str, stream: bool = False, timeout: int = 30, verify: bool = True) -> Optional[requests.Response]:
    for attempt in range(1, RETRY_COUNT + 1):
        try:
            resp = session().get(url, stream=stream, timeout=timeout, allow_redirects=True, verify=verify)
            if resp.status_code == 200:
                return resp
            if resp.status_code in (429, 503):
                wait = RETRY_BACKOFF * (2 ** attempt)
                print(f"  [{resp.status_code}] {url} — wait {wait:.0f}s", flush=True)
                time.sleep(wait)
                continue
            print(f"  [HTTP {resp.status_code}] {url}", flush=True)
            return None
        except requests.exceptions.SSLError:
            if verify:
                print(f"  [SSL err] {url} — retrying without verify", flush=True)
                return get(url, stream=stream, timeout=timeout, verify=False)
            print(f"  [SSL err no-verify] {url}", flush=True)
            return None
        except requests.exceptions.RequestException as exc:
            wait = RETRY_BACKOFF * attempt
            print(f"  [err attempt {attempt}] {url}: {exc}", flush=True)
            time.sleep(wait)
    return None


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(65536), b""):
            h.update(chunk)
    return h.hexdigest()


def now_iso() -> str:
    return datetime.now(timezone.utc).isoformat(timespec="seconds")


def slugify(text: str) -> str:
    text = text.lower().strip()
    text = re.sub(r"[^\w\s-]", "", text)
    text = re.sub(r"[\s_]+", "-", text)
    return re.sub(r"-+", "-", text)[:80]


def section_dir(section: int) -> Path:
    return DOWNLOADS_DIR / SECTION_DIRS[section]


def ensure_dir(path: Path) -> Path:
    path.mkdir(parents=True, exist_ok=True)
    return path


def save_pdf(url: str, dest_dir: Path, filename: str, dry_run: bool = False, verify: bool = True) -> tuple[str, str]:
    """Download a PDF to dest_dir/filename. Returns (local_path, sha256)."""
    dest = dest_dir / filename
    if dest.exists():
        return str(dest.relative_to(REPO_ROOT)), sha256_file(dest)
    if dry_run:
        return str(dest.relative_to(REPO_ROOT)), ""
    if not robots_allowed(url):
        return "", "robots-disallowed"
    resp = get(url, stream=True, timeout=60, verify=verify)
    time.sleep(DELAY)
    if resp is None:
        return "", ""
    with dest.open("wb") as f:
        for chunk in resp.iter_content(65536):
            f.write(chunk)
    return str(dest.relative_to(REPO_ROOT)), sha256_file(dest)


def save_html(url: str, dest_dir: Path, filename: str = "page.html", dry_run: bool = False, verify: bool = True) -> tuple[str, str]:
    """Fetch a URL and save raw HTML. Returns (local_path, sha256)."""
    dest = dest_dir / filename
    if dest.exists():
        return str(dest.relative_to(REPO_ROOT)), sha256_file(dest)
    if dry_run:
        return str(dest.relative_to(REPO_ROOT)), ""
    if not robots_allowed(url):
        return "", "robots-disallowed"
    resp = get(url, verify=verify)
    time.sleep(DELAY)
    if resp is None:
        return "", ""
    dest.write_text(resp.text, encoding="utf-8")
    return str(dest.relative_to(REPO_ROOT)), sha256_file(dest)


def make_result(section: int, slug: str, url: str, local_path: str,
                sha256: str, status: str, note: str, tier: str = "A") -> FetchResult:
    return FetchResult(
        section=section,
        slug=slug,
        url=url,
        local_path=local_path,
        status=status,
        note=note,
        sha256=sha256,
        fetched_at=now_iso(),
        tier=tier,
    )


def ia_pdf_url(identifier: str, resp_detail: Optional[requests.Response] = None) -> Optional[str]:
    """Resolve the best PDF URL from an Internet Archive identifier."""
    candidates = [
        f"https://archive.org/download/{identifier}/{identifier}.pdf",
        f"https://archive.org/download/{identifier}/{identifier}_text.pdf",
    ]
    for url in candidates:
        r = session().head(url, timeout=15, allow_redirects=True)
        time.sleep(DELAY)
        if r.status_code == 200:
            return url
    if resp_detail is None:
        resp_detail = get(f"https://archive.org/details/{identifier}")
        time.sleep(DELAY)
    if resp_detail is None:
        return None
    soup = BeautifulSoup(resp_detail.text, "lxml")
    for a in soup.find_all("a", href=True):
        href = a["href"]
        if href.lower().endswith(".pdf"):
            return urljoin("https://archive.org", href)
    return None


# ---------------------------------------------------------------------------
# Section fetchers
# ---------------------------------------------------------------------------

def fetch_section_1(dry_run: bool) -> list[FetchResult]:
    results = []
    base = section_dir(1)

    items = [
        ("wariga-gemet-ia-v1", "wariga-gemet",
         "Lontar Wariga Gemet (Internet Archive v1)", "A"),
        ("wariga-gede-gemet-ia-v2", "wariga-gede-gemet",
         "Lontar Wariga Gede Gemet (Internet Archive v2)", "A"),
    ]
    for slug, ia_id, label, tier in items:
        dest = ensure_dir(base / slug)
        print(f"  [§1] {label}", flush=True)
        pdf_url = ia_pdf_url(ia_id)
        if pdf_url:
            lp, sha = save_pdf(pdf_url, dest, f"{ia_id}.pdf", dry_run)
            status = STATUS_OK if lp else STATUS_FAILED
            note = f"Internet Archive PDF: {pdf_url}"
        else:
            lp, sha = save_html(f"https://archive.org/details/{ia_id}", dest, "page.html", dry_run)
            status = STATUS_SNAPSHOT if lp else STATUS_FAILED
            note = "PDF link not resolved; HTML details page saved"
        results.append(make_result(1, slug, f"https://archive.org/details/{ia_id}", lp, sha, status, note, tier))

    # Wikimedia Commons PDF — resolve actual upload path from the file page
    slug = "wariga-gemet-wikimedia-pdf"
    dest = ensure_dir(base / slug)
    wm_url = "https://commons.wikimedia.org/wiki/File:Bali-lontar-wariga-gemet-02-250ppi.pdf"
    print(f"  [§1] Wariga Gemet 250ppi PDF (Wikimedia Commons)", flush=True)
    lp_html, sha_html = save_html(wm_url, dest, "wikimedia-file-page.html", dry_run)
    # Parse the file page to find the actual PDF download link
    pdf_direct = None
    if lp_html and not dry_run:
        fpage_html = (REPO_ROOT / lp_html).read_text(encoding="utf-8")
        fsoup = BeautifulSoup(fpage_html, "lxml")
        for a in fsoup.find_all("a", href=True):
            href = a["href"]
            if ".pdf" in href.lower() and "upload.wikimedia.org" in href:
                pdf_direct = href if href.startswith("http") else "https:" + href
                break
    if pdf_direct:
        lp_pdf, sha_pdf = save_pdf(pdf_direct, dest, "Bali-lontar-wariga-gemet-02-250ppi.pdf", dry_run)
    else:
        lp_pdf, sha_pdf = "", ""
    if lp_pdf:
        results.append(make_result(1, slug, wm_url, lp_pdf, sha_pdf, STATUS_OK,
                                   f"Direct PDF from Wikimedia Commons: {pdf_direct}", "A"))
    else:
        results.append(make_result(1, slug, wm_url, lp_html or "", sha_html or "",
                                   STATUS_SNAPSHOT if lp_html else STATUS_FAILED,
                                   "PDF link not resolved from file page; HTML saved", "A"))

    # UI Digital Lontar — Tier B (HTML snapshot); expired cert → verify=False
    slug = "serat-etang-wariga-gemet-ui"
    dest = ensure_dir(base / slug)
    ui_url = "https://lontar.ui.ac.id/detail?id=20435801"
    print(f"  [§1] Serat Etang Wariga Gemet (UI Digital Lontar) [Tier B]", flush=True)
    lp, sha = save_html(ui_url, dest, "page.html", dry_run, verify=False)
    results.append(make_result(1, slug, ui_url, lp, sha,
                               STATUS_SNAPSHOT if lp else STATUS_FAILED,
                               "Tier B: UI catalog entry; expired cert, fetched with verify=False", "B"))
    return results


def fetch_section_2(dry_run: bool) -> list[FetchResult]:
    results = []
    base = section_dir(2)

    # komangputra — paginated scrape
    slug = "komangputra-wariga-gemet"
    dest = ensure_dir(base / slug)
    print(f"  [§2] komangputra.com Wariga Gemet (paginated)", flush=True)
    base_url = "https://www.komangputra.com/lontar-bali-wariga-gemet.html"
    combined_md = dest / "transcription.md"
    saved_pages = []
    if not combined_md.exists() or combined_md.stat().st_size < 100:
        all_text: list[str] = []
        seen_hashes: set[str] = set()
        for page_num in range(1, 21):
            url = base_url if page_num == 1 else f"{base_url}/{page_num}"
            if not dry_run and not robots_allowed(url):
                break
            resp = get(url)
            time.sleep(DELAY)
            if resp is None:
                break
            content_hash = hashlib.md5(resp.text.encode()).hexdigest()
            if content_hash in seen_hashes:
                break
            seen_hashes.add(content_hash)
            html_file = dest / f"folio-{page_num:02d}.html"
            if not dry_run:
                html_file.write_text(resp.text, encoding="utf-8")
            saved_pages.append(str(html_file.relative_to(REPO_ROOT)))
            soup = BeautifulSoup(resp.text, "lxml")
            article = soup.find("article") or soup.find("div", class_=re.compile(r"entry|content|post"))
            text_content = article.get_text(separator="\n", strip=True) if article else ""
            if text_content:
                all_text.append(f"\n\n## Page {page_num}\n\n{text_content}")
        if not dry_run and all_text:
            combined_md.write_text("# Wariga Gemet — komangputra.com transcription\n" + "".join(all_text),
                                   encoding="utf-8")
    lp = str(combined_md.relative_to(REPO_ROOT))
    sha = sha256_file(combined_md) if combined_md.exists() and not dry_run else ""
    status = STATUS_OK if (combined_md.exists() and combined_md.stat().st_size > 100) else STATUS_FAILED
    note = f"Scraped {len(saved_pages)} page(s); combined text in transcription.md"
    results.append(make_result(2, slug, base_url, lp, sha, status, note))

    # BasaBali Wiki
    slug = "basabali-lontar-wariga-gemet"
    dest = ensure_dir(base / slug)
    url = "https://dictionary.basabali.org/Lontar_Wariga_Gemet"
    print(f"  [§2] BasaBali Wiki: Lontar Wariga Gemet", flush=True)
    lp, sha = save_html(url, dest, "page.html", dry_run)
    results.append(make_result(2, slug, url, lp, sha,
                               STATUS_OK if lp else STATUS_FAILED,
                               "BasaBali Wiki article HTML"))

    # Scribd mirrors — Tier B
    scribd_items = [
        ("scribd-wariga-gemet-21p", "https://www.scribd.com/document/780621393/Lontar-Wariga-Gemet",
         "Scribd mirror 21 pages"),
        ("scribd-wariga-gemet-29p", "https://www.scribd.com/document/743335371/Lontar-Bali-Wariga-Gemet",
         "Scribd mirror 29 pages (Wayan Budiana)"),
    ]
    for slug, url, label in scribd_items:
        dest = ensure_dir(base / slug)
        print(f"  [§2] {label} [Tier B]", flush=True)
        lp, sha = save_html(url, dest, "page.html", dry_run)
        stub = dest / "README.md"
        if not dry_run and not stub.exists():
            stub.write_text(
                f"# {label}\n\n"
                f"⚠ Unverified Scribd mirror. URL: {url}\n\n"
                "Scribd requires authentication for full access. "
                "Snapshot HTML is likely a login wall. Acquire manually if needed.\n",
                encoding="utf-8",
            )
        results.append(make_result(2, slug, url, lp, sha,
                                   STATUS_SNAPSHOT if lp else STATUS_FAILED,
                                   f"Tier B: {label}; likely login wall", "B"))
    return results


def fetch_section_3(_dry_run: bool) -> list[FetchResult]:
    results = []
    base = section_dir(3)
    ensure_dir(base)
    readme = base / "README.md"
    if not readme.exists():
        readme.write_text(
            "# §3 — Related Lontar (Search Leads)\n\n"
            "These lontar are referenced in Wariga Gemet but have no confirmed public URL.\n\n"
            "## Search leads\n\n"
            "- **Lontar Sundarigama**: search Internet Archive (`site:archive.org sundarigama`), "
            "BasaBali Wiki, PDK Bali.\n"
            "- **Lontar Wariga Uliken**: cross-referenced in Wariga Gemet; search Gedong Kirtya catalog.\n"
            "- **Lontar Medangkemulan**: search Unud repository and KITLV.\n"
            "- **Lontar Bagawan Garga**: search Unud repository.\n"
            "- **Lontar Lelanusan**: tatkala.co article cites this; search BasaBali and PDK Bali.\n",
            encoding="utf-8",
        )
    for slug, label in [
        ("lontar-sundarigama", "Lontar Sundarigama"),
        ("lontar-wariga-uliken", "Lontar Wariga Uliken"),
        ("lontar-medangkemulan", "Lontar Medangkemulan"),
        ("lontar-bagawan-garga", "Lontar Bagawan Garga"),
        ("lontar-lelanusan", "Lontar Lelanusan"),
    ]:
        results.append(make_result(3, slug, "", str((base / "README.md").relative_to(REPO_ROOT)),
                                   "", STATUS_MANUAL,
                                   "No confirmed public URL; see §3 README for search leads", "C"))
    return results


def fetch_section_4(dry_run: bool) -> list[FetchResult]:
    results = []
    base = section_dir(4)

    # Ginaya 2018 — open access PDF
    # Note: sloap.org TLS times out from some IPs; download manually if needed:
    #   https://sloap.org/journals/index.php/ijllc/article/download/173/715/812
    #   → references/downloads/s4-academic/ginaya-2018-balinese-calendar-system/ginaya-2018-balinese-calendar-system.pdf
    slug = "ginaya-2018-balinese-calendar-system"
    dest = ensure_dir(base / slug)
    url = "https://sloap.org/journals/index.php/ijllc/article/download/173/715/812"
    print(f"  [§4] Ginaya 2018 open-access PDF", flush=True)
    lp, sha = save_pdf(url, dest, "ginaya-2018-balinese-calendar-system.pdf", dry_run)
    if not lp:
        # Fallback: article landing page
        lp, sha = save_html("https://sloap.org/journals/index.php/ijllc/article/view/173",
                            dest, "article-page.html", dry_run)
    if not lp:
        # Last resort: Wayback Machine
        lp, sha = save_pdf(
            "https://web.archive.org/web/2023/https://sloap.org/journals/index.php/ijllc/article/download/173/715/812",
            dest, "ginaya-2018-balinese-calendar-system.pdf", dry_run)
    results.append(make_result(4, slug, url, lp, sha,
                               STATUS_OK if (lp and lp.endswith(".pdf")) else (STATUS_SNAPSHOT if lp else STATUS_FAILED),
                               "IJLLC open-access; epistemology + urip-based derivation"))

    # Proudfoot 2007 — try scispace landing page, then JSTOR open access, then snapshot
    slug = "proudfoot-2007-in-search-of-lost-time"
    dest = ensure_dir(base / slug)
    url = "https://scispace.com/pdf/in-search-of-lost-time-javanese-and-balinese-understandings-4hivzafwai.pdf"
    alt_urls = [
        "https://brill.com/view/journals/bki/163/1/article-p86_5.xml",
        "https://www.jstor.org/stable/27868346",
        "https://scispace.com/papers/in-search-of-lost-time-javanese-and-balinese-understandings-4hivzafwai",
    ]
    print(f"  [§4] Proudfoot 2007 (scispace + fallbacks)", flush=True)
    lp, sha = save_pdf(url, dest, "proudfoot-2007-in-search-of-lost-time.pdf", dry_run)
    if not lp:
        for alt in alt_urls:
            lp, sha = save_html(alt, dest, f"{slugify(alt.split('/')[-1]) or 'page'}.html", dry_run)
            if lp:
                break
    note = ("BKI Proudfoot 2007 via scispace" if (lp and lp.endswith(".pdf"))
            else "Snapshot only; DOI: 10.1163/22134379-90003753; acquire via ILL")
    results.append(make_result(4, slug, url, lp, sha,
                               STATUS_OK if (lp and lp.endswith(".pdf")) else (STATUS_SNAPSHOT if lp else STATUS_FAILED),
                               note))

    # Cambridge — Tier C paywall
    slug = "dershowitz-reingold-balinese-pawukon"
    dest = ensure_dir(base / slug)
    url = "https://www.cambridge.org/core/books/calendrical-calculations/balinese-pawukon-calendar/AE3E7D55A609FFA3017A40C70A15A758"
    results.append(make_result(4, slug, url, "", "", STATUS_MANUAL,
                               "Tier C: Cambridge paywall; acquire via library access", "C"))

    # JoMEaL — original URL 404; try ejournal subdomain + Wayback
    slug = "jomeal-ethnomathematics-balinese-calendar"
    dest = ensure_dir(base / slug)
    url = "https://journal.unej.ac.id/JoMEaL/article/view/1913"
    jomeal_urls = [
        "https://ejournal.unej.ac.id/index.php/JoMEaL/article/view/1913",
        "https://journal.unej.ac.id/index.php/JoMEaL/article/view/1913",
        "https://web.archive.org/web/2024/https://journal.unej.ac.id/JoMEaL/article/view/1913",
        "https://web.archive.org/web/2024/https://ejournal.unej.ac.id/index.php/JoMEaL/article/view/1913",
    ]
    print(f"  [§4] JoMEaL ethnomathematics article", flush=True)
    lp, sha = "", ""
    for jurl in jomeal_urls:
        lp, sha = save_html(jurl, dest, "page.html", dry_run)
        if lp:
            break
    if lp:
        page_html = (REPO_ROOT / lp).read_text(encoding="utf-8") if lp and not dry_run else ""
        soup = BeautifulSoup(page_html, "lxml") if page_html else None
        pdf_link = None
        if soup:
            for a in soup.find_all("a", href=True):
                if "pdf" in a["href"].lower() or "download" in a["href"].lower():
                    pdf_link = urljoin(url, a["href"])
                    break
        if pdf_link:
            lp_pdf, sha_pdf = save_pdf(pdf_link, dest, "jomeal-ethnomathematics.pdf", dry_run)
            if lp_pdf:
                lp, sha = lp_pdf, sha_pdf
    results.append(make_result(4, slug, url, lp, sha,
                               STATUS_OK if lp else STATUS_FAILED,
                               "JoMEaL; LCM(5,6,7)=210 structural foundation"))

    # Gale OneFile — Tier B
    slug = "gale-multi-parameter-balinese-calendar"
    dest = ensure_dir(base / slug)
    url = "https://go.gale.com/ps/i.do?p=AONE&u=googlescholar&id=GALE%7CA616056168"
    print(f"  [§4] Gale OneFile abstract [Tier B]", flush=True)
    lp, sha = save_html(url, dest, "page.html", dry_run)
    results.append(make_result(4, slug, url, lp, sha,
                               STATUS_SNAPSHOT if lp else STATUS_FAILED,
                               "Tier B: Gale abstract; full text institutional", "B"))

    # Walisongo thesis — Tier C
    results.append(make_result(4, "ramdhani-2017-walisongo-thesis",
                               "http://eprints.walisongo.ac.id/", "", "", STATUS_MANUAL,
                               "Tier C: search Walisongo eprints for Ramdhani F.Z. 2017", "C"))

    # Suarka 2008 — Tier C
    results.append(make_result(4, "suarka-2008-sistem-penanggalan-bali",
                               "", "", "", STATUS_MANUAL,
                               "Tier C: Unud seminar paper; check Unud eprints / contact author", "C"))
    return results


def fetch_section_5(dry_run: bool) -> list[FetchResult]:
    results = []
    base = section_dir(5)

    items_a = [
        ("historia-id-jual-beli-bali-kuno",
         "https://www.historia.id/article/jual-beli-semasa-bali-kuno-dp9ey",
         "Historia.id: Jual-Beli Semasa Bali Kuno"),
        ("bl-blog-pawukon-manuscripts",
         "https://blogs.bl.uk/asian-and-african/2018/11/pawukon-javanese-calendrical-manuscripts.html",
         "British Library blog: Pawukon Javanese manuscripts"),
    ]
    for slug, url, label in items_a:
        dest = ensure_dir(base / slug)
        print(f"  [§5] {label}", flush=True)
        lp, sha = save_html(url, dest, "page.html", dry_run)
        results.append(make_result(5, slug, url, lp, sha,
                                   STATUS_OK if lp else STATUS_FAILED, label))

    # Leiden KITLV — Tier B
    slug = "leiden-kitlv-digital-collections"
    dest = ensure_dir(base / slug)
    url = "https://digitalcollections.universiteitleiden.nl/"
    print(f"  [§5] Leiden KITLV landing [Tier B]", flush=True)
    lp, sha = save_html(url, dest, "page.html", dry_run)
    results.append(make_result(5, slug, url, lp, sha,
                               STATUS_SNAPSHOT if lp else STATUS_FAILED,
                               "Tier B: landing page saved; search 'pasar Bali' / 'Bali markt' manually", "B"))

    # Prasasti Bebetin + Ardika — Tier C
    results.append(make_result(5, "prasasti-bebetin",
                               "", "", "", STATUS_MANUAL,
                               "Tier C: track via Balai Arkeologi Denpasar or Ardika publication", "C"))
    results.append(make_result(5, "ardika-1988-ekskavasi-pacung-sembiran-julah",
                               "", "", "", STATUS_MANUAL,
                               "Tier C: check Unud/Arkenas archives for Ardika 1988 report", "C"))
    return results


def fetch_section_6(dry_run: bool) -> list[FetchResult]:
    results = []
    base = section_dir(6)

    wiki_items = [
        ("wikipedia-pawukon-calendar",
         "https://en.wikipedia.org/wiki/Pawukon_calendar",
         "Wikipedia: Pawukon calendar"),
        ("wikipedia-javanese-calendar",
         "https://en.wikipedia.org/wiki/Javanese_calendar",
         "Wikipedia: Javanese calendar"),
        ("wikipedia-triwara-id",
         "https://id.wikipedia.org/wiki/Triwara",
         "Wikipedia ID: Triwara"),
        ("wikipedia-balinese-pawukon-talk",
         "https://en.wikipedia.org/wiki/Talk:Balinese_pawukon_calendar",
         "Wikipedia talk: Balinese pawukon calendar"),
    ]
    for slug, url, label in wiki_items:
        dest = ensure_dir(base / slug)
        print(f"  [§6] {label}", flush=True)
        lp, sha = save_html(url, dest, "page.html", dry_run)
        raw_url = url.replace("wikipedia.org/wiki/", "wikipedia.org/wiki/Special:Export/") \
                     .replace("en.wikipedia.org/wiki/Talk:", "en.wikipedia.org/wiki/Special:Export/Talk:")
        if not dry_run and lp:
            save_html(url + "?action=raw", dest, "page.wikitext", dry_run)
        results.append(make_result(6, slug, url, lp, sha,
                                   STATUS_OK if lp else STATUS_FAILED, label))

    # Calendopedia
    slug = "calendopedia-balinese-calendar"
    dest = ensure_dir(base / slug)
    url = "https://www.calendopedia.com/balinese.htm"
    print(f"  [§6] Calendopedia: Balinese Calendar", flush=True)
    lp, sha = save_html(url, dest, "page.html", dry_run)
    results.append(make_result(6, slug, url, lp, sha,
                               STATUS_OK if lp else STATUS_FAILED,
                               "Calendopedia Balinese calendar reference"))
    return results


def fetch_section_7(dry_run: bool) -> list[FetchResult]:
    results = []
    base = section_dir(7)

    items = [
        ("kalenderbali-info-referensi",
         "https://kalenderbali.info/referensi",
         "kalenderbali.info: algorithmic reference"),
        ("kalenderbali-org-referensialaayu",
         "https://kalenderbali.org/referensialaayu.php",
         "kalenderbali.org: dewasa ayu / wewaran tables"),
        ("babadbali-pewarigaan-triwara",
         "https://www.babadbali.com/pewarigaan/triwara.htm",
         "BabadBali: Pewarigaan / Triwara"),
    ]
    for slug, url, label in items:
        dest = ensure_dir(base / slug)
        print(f"  [§7] {label}", flush=True)
        lp, sha = save_html(url, dest, "page.html", dry_run)
        results.append(make_result(7, slug, url, lp, sha,
                                   STATUS_OK if lp else STATUS_FAILED, label))
    return results


def fetch_section_8(dry_run: bool) -> list[FetchResult]:
    results = []
    base = section_dir(8)

    items = [
        ("tribun-bali-padewasan-perkawinan",
         "https://bali.tribunnews.com/2021/02/16/padewasan-perkawinan-dalam-ajaran-hindu-bali-berikut-penjelasan-ida-pedanda-gede-buruan",
         "Tribun Bali: Ida Pedanda Gede Buruan on Padewasan"),
        ("radio-denpasar-ala-ayuning-dewasa",
         "https://www.radio.denpasarkota.go.id/berita/relevansi-ala-ayuning-dewasa-sebagai-pedoman-melaksanakan-upacara-di-bali",
         "Radio Publik Denpasar Kota: Ala Ayuning Dewasa"),
        ("tatkala-co-sang-hyang-eta-eto",
         "https://tatkala.co/2020/06/07/sang-hyang-eta-eto-memahami-kalender-hindu-bali-baik-buruk-hari-dengan-rumusan-lanus/",
         "tatkala.co: Sang Hyang Eta-Eto (Lanus framework)"),
        ("sastra-bali-wewaran",
         "https://sastrabali.com/wewaran/",
         "Sastra Bali: Wewaran"),
        ("cakepane-wariga-dan-dewasa",
         "https://cakepane.blogspot.com/2010/04/wariga-dan-dewasa-merupakan-ilmu.html",
         "cakepane.blogspot: Wariga dan Dewasa"),
        ("komangputra-wariga-gemet-page2",
         "https://www.komangputra.com/lontar-bali-wariga-gemet.html/2",
         "komangputra.com: Wariga Gemet page 2 (folio 5a Tri Wara rules)"),
    ]
    for slug, url, label in items:
        dest = ensure_dir(base / slug)
        print(f"  [§8] {label}", flush=True)
        lp, sha = save_html(url, dest, "page.html", dry_run)
        results.append(make_result(8, slug, url, lp, sha,
                                   STATUS_OK if lp else STATUS_FAILED, label))
    return results


def fetch_section_9(_dry_run: bool) -> list[FetchResult]:
    base = section_dir(9)
    ensure_dir(base)
    readme = base / "README.md"
    if not readme.exists():
        readme.write_text(
            "# §9 — Physical / Print Books\n\n"
            "These cannot be automatically downloaded. Acquire via bookstores, libraries, or ILL.\n\n"
            "## Checklist\n\n"
            "- [ ] **I Ketut Bangbang Gde Rawi — *Wariga Dewasa* / *Kalender Bali*** "
            "(standard almanac; Balinese bookstores)\n"
            "- [ ] **Arya Suryawan — *Ala-Ayu Padewasan Wariga Dewasa*** "
            "(Tokopedia: https://www.tokopedia.com/mbukubali/ala-ayu-padewasan-wariga-dewasa)\n"
            "- [ ] **Ida Pedanda Gede Buruan — *Padewasan Kapelek*** "
            "(primary source for folio 11a Alahing Sasih citation)\n"
            "- [ ] **Fred B. Eiseman — *Bali: Sekala and Niskala Vol. I*** "
            "(ethnographic; library or second-hand)\n"
            "- [ ] **Ian Proudfoot — *Old Muslim Calendars of Southeast Asia* (Brill, 2006)** "
            "(ILL via university library)\n",
            encoding="utf-8",
        )
    items = [
        "rawi-wariga-dewasa-kalender-bali",
        "suryawan-ala-ayu-padewasan-wariga-dewasa",
        "buruan-padewasan-kapelek",
        "eiseman-bali-sekala-niskala",
        "proudfoot-old-muslim-calendars",
    ]
    return [
        make_result(9, slug, "", str(readme.relative_to(REPO_ROOT)), "",
                    STATUS_MANUAL, "Tier C: print book; see §9 README", "C")
        for slug in items
    ]


def fetch_section_10(dry_run: bool) -> list[FetchResult]:
    results = []
    base = section_dir(10)

    # Leiden KITLV — already hit in §5, snapshot here too if useful
    slug = "leiden-kitlv-digital-collections"
    dest = ensure_dir(base / slug)
    url = "https://digitalcollections.universiteitleiden.nl/"
    print(f"  [§10] Leiden KITLV landing [Tier B]", flush=True)
    lp, sha = save_html(url, dest, "page.html", dry_run)
    results.append(make_result(10, slug, url, lp, sha,
                               STATUS_SNAPSHOT if lp else STATUS_FAILED,
                               "Tier B: KITLV landing; search 'pasar Bali' / 'Bali markt' manually", "B"))

    # ANRI — Tier B; try http fallback if https fails
    slug = "anri-arsip-nasional"
    dest = ensure_dir(base / slug)
    url = "https://anri.go.id/"
    print(f"  [§10] ANRI landing [Tier B]", flush=True)
    lp, sha = save_html(url, dest, "page.html", dry_run)
    if not lp:
        lp, sha = save_html("http://anri.go.id/", dest, "page.html", dry_run)
    results.append(make_result(10, slug, url, lp, sha,
                               STATUS_SNAPSHOT if lp else STATUS_FAILED,
                               "Tier B: ANRI landing; Dutch colonial archives", "B"))

    # Tier C manual items
    manual_items = [
        ("pdk-bali-lontar-digitization",
         "Pusat Dokumentasi Kebudayaan Bali (PDK Bali) — lontar digitization program"),
        ("gedong-kirtya-singaraja",
         "Gedong Kirtya (Singaraja) — lontar library; some digitized"),
        ("unud-institutional-repository",
         "Universitas Udayana institutional repository — search SINTA & Unud eprints"),
        ("google-scholar-pawukon-wewaran-ac-id",
         "Google Scholar: 'pawukon' OR 'wewaran' site:ac.id"),
    ]
    for slug, note in manual_items:
        results.append(make_result(10, slug, "", "", "", STATUS_MANUAL,
                                   f"Tier C: {note}", "C"))

    readme = base / "README.md"
    if not readme.exists():
        readme.write_text(
            "# §10 — Archival Collections\n\n"
            "## Auto-fetched snapshots\n"
            "- Leiden KITLV landing page (`leiden-kitlv-digital-collections/page.html`)\n"
            "- ANRI landing page (`anri-arsip-nasional/page.html`)\n\n"
            "## Manual acquisition required\n"
            "- **PDK Bali**: visit or contact Pusat Dokumentasi Kebudayaan Bali, Denpasar.\n"
            "- **Gedong Kirtya**: Jl. Veteran, Singaraja; lontar holdings catalog.\n"
            "- **Unud repository**: https://eprints.unud.ac.id — search 'wariga' / 'pawukon'.\n"
            "- **Google Scholar ac.id dork**: `pawukon OR wewaran site:ac.id` in Scholar.\n",
            encoding="utf-8",
        )
    return results


# ---------------------------------------------------------------------------
# Manifest I/O
# ---------------------------------------------------------------------------

def load_manifest() -> dict[str, dict]:
    if MANIFEST_PATH.exists():
        with MANIFEST_PATH.open(encoding="utf-8") as f:
            data = json.load(f)
        return {item["slug"]: item for item in data.get("items", [])}
    return {}


def save_manifest(results: list[FetchResult]) -> None:
    existing = load_manifest()
    for r in results:
        existing[r.slug] = asdict(r)
    output = {
        "generated_at": now_iso(),
        "items": sorted(existing.values(), key=lambda x: (x["section"], x["slug"])),
    }
    with MANIFEST_PATH.open("w", encoding="utf-8") as f:
        json.dump(output, f, indent=2, ensure_ascii=False)


# ---------------------------------------------------------------------------
# resources.md updater
# ---------------------------------------------------------------------------

# Maps section number + slug → resources.md checkbox pattern
# We match on the slug keywords appearing in the URL or title line.
_SLUG_TO_MD_PATTERN: list[tuple[int, str, str, str]] = [
    # (section, slug, url_fragment, md_marker_to_match_on_line)
    # Used to find the right line and update its checkbox + append path.
]


def _checkbox_mark(status: str, tier: str) -> str:
    if tier == "C" or status == STATUS_MANUAL:
        return "[ ]"
    if status == STATUS_OK:
        return "[x]"
    if status == STATUS_SNAPSHOT:
        return "[~]"
    return "[ ]"


def update_resources_md(results: list[FetchResult]) -> None:
    """Update checkboxes and append local paths in references/resources.md."""
    text = RESOURCES_MD.read_text(encoding="utf-8")
    lines = text.splitlines(keepends=True)

    # Build a mapping of URL → result for fast lookup
    url_to_result: dict[str, FetchResult] = {}
    for r in results:
        if r.url:
            url_to_result[r.url] = r
        # also index by url fragment for robustness
        if r.url:
            url_to_result[r.url.rstrip("/")] = r

    def find_result_for_line(line: str) -> Optional[FetchResult]:
        # Look for a URL in the line and find the matching result
        url_match = re.search(r"https?://[^\s`]+", line)
        if not url_match:
            return None
        url = url_match.group(0).rstrip("`).>,")
        if url in url_to_result:
            return url_to_result[url]
        url_stripped = url.rstrip("/")
        if url_stripped in url_to_result:
            return url_to_result[url_stripped]
        # partial match on the URL
        for r_url, r in url_to_result.items():
            if r_url and (url in r_url or r_url in url):
                return r
        return None

    new_lines = []
    for line in lines:
        # Only process lines that start with a checkbox
        stripped = line.lstrip()
        if not stripped.startswith("- [ ]") and not stripped.startswith("- [~]") and not stripped.startswith("- [x]"):
            new_lines.append(line)
            continue

        result = find_result_for_line(line)
        if result is None:
            new_lines.append(line)
            continue

        new_mark = _checkbox_mark(result.status, result.tier)
        # Replace the existing checkbox marker
        updated = re.sub(r"\[ \]|\[~\]|\[x\]", new_mark, line, count=1)

        # Append local path (strip any existing annotation first to stay idempotent)
        if result.local_path:
            updated = re.sub(r"\s*→\s*`[^`]+`", "", updated.rstrip("\n")) + "\n"
            path_note = f" → `{result.local_path}`"
            updated = updated.rstrip("\n") + path_note + "\n"

        new_lines.append(updated)

    RESOURCES_MD.write_text("".join(new_lines), encoding="utf-8")
    print(f"\n  resources.md updated.", flush=True)


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

ALL_SECTIONS = list(range(1, 11))

SECTION_FETCHERS = {
    1: fetch_section_1,
    2: fetch_section_2,
    3: fetch_section_3,
    4: fetch_section_4,
    5: fetch_section_5,
    6: fetch_section_6,
    7: fetch_section_7,
    8: fetch_section_8,
    9: fetch_section_9,
    10: fetch_section_10,
}


def main() -> None:
    parser = argparse.ArgumentParser(description="Fetch Balinese calendar research resources.")
    parser.add_argument("--dry-run", action="store_true",
                        help="Skip network requests; just create directory structure and print plan.")
    parser.add_argument("--section", type=int, choices=ALL_SECTIONS,
                        help="Only fetch a specific section (1–10).")
    args = parser.parse_args()

    ensure_dir(DOWNLOADS_DIR)

    sections = [args.section] if args.section else ALL_SECTIONS
    all_results: list[FetchResult] = []

    for sec in sections:
        print(f"\n=== Section {sec}: {SECTION_DIRS[sec]} ===", flush=True)
        fetcher = SECTION_FETCHERS[sec]
        try:
            results = fetcher(args.dry_run)
        except Exception as exc:
            print(f"  [ERROR] Section {sec} failed: {exc}", flush=True)
            results = []
        all_results.extend(results)

    if not args.dry_run:
        save_manifest(all_results)
        update_resources_md(all_results)

    # Summary
    print("\n=== Summary ===", flush=True)
    counts = {STATUS_OK: 0, STATUS_SNAPSHOT: 0, STATUS_FAILED: 0, STATUS_MANUAL: 0, STATUS_SKIPPED: 0}
    for r in all_results:
        counts[r.status] = counts.get(r.status, 0) + 1
    for status, count in counts.items():
        if count:
            print(f"  {status:10s}: {count}", flush=True)
    print(f"  {'total':10s}: {len(all_results)}", flush=True)
    print(f"\nManifest: {MANIFEST_PATH}", flush=True)


if __name__ == "__main__":
    main()
