// SPDX-License-Identifier: AGPL-3.0-only OR LicenseRef-Continuum-Commercial
// Copyright (c) 2026 Joshua Sultan (Burlington, NC). All rights reserved.
// Patent Pending (USPTO 35 U.S.C. § 111(b) & Benelux BOIP i-DEPOT Sealed).

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ComputeTaskType {
    LocalLlmInference,
    OpticalCharacterRecognition,
    MediaTranscodeVideo,
    VectorEmbeddingQuery,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComputeJobManifest {
    pub job_id: String,
    pub task_type: ComputeTaskType,
    pub target_silicon_tier: String,
    pub payload: Vec<u8>,
}

impl ComputeJobManifest {
    pub fn new(job_id: &str, task_type: ComputeTaskType, target: &str, payload: Vec<u8>) -> Self {
        Self {
            job_id: job_id.to_string(),
            task_type,
            target_silicon_tier: target.to_string(),
            payload,
        }
    }
}

pub fn compute_worker_status() -> String {
    "ContinuumCompute Distributed Worker Pool Initialized (wgpu / WASI Sharding Ready)".to_string()
}
