//! B `systems/wall/shapes/circle/ui_edit_circle.rs`: circle radius editing.
//!
//! Evidence: `chapters/systems/wall/shapes/circle/ui_edit_circle.rs.md` (O,
//! 0x14165d460). Radius drags update the wall curve and enclosure; the release
//! only records history when the radius changed by more than 0.01, otherwise
//! it restores archived decorators (extract_before_after_all, host 待接线).

use bevy_ecs::prelude::*;
use ::utils::geometry::Circle2d;

use super::super::{
    EditCircleCmd, FinishEditCircleCmd, to_authority_curve,
};
use crate::recovered_building::wall_state::WallShapeSnapshot;
use crate::resources::walls::InnerWalls;
use crate::systems::wall::{
    history_recording::RecordWallHistory,
    valid_enclosure::{Shape, ShapeParameters},
};

/// Radius-change history threshold (0x3c23d70a @0x142960674).
const CHANGE_THRESHOLD: f32 = 0.01;
/// Circle sample step (0x3e4ccccd @0x1429cd068).
const CIRCLE_SAMPLE_STEP: f32 = 0.2;

/// Edit state capturing the pre-drag wall for the release decision.
#[derive(Resource, Clone, Debug, Default)]
pub struct CircleEditState {
    pub previous: Option<crate::resources::walls::WallState>,
}

fn circle_snapshot(circle: Circle2d) -> WallShapeSnapshot {
    WallShapeSnapshot::from_observed(
        ShapeParameters::from_shape(Shape::Circle(circle)).to_record(),
    )
}

/// B 0x14165d460 (14-param system). Event-driven radius edit; the trigger
/// volume hit-test is the UI producer (待接线).
pub fn ui_edit_circle(
    mut requests: EventReader<EditCircleCmd>,
    mut finish: EventReader<FinishEditCircleCmd>,
    mut inner: ResMut<InnerWalls>,
    mut state: ResMut<CircleEditState>,
    mut history: EventWriter<RecordWallHistory>,
) {
    for event in requests.read() {
        let Some(previous) = inner.get(event.wall).cloned() else {
            continue;
        };
        let circle = Circle2d::new(event.center, event.radius);
        let curve = circle.as_curve2(CIRCLE_SAMPLE_STEP);
        inner.update_wall_state_curve(event.wall, to_authority_curve(&curve));
        inner.update_wall_state_enclosure(event.wall, circle_snapshot(circle));
        state.previous = Some(previous);
    }
    for event in finish.read() {
        let Some(previous) = state.previous.take() else {
            continue;
        };
        let Some(current) = inner.get(event.wall) else {
            continue;
        };
        let prev_params = ShapeParameters::from_snapshot(previous.get_enclosure_params().unwrap());
        let cur_params = ShapeParameters::from_snapshot(current.get_enclosure_params().unwrap());
        let delta = (cur_params.as_circle().radius - prev_params.as_circle().radius).abs();
        if delta <= CHANGE_THRESHOLD {
            // No visible change: the original restores archived decorators
            // (extract_before_after_all 0x140b2c4b0; host 待接线).
            continue;
        }
        let edit = crate::recovered_building::wall_history_edit::WallCompoundHistoryEdit {
            groups: vec![vec![crate::recovered_building::wall_history_edit::WallHistoryEdit::StateChange {
                wall: crate::recovered_building::wall_state::WallId(event.wall.0),
                previous: previous.snapshot().clone(),
                next: current.snapshot().clone(),
                observed_operation: [0; 12],
            }]],
        };
        history.write(RecordWallHistory(std::sync::Arc::new(edit)));
    }
}
