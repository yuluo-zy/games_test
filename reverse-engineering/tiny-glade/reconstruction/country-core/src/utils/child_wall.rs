//! `utils/child_wall.rs` — `ChildWall` (a sub-segment of a wall).
//!
//! Source: B skeleton `crates/country-core/src/utils/child_wall.rs`.
//! PDB named entity: `ChildWall`.
//! O evidence (Ghidra, B patched EXE; `ChildWall::slice` @0x140a24000):
//! layout 16 B `{ wall_id: u64 @0, t0: f32 @8, t1: f32 @0xc }`; `slice`
//! computes two `utils::lerp` values and **panics** if either is outside
//! `[0,1]` or NaN. `ChildWall::new` inlined into `wall_splitter`.

#![allow(dead_code, unused_imports)]

/// `ChildWall` (O: 16 B).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ChildWall {
    /// parent wall id (O @0).
    pub wall_id: u64,
    /// segment start parameter (O @8).
    pub t0: f32,
    /// segment end parameter (O @0xc).
    pub t1: f32,
}

impl ChildWall {
    /// `ChildWall::new` (O inline into `wall_splitter`).
    pub fn new(wall_id: u64, t0: f32, t1: f32) -> ChildWall {
        ChildWall { wall_id, t0, t1 }
    }
    /// `ChildWall::slice` @0x140a24000 (O): split at `t` (lerp + bounds panic).
    pub fn slice(&self, _t: f32) -> (ChildWall, ChildWall) {
        todo!("recover ChildWall::slice (two lerps, [0,1] panic)")
    }
}
