#!/usr/bin/env bash
set -e

VERSION="0.1.0"
TARGET_DIR="dist/continuum-v${VERSION}"

echo "[*] Initializing Release Candidate Packaging for Continuum v${VERSION}..."
mkdir -p "${TARGET_DIR}/bin" "${TARGET_DIR}/docs"

echo "[*] Building release workspace binaries..."
cargo build --release --workspace

echo "[*] Copying compiled binaries and libraries..."
if [ -f "target/release/continuumd" ]; then
    cp target/release/continuumd "${TARGET_DIR}/bin/"
fi
if [ -f "target/release/libcontinuum_shims.so" ]; then
    cp target/release/libcontinuum_shims.so "${TARGET_DIR}/bin/"
fi

echo "[*] Copying architectural documentation and licenses..."
cp -r docs/*.md "${TARGET_DIR}/docs/"
cp LICENSE-APACHE LICENSE-AGPLv3 LICENSE-COMMERCIAL.md CLA.md "${TARGET_DIR}/"

echo "[*] Creating release archive..."
tar -czf "dist/continuum-v${VERSION}-release.tar.gz" -C dist "continuum-v${VERSION}"
echo "[+] Release candidate package successfully created at: dist/continuum-v${VERSION}-release.tar.gz"
