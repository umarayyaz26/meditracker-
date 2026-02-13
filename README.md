# MediTracker

A fast, reliable CLI tool for healthcare professionals to manage patient and medication records. Built with Rust.

## Links

- **GitHub Repository:** https://github.com/umarayyaz26/meditracker
- **Website (advertisement):** https://umarayyaz26.github.io/meditracker

---

## Features

- **Patient management** — Add, view, and remove patients (ID, name, age, gender, disease)
- **Medication tracking** — Add, update, and remove medications with schedules per patient
- **JSON persistence** — Data saved automatically to `~/.meditracker/data.json`
- **CSV export** — Export all records for analysis or sharing
- **CLI arguments** — Custom data path via `--data-path`
- **Cross-platform Linux binaries** — Pre-built for `x86_64` and `aarch64` (musl)

## Installation

### Pre-built binaries (Linux)

Download from [Releases](https://github.com/umarayyaz26/meditracker/releases):

```bash
# x86_64
curl -sL https://github.com/umarayyaz26/meditracker/releases/latest/download/meditracker-x86_64-unknown-linux-musl.tar.gz | tar xz
chmod +x meditracker && ./meditracker --help

# aarch64 (ARM)
curl -sL https://github.com/umarayyaz26/meditracker/releases/latest/download/meditracker-aarch64-unknown-linux-musl.tar.gz | tar xz
```

### From source

```bash
git clone https://github.com/umarayyaz26/meditracker
cd meditracker
cargo build --release
```

The binary will be at `target/release/meditracker`.

## Usage

```bash
meditracker              # Use default data path (~/.meditracker/data.json)
meditracker --data-path /path/to/data.json
meditracker --help
meditracker --version
```

Interactive menu:

```
╔══════════════════════════════════════╗
║         MediTracker v1.0.2           ║
╚══════════════════════════════════════╝
  1. Add Patient
  2. View Patients
  3. Add Medication
  4. View Medications
  5. Remove Medication
  6. Update Medication
  7. Remove Patient
  8. Export to CSV
  9. Exit
```

## Building Linux binaries locally

```bash
# Install cross (requires Docker)
cargo install cross

# Build for x86_64 Linux
cross build --target x86_64-unknown-linux-musl --release

# Build for aarch64 Linux
cross build --target aarch64-unknown-linux-musl --release
```

Or use the script (Linux/macOS):

```bash
./scripts/build-linux.sh
```

## Project structure

```
meditracker/
├── src/
│   ├── main.rs      # Entry point and menu logic
│   ├── models.rs    # Patient, Medication structs
│   ├── storage.rs   # JSON load/save
│   └── io_utils.rs  # Input helpers
├── website/         # Advertisement site
│   ├── index.html
│   └── styles.css
├── scripts/
│   └── build-linux.sh
└── Cargo.toml
```

## License

MIT
