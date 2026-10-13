//! `geometry/instanced_wall.rs` — `InstancedWall` / `BrickSourceId` /
//! `BrickTransformSsbo` / `WallBrickInstancedDrawProxy` + brick bitmask SSBOs.
//!
//! Source: B skeleton `crates/country-core/src/geometry/instanced_wall.rs`.
//! PDB named entities: `BrickDrawListSsbo`, `BrickNextVisibilityBitmaskSsbo`,
//! `BrickPrevVisibilityBitmaskSsbo`, `BrickSourceId`, `BrickTransformSsbo`,
//! `InstancedWall`, `WallBrickInstancedDrawProxy`, and flag constants
//! `DISCARD_ABOVE`, `FORCE_LOD0`, `LESS_SNOWY`, `NO_ERASER_HIGHLIGHT`,
//! `ROOF_STITCH`, `SLIGHT_OVERLAP`, `STEPS`.
//!
//! O evidence (Ghidra, B patched EXE):
//! - `InstancedWall::new` @0x140858800 = 48 B `{ id: u64,
//!   brick_transforms: Vec<BrickTransformSsbo> (ptr,len,cap), writer:
//!   Option<Box<SparseInstanceBufferWriter<BrickTransformSsbo>>>,
//!   storage: Option<...> }`.
//! - `upload_to_storage_buffer` @0x140858880 — clears the sparse writer and
//!   pushes each `BrickTransformSsbo` (**stride 0x60 = 96 B**).
//! - `BrickSourceId::from_wall_id` @0x140858750 (13 callers) /
//!   `with_transient_stair_edge_id` @0x140858780 (2 callers).
//! - `WallBrickInstancedDrawProxy` marker = `{kind u16 @+0x48, sub u8 @+0x4a}`
//!   (O from `glade_startup_walls` spawns).

#![allow(dead_code, unused_imports)]

use crate::Gap;

/// `BrickSourceId` (O named entity): u32 pair `{kind, id}`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BrickSourceId {
    pub kind: u32,
    pub id: u32,
}

impl BrickSourceId {
    /// `from_wall_id` @0x140858750 (O).
    pub fn from_wall_id(_wall: &Gap) -> BrickSourceId {
        BrickSourceId::default()
    }
    /// `with_transient_stair_edge_id` @0x140858780 (O).
    pub fn with_transient_stair_edge_id(mut self, _edge: u32) -> BrickSourceId {
        self.id = _edge;
        self
    }
}

/// `BrickTransformSsbo` (O named entity; 0x60 B per brick).
#[derive(Clone, Copy, Debug)]
pub struct BrickTransformSsbo(pub [u8; 96]);

impl Default for BrickTransformSsbo {
    fn default() -> Self {
        BrickTransformSsbo([0u8; 96])
    }
}

/// `WallBrickInstancedDrawProxy` (O named entity; marker
/// `{kind u16, sub u8}`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WallBrickInstancedDrawProxy {
    pub kind: u16,
    pub sub: u8,
}

/// `BrickDrawListSsbo` (O named entity).
#[derive(Clone, Debug, Default)]
pub struct BrickDrawListSsbo(pub Gap);
/// `BrickNextVisibilityBitmaskSsbo` (O named entity).
#[derive(Clone, Debug, Default)]
pub struct BrickNextVisibilityBitmaskSsbo(pub Gap);
/// `BrickPrevVisibilityBitmaskSsbo` (O named entity).
#[derive(Clone, Debug, Default)]
pub struct BrickPrevVisibilityBitmaskSsbo(pub Gap);

/// `InstancedWall` (O named entity; 48 B).
#[derive(Clone, Debug, Default)]
pub struct InstancedWall {
    pub id: u64,
    pub brick_transforms: Vec<BrickTransformSsbo>,
    pub _gap: Gap,
}

impl InstancedWall {
    /// `InstancedWall::new` @0x140858800 (O).
    pub fn new(
        id: u64,
        brick_transforms: Vec<BrickTransformSsbo>,
        _writer: Gap,
        _storage: Gap,
    ) -> InstancedWall {
        InstancedWall {
            id,
            brick_transforms,
            _gap: Gap,
        }
    }
    /// `upload_to_storage_buffer` @0x140858880 (O; sparse-writer clear+push).
    pub fn upload_to_storage_buffer(&mut self) {
        todo!("recover InstancedWall::upload_to_storage_buffer (SparseInstanceBufferWriter push)")
    }
    /// `update` @0x140858830 (O).
    pub fn update(&mut self) {
        todo!("recover InstancedWall::update")
    }
}
