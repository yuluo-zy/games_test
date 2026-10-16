//! B `systems/wall/eraser/archivist.rs`: wall eraser archivist.
//!
//! Evidence: `chapters/systems/wall/eraser/archivist.rs.md` (O, new
//! 0x14092e6e0 / finish 0x14092f990). The archivist caches the affected
//! walls' original snapshots and their erased ranges; `finish` compiles the
//! RoofDelete + Wall compound history via
//! [`record_wall_bulldoze`] and emits `RecordWallHistory` /
//! `RecordRoofHistory`. The ChildWalls rewrite pairing is host 待接线.

use bevy_ecs::prelude::*;
use hashbrown::HashMap;
use std::sync::Arc;

use crate::recovered_building::{
    wall_bulldoze::record_wall_bulldoze,
    wall_history_edit::WallAuthoritySnapshot,
    wall_state::WallId as RecoveredWallId,
};
use crate::resources::{
    roofs::{Roof, commands::RecordRoofHistory},
    walls::{InnerWalls, WallId, public_walls::PublicWalls},
};
use crate::systems::wall::history_recording::RecordWallHistory;

/// Erased range captured per affected wall.
#[derive(Clone, Copy, Debug)]
pub struct ErasedRange {
    pub low: f32,
    pub high: f32,
}

/// B `WallEraserArchivist` (0x90 B record family). The reconstruction keeps
/// the original snapshot per wall and the erased u-range.
#[derive(Resource, Clone, Debug, Default)]
pub struct WallEraserArchivist {
    affected: HashMap<WallId, WallAuthoritySnapshot>,
    ranges: HashMap<WallId, ErasedRange>,
}

impl WallEraserArchivist {
    /// B new (0x14092e6e0).
    pub fn new() -> Self {
        Self::default()
    }

    /// Inline B line 39.
    pub fn affected_walls(&self) -> impl Iterator<Item = WallId> + '_ {
        self.affected.keys().copied()
    }

    /// Inline B line 43.
    pub fn has_anything_been_affected(&self) -> bool {
        !self.affected.is_empty()
    }

    /// B cache_if_original_wall (0x14092ea50): cache the first-seen original.
    pub fn cache_if_original_wall(
        &mut self,
        wall: WallId,
        snapshot: &WallAuthoritySnapshot,
        range: ErasedRange,
    ) {
        self.affected.entry(wall).or_insert_with(|| snapshot.clone());
        self.ranges.insert(wall, range);
    }

    /// B record_rewiring_event (0x14092e8e0): replace the original snapshot.
    pub fn record_rewiring_event(&mut self, wall: WallId, snapshot: &WallAuthoritySnapshot) {
        self.affected.insert(wall, snapshot.clone());
    }

    /// B shrink_wall (0x14092eff0) alias: cache a shrink range.
    pub fn shrink_wall(&mut self, wall: WallId, range: ErasedRange) {
        if let Some(existing) = self.ranges.get_mut(&wall) {
            *existing = range_min_max(*existing, range);
        } else {
            self.ranges.insert(wall, range);
        }
    }

    pub fn take(&mut self) -> Vec<(WallId, WallAuthoritySnapshot, ErasedRange)> {
        let mut out = Vec::new();
        for (wall, original) in self.affected.drain() {
            let range = self
                .ranges
                .remove(&wall)
                .unwrap_or(ErasedRange { low: 0.0, high: 1.0 });
            out.push((wall, original, range));
        }
        out
    }
}

fn range_min_max(a: ErasedRange, b: ErasedRange) -> ErasedRange {
    ErasedRange {
        low: a.low.min(b.low),
        high: a.high.max(b.high),
    }
}

/// B archivist finish (0x14092f990): compile the RoofDelete + Wall compound
/// history for every affected wall.
pub fn finish_wall_eraser(
    mut archivist: ResMut<WallEraserArchivist>,
    _inner: Res<InnerWalls>,
    public: Res<PublicWalls>,
    roofs: Query<&Roof>,
    mut wall_history: EventWriter<RecordWallHistory>,
    mut roof_history: EventWriter<RecordRoofHistory>,
) {
    for (wall, original, range) in archivist.take() {
        // The roof row feeds the RoofDelete FIRST history group.
        let roof = public
            .roof_entity(wall)
            .and_then(|entity| roofs.get(entity).ok())
            .map(|roof| roof.0);
        match record_wall_bulldoze(
            RecoveredWallId(wall.0),
            &original,
            range.low,
            range.high,
            0,
            roof.as_ref(),
        ) {
            Ok(history) => {
                if let Some(roof_edit) = history.roof {
                    roof_history.write(RecordRoofHistory(roof_edit));
                }
                wall_history.write(RecordWallHistory(Arc::new(history.wall)));
            }
            Err(_) => {}
        }
    }
}
