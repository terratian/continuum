# CONTINUUM COMMERCIAL OEM & ENTERPRISE LICENSE SPECIFICATION
**Licensor & Inventor:** Joshua Sultan (Burlington, North Carolina, US)  
**Protected Property:** The Continuum Software Suite (`continuumd`, `continuum-core`, `continuum-wasm`, Enterprise Signaling Control Plane, and Associated Patent Claims)

---

## 1. Purpose of the Dual-License Model
**Continuum** employs an asymmetric dual-licensing architecture designed to empower individual developers and open-source Linux/Android enthusiasts while prohibiting unauthorized corporate extraction ("poaching") or proprietary cloud bundling:

1. **Open-Source Community Tier (`LICENSE-AGPLv3`):** Free for personal, academic, and strictly open-source use where all derivative works, linked host containers, and network-accessible service wrappers are disclosed in full under the AGPLv3.
2. **Commercial OEM & Enterprise Tier (`LICENSE-COMMERCIAL.md`):** Required for any entity that wishes to:
   - Embed `continuumd` or `continuum-wasm` inside a closed-source operating system image, Virtual Application Delivery (VAD) platform (e.g., Cameyo, Citrix, cloud desktop hosts), or proprietary browser/PWA wrapper;
   - Operate the Continuum signaling coordinator or split-tunnel virtual I/O bridge as a managed commercial service without triggering AGPLv3 Section 13 source-code disclosure;
   - Obtain an explicit commercial patent license under the Inventor's pending and issued United States and International patents; or
   - Acquire or transfer ownership of an Enterprise Vault Mesh or territorial exclusivity.

---

## 2. Commercial Licensing Tiers & Valuation Structure

| Commercial Tier | Target Licensee | Included Rights & Patent Grant | Indicative Structure |
| :--- | :--- | :--- | :--- |
| **Tier A: Prosumer / Indie Sovereign Key** | Individual power users & small studios (<5 seats) | Signed binary releases, managed zero-knowledge TURN relay access, and on-chain L2 Vault NFT ownership key. | **$7/mo** or **$149 Lifetime Vault Key** |
| **Tier B: Enterprise Fleet & VAD OEM License** | Virtual App Delivery (VAD) platforms, enterprise IT fleets, and OS vendors | Full exemption from AGPLv3 copyleft; commercial patent license for Split-Tunnel I/O Peeling & `vhost-user` synthesis; private multi-tenant OAuth 2.1 coordinator binary. | **Per-Seat Annual Royalty** or **Enterprise Site License** |
| **Tier C: Strategic Acquisition / Exclusive Assignment** | Platform Acquirers (OS / Cloud Infrastructure Vendors) | Full assignment or exclusive field-of-use transfer of USPTO Patent Portfolio, Benelux BOIP i-DEPOT Trade Secrets, Trademark Rights, and Root Cryptographic Authority. | **Negotiated M&A / IP Assignment Agreement** |

---

## 3. Anti-Circumvention & Forensic Watermark Covenant
1. **Clean-Room & AI Translation Prohibition:** Because the architectural methods of Continuum (including Split-Tunnel VAD peripheral peeling, in-transit `WASI` state transcoding inside an `MLS` ratchet, and dual-path DID/OAuth ownership handoff) are the subject of pending federal utility patent claims, translating or re-implementing the source code into another programming language (whether via human clean-room teams or automated Large Language Model translation) without a Commercial License constitutes direct patent infringement under 35 U.S.C. § 271.
2. **Preservation of Forensic Canary Traps:** Removing, obfuscating, or altering the deterministic cryptographic watermarks (`0x434F4E54_494E5555`) or reserved Protobuf handshake fields (`419`, `882`, `1729`) in an unlicensed derivative binary constitutes willful infringement and a violation of the Digital Millennium Copyright Act (17 U.S.C. § 1202 — Integrity of Copyright Management Information).

---

## 4. Licensing Inquiries & Ownership Transfer Protocol
To execute a Commercial OEM License, request a clean-room due-diligence sandbox, or initiate a cryptographic Vault / IP ownership transfer, contact the Rights Holder:

* **Inventor & Licensor:** Joshua Sultan
* **Jurisdiction:** Burlington, North Carolina, United States
* **Repository Authority:** `git@github.com:terratian/continuum.git`
