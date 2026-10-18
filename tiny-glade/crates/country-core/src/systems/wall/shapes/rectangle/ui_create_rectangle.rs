//! B `systems/wall/shapes/rectangle/ui_create_rectangle.rs`: rectangle drawing
//! preview.
//!
//! Evidence: `chapters/systems/wall/shapes/rectangle/ui_create_rectangle.rs.md`
//! (O, 0x14155d410). The drag rectangle dims/orientation are written to
//! PreviewRectangle and (on submit) emitted as CreateRectangleCmd. The exact
//! `rectangle_dims_from_cursor_pos` (0x140c99520) projection is 待取证
//! (structural approximation below); the `wall_rectangle_canvas` input action
//! is proxied by [`RectangleCanvasInput`].

use bevy_ecs::prelude::*;
use glam::Vec2;

use super::get_rectangle_orientation;
use super::super::{CreateRectangleCmd, PreviewRectangle, SelectedWallShape};
use crate::resources::cursor_terrain_raycast::CursorTerrainRaycast;
use crate::systems::ui::signifiers::signifier_create::UiSignifierStream;
use crate::systems::ui::signifiers::signifier_factory::SignifierFactory;
use ::utils::geometry::Rectangle2d;

/// Local proxy for the `wall_rectangle_canvas` input action (待取证).
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct RectangleCanvasInput {
    pub anchor: Option<Vec2>,
    pub submitting: bool,
}

/// Native boxed `FnOnce` environment (fresh 0x14155e460): Arc header
/// `{strong=1, weak=1}`, `Rectangle2d` at `+0x10`, flag byte at `+0x28`
/// (allocation `0x30`). `data = inner + 0x10` for the `vtable[+0x28]` call.
///
/// Seam (REBUILD4): the call_once body (0x141559a10 =
/// `Rectangle2d::as_curve2` + `crashy_resample_remap_per_segment(0.5, xvy)`)
/// and the entity build are the render-host seam; `marker` stays `0`.
#[repr(C)]
pub struct RectangleSignifierEnv {
    pub strong: u64,
    pub weak: u64,
    pub rectangle: Rectangle2d,
    pub flag: u8,
    pub pad: [u8; 7],
}

/// `Rectangle2d` native `Display` (0x141559ce0 = `debug_struct_field4_finish`):
/// fields in native order `orientation, width, length, center`.
fn rectangle_display(rectangle: &Rectangle2d) -> String {
    format!(
        "Rectangle2d {{ orientation: {:?}, width: {:?}, length: {:?}, center: {:?} }}",
        rectangle.axis, rectangle.size.x, rectangle.size.y, rectangle.center
    )
}

/// B 0x14155e460: `cursor_rectangle_` preview signifier factory.
///
/// Native payload = `format!("cursor_rectangle_{}{}", rectangle, flag)`
/// (pieces `"cursor_rectangle_"` + `""`), identical 5-qword record layout to
/// [`SignifierFactory`].
pub fn rectangle_signifier(rectangle: Rectangle2d, flag: bool) -> SignifierFactory {
    let name = format!("cursor_rectangle_{}{}", rectangle_display(&rectangle), flag);
    let env = RectangleSignifierEnv {
        strong: 1,
        weak: 1,
        rectangle,
        flag: u8::from(flag),
        pad: [0; 7],
    };
    // Layout-only pointer (see `circle_signifier`); the vtable invocation is
    // the registered build seam.
    SignifierFactory::new(
        name.as_bytes(),
        &env as *const RectangleSignifierEnv as *const u8,
        0,
    )
}

