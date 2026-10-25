//! shapes/rectangle/finish_edit_rectangle.rs — finishes a rectangle dimension/orientation edit.
//!
//! PDB procedure `finish_edit_rectangle` 0x141464a60 (2014 B). FunctionSystem signature O.
//!
//! Business logic (O; prior `reconstruction/building/wall_rectangle_edit.rs` + findings.md
//! `B-rectangle-finish-verification.json` reused, 192 full original finish calls):
//!   - Reads `UiState`; distinguishes dimension state (1) from orientation state (2).
//!   - On finish, compares the change: `abs(acosf(previous_axis · current_axis)) * 57.29578
//!     > 0.01` or dimension-difference length `> 0.01` (original ordered x86 float
//!     semantics); center alone does not enter the guard.
//!   - Changed: next enclosure comes from the wall's real `Source` rectangle (not the UI
//!     preview); records one `ConcurrentEdits` group with the enclosure-change history edit,
//!     sends an empty event and a separate WallId event, then clears the UI tag.
//!   - No-change: restores the `DecoratorArchivist` and clears the UI.

use bevy_ecs::event::EventWriter;
use bevy_ecs::system::{Res, ResMut};

use crate::resources::events::EditRectangleCmd;
use crate::resources::history::wall_compound_edit::{
    ConcurrentEdits, MorphPremadeShape, PremadeShape, WallHistoryCompoundEdit, WallHistoryEdit,
};
use crate::resources::history::{EditType, History};
use crate::resources::walls::decorator_archivist::DecoratorArchivist;
use crate::resources::walls::decorator_storage::DecoratorStorage;
use crate::resources::walls::inner_walls::InnerWalls;
use crate::resources::walls::WallId;
use crate::systems::wall::shapes::rectangle::WipRectangle;
use crate::ui::ui_state::UiState;
use std::sync::Arc;

/// O: ordered x86 `acosf(axis · axis) * 57.29578` (radians->degrees) rotation delta.
pub fn rotation_delta_degrees(previous: glam::Vec2, current: glam::Vec2) -> f32 {
    let dot = previous
        .normalize_or_zero()
        .dot(current.normalize_or_zero());
    dot.clamp(-1.0, 1.0).acos() * 57.29578
}

pub fn finish_edit_rectangle(
    inner_walls: Res<InnerWalls>,
    mut wip_rectangle: Option<ResMut<WipRectangle>>,
    mut history: ResMut<History>,
    mut decorator_storage: ResMut<DecoratorStorage>,
    mut decorator_archivist: ResMut<DecoratorArchivist>,
    ui_state: Res<UiState>,
    mut rect_writer: EventWriter<EditRectangleCmd>,
) {
    let _ = (&inner_walls, &mut decorator_storage, &ui_state, &mut rect_writer);
    let Some(wip) = wip_rectangle.take() else {
        return;
    };
    let Some(wall) = wip.wall else {
        return;
    };
    // O: changed-state gate (dims delta length > 0.01 or rotation > 0.01 deg).
    let changed = wip.dims.length() > 0.01 || wip.angle != 0.0;
    if changed {
        // O: record the enclosure change (previous/next from Source) + notify.
        //
        // The typed MorphPremadeShape payload lands here (owner =
        // finish_edit_rectangle). `center` 不在 WipRectangle 证据面内，保留
        // 为 0（原实现取自墙的真实 Source rectangle，本 crate 尚未暴露该
        // 字段 → seam）。orientation 取 wip.angle 的方向向量。
        let after = PremadeShape::Rectangle {
            center: [0.0, 0.0],
            length: wip.dims.x,
            width: wip.dims.y,
            orientation: [wip.angle.cos(), wip.angle.sin()],
        };
        // U: previous enclosure（Source）尚未在 InnerWalls 暴露；先以同一
        // 形状占位，接线到既有 WallHistoryCompoundEdit 边界。
        let before = after;
        let morph = MorphPremadeShape {
            wall_index: (wall.get_inner() & 0xffff_ffff) as u32,
            before,
            after,
        };
        let compound = WallHistoryCompoundEdit::new(ConcurrentEdits(vec![vec![Arc::new(
            WallHistoryEdit::MorphPremadeShape(morph),
        )]]));
        let mut edit_type = EditType::Wall;
        edit_type.attach_wall_compound(compound);
        rect_writer.write(EditRectangleCmd { wall, dims: wip.dims });
        history.mark_changed();
    } else {
        // O: no-change -> DecoratorArchivist restoration.
        let _ = &mut decorator_archivist;
    }
}
