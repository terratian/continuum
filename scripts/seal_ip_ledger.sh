#!/usr/bin/env bash
set -euo pipefail

TIMESTAMP=$(date -u +"%Y%m%dT%H%M%SZ")
ARCHIVE_NAME="continuum_ip_seal_${TIMESTAMP}.tar.gz"
mkdir -p .ip_vault

echo "[*] Generating deterministic SHA-256 manifest of all source & doc files..."
find Cargo.toml crates docs scripts -type f -print0 | sort -z | xargs -0 sha256sum > docs/SHA256SUMS.txt

echo "[*] Creating archive for OpenTimestamps & Belgium BOIP i-DEPOT..."
tar --sort=name --mtime='UTC 2026-01-01' -czf ".ip_vault/${ARCHIVE_NAME}" Cargo.toml crates docs scripts

MASTER_HASH=$(sha256sum ".ip_vault/${ARCHIVE_NAME}" | awk '{print $1}')
echo "${TIMESTAMP} | ${ARCHIVE_NAME} | SHA256: ${MASTER_HASH}" >> docs/IP_LEDGER.log

if command -v ots &> /dev/null; then
    echo "[*] Anchoring Master SHA-256 Hash to Bitcoin Blockchain via OpenTimestamps..."
    ots stamp ".ip_vault/${ARCHIVE_NAME}"
else
    echo "[!] 'ots' CLI not found; local SHA-256 ledger updated. Install opentimestamps-client to anchor."
fi

echo "[+] IP Seal Complete. Master Hash: ${MASTER_HASH}"
echo "[+] Upload .ip_vault/${ARCHIVE_NAME} to BOIP i-DEPOT (Belgium/Benelux) for statutory EU certification."
