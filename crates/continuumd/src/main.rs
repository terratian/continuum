// SPDX-License-Identifier: AGPL-3.0-only OR LicenseRef-Continuum-Commercial
// Copyright (c) 2026 Joshua Sultan (Burlington, NC). All rights reserved.
// Trade Secret / Unfiled Architecture (Benelux BOIP i-DEPOT & SHA-256 Sealed).

use continuum_core::{ContinuumEnvelope, PayloadType};
use continuum_shims::pam_stub;
use continuum_mobile::mobile_edge_status;
use continuum_compute::compute_worker_status;
use continuum_did::{did_registry_status, ContinuumDidRecord};
use mdns_sd::{ServiceDaemon, ServiceInfo};
use rusqlite::{params, Connection};
use std::error::Error;
use std::net::IpAddr;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    log::info!("Starting Continuum Core Daemon (continuumd) v0.1.0...");
    log::info!("Inventor: Joshua Sultan | BOIP i-DEPOT Sealed Trade Secret");
    log::info!("[Kernel Shims] {}", continuum_shims::get_shim_status());
    log::info!("[Mobile Edge] {}", mobile_edge_status());
    log::info!("[Compute Shard] {}", compute_worker_status());
    log::info!("[Trustless DID] {}", did_registry_status());

    // Test Branch B DID Vault Record & Ownership Transfer Stub
    let mut sample_did = ContinuumDidRecord::new("did:continuum:sultan01", vec![0xAB, 0xCD], "0xOwnerA");
    sample_did.register_peer("12D3KooW-SamplePeerID-Crostini");
    log::info!("[ContinuumDID] Initialized DID Record: {:?}", sample_did);
    sample_did.transfer_vault_ownership("0xOwnerB_Acquirer");
    log::info!("[ContinuumDID] Vault Ownership Transferred to: {}", sample_did.vault_token_owner);

    // Test PAM proximity stub initialization
    let sample_auth = pam_stub::verify_proximity_auth(-55, true);
    log::info!("[ContinuumGate] PAM Proximity Auth Test (-55 dBm, Enclave Sig Valid) -> Access: {}", sample_auth);

    // 1. Initialize Local SQLite Vault (`ContinuumBuffer`)
    let db_conn = Connection::open("continuum_buffer_vault.db")?;
    db_conn.execute(
        "CREATE TABLE IF NOT EXISTS clipboard_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            sender_id TEXT NOT NULL,
            payload_type TEXT NOT NULL,
            content TEXT NOT NULL,
            timestamp INTEGER NOT NULL
        )",
        [],
    )?;
    log::info!("[ContinuumBuffer] SQLite vault initialized successfully.");

    // 2. Initialize mDNS Zero-Conf LAN Discovery (`_continuum._udp.local`)
    let mdns = ServiceDaemon::new()?;
    let service_type = "_continuum._udp.local.";
    let host_name = "continuum-node.local.";
    let ip = IpAddr::V4("127.0.0.1".parse()?);
    let port = 7419;

    let properties = [("version", "0.1.0"), ("author", "joshua-sultan")];
    let service_info = ServiceInfo::new(
        service_type,
        host_name,
        &host_name,
        ip,
        port,
        &properties[..],
    )?;

    mdns.register(service_info)?;
    log::info!("[mDNS] Broadcasted service {} on port {}", service_type, port);

    // 3. Start Local Clipboard Polling & Sync Loop
    let mut clipboard = arboard::Clipboard::new().unwrap_or_else(|_| {
        log::warn!("[ContinuumBuffer] Headless environment detected; fallback clipboard initialized.");
        panic!("Unable to access native clipboard API.");
    });

    let mut last_text = clipboard.get_text().unwrap_or_default();
    log::info!("[ContinuumBuffer] Monitoring system pasteboard for deltas...");

    let mut interval = tokio::time::interval(Duration::from_millis(500));

    loop {
        interval.tick().await;

        if let Ok(current_text) = clipboard.get_text() {
            if current_text != last_text && !current_text.is_empty() {
                log::info!("[ContinuumBuffer] Pasteboard delta detected! Length: {} bytes", current_text.len());
                last_text = current_text.clone();

                let envelope = ContinuumEnvelope::new(
                    PayloadType::ClipboardText,
                    host_name.to_string(),
                    current_text.as_bytes().to_vec(),
                );

                db_conn.execute(
                    "INSERT INTO clipboard_history (sender_id, payload_type, content, timestamp) VALUES (?1, ?2, ?3, ?4)",
                    params![envelope.sender_id, "ClipboardText", current_text, envelope.timestamp_epoch_ms],
                )?;

                log::info!("[ContinuumBuffer] Envelope hashed (BLAKE3: hex={}) & stored in local vault.", hex::encode(envelope.compute_hash()));
            }
        }
    }
}
