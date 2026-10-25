//! Reconstruction of Tiny Glade's tree-tool module (package `system-tree` in baseline B),
//! built on the ORIGINAL dependencies (bevy_ecs/bevy_app 0.16.0, glam 0.29.3,
//! fastrand 1.8.0, log 0.4.27). Evidence index: see NOTES.md.
//! Source docs: evidence/multi-baseline/tree-*.md.

pub mod bend;
pub mod convert;
pub mod despawn;
pub mod plugin;
pub mod process_cmds;
pub mod recalc;
pub mod replay;
pub mod reset;
pub mod resources;
pub mod storage;
pub mod types;

#[cfg(test)]
pub(crate) mod test_util;

pub use plugin::{GameSystemSet, TreeToolPlugin};
pub use resources::*;
pub use storage::{TreeEntry, TreeStorage};
pub use types::*;
