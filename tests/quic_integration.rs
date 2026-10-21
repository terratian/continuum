// SPDX-License-Identifier: AGPL-3.0-only OR LicenseRef-Continuum-Commercial
// Copyright (c) 2026 Joshua Sultan (Burlington, NC). All rights reserved.
// Patent Pending (USPTO 35 U.S.C. § 111(b) & Benelux BOIP i-DEPOT Sealed).

use continuum_net::ContinuumTransportNode;
use std::net::SocketAddr;

#[tokio::test]
async fn test_quic_server_loopback_initialization() {
    let bind_addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let transport_node = ContinuumTransportNode::new(bind_addr);
    
    // Initialize QUIC server endpoint with ephemeral port
    let endpoint_result = transport_node.start_quic_server().await;
    assert!(endpoint_result.is_ok(), "Failed to bind Quinn QUIC server endpoint");
    
    let endpoint = endpoint_result.unwrap();
    let local_addr = endpoint.local_addr().unwrap();
    log::info!("[Test] QUIC server successfully bound and listening on {}", local_addr);
    
    endpoint.close(0u32.into(), b"test complete");
}
