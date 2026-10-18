//! `ui_terrain_focus_mode.rs` — 地形聚焦模式（B 0x1409a4fb0）。
//!
//! 证据：fresh 反编译（restorer-terrain-clutter/jobs/decomp/1409a4fb0，120 KB 上限截断）+
//! closures（logic-verifier-terrain/jobs/focus_closures-result.json）。
//! 逐语句对照（未截断段）：
//! - 门 `*(int*)*in_RCX != 5 → return`（AppMode::Editing(EditSubject::Terrain)）；
//! - `fVar52 = (float)((int*)*in_RCX)[1]` = `EditSubject::Terrain(u32)` 载荷（上一聚焦值）；
//! - **无活动笔画路径**（`*(longlong*)(lVar41+0x20) == 0`）：
//!   `Rapier::raycast_simple`；命中 tag `0x11` → `fVar52 = f32::MAX`（未命中哨兵
//!   `0x7f7fffff`），否则 `fVar52 = 3D 距离(相机,命中点)`；
//!   悬停位 `tag&1` → 悬停点（+4/+8/+0xc）；条件
//!   `3Ddist(相机,悬停点) < fVar52 ∧ selection.tag==1 ∧ 上一聚焦值 != selection.value`
//!   → 写 `WindowFocusEvent(value)`（stride 0x10）；
//! - **有活动笔画路径**：笔画点集 min/max（minps/maxps NaN 保持）→ 中心 →
//!   `Circle2d::new` → `as_curve3`；`scale = height*7.0 + (1-height)*2.5`；
//!   `AppendOnlyVec::push<UiAppendSignifier>`（含截断段内共 5 处）；
//!   `CircularF32` new/distance/set_value/as_f32/lerp；点击 widget → 写
//!   `RequestAppModeChange` + `CameraMatrices::xz_projective_rotation`。
//! - closure$1 = SignifierEntityBuilder::build（shader "vertex_color_shader"）+
//!   bevy command push `insert<DrawTopology>`；closure$2/3/4 同 builder，名字分别为
//!   "brush_arrow_flat_to_single_forward" / "brush_arrow_flat_to_w_flat_single_up" /
//!   动态捕获串（RNE 直读）。
//!
//! crate 无 SignifierEntityBuilder/宿主实体机制，signifier 以 [`UiSignifierStream`] 承载，
//! `DrawTopology` insert 以 [`SignifierDrawTopology`] 事件登记；相机/焦点 HashMap/
//! CircularF32 的宿主接线登记待接（不伪造实体）。
use bevy_ecs::prelude::*;
use glam::Vec3;

use crate::{
    name_hash::NameHash,
    resources::terrain_editing::TerrainEditingData,
    systems::{
        mode_manager::{AppMode, EditSubject},
        ui::signifiers::signifier_create::{UiAppendSignifier, UiSignifierStream},
    },
};

/// 未命中哨兵距离（O：`0x7f7fffff` = f32::MAX）。
pub const RAYCAST_MISS_DISTANCE: f32 = f32::MAX;
/// 命中 tag（O：raycast_simple 命中种类 0x11 = 未命中哨兵）。
pub const RAYCAST_MISS_TAG: u8 = 0x11;
/// 聚焦判定阈值（O：`_UNK_142960674` = 0.01；截断段常量表登记）。
pub const FOCUS_EPSILON: f32 = 0.01;
/// 聚焦推进速度（O：`_UNK_142a16d10` = −10.0）。
pub const FOCUS_SPEED: f32 = -10.0;
/// 聚焦常量（O：`_UNK_1429cd128` = 1.2）。
pub const FOCUS_LEAD: f32 = 1.2;
/// lerp 系数（O：`_UNK_142a7cd48` = 7.0）。
pub const FOCUS_LERP: f32 = 7.0;
/// 缩放常量（O：`_UNK_1429cd064` = 2.5）。
pub const FOCUS_SCALE: f32 = 2.5;
/// 有笔画路径 scale 的高度系数（O：`height*7.0 + (1-height)*2.5`）。
pub const STROKE_SCALE_HIGH: f32 = 7.0;
/// 有笔画路径 scale 的低高度基值（O：2.5）。
pub const STROKE_SCALE_LOW: f32 = 2.5;

