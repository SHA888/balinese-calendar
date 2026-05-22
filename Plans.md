# Plans — balinese-calendar

## v0.2.2 — Wariga Lookup Completeness

Patch release: Complete two placeholder-algorithm Wariga lookup tables from OCR sources. No API changes, pure data extraction.

| Task | Description | DoD | Depends | Status |
|------|-------------|-----|---------|--------|
| 2.1 | **Dauh Sukaranti** — Extract 12×5 lookup table from Wariga Sundari Bungkah | fixture JSON created, tests passing, impl replaces placeholder | - | cc:完了 |
| 2.2 | **Tenung Patemuan Adan** — Extract letter→urip mapping (18 consonant groups) from Lontar Joyoboyo | fixture JSON created, tests passing, impl replaces placeholder | - | cc:完了 |
| 2.3 | Verify tests pass & prepare release | all tests passing, CHANGELOG.md updated with v0.2.2 notes | 2.1, 2.2 | cc:TODO |

### Release checklist (trigger for v0.2.2)
- [ ] Both 2.1 and 2.2 complete and tested
- [ ] `cargo semver-checks` passes
- [ ] Fixture tests cover all 12 urip values (Dauh) and representative letters (Tenung)
- [ ] CHANGELOG.md updated
- [ ] Git tag `v0.2.2` created
