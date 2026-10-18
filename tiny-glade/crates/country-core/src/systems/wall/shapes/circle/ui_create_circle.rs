//! B `systems/wall/shapes/circle/ui_create_circle.rs`: circle drawing preview.
//!
//! Evidence: `chapters/systems/wall/shapes/circle/ui_create_circle.rs.md` (O,
//! 0x14165bed0) + fresh decomp/disasm 2026-10-17. The native reads the
//! `wall_circle_canvas` input action state (FUN_1409236d0, action code @+0x19):
//!   code 3 → 写预览圆心/半径；code 4 → **累加** `radius = 旧半径 + √(dx²+dz²)`
//!   （输入资源 +0x140 的两 f32），`>4.0` 置位钳制字节（+0xc），不截断半径；
//!   同帧状态跳表 code4 且钳制位置位 → `radius = clamp(圆心到游标投影距离,
//!   0.9, 8.0)`（0x3f666667 / 0x41000000）。
//! The reconstructed library consumes the world-space
//! [`CursorTerrainRaycast`] directly (delta = +0x140 的位移代理) and keeps a
//! local input proxy.
//!
//! 登记（fresh 分歧点）：① 输入动作状态机 code 3/4/<2 与状态跳表
//! code{2,3,4} 已按体实现；② GladeBorder::is_shape_inside + snapping +
//! circle 编辑链（as_curve2 + update curve/enclosure，case7）+
//! PreferredNodeAlignment/AlignKind 记录写 + DebugLine + puffin scope +
//! 60.0 阈值分支为宿主/UI 接线，登记不重建；③ 游标投影在原生走相机矩阵
//! （in_R9），库层以世界 XZ 距离代理。

use bevy_ecs::prelude::*;
use glam::Vec2;

use super::super::{CreateCircleCmd, PreviewCircle, SelectedWallShape};
use crate::resources::cursor_terrain_raycast::CursorTerrainRaycast;
use crate::systems::ui::signifiers::signifier_create::UiSignifierStream;
use crate::systems::ui::signifiers::signifier_factory::SignifierFactory;
use ::utils::geometry::Circle2d;

/** Maximum circle radius for the drag accumulator (0x40800000 = 4.0). */
const MAX_RADIUS: f32 = 4.0;
/// Clamped-distance lower bound (0x3f666667 = 0.9).
const MIN_CLAMPED_RADIUS: f32 = 0.9;
/// Clamped-distance upper bound (0x41000000 = 8.0).
const MAX_CLAMPED_RADIUS: f32 = 8.0;

/// Local proxy for the `wall_circle_canvas` input action state (B
/// FUN_1409236d0; exact input resource 待取证).
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct CircleCanvasInput {
    /// Input action code (native action state `+0x19`). fresh 0x14165bed0
    /// dispatch: `3` = 初始/写状态、`4` = 拖拽累加、`<2` = 按下/释放传输。
    pub action_code: u8,
    /// Anchor locked on first press; circle center.
    pub anchor: Option<Vec2>,
    /// Accumulated drawing radius（原生 code-4 累加；无独立初始半径时从 0 起）。
    pub radius: f32,
    /// Native 钳制字节（>4.0 置位；原生 @preview+0xc）。
    pub clamped: bool,
    /// Release submits the CreateCircleCmd.
    pub submitting: bool,
}

/// Native closure environment captured by the signifier factory's boxed
/// `FnOnce` allocation (fresh 0x14165cd40): Arc header `{strong=1, weak=1}`,
/// the `Circle2d` at `+0x10`, the flag byte at `+0x20` (allocation `0x28`).
/// The consumer calls `vtable[+0x28]` with `data = inner + 0x10`.
///
/// Seam (REBUILD4): the vtable `+0x28` call_once body
/// (0x14165cf80 `circle_signifier::closure$0`) is the render-host entity build
/// (`SignifierEntityBuilder`, `DrawTopology`, `DashedLineLength(perimeter)`,
/// `MeshFollowTerrain`). `marker` is left `0` until that builder lands.
#[repr(C)]
pub struct CircleSignifierEnv {
    pub strong: u64,
    pub weak: u64,
    pub circle: Circle2d,
    pub flag: u8,
    pub pad: [u8; 7],
}

/// `Circle2d`'s native `Display` (0x14165bd40 = `debug_struct_field3_finish`):
/// `Circle2d { center: Vec2(x, y), radius: r, rotation: rot }` with Debug field
/// values. The impl lives in the `utils` domain; this local renderer preserves
/// the exact string until the `Display` impl is recovered there.
fn circle_display(circle: &Circle2d) -> String {
    format!(
        "Circle2d {{ center: {:?}, radius: {:?}, rotation: {:?} }}",
        circle.center, circle.radius, circle.rotation
    )
}

