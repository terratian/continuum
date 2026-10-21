# CONTINUUM: MASTER ARCHITECTURAL & ENGINEERING BIBLE
**Author & Inventor:** Joshua Sultan (Burlington, North Carolina, US)
**Classification:** Confidential Trade Secret / Patent Pending Architecture

## 1. Core Thesis & Mission
Continuum is an instruction-set-agnostic presence, state, and hardware virtualization backplane. It unifies fragmented operating environments—Ubuntu Linux (Wayland/X11), ChromeOS (Crostini/ARCVM), Android, iOS/iPadOS, Windows, macOS, and streamed Virtual Application Delivery (VAD / Cameyo) viewports—across heterogeneous silicon (`x86_64`, `ARM64`, `RISC-V`).

## 2. Primary Development Environment & Hardware Testbed
1. **Node Alpha (Primary Workstation):** Apple iMac running native Ubuntu Linux (`x86_64-unknown-linux-gnu`) — primary Rust compilation host, `wgpu` discrete compute target (`ContinuumCompute`), Wayland/`libei`/`v4l2loopback` kernel testbed, and local signaling coordinator.
2. **Node Beta (Virtualized Container Host):** HP Chromebook 14 G6 running ChromeOS with Crostini Debian Linux VM (`x86_64` guest over `CrosVM` / `VirtIO`) — primary testbed for `virtio-wl` / `vhost-user` hardware synthesis, Chrome Manifest V3 `offscreen` extension bridging, and Isolated Web App (IWA) split-tunnel routing.
3. **Node Gamma (Mobile Edge Nodes):** Samsung Galaxy A16 5G (Android `aarch64`) and Apple iPhone 11 (iOS `aarch64`) — edge peripheral providers for `ContinuumSense` (camera feed injection), `ContinuumGate` (BLE RSSI biometric PAM unlock), and `ContinuumBuffer` (cross-OS pasteboard sync).

## 3. The 4-Layer Technical Stack
1. **Layer 1 (`continuumd`):** Rust system daemon (<15MB RAM) managing `tokio` async I/O, `webrtc-rs` DataChannels, `mdns-sd` zero-conf LAN discovery, and local Unix domain IPC.
2. **Layer 2 (`continuum-wasm`):** Deterministic sandboxed `WASI Preview 2` micro-modules running inside an IETF RFC 9420 Messaging Layer Security (`MLS`) ratchet to transcode OS-specific clipboard and state buffers in transit across CPU architectures.
3. **Layer 3 (Kernel & Virtualization Shims):** Hardware synthesis via `vhost-user`, `v4l2loopback`, `libei`, `xdg-desktop-portal`, and `pam_continuum.so`.
4. **Layer 4 (Dual-Path Identity & Signaling):**
   - **Branch A (Federated Zero-Knowledge):** OAuth 2.1 + PKCE signaling broker with strictly client-side `Argon2id` + `MLS` TreeKEM payload encryption.
   - **Branch B (Trustless Sovereign):** W3C `DID` (`did:key` / `did:ethr`) + Layer-2 Smart Contract Device Registry + `libp2p` (`AutoNAT`, `Circuit Relay v2`, `DCUtR`, `Noise` handshake) with ERC-721 Vault token ownership transfer.

## 4. Forensic Canary Traps (Anti-Poaching Watermarks)
- Non-sequential Protobuf wire tags (`419`, `882`, `1729`) are permanently reserved in `continuum.proto` handshake headers.
- Deterministic state-transition salt (`0x434F4E54_494E5555`) is embedded in the `MLS` leaf-node extension payload.
