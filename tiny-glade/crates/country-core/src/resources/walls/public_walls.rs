//! Public wall ownership, associations and publication notifications.
//! B's maps use hashbrown; their native layout is left to that dependency.
//!
//! ## PublicWalls 密集表（O 布局证据，coregap-1 2026-10-17 fresh `set_roof_customization`
//! 0x142299dd0）
//!
//! 原 `PublicWalls` 之外的墙/屋顶**记录密集表**（`system_color::set_roof_customization`
//! 经 `InnerWalls`/`Roof` 结构访问）：
//! - 墙记录表：`InnerWalls+0x1a8` 处 Vec，**stride 0x48**，`+0x18` 为墙记录数组、`+0x38` 为
//!   关联表（`+0x110` 偏移的墙 id 取 `~id*0x30` 反向索引屋顶记录）；
//! - 屋顶存在位集：`+0x28`（指针）/`+0x38`（len），按 `(id >> 6) 字 / (id & 0x3f) 位`；
//! - 屋顶记录：stride 0x30，`+0x28` 字段 = 色表（`u32[roof_index]`）、`+0x50..` 载荷；
//! - 色值源 = `Res<PublicWalls>+0x14`（hashbrown 随机状态派生）。
//! 上述表都内嵌 `InnerWalls`/`RoofShape` 记录（walls 组在途域）——`PublicWalls` 本类型仍为
//! HashMap 抽象，**未落密集表**（登记 GAP，见 state/color RESTORE FIX-6）。
use bevy_ecs::{entity::Entity, event::Event, resource::Resource};
use hashbrown::{HashMap, HashSet};

use super::{InnerWalls, WallId};
use crate::resources::roofs::Roof;
use crate::resources::walls::publication_source::CollectionError;
use crate::resources::walls::publication_source::DirtyPublicationSubscription;
use crate::resources::walls::publication_source::TerrainHeightGrid512;
use crate::resources::walls::publication_source::dirty_tile_candidate;
use crate::resources::walls::publication_source::terrain_requires_republication;
use crate::systems::wall::wall_state::path_preparation::PathPreparationError;
use crate::systems::wall::wall_state::path_preparation::PathRasters512;
use crate::utils::dirty_tiles::DirtyTiles;

pub use super::public_wall_state::PublicWallState;
pub use crate::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash;

#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct OnWallChanged(pub WallId);
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct OnWallDeleted(pub WallId);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WallPublicationEvent {
    Changed(WallId),
    Deleted(WallId),
}

#[derive(Default, Debug)]
pub struct WallPublicationDelta {
    pub changed: HashSet<WallId>,
    pub removed: HashSet<WallId>,
    /// Preserves deleted-before-created-before-updated order, including repeated
    /// changed notices when an ID appears in both authority source sets.
    pub events: Vec<WallPublicationEvent>,
}

/// The two independent original subscriptions are intentionally kept distinct.
/// The producer must provide the real dirty tiles; clear does not manufacture
/// an all-dirty terrain update.
#[derive(Resource, Default, Debug)]
pub struct WallPublicationDirty {
    pub primary: DirtyTiles,
    pub secondary: DirtyTiles,
}

#[derive(Resource, Default)]
pub struct PublicWalls {
    records: HashMap<WallId, PublicWallState>,
    roofs: HashMap<WallId, Entity>,
    failures: HashMap<WallId, CollectionError>,
    path_failures: HashMap<WallId, PathPreparationError>,
    pub last_delta: WallPublicationDelta,
}