/// B 0x14165cd40: `cursor_circle_` preview signifier factory.
///
/// Native payload = `format!("cursor_circle_{}{}", circle, flag)` (pieces
/// `"cursor_circle_"` + `""`, args `Circle2d` Display and `bool` Display) and a
/// 40 B record `{name_len, name_ptr, name_len, &env, vtable}` pushed to the
/// `AppendOnlyVec<UiAppendSignifier>` (0x14146c880). The crate's
/// [`SignifierFactory`] is the same 5-qword layout.
pub fn circle_signifier(circle: Circle2d, flag: bool) -> SignifierFactory {
    let name = format!("cursor_circle_{}{}", circle_display(&circle), flag);
    let env = CircleSignifierEnv {
        strong: 1,
        weak: 1,
        circle,
        flag: u8::from(flag),
        pad: [0; 7],
    };
    // The record's `&env` slot is layout-only in this crate (the stream copies
    // the five qwords; the consumer's vtable invocation is the registered
    // build seam). The address is therefore valid for this call only.
    SignifierFactory::new(name.as_bytes(), &env as *const CircleSignifierEnv as *const u8, 0)
}

/// B 0x14165b330 (identical-code-folded at 0x1409a3620 for terrain focus):
/// `call_once<circle_signifier::closure_env$0, ()>`. Fresh disasm: the only
/// business work is `Circle2d::as_curve3(max_step = 0.2, height = 0.0)`
/// (`MOVSS XMM2,0x1429cd068; XORPS XMM3,XMM3`); the returned 24 B curve is
/// handed to the render host.
pub fn circle_signifier_curve(circle: &Circle2d) -> utils::curve::Curve3XZ {
    circle.as_curve3(0.2, 0.0)
}

