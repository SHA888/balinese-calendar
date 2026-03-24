# Installation

## Requirements

- **Rust**: 1.70.0 or later (MSRV - Minimum Supported Rust Version)
- **Cargo**: Comes with Rust installation

## Installing Rust

If you don't have Rust installed, visit [rustup.rs](https://rustup.rs/) and follow the instructions for your platform.

### Linux/macOS

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Windows

Download and run [rustup-init.exe](https://rustup.rs/).

## Adding to Your Project

### Using Cargo

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
balinese-calendar = "0.1.2"
```

Then run:

```bash
cargo build
```

### With Features

Enable optional features as needed:

```toml
[dependencies]
balinese-calendar = { version = "0.1.2", features = ["astronomical", "serde"] }
```

Available features:
- **`astronomical`**: Astronomical calculations (sunrise/sunset)
- **`serde`**: Serialization support for all types

### From Git

To use the latest development version:

```toml
[dependencies]
balinese-calendar = { git = "https://github.com/SHA888/balinese-calendar" }
```

Or a specific branch:

```toml
[dependencies]
balinese-calendar = { git = "https://github.com/SHA888/balinese-calendar", branch = "develop" }
```

## Verifying Installation

Create a simple test program:

```rust
use balinese_calendar::BalineseDate;

fn main() {
    match BalineseDate::from_ymd(2026, 3, 25) {
        Ok(date) => println!("✓ Installation successful! Wuku: {}", date.wuku.name()),
        Err(e) => eprintln!("✗ Error: {}", e),
    }
}
```

Run it:

```bash
cargo run
```

You should see: `✓ Installation successful! Wuku: Sinta`

## Platform Support

The crate is tested on:
- Linux (Ubuntu, Debian, Fedora)
- macOS (Intel and Apple Silicon)
- Windows (MSVC and GNU toolchains)

## Troubleshooting

### Compilation Errors

If you encounter compilation errors, ensure:
1. Your Rust version is 1.70.0 or later: `rustc --version`
2. Dependencies are up to date: `cargo update`
3. Clean build: `cargo clean && cargo build`

### MSRV Issues

If you need to use an older Rust version, check the [changelog](../reference/changelog.md) for the last version supporting your Rust version.

### Network Issues

If cargo fails to download dependencies:
1. Check your internet connection
2. Try using a different registry mirror
3. Check if you're behind a proxy

## Next Steps

Now that you have the crate installed, proceed to the [Quick Start](./quick-start.md) guide!
