// SPDX-License-Identifier: AGPL-3.0-only OR LicenseRef-Continuum-Commercial
// Copyright (c) 2026 Joshua Sultan (Burlington, NC). All rights reserved.
// Patent Pending (USPTO 35 U.S.C. § 111(b) & Benelux BOIP i-DEPOT Sealed).

use continuum_core::{ContinuumEnvelope, PayloadType};
use rusqlite::Connection;

#[test]
fn test_sqlite_vault_envelope_persistence() {
    let conn = Connection::open_in_memory().unwrap();
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS clipboard_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            sender_id TEXT NOT NULL,
            payload_type TEXT NOT NULL,
            content TEXT NOT NULL,
            timestamp INTEGER NOT NULL
        )",
        [],
    ).unwrap();

    let envelope = ContinuumEnvelope::new(
        PayloadType::ClipboardText,
        "test-node-imac".to_string(),
        b"Continuum distributed sync payload test".to_vec(),
    );

    let content_str = String::from_utf8(envelope.payload.clone()).unwrap();

    conn.execute(
        "INSERT INTO clipboard_history (sender_id, payload_type, content, timestamp) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![envelope.sender_id, "ClipboardText", content_str, envelope.timestamp_epoch_ms],
    ).unwrap();

    let mut stmt = conn.prepare("SELECT sender_id, content FROM clipboard_history WHERE sender_id = ?1").unwrap();
    let mut rows = stmt.query(rusqlite::params!["test-node-imac"]).unwrap();

    let mut count = 0;
    while let Some(row) = rows.next().unwrap() {
        let sender: String = row.get(0).unwrap();
        let val: String = row.get(1).unwrap();
        assert_eq!(sender, "test-node-imac");
        assert_eq!(val, "Continuum distributed sync payload test");
        count += 1;
    }

    assert_eq!(count, 1, "Expected exactly one vault record stored");
    log::info!("[Test] SQLite vault envelope persistence verified successfully.");
}
