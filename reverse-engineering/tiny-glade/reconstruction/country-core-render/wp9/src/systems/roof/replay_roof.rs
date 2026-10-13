//! replay_roof.rs — 屋顶回放（WP9）。
//!
//! PDB 骨架 `systems/roof/src/replay_roof.rs`。机器码：`replay_roof` 0x141a37ad0（line 23）、
//! `impl$0::process` 0x1421b48e0。

#![allow(dead_code, unused_imports)]

use crate::resources::history::roof_edit::RoofReplayCmd;
use crate::typed_gap::Gap;

/// replay_roof（O：0x141a37ad0）：回放 RoofReplayCmd。
pub fn replay_roof(_commands: &[RoofReplayCmd]) {
    let _ = _commands;
}

/// RoofReplayCmd::process（O：0x1421b48e0）：单命令应用。
pub fn process_replay(_cmd: &RoofReplayCmd) {
    let _ = _cmd;
}
