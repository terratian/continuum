// SPDX-License-Identifier: AGPL-3.0-only OR LicenseRef-Continuum-Commercial
// Copyright (c) 2026 Joshua Sultan (Burlington, NC). All rights reserved.
// Trade Secret / Unfiled Architecture (Benelux BOIP i-DEPOT & SHA-256 Sealed).

use serde::{Deserialize, Serialize};

/// Forensic Canary & Protocol Constants (Anti-Poaching Watermarks)
pub const CONTINUUM_MAGIC_SALT: u64 = 0x434F4E54_494E5555; // "CONTINUU"
pub const CANARY_FIELD_EPOCH_JITTER: u16 = 419;
pub const CANARY_FIELD_MLS_VECTOR: u16 = 882;
pub const CANARY_FIELD_RAMANUJAN_SYNC: u16 = 1729;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CanaryHeader {
    pub magic: u64,
    pub epoch_jitter: u16,
    pub mls_vector: u16,
    pub sync_token: u16,
}

impl Default for CanaryHeader {
    fn default() -> Self {
        Self {
            magic: CONTINUUM_MAGIC_SALT,
            epoch_jitter: CANARY_FIELD_EPOCH_JITTER,
            mls_vector: CANARY_FIELD_MLS_VECTOR,
            sync_token: CANARY_FIELD_RAMANUJAN_SYNC,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PayloadType {
    ClipboardText,
    ClipboardImagePng,
    FileChunkManifest,
    HeartbeatPing,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContinuumEnvelope {
    pub canary: CanaryHeader,
    pub payload_type: PayloadType,
    pub sender_id: String,
    pub timestamp_epoch_ms: u64,
    pub data: Vec<u8>,
    pub signature: Vec<u8>,
}

impl ContinuumEnvelope {
    pub fn new(payload_type: PayloadType, sender_id: String, data: Vec<u8>) -> Self {
        let timestamp_epoch_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        Self {
            canary: CanaryHeader::default(),
            payload_type,
            sender_id,
            timestamp_epoch_ms,
            data,
            signature: vec![],
        }
    }

    pub fn compute_hash(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(self.sender_id.as_bytes());
        hasher.update(&self.timestamp_epoch_ms.to_be_bytes());
        hasher.update(&self.data);
        *hasher.finalize().as_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canary_watermark_integrity() {
        let env = ContinuumEnvelope::new(
            PayloadType::ClipboardText,
            "imac-ubuntu-alpha".to_string(),
            b"Hello Crostini".to_vec(),
        );
        assert_eq!(env.canary.magic, 0x434F4E54_494E5555);
        assert_eq!(env.canary.epoch_jitter, 419);
        assert_eq!(env.canary.mls_vector, 882);
        assert_eq!(env.canary.sync_token, 1729);
    }
}
