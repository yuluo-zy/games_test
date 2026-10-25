//! `process_tree_cmds` (VA 0x142267430, source process_tree_cmds.rs:24-201).
//! Evidence: tree-process-cmds-analysis.md. Order Move -> Insert -> Delete.

use crate::resources::{ActiveSession, WaterRaster};
use crate::storage::{TreeEntry, TreeStorage};
use crate::types::*;
use bevy_ecs::prelude::*;

pub const MOVE_MISSING_LOG: &str = "MoveTreeCmd references a non-existing tree"; // 0x143035018
pub const MISSING_PREFAB_NAME: &str = "MISSING_TREE_PREFAB_NAME"; // .data 0x143034d80
/// Prefab table index stored for new trees.
/// UNKNOWN(evidence: tree-process-cmds-analysis.md "TreeStorage 布局"): how the
/// original derives the low32; Move looks up by entity instead.
pub const PREFAB_INDEX_UNKNOWN: u32 = 0;
/// Candidate list of 0x14226a170 random_tree_prefab.
/// UNKNOWN(evidence: tree-process-cmds-analysis.md): names live in .data (0x143034d30..)
/// and are not recovered, so the list is empty and the fallback literal is used.
pub const RANDOM_TREE_PREFAB_CANDIDATES: &[&str] = &[];

/// `in_water = 0.0 < water.sample(x, z)` (SETA @0x47C / @0xB26).
pub fn in_water(water: &WaterRaster, x: f32, z: f32) -> bool {
    0.0 < water.sample(x, z)
}

/// Effective spawn flags: non-dragging -> 0 (@0x700), dragging -> `flags & 3`.
pub fn effective_flags(session: &ActiveSession, flags: u32) -> u32 {
    if session.dragging { flags & 3 } else { 0 }
}

/// Uniform pick via fastrand's global RNG (the original uses fastrand; 0x14226a170).
pub fn pick_random_prefab(candidates: &[&str]) -> Option<String> {
    if candidates.is_empty() {
        return None;
    }
    Some(candidates[fastrand::usize(..candidates.len())].to_owned())
}

pub fn random_tree_prefab() -> Option<String> {
    pick_random_prefab(RANDOM_TREE_PREFAB_CANDIDATES)
}

/// Prefab name: use the event's name when present, else random pick, else the
/// fallback literal. UNKNOWN(evidence: tree-process-cmds-analysis.md): the real
/// code memcmp-matches against known names (.data 0x143034d30..) before choosing.
pub fn resolve_prefab_name(name: &str, random: impl FnOnce() -> Option<String>) -> String {
    if !name.is_empty() {
        return name.to_owned();
    }
    random().unwrap_or_else(|| MISSING_PREFAB_NAME.to_owned())
}

/// The system. Parameter list per tree-process-cmds-analysis.md "系统角色".
/// Order: Move -> Insert -> Delete.
#[allow(clippy::too_many_arguments)]
pub fn process_tree_cmds(
    mut tree_storage: ResMut<TreeStorage>,
    mut prefab_instances: Query<&mut PrefabInstanceData>,
    mut move_cmds: EventReader<MoveTreeCmd>,
    mut delete_cmds: EventReader<DeleteTreeCmd>,
    mut insert_cmds: EventReader<InsertTreeCmd>,
    mut spawn_prefab: EventWriter<SpawnPrefabCmd>,
    mut commands: Commands,
    mut play_sound: EventWriter<PlayTreeSoundCmd>,
    active_session: Res<ActiveSession>,
    water_raster: Res<WaterRaster>,
    mut despawn_prefab: EventWriter<DespawnPrefabCmd>,
) {
    // --- Move (source lines 43-74)
    for cmd in move_cmds.read() {
        let Some(entry) = tree_storage.get(cmd.index) else {
            log::warn!("{MOVE_MISSING_LOG}");
            continue;
        };
        // Skipped when the instance is missing: UNKNOWN(evidence: analysis "Move 分支").
        let Ok(mut inst) = prefab_instances.get_mut(entry.entity) else { continue };
        inst.position = cmd.position;
        play_sound.write(PlayTreeSoundCmd {
            kind: TreeSoundKind::Move,
            in_water: in_water(&water_raster, cmd.position.x, cmd.position.z),
            flag: false,
            // UNKNOWN(evidence): original runs inverse_lerp on PrefabInstanceData+0xc.
            height: inst.field_0c_unknown,
            position: cmd.position,
        });
    }

    // --- Insert (74-176)
    for cmd in insert_cmds.read() {
        let flags = effective_flags(&active_session, cmd.flags);
        let name = resolve_prefab_name(&cmd.name, random_tree_prefab);
        let mut spawned = commands.spawn_empty();
        let entity = spawned.id();
        spawned.insert(TreeComponent);
        // Initial animation value UNKNOWN (1.0 is a harness assumption).
        spawned.insert(TreeAnimation { value_at_4: 1.0 });
        spawned.insert(TreeMoveBend::default());
        spawned.insert(TreeAmbience);
        spawn_prefab.write(SpawnPrefabCmd {
            name,
            field_10_unknown: entity.to_bits(),
            position: cmd.position,
            field_24: cmd.height,
            field_28: cmd.field_28_unknown,
            flags,
            field_30_one: 1.0,
        });
        let old = tree_storage.insert(cmd.index, TreeEntry::new(PREFAB_INDEX_UNKNOWN, entity));
        play_sound.write(PlayTreeSoundCmd {
            kind: TreeSoundKind::Insert,
            in_water: in_water(&water_raster, cmd.position.x, cmd.position.z),
            // Draft interpretation, UNKNOWN(evidence: analysis "ActiveSession").
            flag: !cmd.name.is_empty() && (active_session.deny_load || active_session.corrupted),
            height: cmd.height,
            position: cmd.position,
        });
        // Replaced slot with non-zero high32: payload = packed insert return value
        // (@0xC03..0xE46). Meaning UNKNOWN(evidence: analysis 剩余unknown).
        if let Some(prev) = old.filter(|p| p.entity_index() != 0) {
            log::warn!("insert replaced existing tree slot {}", cmd.index);
            despawn_prefab.write(DespawnPrefabCmd { entity_bits: prev.raw() });
        }
    }

    // --- Delete (182-201)
    for cmd in delete_cmds.read() {
        let Some(entry) = tree_storage.remove(cmd.index) else { continue };
        let entity = entry.entity;
        let Ok(mut marked) = commands.get_entity(entity) else { continue };
        marked.insert(TreeMarkedForDeletion);
        // Sound needs the instance position; skipped when absent (UNKNOWN).
        if let Ok(inst) = prefab_instances.get_mut(entity) {
            play_sound.write(PlayTreeSoundCmd {
                kind: TreeSoundKind::Delete,
                // UNKNOWN(evidence): water is not evidenced as sampled for delete.
                in_water: false,
                flag: false,
                height: inst.field_0c_unknown,
                position: inst.position,
            });
        }
        // No DespawnPrefabCmd here: despawn_trees sends it.
    }
}

#[cfg(test)]
#[path = "process_cmds_tests.rs"]
mod tests;