impl PublicWalls {
    /// B 0x140a91ef0: a present row must have a valid simplified Curve2.
    pub fn get(&self, wall: WallId) -> Option<&PublicWallState> {
        self.records
            .get(&wall)
            .inspect(|state| state.assert_valid())
    }
    /// B 0x140a92010 is an unchecked *validation* lookup, still returning None
    /// for a missing ID. The old chapter's description as unwrap was incorrect.
    pub fn get_unchecked(&self, wall: WallId) -> Option<&PublicWallState> {
        self.records.get(&wall)
    }
    pub fn get_mut(&mut self, wall: WallId) -> Option<&mut PublicWallState> {
        let state = self.records.get_mut(&wall)?;
        state.assert_valid();
        Some(state)
    }
    pub fn all(&self) -> impl Iterator<Item = (WallId, &PublicWallState)> {
        self.records.iter().map(|(&id, state)| (id, state))
    }
    pub fn wall_ids(&self) -> impl Iterator<Item = WallId> + '_ {
        self.records.keys().copied()
    }
    /// B 0x140a92100 (fresh disasm 2026-10-17): **two probes**.
    ///
    /// 1. map `failures` (`+0x60` ctrl / `+0x68` mask / `+0x78` items): a hit
    ///    executes `XOR ESI,ESI`, i.e. a recorded collection failure returns
    ///    `false` regardless of the roof map.
    /// 2. map `roofs` (`+0x30`): on a hit the value at `entry+0xc` (the
    ///    `Entity` generation half) is tested with `CMP dword,0; SETNZ AL`;
    ///    `AND SIL,AL` combines. This is `Option<Entity>::is_some`, not a
    ///    `contains_key` test.
    pub fn has_roof(&self, wall: WallId) -> bool {
        !self.failures.contains_key(&wall)
            && self
                .roofs
                .get(&wall)
                .is_some_and(|entity| entity.to_bits() >> 32 != 0)
    }
    pub fn roof_entity(&self, wall: WallId) -> Option<Entity> {
        self.roofs.get(&wall).copied()
    }
    /// B 0x140a923a0 (fresh disasm 2026-10-17): `roofs` probe → entity slot
    /// validation → `InnerWalls` dense roof record (`+0x1a8` chunk table,
    /// presence bitset `+0x28`/`+0x38`, `+0x110` slot, `~slot*0x30` leaf) →
    /// `&RoofComponent` at `+0x58` stride. Every failure path returns `None`.
    ///
    /// Seam (registered in REBUILD4): the dense slot table and the ECS
    /// component store are not owned by `PublicWalls`. `resolve` is the
    /// storage-side `Entity -> Option<&Roof>` lookup (`Query<&Roof>`), which
    /// performs the same index/generation validation natively done by the
    /// dense record walk. Unlock: `InnerWalls` exposes the `+0x1a8` dense roof
    /// tables so the chain can be reproduced literally.
    pub fn roof_component<'a>(
        &self,
        wall: WallId,
        resolve: impl FnOnce(Entity) -> Option<&'a Roof>,
    ) -> Option<&'a Roof> {
        let entity = self.roofs.get(&wall).copied()?;
        // Native `uVar9 >> 0x20 == 0` skips the entity (generation 0 = None).
        if entity.to_bits() >> 32 == 0 {
            return None;
        }
        resolve(entity)
    }
    /// B 0x140a92530 (fresh disasm 2026-10-17): inlined `get` validation
    /// (`count<2 || !(length>0)` → `panic`) then the 28 B shape record at
    /// record `+0x260`. The record's tag doubles as the `Option` niche:
    /// `tag < 2` copies 28 B, `tag >= 2` writes the canonical `None` tag `2`.
    pub fn get_shape_params(
        &self,
        wall: WallId,
    ) -> Option<crate::systems::wall::valid_enclosure::ShapeParameters> {
        use crate::systems::wall::valid_enclosure::ShapeParameters;
        let state = self.get(wall)?;
        let record = ShapeParameters::from_snapshot(state.shape());
        record.is_premade().then_some(record)
    }
    pub fn associate_roof(&mut self, wall: WallId, entity: Entity) -> Option<Entity> {
        self.roofs.insert(wall, entity)
    }
    pub fn remove_roof(&mut self, wall: WallId) -> Option<Entity> {
        self.roofs.remove(&wall)
    }
    pub fn replace_roof_entities(&mut self, roofs: HashMap<WallId, Entity>) -> Vec<WallId> {
        let changes = self
            .wall_ids()
            .filter(|id| self.roofs.contains_key(id) != roofs.contains_key(id))
            .collect();
        self.roofs = roofs;
        changes
    }
    pub fn failures(&self) -> &HashMap<WallId, CollectionError> {
        &self.failures
    }
    pub fn path_failures(&self) -> &HashMap<WallId, PathPreparationError> {
        &self.path_failures
    }

    pub fn consume(&mut self, inner: &mut InnerWalls, terrain: &TerrainHeightGrid512) {
        self.consume_with_paths(
            inner,
            terrain,
            None,
            &WallPublicationDirty::default(),
            &WallSpatialHash::default(),
        );
    }

    /// B 0x140a8b530 processes removal, creation and editing sets separately.
    /// 0x140b772b0 is hashbrown::RawTable::remove_entry, not an animation test.
    /// The original invalid-source guard skips rebuilding and keeps any old row.
    pub fn consume_with_paths(
        &mut self,
        inner: &mut InnerWalls,
        terrain: &TerrainHeightGrid512,
        paths: Option<&PathRasters512>,
        dirty: &WallPublicationDirty,
        spatial: &WallSpatialHash,
    ) {
        let mut changes = inner.take_changes();
        self.last_delta = WallPublicationDelta::default();
        for id in spatial.candidates(&dirty.primary) {
            if let Some(state) = self.get(id) {
                if dirty_tile_candidate(
                    DirtyPublicationSubscription::Primary,
                    state.geometry.metrics.minimum_relative_height,
                    changes.updated.contains(&id),
                ) {
                    changes.updated.insert(id);
                }
            }
        }
        for id in spatial.candidates(&dirty.secondary) {
            if let Some(state) = self.get(id) {
                if dirty_tile_candidate(
                    DirtyPublicationSubscription::Secondary,
                    state.geometry.metrics.minimum_relative_height,
                    changes.updated.contains(&id),
                ) && terrain_requires_republication(
                    &state.geometry.outline,
                    state.authority.elevation(),
                    terrain,
                ) {
                    changes.updated.insert(id);
                }
            }
        }
        for id in changes.removed {
            self.failures.remove(&id);
            self.path_failures.remove(&id);
            if self.records.remove(&id).is_some() {
                self.last_delta.removed.insert(id);
                self.last_delta
                    .events
                    .push(WallPublicationEvent::Deleted(id));
            }
        }
        for (ids, adding) in [(changes.created, true), (changes.updated, false)] {
            for id in ids {
                let Some(authority) = inner.get(id) else {
                    continue;
                };
                let source = authority.source();
                if source.curve.points.len() < 2 || source.curve.length <= 0.0 {
                    self.failures.insert(id, CollectionError::InvalidSource);
                    continue;
                }
                // Preserve the original duplicate-created side effects: changed
                // is emitted and the old public row removed before the assertion.
                if adding && self.records.contains_key(&id) {
                    self.last_delta.changed.insert(id);
                    self.last_delta
                        .events
                        .push(WallPublicationEvent::Changed(id));
                    self.records.remove(&id);
                    assert!(!adding, "assertion failed: !is_adding");
                }
                let mut record = match PublicWallState::collect(authority, terrain) {
                    Ok(record) => record,
                    Err(error) => {
                        self.failures.insert(id, error);
                        continue;
                    }
                };
                self.path_failures.remove(&id);
                if let Some(paths) = paths {
                    if let Err(error) = record.prepare_paths(paths) {
                        self.path_failures.insert(id, error);
                    }
                }
                if let Some(old) = self.records.get_mut(&id) {
                    record.material_stats =
                        super::material_stats::MaterialStats::for_new_publication(
                            &mut old.material_stats,
                        );
                }
                self.records.insert(id, record);
                self.failures.remove(&id);
                self.last_delta.changed.insert(id);
                self.last_delta
                    .events
                    .push(WallPublicationEvent::Changed(id));
            }
        }
    }
}

