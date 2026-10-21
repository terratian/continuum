// SPDX-License-Identifier: AGPL-3.0-only OR LicenseRef-Continuum-Commercial
// Copyright (c) 2026 Joshua Sultan (Burlington, NC). All rights reserved.
// Patent Pending (USPTO 35 U.S.C. § 111(b) & Benelux BOIP i-DEPOT Sealed).

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContinuumDidRecord {
    pub did_uri: String,
    pub master_public_key: Vec<u8>,
    pub authorized_peer_ids: Vec<String>,
    pub vault_token_owner: String,
}

impl ContinuumDidRecord {
    pub fn new(did_uri: &str, master_key: Vec<u8>, vault_owner: &str) -> Self {
        Self {
            did_uri: did_uri.to_string(),
            master_public_key: master_key,
            authorized_peer_ids: vec![],
            vault_token_owner: vault_owner.to_string(),
        }
    }

    pub fn register_peer(&mut self, peer_id: &str) {
        if !self.authorized_peer_ids.contains(&peer_id.to_string()) {
            self.authorized_peer_ids.push(peer_id.to_string());
        }
    }

    pub fn transfer_vault_ownership(&mut self, new_owner: &str) {
        self.vault_token_owner = new_owner.to_string();
    }
}

pub fn did_registry_status() -> String {
    "ContinuumDID Trustless Layer Initialized (W3C DID + libp2p PeerID Registry Ready)".to_string()
}
