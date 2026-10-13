//! `meta_commander.rs` — INTEGRATION MARKER (wp15-owned `MetaCommand` enum).

#![allow(dead_code, unused_imports)]

use crate::Gap;

/// `MetaCommand` (wp15; meta-command queue item consumed by `finish_timelapse`).
#[derive(Clone, Debug, Default)]
pub struct MetaCommand {
    pub _gap: Gap,
}
