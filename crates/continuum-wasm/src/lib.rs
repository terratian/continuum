// SPDX-License-Identifier: AGPL-3.0-only OR LicenseRef-Continuum-Commercial
// Copyright (c) 2026 Joshua Sultan (Burlington, NC). All rights reserved.

use wasm_bindgen::prelude::*;
use continuum_core::transcoder::transcode_buffer;

#[wasm_bindgen]
pub fn init_wasm_bridge() -> String {
    "Continuum Wasm Bridge Initialized (WASI Transcoder Active)".to_string()
}

#[wasm_bindgen]
pub fn wasi_transcode(source_mime: &str, target_mime: &str, data: &[u8]) -> Result<Vec<u8>, JsValue> {
    transcode_buffer(source_mime, target_mime, data)
        .map_err(|e| JsValue::from_str(&e))
}