/// B 0x14165bed0 (12-param system).
pub fn ui_create_circle(
    selected: Res<SelectedWallShape>,
    cursor: Res<CursorTerrainRaycast>,
    mut preview: ResMut<PreviewCircle>,
    mut input: ResMut<CircleCanvasInput>,
    mut commands: EventWriter<CreateCircleCmd>,
    mut stream: ResMut<UiSignifierStream>,
) {
    if !selected.is_circle() {
        *preview = PreviewCircle::default();
        input.anchor = None;
        input.radius = 0.0;
        input.clamped = false;
        input.action_code = 0;
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
            input.radius = 0.0;
            cursor_pos
        }
    };
    // 原生输入动作码（native action state +0x19）：
    //   3 (start)  → 把预览圆心/半径写入 canvas 状态（+0x10..+0x20）
    //   4 (drag)   → radius = 旧半径 + √(dx²+dz²)（输入资源 +0x140）；>4.0 置位
    //                钳制字节（+0xc），**不截断半径**
    //   <2 (press/release) → 原生写按下/释放事件记录（宿主接线）
    match input.action_code {
        3 => {
            preview.center = anchor;
            preview.radius = input.radius;
        }
        4 => {
            // 输入资源 +0x140 的两 f32 = 本帧游标位移；Rust 以 cursor.delta.xz 代理。
            let distance = Vec2::new(cursor.delta.x, cursor.delta.z).length();
            let radius = input.radius + distance;
            if radius > MAX_RADIUS {
                input.clamped = true;
            }
            input.radius = radius;
        }
        _ => {}
    }
    // 原生状态跳表（bVar25 = (action_code−2) ∈ {0,1,2}，其余走默认分支）：
    //   code2 → 圆心/预览状态写；code3 → 圆参数记录（tag 2）；
    //   code4 且钳制字节置位 → 半径改由“圆心到游标投影距离” clamp[0.9, 8.0]
    //   得到，并记录 |Δradius|（PreferredNodeAlignment/AlignKind 记录为宿主）。
    match input.action_code {
        2 => {
            preview.center = anchor;
            preview.radius = input.radius;
        }
        3 => {
            // 原生写 ui 记录 tag 2（圆心/半径），宿主接线，登记。
        }
        4 => {
            if input.clamped {
                // 原生以矩阵把游标投到圆所在平面；库层以世界 XZ 距离代理。
                let distance = (cursor_pos - anchor).length();
                input.radius = distance.clamp(MIN_CLAMPED_RADIUS, MAX_CLAMPED_RADIUS);
            }
        }
        _ => {
            // 原生 GladeBorder::is_shape_inside / PreferredNodeAlignment 分支，
            // 宿主接线，登记。
        }
    }
    preview.center = anchor;
    preview.radius = input.radius;

    if input.submitting {
        commands.write(CreateCircleCmd {
            center: anchor,
            radius: input.radius,
        });
        input.anchor = None;
        input.radius = 0.0;
        input.clamped = false;
        input.action_code = 0;
        input.submitting = false;
    }
    stream.write_factory(&circle_signifier(
        Circle2d {
            center: preview.center,
            radius: preview.radius,
            rotation: preview.rotation,
        },
        // Native second call site (0x14165c595): `XOR R8B,1` of the clamped
        // byte. The unconditional `flag = 1` call (0x14165c492) is the
        // state-machine branch not modelled by this system shell.
        !input.clamped,
    ));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::cursor_terrain_raycast::CursorTerrainRaycast;
    use crate::systems::wall::shapes::SelectedWallShape;
    use glam::Vec3;

    fn run_once(input: &mut CircleCanvasInput, cursor_pos: Vec3, delta: Vec3) -> f32 {
        let mut world = World::new();
        world.insert_resource(SelectedWallShape {
            tag: SelectedWallShape::WALL_SHAPES_TAG,
            ty: SelectedWallShape::TY_CIRCLE,
        });
        world.insert_resource(CursorTerrainRaycast {
            tag: 1,
            pos: cursor_pos,
            delta,
            speed: 0.0,
        });
        world.insert_resource(*input);
        world.insert_resource(PreviewCircle::default());
        world.init_resource::<UiSignifierStream>();
        world.init_resource::<Events<CreateCircleCmd>>();
        let mut schedule = Schedule::default();
        schedule.add_systems(ui_create_circle);
        schedule.run(&mut world);
        let result = *world.resource::<CircleCanvasInput>();
        *input = result;
        world.resource::<PreviewCircle>().radius
    }

    #[test]
    fn drag_accumulates_radius_and_flags_over_four() {
        let anchor = Vec2::new(10.0, 10.0);
        let mut input = CircleCanvasInput {
            action_code: 4,
            anchor: Some(anchor),
            radius: 0.0,
            clamped: false,
            submitting: false,
        };
        // 位移 1.0 → radius = 0 + 1 = 1.0（≤4.0，不置位、不重算）。
        let radius = run_once(&mut input, Vec3::new(10.0, 0.0, 10.0), Vec3::new(1.0, 0.0, 0.0));
        assert!((radius - 1.0).abs() < 1e-6);
        assert!(!input.clamped);
        // 再累加 1.0 → 2.0。
        let radius = run_once(&mut input, Vec3::new(10.0, 0.0, 10.0), Vec3::new(1.0, 0.0, 0.0));
        assert!((radius - 2.0).abs() < 1e-6);
        assert!(!input.clamped);
    }

    #[test]
    fn accumulation_over_four_switches_to_clamped_distance() {
        let anchor = Vec2::new(10.0, 10.0);
        let mut input = CircleCanvasInput {
            action_code: 4,
            anchor: Some(anchor),
            radius: 0.0,
            clamped: false,
            submitting: false,
        };
        // 位移 5.0 → 累加半径 5.0 > 4.0 置位；随后原生改由圆心到游标投影
        // 距离 clamp[0.9, 8.0]：|12−10| = 2.0。
        let radius = run_once(&mut input, Vec3::new(12.0, 0.0, 10.0), Vec3::new(5.0, 0.0, 0.0));
        assert!(input.clamped);
        assert!((radius - 2.0).abs() < 1e-6);
        // 游标贴近圆心 → 下限 0.9。
        let radius = run_once(&mut input, Vec3::new(10.1, 0.0, 10.0), Vec3::new(5.0, 0.0, 0.0));
        assert!((radius - 0.9).abs() < 1e-6);
    }
}

#[cfg(test)]
mod signifier_tests {
    use super::*;

    #[test]
    fn circle_signifier_name_uses_native_display_format() {
        let circle = Circle2d {
            center: Vec2::new(1.0, 2.0),
            radius: 3.0,
            rotation: 0.5,
        };
        let factory = circle_signifier(circle, false);
        // Display impl 0x14165bd40 = debug_struct_field3_finish (center/radius/rotation).
        assert_eq!(
            factory.payload(),
            b"cursor_circle_Circle2d { center: Vec2(1.0, 2.0), radius: 3.0, rotation: 0.5 }false"
        );
    }

    #[test]
    fn circle_signifier_curve_is_flat_at_as_curve3_step() {
        let circle = Circle2d {
            center: Vec2::new(1.0, 2.0),
            radius: 3.0,
            rotation: 0.0,
        };
        let curve = circle_signifier_curve(&circle);
        assert!(!curve.points.is_empty());
        assert!(curve.points.iter().all(|point| point.y == 0.0));
    }
}