/// 聚焦射线回读适配（I：`Rapier::raycast_simple` 结果；未命中 tag = 0x11）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct TerrainFocusRaycast {
    /// 命中 tag（0x11 = 未命中）。
    pub tag: u8,
    /// 命中点（世界）。
    pub hit_pos: Vec3,
    /// 相机位置（3D 距离计算输入）。
    pub camera_pos: Vec3,
}

/// 悬停采样适配（I：光标命中位 + 位置；O 读取 `tag&1`、+4/+8/+0xc）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct TerrainFocusHover {
    pub tag: u32,
    pub pos: Vec3,
}

/// 聚焦选择适配（I：`*local_348 == 1` 判别 + `local_348[2]` 载荷）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct TerrainFocusSelection {
    /// 判别（O：`*local_348 == 1`）。
    pub tag: i32,
    /// 载荷值（O：`*(longlong*)(local_348+2)`）。
    pub value: i64,
}

/// 聚焦模式触发事件（I：`WindowFocusEvent` 载荷 = 聚焦值，stride 0x10）。
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct TerrainFocusTrigger {
    pub value: i64,
}

/// signifier 名字（O：closure$1 RNE 直读 = "vertex_color_shader"）。
pub const VERTEX_COLOR_SHADER_NAME: &str = "vertex_color_shader";
/// closure$2 名字（O：RNE 直读）。
pub const BRUSH_ARROW_SINGLE_FORWARD_NAME: &str = "brush_arrow_flat_to_single_forward";
/// closure$3 名字（O：RNE 直读）。
pub const BRUSH_ARROW_W_FLAT_SINGLE_UP_NAME: &str = "brush_arrow_flat_to_w_flat_single_up";

/// DrawTopology 组件（O：closure$1 `insert<DrawTopology>`；字段语义 U）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct DrawTopology;

/// signifier 实体 DrawTopology insert 事件（I：宿主在 signifier 实体上插入
/// [`DrawTopology`]；对应 closure$1 的 command push insert）。
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct SignifierDrawTopology {
    pub name: NameHash,
}

/// 由笔画点集求聚焦圆（O：minps/maxps 逐点 min/max x/z → 中心；到中心最近点距离 = 半径）。
pub fn stroke_focus_circle(points: &[[f32; 3]]) -> Option<(glam::Vec2, f32)> {
    let mut iter = points.iter();
    let first = iter.next()?;
    let mut min_x = first[0];
    let mut max_x = first[0];
    let mut min_z = first[2];
    let mut max_z = first[2];
    for point in iter {
        if point[0] < min_x {
            min_x = point[0];
        }
        if max_x < point[0] {
            max_x = point[0];
        }
        if point[2] < min_z {
            min_z = point[2];
        }
        if max_z < point[2] {
            max_z = point[2];
        }
    }
    let center = glam::Vec2::new((min_x + max_x) * 0.5, (min_z + max_z) * 0.5);
    let mut radius = 1.0e6_f32;
    for point in points {
        let dx = point[0] - center.x;
        let dz = point[2] - center.y;
        let dist = (dz * dz + dx * dx).sqrt();
        if dist < radius {
            radius = dist;
        }
    }
    Some((center, radius))
}

