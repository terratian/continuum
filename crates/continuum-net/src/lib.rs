// SPDX-License-Identifier: AGPL-3.0-only OR LicenseRef-Continuum-Commercial
// Copyright (c) 2026 Joshua Sultan (Burlington, NC). All rights reserved.
// Patent Pending (USPTO 35 U.S.C. § 111(b) & Benelux BOIP i-DEPOT Sealed).

use continuum_core::{ContinuumEnvelope, PayloadType, CONTINUUM_MAGIC_SALT};
use quinn::{Endpoint, ServerConfig};
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;

pub struct ContinuumTransportNode {
    bind_addr: SocketAddr,
}

impl ContinuumTransportNode {
    pub fn new(bind_addr: SocketAddr) -> Self {
        Self { bind_addr }
    }

    /// Initializes a QUIC server endpoint with self-signed TLS certs and ALPN "continuum-quic-v1"
    pub async fn start_quic_server(&self) -> Result<Endpoint, Box<dyn Error>> {
        let cert = rcgen::generate_simple_self_signed(vec!["continuum-node.local".into()])?;
        let cert_der = cert.cert.der().to_vec();
        let priv_key = cert.key_pair.serialize_der();

        let cert_chain = vec![rustls::pki_types::CertificateDer::from(cert_der)];
        let key_der = rustls::pki_types::PrivateKeyDer::Pkcs8(priv_key.into());

        let mut server_crypto = rustls::ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(cert_chain, key_der)?;

        server_crypto.alpn_protocols = vec![b"continuum-quic-v1".to_vec()];

        let server_config = ServerConfig::with_crypto(Arc::new(server_crypto));
        let endpoint = Endpoint::server(server_config, self.bind_addr)?;

        log::info!("[ContinuumNet] QUIC transport listener active on {}", self.bind_addr);
        Ok(endpoint)
    }
}

/// Validates an incoming QUIC connection stream payload against the Continuum canary watermark
pub fn validate_handshake_envelope(envelope: &ContinuumEnvelope) -> bool {
    envelope.canary.magic == CONTINUUM_MAGIC_SALT && envelope.canary.epoch_jitter == 419
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quic_handshake_canary_validation() {
        let env = ContinuumEnvelope::new(
            PayloadType::HeartbeatPing,
            "node-alpha".to_string(),
            vec![],
        );
        assert!(validate_handshake_envelope(&env));
    }
}
