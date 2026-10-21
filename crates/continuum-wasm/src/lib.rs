// SPDX-License-Identifier: AGPL-3.0-only OR LicenseRef-Continuum-Commercial
// Copyright (c) 2026 Joshua Sultan (Burlington, NC). All rights reserved.
// Patent Pending (USPTO 35 U.S.C. § 111(b) & Benelux BOIP i-DEPOT Sealed).

use wasm_bindgen::prelude::*;
use continuum_core::transcoder::transcode_buffer;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct WasmIpcMessage {
    pub action: String,
    pub source_mime: String,
    pub target_mime: String,
    pub payload: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct WasmIpcResponse {
    pub status: String,
    pub data: Vec<u8>,
}

#[wasm_bindgen]
pub fn init_wasm_bridge() -> String {
    "Continuum Wasm Bridge Initialized (Structured IPC & Transcoder Active)".to_string()
}

#[wasm_bindgen]
pub fn handle_ipc_message(json_str: &str) -> Result<String, JsValue> {
    let msg: WasmIpcMessage = serde_json::from_str(json_str)
        .map_err(|e| JsValue::from_str(&format!("JSON Parse Error: {}", e)))?;
    
    let result = transcode_buffer(&msg.source_mime, &msg.target_mime, &msg.payload)
        .map_err(|e| JsValue::from_str(&e))?;

    let resp = WasmIpcResponse {
        status: "success".to_string(),
        data: result,
    };

    serde_json::to_string(&resp)
        .map_err(|e| JsValue::from_str(&format!("JSON Serialize Error: {}", e)))
}
