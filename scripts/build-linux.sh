#!/bin/bash
# Build MediTracker for Linux (x86_64 and aarch64)
# Requires: rustup, cross (cargo install cross) or Docker

set -e
cd "$(dirname "$0")/.."

echo "Building MediTracker for Linux..."

for target in x86_64-unknown-linux-musl aarch64-unknown-linux-musl; do
    echo "  -> $target"
    rustup target add "$target" 2>/dev/null || true
    cross build --target "$target" --release 2>/dev/null || cargo build --release --target "$target"
    mkdir -p release
    cp "target/$target/release/meditracker" "release/meditracker-$target" 2>/dev/null || cp "target/$target/release/meditracker" "release/"
done

echo "Done. Binaries in ./release/"
