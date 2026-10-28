//! B `systems/wall/shapes/rectangle/finish_edit_rectangle.rs`: rectangle edit
//! finish.
//!
//! Evidence: `chapters/systems/wall/shapes/rectangle/finish_edit_rectangle.rs.md`
//! (O, 0x141464a60) + `systems/wall/shapes/rectangle/edit_decision.rs` (the
//! `finish_rectangle_edit_decision` core). The 0.01 dual criteria (orientation
//! degrees, dimension length) decide whether the change is recorded; the
//! decorator restore is host 待接线.

use bevy_ecs::prelude::*;

use super::super::FinishEditRectangleCmd;
use crate::systems::wall::shapes::rectangle::edit_decision::RectangleEditState;
use crate::systems::wall::shapes::rectangle::edit_decision::finish_rectangle_edit_decision;
use crate::resources::walls::InnerWalls;
use crate::resources::history::{History, EditType};
use crate::resources::walls::decorator_archivist::LiveDecoratorArchivist;
use crate::resources::walls::decorator_storage::DecoratorStorage;
use crate::resources::onboarding::NotifyHintSystemShapeEditWasPerformed;
use crate::resources::critters::sheep::CmdCheckForSheepInsideBuilding;

/// B OnRectangleEdited carries the active 80-byte Wip snapshot.
#[derive(Event, Clone, Debug)]
pub struct OnRectangleEdited(pub [u8; 80]);

/// Wip snapshot resource (variant 1 = orientation, 2 = dimensions).
#[derive(Resource, Clone, Debug)]
pub struct RectangleEditUiState(pub [u8; 80]);

impl Default for RectangleEditUiState {
    fn default() -> Self {
        Self([0; 80])
    }
}

