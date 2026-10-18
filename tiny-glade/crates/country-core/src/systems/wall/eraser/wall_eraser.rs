//! B `systems/wall/eraser/wall_eraser.rs`: freehand wall eraser.
//!
//! Evidence: `chapters/systems/wall/eraser/wall_eraser.rs.md` (O, 0x140accab0).
//! Each stroke slices the wall curve, marks the enclosure freehand, clamps the
//! height by the remaining length (`wall_height_based_on_length`, 0.75..14.0)
//! and applies the flat-roof `-0.79` compensation; the original snapshot is
//! cached in the archivist for the finish history. The ChildWall split /
//! rewiring path is 待取证.

use bevy_ecs::prelude::*;

use super::archivist::{ErasedRange, WallEraserArchivist};
use crate::resources::history::wall_edit::slice_source_curve;
use crate::resources::walls::publication_fields::WallShapeSnapshot;
use crate::resources::{
    roofs::Roof,
    walls::{InnerWalls, WallState, public_walls::PublicWalls},
};
use crate::systems::wall::eraser::WallEraserStroke;

/// Height lerp range for `wall_height_based_on_length`
/// (0.75 @0x1429edb90, 14.0 @0x142a7de00).
const MIN_HEIGHT: f32 = 0.75;
const MAX_HEIGHT: f32 = 14.0;
/// Flat-roof erase compensation (-0.79, _UNK_142a7e320).
const FLAT_ROOF_HEIGHT_DELTA: f32 = -0.79;

fn freehand_shape() -> WallShapeSnapshot {
    let mut bytes = [0; 28];
    bytes[..4].copy_from_slice(&2u32.to_le_bytes());
    WallShapeSnapshot::from_observed(bytes)
}

/// Inline B line 599: taller walls grow with length, never exceeding the
/// current height. Exact u source 待取证; length / MAX_HEIGHT normalized.
fn wall_height_based_on_length(length: f32, current: f32) -> f32 {
    let t = if length < 0.0 {
        0.0
    } else if length > MAX_HEIGHT {
        1.0
    } else {
        length / MAX_HEIGHT
    };
    let height = MIN_HEIGHT + (MAX_HEIGHT - MIN_HEIGHT) * t;
    if height < current { height } else { current }
}

/// B 0x140accab0 (drag-brush eraser system). The stroke u-range is supplied by
/// the UI producer; per affected wall the authority is sliced and cached.
pub fn wall_eraser(
    mut strokes: EventReader<WallEraserStroke>,
    mut inner: ResMut<InnerWalls>,
    public: Res<PublicWalls>,
    roofs: Query<&Roof>,
    mut archivist: ResMut<WallEraserArchivist>,
) {
    for stroke in strokes.read() {
        let Some(state) = inner.get(stroke.wall).cloned() else {
            continue;
        };
        let original = state.snapshot().clone();
        let range = ErasedRange {
            low: stroke.low,
            high: stroke.high,
        };
        archivist.cache_if_original_wall(stroke.wall, &original, range);
        let Ok(sliced) = slice_source_curve(&original.source.curve, stroke.low, stroke.high)
        else {
            continue;
        };
        let erased_height = wall_height_based_on_length(sliced.length, state.height());
        let mut next = original.clone();
        next.source.curve = sliced;
        next.source.shape = freehand_shape();
        next.source.curve_kind_tag = 2;
        next.source.extra = erased_height;
        // Flat roof with no rims compensates the erased height downward.
        if state.is_premade_shape() {
            if let Some(roof) = public
                .roof_entity(stroke.wall)
                .and_then(|entity| roofs.get(entity).ok())
            {
                if roof.is_flat_w_no_rim() {
                    next.source.extra += FLAT_ROOF_HEIGHT_DELTA;
                }
            }
        }
        let freehand_ty = state.freehand_ty();
        inner.update_wall_state(
            stroke.wall,
            WallState::from_snapshot(next, freehand_ty),
        );
    }
}
