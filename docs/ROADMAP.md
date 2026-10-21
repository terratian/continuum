# CONTINUUM: ENGINEERING & VALIDATION ROADMAP
**Author & Inventor:** Joshua Sultan (Burlington, North Carolina, US)  
**Primary Testbed Matrix:**
* **Node Alpha (Workstation & GPU Host):** Mid-2011 27" Apple iMac running Ubuntu Linux (`x86_64-unknown-linux-gnu`)
* **Node Beta (Container & VAD Client Host):** HP Chromebook 14 G6 running ChromeOS + Crostini Debian VM (`x86_64` over `CrosVM`/`VirtIO`)
* **Node Gamma-1 (Android Edge):** Samsung Galaxy A16 5G (`aarch64-linux-android`)
* **Node Gamma-2 (iOS Edge):** Apple iPhone 11 (`aarch64-apple-ios`)
* **Source Control & Provenance:** Private GitHub (`git@github.com:terratian/continuum.git`) + OpenTimestamps Bitcoin Anchor + Belgium/Benelux BOIP i-DEPOT

---

## Phase 1: Foundation, IP Lock & LAN Core Mesh (Weeks 1 – 3)
- [x] **M1.1 — Workspace & IP Sealing Bootstrap:** Initialize Cargo workspace (`continuum-core`, `continuumd`, `continuum-wasm`), `scripts/seal_ip_ledger.sh`, OpenTimestamps `.ots` generation, Belgium BOIP i-DEPOT archive preparation, and USPTO Provisional Patent specification.
- [ ] **M1.2 — Protobuf Schema & Canary Traps (`continuum-core`):** Implement `continuum.proto` via `prost`, embedding forensic canary tags (`419`, `882`, `1729`) and `BLAKE3` chunk verification.
- [ ] **M1.3 — Zero-Conf LAN Discovery (`mdns-sd`):** Broadcast `_continuum._udp.local` service records on Node Alpha (Ubuntu iMac) and discover across the Crostini VM bridge on Node Beta.
- [ ] **M1.4 — Encrypted Transport & MLS Ratchet:** Establish authenticated P2P `QUIC` and `WebRTC` DataChannels using an `Argon2id` bootstrap phrase or local terminal QR code exchange.
- [ ] **M1.5 — Live `ContinuumBuffer` & `ContinuumDrop` Test (Ubuntu <-> Crostini):** Hook `wl-clipboard` / `xclip`, persist encrypted clipboard history in local `SQLite`, and verify sub-50ms copy-paste and P2P directory drops.

## Phase 2: Browser Sandbox, WASI Transcoding & Split-Tunnel VAD Bridge (Weeks 4 – 6)
- [ ] **M2.1 — `continuum-wasm` Compilation:** Compile packet framing and WebRTC signaling to `wasm32-unknown-unknown`.
- [ ] **M2.2 — Chrome Manifest V3 Extension & Native Messaging Bridge:** Build MV3 extension using `chrome.offscreen` and `connectNative` host bridge to `continuumd`.
- [ ] **M2.3 — Sandboxed `WASI Preview 2` Buffer Transcoder:** Embed `wasmtime` inside `continuumd` to execute `transcode_mime.wasm` in transit.
- [ ] **M2.4 — Split-Tunnel VAD Proof-of-Concept:** Route clipboard and local I/O directly between a streamed web app viewport on the Chromebook and the Ubuntu iMac over LAN P2P.

## Phase 3: Mobile Edge Nodes (`Galaxy A16` & `iPhone 11`) & Kernel Virtualization (Weeks 7 – 10)
- [ ] **M3.1 — Android Client (`Galaxy A16 5G`):** Cross-compile `continuum-core` via `cargo-ndk` with Foreground Service + Quick Settings Tile.
- [ ] **M3.2 — iOS Client & Sandbox Keep-Alive (`iPhone 11`):** Build XCFramework with 1x1 `AVPictureInPictureController` keep-alive and `UIInputViewController` keyboard socket.
- [ ] **M3.3 — `ContinuumSense` Virtual Camera Injection:** Pipe mobile WebRTC video streams into Ubuntu `/dev/video10` via `v4l2loopback`.
- [ ] **M3.4 — `ContinuumGate` Proximity Biometric Linux PAM Module (`pam_continuum.so`):** Unlock Ubuntu `sudo` via mobile biometric signature + BLE RSSI `< 1.5m` verification.
- [ ] **M3.5 — `ContinuumKVM` & `ContinuumDisplay`:** Hook `libei` / `uinput` for cross-screen cursor and keyboard transitions.

## Phase 4: `ContinuumCompute` GPU Offloading & Branch B Trustless Registry (Weeks 11 – 14)
- [ ] **M4.1 — `ContinuumCompute` Distributed Worker Pool:** Offload optical CV, media transcoding, and local LLM tasks from Chromebook/Mobile to the Ubuntu iMac GPU.
- [ ] **M4.2 — Branch B (`rust-libp2p` Serverless Signaling):** Implement `Kademlia DHT`, `AutoNAT`, `Circuit Relay v2`, `DCUtR`, and `Noise` handshake.
- [ ] **M4.3 — L2 Smart Contract Device Registry & Vault Ownership Transfer:** Deploy `ContinuumRegistry.sol` and verify automatic `MLS` key rotation upon ERC-721 Vault token transfer.

## Phase 5: IP Consolidation, Packaging & OEM / Enterprise Readiness (Weeks 15 – 16)
- [ ] **M5.1 — Final IP Ledger Seal:** Anchor `.ots` proof on Bitcoin, upload final archive to Belgium BOIP i-DEPOT, and file USPTO Provisional Patent application ($65 Micro-Entity).
- [ ] **M5.2 — Stripped Binary Demo Packaging:** Produce watermarked binaries and obfuscated Wasm bundles for enterprise VAD / ChromeOS demonstrations.
