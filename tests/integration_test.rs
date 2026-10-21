// SPDX-License-Identifier: AGPL-3.0-only OR LicenseRef-Continuum-Commercial
// Copyright (c) 2026 Joshua Sultan (Burlington, NC). All rights reserved.
// Patent Pending (USPTO 35 U.S.C. § 111(b) & Benelux BOIP i-DEPOT Sealed).

use continuum_core::{ContinuumEnvelope, PayloadType};
use continuum_compute::{ComputeJobManifest, ComputeTaskType};
use continuum_did::ContinuumDidRecord;
use continuum_shims::pam_stub;

#[test]
fn test_end_to_end_crate_interoperability() {
    // 1. Verify ContinuumCore Canary & Envelope Hashing
    let envelope = ContinuumEnvelope::new(
        PayloadType::ClipboardText,
        "imac-ubuntu-alpha".to_string(),
        b"Cross-crate integration test payload".to_vec(),
    );
    assert_eq!(envelope.canary.magic, 0x434F4E54_494E5555);
    let hash = envelope.compute_hash();
    assert_eq!(hash.len(), 32);

    // 2. Verify ContinuumCompute Job Manifest
    let job = ComputeJobManifest::new(
        "job-999",
        ComputeTaskType::LocalLlmInference,
        "x86_64-gpu",
        vec![0x01, 0x02, 0x03],
    );
    assert_eq!(job.job_id, "job-999");

    // 3. Verify Branch B ContinuumDID Registry & Vault Transfer
    let mut did_record = ContinuumDidRecord::new("did:continuum:sultan-test", vec![0x11, 0x22], "0xAlice");
    did_record.register_peer("12D3KooW-CrostiniNode");
    assert_eq!(did_record.authorized_peer_ids.len(), 1);
    did_record.transfer_vault_ownership("0xBob-Acquirer");
    assert_eq!(did_record.vault_token_owner, "0xBob-Acquirer");

    // 4. Verify ContinuumShims PAM Proximity Auth Stub
    let auth_result = pam_stub::verify_proximity_auth(-50, true);
    assert!(auth_result);
}