/// `ui_terrain_focus_mode` @0x1409a4fb0。
#[allow(clippy::too_many_arguments)]
pub fn ui_terrain_focus_mode(
    mode: Res<AppMode>,
    raycast: Res<TerrainFocusRaycast>,
    hover: Res<TerrainFocusHover>,
    selection: Res<TerrainFocusSelection>,
    terrain: Res<TerrainEditingData>,
    mut stream: ResMut<UiSignifierStream>,
    mut draw_topology: EventWriter<SignifierDrawTopology>,
    mut triggers: EventWriter<TerrainFocusTrigger>,
) {
    // 门 = AppMode::Editing(EditSubject::Terrain)（O：fresh `!= 5 → return`）。
    if mode.native_tag() != 5 {
        return;
    }
    // `EditSubject::Terrain(u32)` 载荷（O：`((int*)*in_RCX)[1]`）。
    let previous_focus = match *mode {
        AppMode::Editing(EditSubject::Terrain(value)) => value,
        _ => 0,
    };
    // **无活动笔画路径**（O：`*(longlong*)(lVar41+0x20) == 0`）。
    if terrain.strokes.is_empty() {
        // raycast_simple 距离：tag 0x11 → f32::MAX；否则 3D 距离。
        let focus_distance = if raycast.tag == RAYCAST_MISS_TAG {
            RAYCAST_MISS_DISTANCE
        } else {
            (raycast.hit_pos - raycast.camera_pos).length()
        };
        // 悬停点距离（tag&1 才读取位置，否则 0）。
        let hover_pos = if (hover.tag & 1) != 0 {
            hover.pos
        } else {
            Vec3::ZERO
        };
        let hover_distance = (hover_pos - raycast.camera_pos).length();
        if hover_distance < focus_distance
            && selection.tag == 1
            && previous_focus as f32 != selection.value as f32
        {
            // WindowFocusEvent(value)（stride 0x10）。
            triggers.write(TerrainFocusTrigger {
                value: selection.value,
            });
        }
        return;
    }

    // **有活动笔画路径**：最后一个笔画的包围盒 → Circle2d → scale。
    let Some(last) = terrain.strokes.values().last() else {
        return;
    };
    let (center, _radius) = stroke_focus_circle(&last.points).unwrap_or((glam::Vec2::ZERO, 0.0));
    // O：`scale = height*7.0 + (1-height)*2.5`（height = 笔画高度归一值）。
    let height = last.height_ws.clamp(0.0, 1.0);
    let scale = height * STROKE_SCALE_HIGH + (1.0 - height) * STROKE_SCALE_LOW;
    let position = Vec3::new(center.x, last.height_ws, center.y);

    // closure$1：SignifierEntityBuilder::build(shader "vertex_color_shader") +
    // command push `insert<DrawTopology>`。
    let shader_name = NameHash::compute(VERTEX_COLOR_SHADER_NAME);
    stream.write(UiAppendSignifier {
        tag: 1,
        kind: 1,
        position,
        scale,
        name: shader_name.0,
    });
    draw_topology.write(SignifierDrawTopology { name: shader_name });
    // closure$2：brush_arrow_flat_to_single_forward。
    stream.write(UiAppendSignifier {
        tag: 1,
        kind: 1,
        position,
        scale,
        name: NameHash::compute(BRUSH_ARROW_SINGLE_FORWARD_NAME).0,
    });
    // closure$3：brush_arrow_flat_to_w_flat_single_up。
    stream.write(UiAppendSignifier {
        tag: 1,
        kind: 1,
        position,
        scale,
        name: NameHash::compute(BRUSH_ARROW_W_FLAT_SINGLE_UP_NAME).0,
    });
    // closure$4：动态捕获名字串（宿主 signifier 名；未接入，登记不伪造）。
    // CircularF32（new/distance/set_value/as_f32/lerp）与 CameraMatrices::
    // xz_projective_rotation、RequestAppModeChange 点击路径依赖宿主相机/焦点 HashMap，
    // 登记待接；常量（0.01/−10/1.2/7.0）已落库。
    let _ = (
        FOCUS_EPSILON,
        FOCUS_SPEED,
        FOCUS_LEAD,
        FOCUS_LERP,
        FOCUS_SCALE,
    );
}
