#!/bin/bash
set -e

PROG="name"
OUTDIR="target/release"

echo "Starting cross-platform builds using cargo..."

mkdir -p "$OUTDIR"

# -------------------------
# Linux (x86_64 MUSL static)
# -------------------------
echo "Building Linux x86_64..."
cargo build --release --target x86_64-unknown-linux-musl
powershell.exe Copy-Item "target/x86_64-unknown-linux-musl/release/$PROG" "$OUTDIR/${PROG}-linux-amd64" -Force

# -------------------------
# Linux ARM64 (AArch64)
# -------------------------
echo "Building Linux ARM64..."
cargo build --release --target aarch64-unknown-linux-gnu
powershell.exe Copy-Item "target/aarch64-unknown-linux-gnu/release/$PROG" "$OUTDIR/${PROG}-linux-arm64" -Force

# -------------------------
# Linux RISC-V 64 (gc)
# -------------------------
echo "Building Linux RISC-V..."
cargo build --release --target riscv64gc-unknown-linux-gnu
powershell.exe Copy-Item "target/riscv64gc-unknown-linux-gnu/release/$PROG" "$OUTDIR/${PROG}-linux-riscv64gc" -Force

# -------------------------
# Windows
# -------------------------
echo "Building Windows x86_64..."
cargo build --release --target x86_64-pc-windows-gnu
powershell.exe Copy-Item "target/x86_64-pc-windows-gnu/release/${PROG}.exe" "$OUTDIR/${PROG}-windows-amd64.exe" -Force

# -------------------------
# macOS (Intel)
# -------------------------
echo "Building macOS x86_64..."
cargo build --release --target x86_64-apple-darwin
powershell.exe Copy-Item "target/x86_64-apple-darwin/release/$PROG" "$OUTDIR/${PROG}-macos-amd64" -Force

# -------------------------
# macOS (Apple Silicon)
# -------------------------
echo "Building macOS ARM64..."
cargo build --release --target aarch64-apple-darwin
powershell.exe Copy-Item "target/aarch64-apple-darwin/release/$PROG" "$OUTDIR/${PROG}-macos-arm64" -Force

# -------------------------
echo "All builds complete!"
echo "Binaries are in $OUTDIR:"
ls -lh "$OUTDIR/${PROG}-"*