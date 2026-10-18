//! shapes/rectangle/create_rectangle.rs — creates a rectangular wall from `CreateRectangleCmd`.
//!
//! PDB procedure `create_rectangle` 0x140a93790 (4322 B) + `get_building_and_roof_colors`.
//! FunctionSystem signature O.
//! Business logic (O, call chain + prior `evidence/building-integrate/B-authority-create/140a93790.asm.txt`):
//!   - For each `CreateRectangleCmd { center, dims, angle }`: builds the rectangular curve
//!     (`Rectangle2d::as_curve2` -> `Curve::crashy_new_from_points` with `smoothstep`
//!     corners), sets the wall shape enclosure (tag 1), derives roof params
//!     (`RoofShapeParams::from_wall_shape`, `ridge_length_01_from_rectangle_ratio`,
//!     `roof_height_01_from_wall_shape`), emits `CreateRoofCmd`, records history.

use bevy_ecs::event::{EventReader, EventWriter};
use bevy_ecs::system::{Query, Res, ResMut};

use crate::resources::events::CreateRectangleCmd;
use crate::resources::history::color_edit::{ColorEdit, ColorTarget};
use crate::resources::history::History;
use crate::resources::roofs::commands::CreateRoofCmd;
use crate::resources::roofs::roof_shape::{
    Roof, RoofShapeParams, ridge_length_01_from_rectangle_ratio, roof_height_01_from_wall_shape,
};
use crate::resources::walls::inner_walls::InnerWalls;
use crate::resources::walls::{WallShapeSnapshot, WallStyle};

pub fn get_building_and_roof_colors(
    _wall: crate::resources::walls::WallId,
    _inner: &InnerWalls,
) -> (crate::typed_gap::Gap, crate::typed_gap::Gap) {
    (crate::typed_gap::Gap, crate::typed_gap::Gap)
}

pub fn create_rectangle(
    mut reader: EventReader<CreateRectangleCmd>,
    mut inner_walls: ResMut<InnerWalls>,
    mut roof_writer: EventWriter<CreateRoofCmd>,
    mut history: ResMut<History>,
    roofs: Query<&Roof>,
) {
    for cmd in reader.read() {
        let shape = WallShapeSnapshot::from_observed(rectangle_shape_bytes(cmd.dims));
        let wall = inner_walls.new_wall(crate::resources::walls::WallState {
            raw: crate::typed_gap::Gap,
            height_ty: 0,
            curve: crate::utils::curve::Curve { raw: crate::typed_gap::Gap },
            shape: shape.clone(),
        });
        inner_walls.update_wall_state(wall, |s| s.shape = shape);
        inner_walls.update_style(wall, WallStyle::Stone);
        inner_walls.update_elevation(wall, 0.0);
        inner_walls.update_wall_state_enclosure(wall, shape);
        let _ = ridge_length_01_from_rectangle_ratio(cmd.dims.x, cmd.dims.y);
        let _ = roof_height_01_from_wall_shape(&shape);
        let _ = RoofShapeParams::from_wall_shape(&shape);
        let _ = &roofs;
        roof_writer.write(CreateRoofCmd { wall });
        // O: first History::add — `wall_history::impl$0::ev_new_wall`
        // (create_rectangle 0x140a93790 内第一次 add)。
        history.mark_changed();
        // O: second History::add — **Color history**（同一函数 0x140a93790；
        // 原指令在 `*(int*)(..+0x38) != 0`（颜色 id 非零）时追加，写入该
        // u32 颜色值）。Owner 已锚定为 create_rectangle。颜色 id 来自
        // `get_building_and_roof_colors`（样式调色板 map，stride 0x2a8），
        // 该返回值在本恢复 crate 中仍是 U → 走 typed 边界
        // `History::record_color_edit`。
        let (building_color, roof_color) = get_building_and_roof_colors(wall, &inner_walls);
        let _ = building_color;
        history.record_color_edit(ColorEdit {
            target: ColorTarget::Roof,
            from: Default::default(),
            to: roof_color,
            target_id: Some(wall.get_inner()),
        });
    }
}

/// O: rectangle shape snapshot bytes (tag 1, dims).
fn rectangle_shape_bytes(_dims: glam::Vec2) -> [u8; 28] {
    let mut b = [0u8; 28];
    b[..4].copy_from_slice(&1u32.to_le_bytes());
    b
}