#[derive(Resource, Default, Debug)]
pub struct WallEntities(pub HashMap<WallId, Entity>);

impl WallEntities {
    pub fn wall_entity(&self, wall: WallId) -> Option<Entity> {
        self.0.get(&wall).copied()
    }
}

/// Original delete_wall owns and drains every plaster child of the deleted wall.
#[derive(Resource, Default, Debug)]
pub struct WallPlasters(pub HashMap<WallId, Vec<Option<Entity>>>);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::roofs::Roof;
    use crate::resources::roofs::geometry::surface::Curve2;
    use crate::resources::walls::WallState;
    use crate::resources::walls::publication_source::TerrainHeightGrid512;
    use crate::systems::wall::valid_enclosure::{Shape, ShapeParameters};

    fn curve() -> Curve2 {
        Curve2::try_new(vec![[0.0, 0.0], [4.0, 0.0]], false).unwrap()
    }

    fn terrain() -> TerrainHeightGrid512 {
        TerrainHeightGrid512 {
            encoded: vec![0u16; 512 * 512].into(),
            minimum_y: 0.0,
            maximum_y: 0.0,
            sampling_fix: false,
        }
    }

    fn wall_with_shape(shape: [u8; 28]) -> WallState {
        let mut state = WallState::new(curve(), 4.0, 0, 0, false);
        state.snapshot.source.shape =
            crate::resources::walls::publication_fields::WallShapeSnapshot::from_observed(shape);
        state
    }

    #[test]
    fn has_roof_requires_no_failure_and_a_live_entity() {
        let mut public = PublicWalls::default();
        assert!(!public.has_roof(WallId(1)));
        public.associate_roof(WallId(1), Entity::from_raw(9));
        assert!(public.has_roof(WallId(1)));
        // 0x140a92100: first-map hit performs `XOR ESI,ESI`.
        public
            .failures
            .insert(WallId(1), CollectionError::InvalidSource);
        assert!(!public.has_roof(WallId(1)));
    }

    #[test]
    fn get_shape_params_returns_the_28_byte_premade_record_only() {
        let record = ShapeParameters::from_shape(Shape::Circle(utils::geometry::Circle2d {
            center: glam::Vec2::new(1.0, 2.0),
            radius: 3.0,
            rotation: 0.0,
        }))
        .to_record();
        let mut public = PublicWalls::default();
        public.records.insert(
            WallId(3),
            PublicWallState::collect(&wall_with_shape(record), &terrain()).unwrap(),
        );
        assert_eq!(
            public.get_shape_params(WallId(3)).unwrap().to_record(),
            record
        );

        // tag 2 (freehand) -> canonical None; missing id -> None.
        let mut freehand = PublicWalls::default();
        freehand.records.insert(
            WallId(4),
            PublicWallState::collect(&WallState::new(curve(), 4.0, 0, 0, false), &terrain())
                .unwrap(),
        );
        assert!(freehand.get_shape_params(WallId(4)).is_none());
        assert!(freehand.get_shape_params(WallId(99)).is_none());
    }

    #[test]
    fn roof_component_resolves_the_roof_entity_through_the_storage_seam() {
        let roof = Roof::from_observed([0u8; 88]);
        let mut public = PublicWalls::default();
        assert!(public.roof_component(WallId(1), |_| Some(&roof)).is_none());
        public.associate_roof(WallId(1), Entity::from_raw(7));
        assert!(public.roof_component(WallId(1), |_| Some(&roof)).is_some());
        // Deep storage miss returns None, never panics.
        assert!(public.roof_component(WallId(1), |_| None).is_none());
    }
}
