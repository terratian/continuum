// SPDX-License-Identifier: AGPL-3.0-only OR LicenseRef-Continuum-Commercial
// Copyright (c) 2026 Joshua Sultan (Burlington, NC). All rights reserved.
// Patent Pending (USPTO 35 U.S.C. § 111(b) & Benelux BOIP i-DEPOT Sealed).

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CrossDeviceAction {
    DisplayResize { width: u32, height: u32, refresh_rate: u32 },
    AudioRouteStream { sample_rate: u32, channels: u8, pcm_chunk: Vec<u8> },
    InputInjectEvent { event_type: String, x: f32, y: f32, button_mask: u8 },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CrossDeviceMessage {
    pub message_id: String,
    pub source_node: String,
    pub target_node: String,
    pub action: CrossDeviceAction,
    pub timestamp_ms: u64,
}

impl CrossDeviceMessage {
    pub fn new(source: &str, target: &str, action: CrossDeviceAction) -> Self {
        let timestamp_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        Self {
            message_id: format!("msg-{}", timestamp_ms),
            source_node: source.to_string(),
            target_node: target.to_string(),
            action,
            timestamp_ms,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cross_device_message_serialization() {
        let msg = CrossDeviceMessage::new(
            "imac-ubuntu",
            "crostini-client",
            CrossDeviceAction::DisplayResize { width: 2560, height: 1440, refresh_rate: 60 },
        );
        let serialized = serde_json::to_string(&msg).unwrap();
        assert!(serialized.contains("DisplayResize"));
        let deserialized: CrossDeviceMessage = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.source_node, "imac-ubuntu");
    }
}