/// B 0x141559a10: `call_once<rectangle_signifier::closure_env$0, ()>`.
/// Fresh disasm: `Rectangle2d::as_curve2` then
/// `crashy_resample_remap_per_segment(spacing = 0.5, remap)` where the closure
/// is `xvy(point, 0.0)` (2D → 3D with y = 0; callee 0x140c9cde0). The remapped
/// curve uses `CurveCoordSemantic3D` (symbol 0x14146b5f0).
pub fn rectangle_signifier_curve(rectangle: &Rectangle2d) -> utils::curve::Curve3 {
    rectangle
        .as_curve2()
        .crashy_resample_remap_per_segment(0.5, |_segment, _t, point| {
            utils::swizzlers::xvy(point, 0.0)
        })
}

/// B 0x14155d410 (12+ param system).
pub fn ui_create_rectangle(
    selected: Res<SelectedWallShape>,
    cursor: Res<CursorTerrainRaycast>,
    mut preview: ResMut<PreviewRectangle>,
    mut input: ResMut<RectangleCanvasInput>,
    mut commands: EventWriter<CreateRectangleCmd>,
    mut stream: ResMut<UiSignifierStream>,
) {
    if !selected.is_rectangle() {
        *preview = PreviewRectangle::default();
        return;
    }
    if cursor.tag == 0 {
        return;
    }
    let cursor_pos = Vec2::new(cursor.pos.x, cursor.pos.z);
    let anchor = match input.anchor {
        Some(anchor) => anchor,
        None => {
            input.anchor = Some(cursor_pos);
            cursor_pos
        }
    };
    // Drag rectangle: center at the drag midpoint, dims = double the projected
    // delta along the orientation axes (0x140c99520 待取证).
    let orientation = get_rectangle_orientation(0.0);
    let perpendicular = Vec2::new(-orientation.y, orientation.x);
    let delta = cursor_pos - anchor;
    let width = (delta.dot(orientation)).abs() * 2.0;
    let height = (delta.dot(perpendicular)).abs() * 2.0;
    let center = anchor + delta * 0.5;
    let dimensions = Vec2::new(width, height);
    preview.center = center;
    preview.dimensions = dimensions;
    preview.orientation = orientation;

    if input.submitting {
        commands.write(CreateRectangleCmd {
            center,
            dimensions,
            orientation,
            // 原生事件 +8 的 previous `GlobalId`（0 = 无）由 canvas 输入/选中
            // 建筑携带；当前恢复的 RectangleCanvasInput 未建模该字段，登记 seam。
            previous_wall: None,
        });
        input.anchor = None;
        input.submitting = false;
    }
    stream.write_factory(&rectangle_signifier(
        Rectangle2d {
            axis: orientation,
            center,
            size: dimensions,
        },
        // Native unconditional call site (0x14155db47, `MOV R8B,1`). The other
        // call (0x14155da9a) uses `!clamped` in a branch this shell does not
        // model.
        true,
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rectangle_signifier_name_uses_native_display_format() {
        let rectangle = Rectangle2d {
            axis: Vec2::X,
            center: Vec2::new(1.0, 2.0),
            size: Vec2::new(3.0, 4.0),
        };
        let factory = rectangle_signifier(rectangle, true);
        // Display impl 0x141559ce0 = debug_struct_field4_finish, field order
        // orientation/width/length/center; payload = "cursor_rectangle_" + it + bool.
        assert_eq!(
            factory.payload(),
            b"cursor_rectangle_Rectangle2d { orientation: Vec2(1.0, 0.0), width: 3.0, length: 4.0, center: Vec2(1.0, 2.0) }true"
        );
        assert_eq!(factory.payload().len(), factory.len());
    }

    #[test]
    fn rectangle_signifier_curve_resamples_to_flat_3d() {
        let rectangle = Rectangle2d {
            axis: Vec2::X,
            center: Vec2::ZERO,
            size: Vec2::new(4.0, 2.0),
        };
        let curve = rectangle_signifier_curve(&rectangle);
        // as_curve2 的 4 角 + 0.5 间距重采样；xvy 使所有 y 保持 0。
        assert!(curve.points.len() >= 5);
        assert!(curve.points.iter().all(|point| point.y == 0.0));
    }
}