/// B 0x141464a60 (9-param system).
pub fn finish_edit_rectangle<S: Send + Sync + 'static>(
    mut finish: EventReader<FinishEditRectangleCmd>,
    mut ui_state: ResMut<RectangleEditUiState>,
    inner: Res<InnerWalls>,
    mut history: ResMut<History<S>>,
    mut storage: ResMut<DecoratorStorage>,
    mut archivist: ResMut<LiveDecoratorArchivist>,
    mut edited: EventWriter<OnRectangleEdited>,
    mut hint: EventWriter<NotifyHintSystemShapeEditWasPerformed>,
    mut sheep: EventWriter<CmdCheckForSheepInsideBuilding>,
) {
    let mut state = RectangleEditState(ui_state.0);
    let snapshot = finish_rectangle_edit_decision(&mut state, false, None)
        .expect("invalid rectangle edit state");
    if let Some(snapshot) = snapshot.active_state_snapshot { edited.write(OnRectangleEdited(snapshot)); }
    let has_finish = finish.read().count() != 0;
    let wall = state.wall().map(|id| crate::resources::walls::WallId(id.0));
    let source = wall.and_then(|wall| inner.get(wall)).map(|wall| wall.source());
    let decision = finish_rectangle_edit_decision(&mut state, has_finish, source)
        .expect("invalid rectangle finish state or authority");
    if decision.restore_archived_decorators { archivist.extract_and_restore_to_storage(&mut storage); }
    if let Some(edit) = decision.history {
        history.add(EditType::Wall(std::sync::Arc::new(edit))).expect("rectangle edit history recording");
    }
    if decision.empty_notification { hint.write(NotifyHintSystemShapeEditWasPerformed); }
    if let Some(wall) = decision.wall_notification {
        sheep.write(CmdCheckForSheepInsideBuilding { wall: crate::resources::walls::WallId(wall.0) });
    }
    ui_state.0 = state.0;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::walls::{WallId, WallState};
    use crate::resources::roofs::geometry::surface::Curve2;
    use crate::systems::wall::valid_enclosure::{Shape, ShapeParameters};
    use glam::Vec2;
    use utils::geometry::Rectangle2d;

    #[test]
    fn real_finish_schedule_emits_snapshot_and_commits_once_for_last_batch() {
        let mut world = World::new();
        let rect = Rectangle2d::new(Vec2::ZERO, Vec2::splat(2.0), Vec2::X);
        let mut inner = InnerWalls::new();
        let mut wall_state = WallState::new(Curve2::try_new(vec![[0.0, 0.0], [1.0, 0.0]], false).unwrap(), 2.0, 0, 0, false);
        wall_state.snapshot.source.shape = ShapeParameters::from_shape(Shape::Rectangle(rect)).to_snapshot();
        wall_state.snapshot.source.curve_kind_tag = 1;
        let wall = inner.new_wall_from_curve(wall_state);
        world.insert_resource(inner);
        let mut bytes = [0u8; 80];
        bytes[..4].copy_from_slice(&2u32.to_le_bytes());
        bytes[8..16].copy_from_slice(&wall.0.to_le_bytes());
        for (at, value) in [(24,1.0f32),(40,2.0),(44,2.0),(48,0.0),(52,1.0),(64,2.0),(68,2.0)] { bytes[at..at+4].copy_from_slice(&value.to_le_bytes()); }
        world.insert_resource(RectangleEditUiState(bytes));
        let mut history = History::<()>::after_observed_future_clear(); history.start_input_recording(0).unwrap(); world.insert_resource(history);
        world.init_resource::<DecoratorStorage>(); world.init_resource::<LiveDecoratorArchivist>();
        world.init_resource::<Events<FinishEditRectangleCmd>>(); world.init_resource::<Events<OnRectangleEdited>>();
        world.init_resource::<Events<NotifyHintSystemShapeEditWasPerformed>>(); world.init_resource::<Events<CmdCheckForSheepInsideBuilding>>();
        let mut schedule = Schedule::default(); schedule.add_systems(finish_edit_rectangle::<()>);
        schedule.run(&mut world);
        assert_eq!(world.resource_mut::<Events<OnRectangleEdited>>().drain().count(), 1);
        assert!(world.resource::<History<()>>().pending().unwrap().edits.is_empty());
        world.send_event(FinishEditRectangleCmd { wall: WallId(999) });
        world.send_event(FinishEditRectangleCmd { wall: WallId(998) });
        schedule.run(&mut world);
        assert_eq!(world.resource_mut::<Events<OnRectangleEdited>>().drain().count(), 1);
        assert_eq!(world.resource::<History<()>>().pending().unwrap().edits.len(), 1);
        assert_eq!(world.resource_mut::<Events<NotifyHintSystemShapeEditWasPerformed>>().drain().count(), 1);
        let sheep: Vec<_> = world.resource_mut::<Events<CmdCheckForSheepInsideBuilding>>().drain().collect();
        assert_eq!(sheep.len(), 1); assert_eq!(sheep[0].wall, wall);
        assert_eq!(&world.resource::<RectangleEditUiState>().0[..4], &[0;4]);
        // Unordered dimension comparison follows restore, with no second edit.
        bytes[48..52].copy_from_slice(&1.0f32.to_le_bytes());
        bytes[52..56].copy_from_slice(&0.0f32.to_le_bytes());
        bytes[64..68].copy_from_slice(&f32::NAN.to_le_bytes());
        world.resource_mut::<RectangleEditUiState>().0 = bytes;
        world.resource_mut::<LiveDecoratorArchivist>().archive_storage_decorators(0, &crate::resources::walls::decorator_storage::Decorators::default(), Some(wall));
        world.send_event(FinishEditRectangleCmd { wall: WallId(999) });
        schedule.run(&mut world);
        assert_eq!(world.resource::<History<()>>().pending().unwrap().edits.len(), 1);
        assert!(world.resource::<LiveDecoratorArchivist>().extract_before_after(crate::resources::walls::decorator_storage::DecoratorStorageAddr::Wall(wall)).is_none());
        assert_eq!(&world.resource::<RectangleEditUiState>().0[..4], &[0;4]);
    }
}
