//! `systems/ui/signifiers/signifier_create.rs` — `UiSignifierStream` /
//! `UiAppendSignifier` + the `signifier_create` system.
//!
//! Source: B skeleton `crates/country-core/src/systems/ui/signifiers/signifier_create.rs`.
//! PDB named entities: `UiAppendSignifier`, `UiSignifierStream`.
//!
//! O evidence (Ghidra, B patched EXE):
//! - `UiSignifierStream` = **0x170 B** lock-free append-only channel:
//!   read-cursor @+0x160, write-counter @+0x168 (both u64); `write`
//!   @0x140b2cf90 copies a **0x28 B** record (5 qwords) per slot and advances
//!   the read cursor (spin-wait). Resource inserted in `add_ui`.
//! - `signifier_create` @0x140b2cfa0 — drains the stream each frame and spawns
//!   the signifier entities (builder records consumed here).

#![allow(dead_code, unused_imports)]

use crate::Gap;
use crate::systems::ui::signifiers::SignifierEntityBuilder;

/// `UiAppendSignifier` (O named entity; one queued signifier build).
#[derive(Clone, Debug, Default)]
pub struct UiAppendSignifier(pub SignifierEntityBuilder);

/// `UiSignifierStream` (O named entity; 0x170 B concurrent FIFO resource).
#[derive(Clone, Debug, Default)]
pub struct UiSignifierStream {
    /// pending append records (0x28 B each, O).
    pub pending: Vec<UiAppendSignifier>,
}

impl UiSignifierStream {
    /// `write` @0x140b2cf90 (O; 36 callers): append one record.
    pub fn write(&mut self, _signifier: UiAppendSignifier) {
        todo!("recover UiSignifierStream::write (lock-free append + cursor advance)")
    }
}

/// `signifier_create` @0x140b2cfa0 (O).
pub fn signifier_create(_stream: &mut UiSignifierStream) {
    todo!("recover signifier_create (drain stream -> spawn signifier entities)")
}
