// SPDX-License-Identifier: AGPL-3.0-only OR LicenseRef-Continuum-Commercial
// Copyright (c) 2026 Joshua Sultan (Burlington, NC). All rights reserved.
// Trade Secret / Unfiled Architecture (Benelux BOIP i-DEPOT & SHA-256 Sealed).

use continuum_core::{ContinuumEnvelope, PayloadType};
use continuum_net::ContinuumTransportNode;
use continuum_shims::pam_stub;
use continuum_mobile::mobile_edge_status;
use continuum_compute::compute_worker_status;
use continuum_did::{did_registry_status, ContinuumDidRecord};
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use rusqlite::{params, Connection};
use std::error::Error;
use std::net::{IpAddr, SocketAddr};
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

    // 1. Initialize Quinn QUIC Transport Server
    let bind_addr: SocketAddr = "0.0.0.0:7419".parse()?;
    let transport_node = ContinuumTransportNode::new(bind_addr);
    let quic_endpoint = transport_node.start_quic_server().await?;
    log::info!("[ContinuumNet] QUIC transport listening on port 7419 (<50ms LAN fast-path active)");

    // Spawn background task to accept incoming QUIC connections
    tokio::spawn(async move {
        while let Some(incoming) = quic_endpoint.accept().await {
            log::info!("[ContinuumNet] Incoming QUIC connection attempt from IP: {}", incoming.remote_address());
            tokio::spawn(async move {
                match incoming.await {
                    Ok(connection) => {
                        log::info!("[ContinuumNet] Established secure QUIC connection with peer: {}", connection.remote_address());
                    }
                    Err(e) => {
                        log::error!("[ContinuumNet] Failed to establish QUIC connection: {}", e);
                    }
                }
            });
        }
    });

    // 2. Initialize mDNS Zero-Conf LAN Discovery & Active Peer Browser
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

    let receiver = mdns.browse(service_type)?;
    tokio::spawn(async move {
        while let Ok(event) = receiver.recv_async().await {
            match event {
                ServiceEvent::ServiceResolved(info) => {
                    log::info!("[mDNS Peer Discovered] Name: {} | Addr: {:?} | Port: {}", info.get_fullname(), info.get_addresses(), info.get_port());
                }
                ServiceEvent::ServiceRemoved(_, fullname) => {
                    log::info!("[mDNS Peer Offline] Name: {}", fullname);
                }
                _ => {}
            }
        }
    });

    // Test Branch B DID Vault Record & Ownership Transfer Stub
    let mut sample_did = ContinuumDidRecord::new("did:continuum:sultan01", vec![0xAB, 0xCD], "0xOwnerA");
    sample_did.register_peer("12D3KooW-SamplePeerID-Crostini");
    log::info!("[ContinuumDID] Initialized DID Record: {:?}", sample_did);
    sample_did.transfer_vault_ownership("0xOwnerB_Acquirer");
    log::info!("[ContinuumDID] Vault Ownership Transferred to: {}", sample_did.vault_token_owner);

    // Test PAM proximity stub initialization
    let sample_auth = pam_stub::verify_proximity_auth(-55, true);
    log::info!("[ContinuumGate] PAM Proximity Auth Test (-55 dBm, Enclave Sig Valid) -> Access: {}", sample_auth);

    // 3. Initialize Local SQLite Vault (`ContinuumBuffer`)
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

    // 4. Start Local Clipboard Polling & Sync Loop (Graceful Headless Fallback)
    let mut clipboard_opt = match arboard::Clipboard::new() {
        Ok(cb) => {
            log::info!("[ContinuumBuffer] Native pasteboard access initialized successfully.");
            Some(cb)
        }
        Err(_) => {
            log::warn!("[ContinuumBuffer] Headless environment detected (no display server). Operating in headless sync-node mode.");
            None
        }
    };

    let mut last_text = String::new();
    if let Some(ref mut cb) = clipboard_opt {
        last_text = cb.get_text().unwrap_or_default();
    }
    log::info!("[ContinuumBuffer] Monitoring system pasteboard for deltas...");

    let mut interval = tokio::time::interval(Duration::from_millis(500));

    loop {
        interval.tick().await;

        if let Some(ref mut cb) = clipboard_opt {
            if let Ok(current_text) = cb.get_text() {
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
}
