//! Command/event payloads and components of the tree tool.
//!
//! Field offsets quoted from tree-process-cmds-analysis.md "命令字段布局"
//! (slot offsets; the bevy EventInstance id prefix is provided by bevy itself).

use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};

/// MoveTreeCmd, slot stride 0x20: index u64 @+8, position Vec3 @+0x10.
#[derive(Event, Debug, Clone, Copy, PartialEq)]
pub struct MoveTreeCmd {
    pub index: u64,
    pub position: Vec3,
}

/// DeleteTreeCmd, slot stride 0x10: index u64 @+8.
#[derive(Event, Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeleteTreeCmd {
    pub index: u64,
}

/// InsertTreeCmd, slot stride 0x48.
#[derive(Event, Debug, Clone, PartialEq)]
pub struct InsertTreeCmd {
    /// name String @+0x8/+0x10
    pub name: String,
    /// position @+0x18..+0x20
    pub position: Vec3,
    /// height f32 @+0x24 (copied into the insert sound)
    pub height: f32,
    /// f32 @+0x28 (semantics UNKNOWN(evidence: tree-process-cmds-analysis.md))
    pub field_28_unknown: f32,
    /// flags u32 @+0x2c; low 2 bits used while dragging
    pub flags: u32,
    /// byte @+0x30 (semantics UNKNOWN)
    pub byte_30_unknown: u8,
    /// index u64 @+0x38 -> TreeStorage::insert key
    pub index: u64,
    // @+0x40..0x48: never read (UNKNOWN(evidence: tree-process-cmds-analysis.md 剩余unknown))
}

/// SpawnPrefabCmd, slot stride 0x50 (only the evidenced fields).
#[derive(Event, Debug, Clone, PartialEq)]
pub struct SpawnPrefabCmd {
    /// name @+0x0/+0x8
    pub name: String,
    /// u64 @+0x10. UNKNOWN(evidence: tree-process-cmds-analysis.md 剩余unknown).
    /// HYPOTHESIS ONLY: holds `Entity::to_bits()` of the spawned entity so a consumer
    /// can find it; no game behaviour is built on it.
    pub field_10_unknown: u64,
    /// position @+0x18..+0x20
    pub position: Vec3,
    /// f32 @+0x24 / @+0x28 (copied from insert height / field_28; semantics UNKNOWN)
    pub field_24: f32,
    pub field_28: f32,
    /// flags u32 @+0x2c (0 when not dragging, else insert flags & 3)
    pub flags: u32,
    /// f32 @+0x30 constant 1.0 (semantics UNKNOWN)
    pub field_30_one: f32,
}

/// DespawnPrefabCmd, slot stride 0x10: u64 @+8. Sent by `despawn_trees` with
/// `Entity::to_bits()`; by the replaced-slot insert path with the packed TreeStorage
/// value (meaning UNKNOWN), hence a raw u64 rather than an `Entity`.
#[derive(Event, Debug, Clone, Copy, PartialEq, Eq)]
pub struct DespawnPrefabCmd {
    pub entity_bits: u64,
}

impl DespawnPrefabCmd {
    pub fn for_entity(entity: Entity) -> Self {
        Self { entity_bits: entity.to_bits() }
    }
}

/// Sound kind byte @+8 of PlayTreeSoundCmd.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TreeSoundKind {
    Insert = 0,
    Move = 1,
    Delete = 2,
}

/// PlayTreeSoundCmd, slot stride 0x20.
#[derive(Event, Debug, Clone, Copy, PartialEq)]
pub struct PlayTreeSoundCmd {
    pub kind: TreeSoundKind,
    pub in_water: bool,
    /// bool @+0xa. Insert: name-matched && (deny_load || is_corrupted)
    /// (draft interpretation; UNKNOWN(evidence: tree-process-cmds-analysis.md
    /// "ActiveSession")). Move/Delete: false.
    pub flag: bool,
    /// f32 @+0xc
    pub height: f32,
    pub position: Vec3,
}

/// OnTerrainChange event (country_core::systems::terrain; payload not read by recalc; UNKNOWN).
#[derive(Event, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct OnTerrainChange;

/// PrefabInstanceData (size 0x30). Only evidenced fields.
/// The real type lives outside this module; it is mirrored here as a Component.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct PrefabInstanceData {
    /// Vec3 @+0
    pub position: Vec3,
    /// f32 @+0xc (read by Move branch). UNKNOWN semantics.
    pub field_0c_unknown: f32,
    /// f32 pair @+0x24/+0x28 written by tree_move_bend
    pub bend_step: Vec2,
}

impl PrefabInstanceData {
    pub fn at(position: Vec3) -> Self {
        Self { position, field_0c_unknown: 0.0, bend_step: Vec2::ZERO }
    }
}

/// TreeAnimation (size 0x10); f32 @+4 compared with 0.01 by despawn_trees.
/// Field name UNKNOWN(evidence: tree-tool-rest-analysis.md §1).
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct TreeAnimation {
    pub value_at_4: f32,
}

/// TreeMoveBend: Vec2, stride 8.
#[derive(Component, Debug, Clone, Copy, PartialEq, Default)]
pub struct TreeMoveBend(pub Vec2);

/// Marker components added at spawn (payloads UNKNOWN).
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TreeComponent;
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TreeAmbience;
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TreeMarkedForDeletion;
