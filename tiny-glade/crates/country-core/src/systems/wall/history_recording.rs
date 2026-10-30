//! Wall compound records enter the same history transaction as roof/raster.
use crate::resources::history::wall_edit::WallCompoundHistoryEdit;
use crate::resources::history::wall_edit::WallSnapshotError;
use crate::resources::history::wall_edit::record_deleted_wall;
use crate::resources::history::wall_edit::record_deleted_wall_with_height;
use crate::resources::history::wall_edit::record_new_wall;
use crate::resources::walls::publication_fields::WallId as RecoveredWallId;
use crate::resources::history::EditType;
use crate::resources::history::History;
use crate::resources::walls::WallId;
use crate::resources::walls::WallState;
use crate::systems::history::recording::HistoryRecordingFailed;
use bevy_ecs::prelude::*;
use std::sync::Arc;

/// Live edit owners emit this once. Replay writes InnerWalls directly and does
/// not emit a second recording request. The color comes from its real owner.
#[derive(Event, Clone, Debug)]
pub struct RecordWallHistory(pub Arc<WallCompoundHistoryEdit>);

/// 原墙体 producer 在自身调用点同步加入同一 History；未录制时按原版 panic。
pub fn record_wall_immediately<S>(
    record: RecordWallHistory,
    history: &mut History<S>,
) {
    let edit = EditType::Wall(record.0);
    history.add(edit.clone()).unwrap_or_else(|_| {
        panic!("Tried writing {edit:?} to History when the input recording is finished")
    });
}

impl RecordWallHistory {
    /// Original seed-create followed by full-state growth, preserving both
    /// concurrent-edit groups and every captured source metadata byte.
    pub fn new_wall(
        wall: WallId,
        source: &WallState,
        color_id: u32,
    ) -> Result<Self, WallSnapshotError> {
        record_new_wall(RecoveredWallId(wall.0), source.snapshot(), color_id)
            .map(|edit| Self(Arc::new(edit)))
    }

    /// Pass the actual removed authority returned by InnerWalls::remove. No
    /// public-cache approximation or newly invented color is captured.
    pub fn deleted_wall(wall: WallId, source: &WallState, color_id: u32) -> Self {
        Self(Arc::new(record_deleted_wall(
            RecoveredWallId(wall.0),
            source.snapshot(),
            color_id,
        )))
    }

    /// Native continuation-failure Delete retains its independent target
    /// height while the previous source remains an unchanged undo snapshot.
    pub fn deleted_wall_with_height(
        wall: WallId,
        source: &WallState,
        color_id: u32,
        restored_height: f32,
    ) -> Self {
        Self(Arc::new(record_deleted_wall_with_height(
            RecoveredWallId(wall.0),
            source.snapshot(),
            color_id,
            restored_height,
        )))
    }
}

pub fn register_wall_recording_events(world: &mut World) {
    world.init_resource::<Events<RecordWallHistory>>();
    world.init_resource::<Events<RecordColorHistory>>();
}

/// `create_rectangle` second `History::add` — `EditType::Color` (tag 9),
/// producer 0x140a94617 (fresh 2026-10-17).
///
/// Native gate = `settings.gates != 0` (GladeColorCustomization +0x38). The
/// allocated 0x30 B record is an Arc header (strong/weak) plus a 32 B payload
/// `{+0x08 = 4, +0x10 = wall id, +0x18 = 0, +0x1c = settings.gates}`.
///
/// Unlock (registered): mapping this in-memory record onto
/// `EditType::Color(Arc<ColorEdit>)` needs a `ColorEdit` layout that matches
/// this payload; the recovered serde `ColorEdit` is a different in-memory
/// record. Until then this owner carries the observed bytes without inventing
/// a `ColorEdit` variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorHistoryRecord {
    /// Payload +0x08 (native constant 4).
    pub kind: u32,
    /// Payload +0x10.
    pub wall: WallId,
    /// Payload +0x18 (native 0).
    pub from: u32,
    /// Payload +0x1c (settings.gates).
    pub to: u32,
}

impl ColorHistoryRecord {
    /// Native allocation 0x30 = Arc header + this 32 B payload.
    pub const PAYLOAD_SIZE: usize = 0x20;

    /// The payload bytes (offsets relative to the payload pointer).
    pub fn payload(&self) -> [u8; Self::PAYLOAD_SIZE] {
        let mut out = [0u8; Self::PAYLOAD_SIZE];
        out[0x08..0x0c].copy_from_slice(&self.kind.to_le_bytes());
        out[0x10..0x18].copy_from_slice(&self.wall.0.to_le_bytes());
        out[0x18..0x1c].copy_from_slice(&self.from.to_le_bytes());
        out[0x1c..0x20].copy_from_slice(&self.to.to_le_bytes());
        out
    }
}

/// Live owner for the create-rectangle Color history record.
#[derive(Event, Clone, Debug)]
pub struct RecordColorHistory(pub ColorHistoryRecord);

pub fn record_color_history(mut events: EventReader<RecordColorHistory>) {
    // No `EditType::Color` mapping yet (see `ColorHistoryRecord`); consuming
    // the events keeps Bevy retention bounded without inventing history.
    for _ in events.read() {}
}

pub fn record_wall_history<S: Send + Sync + 'static>(
    mut events: EventReader<RecordWallHistory>,
    mut history: ResMut<History<S>>,
    mut failures: EventWriter<HistoryRecordingFailed>,
) {
    for event in events.read() {
        let edit = EditType::Wall(event.0.clone());
        if let Err(error) = history.add(edit.clone()) {
            failures.write(HistoryRecordingFailed { edit, error });
        }
    }
}
