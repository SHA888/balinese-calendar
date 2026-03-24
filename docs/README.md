# Balinese Calendar Documentation

This directory contains the source for the Balinese Calendar documentation, built with [mdBook](https://rust-lang.github.io/mdBook/).

## Building the Documentation

### Prerequisites

Install mdBook:

```bash
cargo install mdbook
```

### Build

```bash
cd docs
mdbook build
```

The built documentation will be in `docs/book/`.

### Serve Locally

To preview the documentation with live reload:

```bash
cd docs
mdbook serve
```

Then open http://localhost:3000 in your browser.

## Documentation Structure

```
docs/
├── book.toml           # mdBook configuration
├── src/
│   ├── SUMMARY.md      # Table of contents
│   ├── introduction.md # Landing page
│   ├── guide/          # User guides
│   ├── concepts/       # Conceptual documentation
│   ├── api/            # API reference
│   ├── development/    # Development guides
│   └── reference/      # Reference materials
└── book/               # Generated output (gitignored)
```

## Contributing to Documentation

1. Edit the markdown files in `docs/src/`
2. Build and preview locally with `mdbook serve`
3. Commit your changes
4. The documentation will be automatically deployed to GitHub Pages on merge to main

## Style Guide

- Use clear, concise language
- Include code examples where helpful
- Use proper markdown formatting
- Add links to related sections
- Keep examples practical and tested

## Deployment

Documentation is automatically built and deployed to GitHub Pages via the `.github/workflows/docs.yml` workflow.

The live documentation is available at:
- **User Guide**: https://sha888.github.io/balinese-calendar/
- **API Reference**: https://sha888.github.io/balinese-calendar/api/balinese_calendar/
