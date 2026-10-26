//! structure_outline.rs — 结构轮廓调试/高亮系统（B `systems::wall::structure_outline`）。
//!
//! 原版系统 `structure_outline`（0x140959fc0，13261 B）是 `add_wall_systems` set 28 chain 的
//! 末位成员；本文件还原它、三个辅助函数与六个签名器闭包。2026-10-20 取证（Ghidra 反编译 +
//! 机器码逐处核对，常量逐字节读 rodata）：
//!
//! ## 12 个参数（`System::run_without_applying_deferred` 0x1404395d0 的资源缺失 panic 串）
//! 1 `EventReader<OutlineWallCmd>`；2 `EventReader<OutlineStairsCmd>`；3 `Res<PublicWalls>`；
//! 4 `Query<&Roof>`（`PublicWalls::roof_component` 0x140a923a0 的组件面）；
//! 5 `UiSignifierStream`；6 `GameTime`（`+0x2c` = `frame_count`，进入签名器名）；
//! 7 `MainCamera`（投影 `+0`、视图 `+0x80`）；8 `WindowSize`（首 u32 = 宽）；
//! 9 `TerrainHeightsData<TerrainHeightSsbo>`；10 `StairsState`；
//! 11 `PlayermadeStairsAssemblyRequests`；12 `EscMenuState`。
//!
//! ## 门控
//! 读 param_12：`tag == 2`（Closed）或 `tag == 10 && flag == 0`（Photomode(false)）才运行；
//! 否则整个系统直接返回，**事件游标不前进**（原版也不读事件）。
//!
//! ## 流程
//! 1. 逐个 `OutlineStairsCmd`（`strength > 0` 才处理，NaN 跳过）：`include_connected == false` →
//!    请求表里**第一个** `segment` 相等的元素调 `FUN_14095fca0`（[`stair_outline_lines`]）；
//!    `true` → `StairsState::get_all_connected_nodes(segment.a)`，两端点任一在连通点集的元素全画。
//! 2. `OutlineWallCmd` 只取**最后一个**事件；`PublicWalls` 里找不到该墙则什么都不做。
//!    `kind`：0 = 仅屋顶；1 = 仅墙；其余（观测值 2）= 墙 + 屋顶。
//!    * 墙部分：`outline_wall_bottom`（预制形状：沿 `(-tz, tx)` 外扩 0.315；自由形：原轮廓）、
//!      预制形状 `outline_wall_top` + 圆的屏幕左右极点竖线 / 矩形四角竖线、自由形
//!      `outline_wall_top`（路径点 xz + 拱高）+ 首尾竖线。
//!    * 屋顶部分（找不到屋顶组件则整段跳过；自由形墙不画屋顶线）：`kind == 0` 时先补画
//!      `outline_wall_top`；圆 → 两条沿屋面剖面的 5 点折线 `outline_circle_roof_*`；
//!      矩形 → 四条角折线 `outline_roof_side_*` + 屋脊线 `outline_roof_ridge_*`。
//!
//! ## 辅助函数
//! * `FUN_140959670` / `FUN_140959860`：`puffin::profile_function!()` 展开里截取函数短名的
//!   `str::contains("::")` 逻辑，只服务 puffin 性能作用域（`func_0x140c16a00` = puffin 开关），
//!   不含游戏语义；`country-core` 不依赖 puffin，故不实现作用域。
//! * `FUN_14095fca0`：[`stair_outline_lines`]；`FUN_140960650`：[`circle_screen_extremes`]。
//! * `FUN_140959aa0` = `format!`（0/1 段无参数短路）；`FUN_140959b80/c40` = 闭包环境 `Arc::new`。
//!
//! ## 签名器记录
//! 每条线都是 `SignifierFactory{name, Arc<closure>}`（闭包 0x140961790/0x140962230/0x140961c60/
//! 0x1409611b0/0x140962790，同一实体构建：`SignifierEntityBuilder` 网格折线 + 自定义 shader
//! `"dashed_line"`，随后入队 `DrawTopology::Alt(strength)` 与 `DashedLineLength(length * 3.0)`），
//! 名字 `"{base}_{WallId}_{frame_count}"`。
//!
//! ## 未闭合 / 推断（报告同名）
//! * `PublicWallState` `+0x280` 的 f32（圆竖线地面点的下限）来源未取到写入者；这里按发布器
//!   `collect_raw_wall_publication` 的地形下限规则（`terrain_floor > 0 ? terrain_floor : -2.7`）
//!   取值，见 [`terrain_floor_y`]。
//! * 实体构建沿用 `DebugLine::try_show` 的 spawn 组合（`SignifierEntityBuilder::build` 仍是库内 GAP）。
//! * `get_all_connected_nodes` 的 `StairsState` 第二个寄存器参数在机器码里未使用。

use std::sync::Arc;

use bevy_ecs::event::Event;
use bevy_ecs::prelude::*;
use glam::{Mat4, Vec2, Vec3};
use utils::curve::{Curve2, Curve3XZ};
use utils::geometry::{Circle2d, Rectangle2d};

use crate::asset_libraries::{mesh_library::MeshAsset, transient_mesh::TransientMesh};
use crate::components::{DashedLineLength, drawable::DrawableMeshBundle};
use crate::render::pass_builder::draw_list::DrawTopology;
use crate::resources::gui::EscMenuState;
use crate::resources::roofs::Roof;
use crate::resources::roofs::roof_visual_shape::{RoofBoundsDims, get_roof_expanded_top_and_bottom};
use crate::resources::stairs::stairs_assembly_params::{
    PlayermadeStairsAssemblyParams, PlayermadeStairsAssemblyRequests,
};
use crate::resources::stairs::stairs_state::{StairSegment, StairsState};
use crate::resources::terrain_height::{TerrainHeightSsbo, TerrainHeightsData};
use crate::resources::time::GameTime;
use crate::resources::walls::WallId;
use crate::resources::walls::public_wall_state::PublicWallState;
use crate::resources::walls::public_walls::PublicWalls;
use crate::resources::window::WindowSize;
use crate::systems::camera_rig::main_camera::MainCamera;
use crate::systems::render_loop::draw_passes::draw_lists::{DontCastShadow, UiPass};
use crate::systems::ui::signifiers::{
    animation::SignifierAnimation,
    signifier_create::UiSignifierStream,
    signifier_factory::SignifierFactory,
};
use crate::systems::wall::valid_enclosure::{Shape, ShapeParameters};
use crate::utils::components::transform::Transform;
use crate::utils::debug_viz::{DebugColor, DebugLine};
use crate::utils::geo::{mesh_from_pts, x_h_y};

/// 墙轮廓命令（O：`structure_outline::OutlineWallCmd`，16 B：事件实例 `{id, wall@+8,
/// strength@+0x10, kind@+0x14}`）。
#[repr(C)]
#[derive(Event, Clone, Copy, Debug, Default, PartialEq)]
pub struct OutlineWallCmd {
    /// 目标墙（`PublicWalls` 键）。
    pub wall: WallId,
    /// 线宽/强度（`DrawTopology::Alt` 的宽度；原版用寄存器 XMM11 保存，无 `> 0` 检查）。
    pub strength: f32,
    /// 0 = 仅屋顶、1 = 仅墙、其余 = 墙 + 屋顶（观测值 2）。
    pub kind: u8,
}
const _: () = assert!(core::mem::size_of::<OutlineWallCmd>() == 16);
const _: () = assert!(core::mem::offset_of!(OutlineWallCmd, strength) == 8);
const _: () = assert!(core::mem::offset_of!(OutlineWallCmd, kind) == 12);

impl OutlineWallCmd {
    fn draws_wall(self) -> bool {
        self.kind != 0
    }
    fn draws_roof(self) -> bool {
        self.kind != 1
    }
}

#[repr(C)]
#[derive(Event, Clone, Copy, Debug)]
pub struct OutlineStairsCmd {
    pub segment: StairSegment,   // +00, two sorted endpoint words
    pub strength: f32,           // +08, consumer ignores <=0
    pub kind: u8,                // +0c, 0 = 轮廓带(白)、1 = 轮廓带 + 中心线(红)、2 = 仅中心线
    pub include_connected: bool, // +0d, whole connected stairs if true
}
const _: () = assert!(core::mem::size_of::<OutlineStairsCmd>() == 16);
const _: () = assert!(core::mem::offset_of!(OutlineStairsCmd, strength) == 8);
const _: () = assert!(core::mem::offset_of!(OutlineStairsCmd, kind) == 12);
const _: () = assert!(core::mem::offset_of!(OutlineStairsCmd, include_connected) == 13);

impl OutlineStairsCmd {
    /// Explicit semantic factory for the observed UI producer; the native
    /// caller writes these fields directly, not a recovered original method.
    pub fn connected_color_hover(segment: StairSegment) -> Self {
        Self {
            segment,
            strength: 2.0,
            kind: 0,
            include_connected: true,
        }
    }
}

// ----------------------------------------------------------------------------- 常量（rodata）
/// 0x142aa3280：墙外扩量（圆半径 +0.315；矩形尺寸 +0.63 = 0x142aa4550/4554）。
const WALL_EXPAND: f32 = f32::from_bits(0x3ea1_47ae);
/// 0x1429df1b0/b4：预制墙底轮廓外扩距离（两轴同值）。
const BOTTOM_OFFSET: f32 = f32::from_bits(0x3ea1_47ae);
/// 0x142a7c870：平顶带沿屋顶对墙顶高度的修正。
const FLAT_RIM_TOP_ADJUST: f32 = f32::from_bits(0xbf23_d70a);
/// 0x1429cd068：`Circle2d::as_curve2/as_curve3` 的 max_step。
const CIRCLE_MAX_STEP: f32 = f32::from_bits(0x3e4c_cccd);
/// 0x142925954：屏幕 x 的“最小值”初值。
const SCREEN_X_MIN_INIT: f32 = 100_000.0;
/// 0x1429cd078：闭包里 `DashedLineLength = length * 3.0`。
const DASH_LENGTH_SCALE: f32 = 3.0;
/// 0x1429258d8：楼梯轮廓带宽度附加值。
const STAIR_WIDTH_PAD: f32 = f32::from_bits(0x3e99_999a);
/// 0x142925950：屋面剖面采样步长（5 个采样 `i * 0.25`）。
const ROOF_PROFILE_STEP: f32 = 0.25;
const ROOF_PROFILE_SAMPLES: usize = 5;
/// 闭包里的自定义 shader 名（`custom_shader`，len 0xb）。
const OUTLINE_SHADER: &str = "dashed_line";

/// 入口门控（O：`tag == 2 || (tag == 10 && flag == 0)`）。
pub fn outline_enabled(menu: &EscMenuState) -> bool {
    let view = menu.view();
    view.tag == 2 || (view.tag == 10 && view.flag == 0)
}

// ----------------------------------------------------------------------------- 签名器闭包
/// 五个闭包 `0x140961790/0x140962230/0x140961c60/0x1409611b0/0x140962790` 的共同实体构建：
/// 折线网格 + `"dashed_line"` shader，入队 `DrawTopology::Alt(strength)` 与
/// `DashedLineLength(length * 3.0)`。各闭包只在如何由 env 得到点列上不同（调用方负责）。
fn outline_line_factory(
    name: &str,
    points: Vec<Vec3>,
    strength: f32,
    length: f32,
) -> SignifierFactory {
    SignifierFactory::from_closure(
        name,
        Arc::new(move |meshes, shaders, commands| {
            let Some(shader) = shaders.get_raster_handle_by_name(OUTLINE_SHADER) else {
                log::warn!("结构轮廓着色器未注册：{OUTLINE_SHADER}");
                return Vec::new();
            };
            let mesh = meshes.add(MeshAsset::new(mesh_from_pts("debug_line", &points), None));
            let entity = commands
                .spawn((
                    DrawableMeshBundle {
                        mesh,
                        shader,
                        transform: Transform::identity(),
                    },
                    TransientMesh::new_with_registry(mesh, meshes.transient_meshes()),
                    DrawTopology::Alt(strength),
                    DontCastShadow,
                    // 点已在世界空间；沿用 DebugLine 的“不被通用动画缩到零”设置。
                    SignifierAnimation {
                        flag_8: 1,
                        duration: 0.0,
                        value: 1.0,
                        t: 1.0,
                        eased: 1.0,
                        ..Default::default()
                    },
                    UiPass,
                    DashedLineLength(length * DASH_LENGTH_SCALE),
                ))
                .id();
            vec![entity]
        }),
    )
}

fn push_line(
    stream: &mut UiSignifierStream,
    name: &str,
    points: Vec<Vec3>,
    strength: f32,
    length: f32,
) {
    stream.write_factory(&outline_line_factory(name, points, strength, length));
}

/// 名字的第二步 `format!("{}_{}_{}", base, wall, frame)`（0x142ab4f50）。
fn suffixed_name(base: &str, wall: WallId, frame: u32) -> String {
    format!("{base}_{wall}_{frame}")
}

// ----------------------------------------------------------------------------- 辅助：几何
/// 推断：圆竖线地面点下限（`PublicWallState+0x280`）。按发布器地形下限规则取
/// `terrain_floor > 0 ? terrain_floor : -2.7`（0xc02ccccd），见模块文档“未闭合”。
pub fn terrain_floor_y(state: &PublicWallState) -> f32 {
    let floor = state.authority.source().terrain_floor;
    if floor > 0.0 { floor } else { f32::from_bits(0xc02c_cccd) }
}

/// 圆竖线地面点：`max(terrain, floor)`，地形 NaN 时取 floor（机器码 `cmpunordss/maxss`）。
fn ground_y(terrain_y: f32, floor: f32) -> f32 {
    if terrain_y.is_nan() || floor > terrain_y { floor } else { terrain_y }
}

/// 预制形状墙底轮廓（`Map::fold` 0x141475090）：对每个 `(point, u)`，取 `u` 所在线段的 3D
/// 切线，归一化后取 xz 左法线 `(-tz, tx)`，外扩 [`BOTTOM_OFFSET`]（y 不变）。
pub fn offset_wall_outline(outline: &[[f32; 3]], outline_u: &[f32], length: f32) -> Vec<Vec3> {
    let points: Vec<Vec3> = outline.iter().copied().map(Vec3::from).collect();
    let curve = Curve3XZ::from_cached_parts(points.clone(), outline_u.to_vec(), length);
    points
        .iter()
        .zip(outline_u)
        .map(|(p, &u)| {
            let segment = curve.get_coord_at_u(u).segment;
            let (a, b) = (points[segment], points[segment + 1]);
            let (dx, dy, dz) = (b.x - a.x, b.y - a.y, b.z - a.z);
            let inv = 1.0 / (dx * dx + dy * dy + dz * dz).sqrt();
            let (tz, tx) = (inv * dz, inv * dx);
            let inv2 = 1.0 / (tx * tx + tz * tz).sqrt();
            let normal = Vec2::new(-inv2 * tz, inv2 * tx);
            Vec3::new(
                p.x + normal.x * BOTTOM_OFFSET,
                p.y + 0.0 * BOTTOM_OFFSET,
                p.z + normal.y * BOTTOM_OFFSET,
            )
        })
        .collect()
}

/// `FUN_140960650`：圆的 3D 折线（`as_curve3(0.2, 0.0)`）投影到屏幕 x，返回最左/最右点
/// `[min, max]`。`first` 是墙轮廓首点（两端初值），`fmax` 初值 0、`fmin` 初值 1e5。
pub fn circle_screen_extremes(
    first: Vec3,
    circle: Circle2d,
    projection: &Mat4,
    view: &Mat4,
    width: f32,
) -> [Vec3; 2] {
    let (mut min_pt, mut max_pt) = (first, first);
    let (mut max_x, mut min_x) = (0.0_f32, SCREEN_X_MIN_INIT);
    for p in circle.as_curve3(CIRCLE_MAX_STEP, 0.0).points {
        let vx = p.z * view.z_axis.x + p.y * view.y_axis.x + p.x * view.x_axis.x + view.w_axis.x;
        let vy = p.z * view.z_axis.y + p.y * view.y_axis.y + p.x * view.x_axis.y + view.w_axis.y;
        let vz = p.z * view.z_axis.z + p.y * view.y_axis.z + p.x * view.x_axis.z + view.w_axis.z;
        let clip_x = vz * projection.z_axis.x
            + vy * projection.y_axis.x
            + vx * projection.x_axis.x
            + projection.w_axis.x;
        let clip_w = vz * projection.z_axis.w
            + vy * projection.y_axis.w
            + vx * projection.x_axis.w
            + projection.w_axis.w;
        let screen_x = ((clip_x / clip_w) * 0.5 + 0.5) * width;
        if max_x < screen_x {
            max_pt = p;
            max_x = screen_x;
        }
        if screen_x < min_x {
            min_pt = p;
            min_x = screen_x;
        }
    }
    [min_pt, max_pt]
}

/// 屋面剖面折线（`Map::fold` 0x141476ba0）：`t = i * 0.25`（5 点），在剖面曲线上取
/// `(u, v)`；水平在 `bottom → top` 间按 `u` 混合，高度 = `floor + v * height`。
pub fn roof_profile_polyline(
    profile: &Curve2,
    floor_y: f32,
    height: f32,
    bottom: Vec2,
    top: Vec2,
) -> Vec<Vec3> {
    (0..ROOF_PROFILE_SAMPLES)
        .map(|i| {
            let t = i as f32 * ROOF_PROFILE_STEP;
            assert!(t <= 1.0, "{t}");
            let coord = profile.get_coord_at_u(t);
            let (a, b) = (profile.points[coord.segment], profile.points[coord.segment + 1]);
            let u = b.x * coord.t + a.x * (1.0 - coord.t);
            let position = Vec2::new(
                u * top.x + (1.0 - u) * bottom.x,
                u * top.y + (1.0 - u) * bottom.y,
            );
            let v = (1.0 - coord.t) * a.y + b.y * coord.t;
            x_h_y(position, floor_y + v * height)
        })
        .collect()
}

/// `FUN_14095fca0`：楼梯请求的轮廓线（1 或 2 条 `DebugLine`）。
///
/// * `kind == 2`：只有中心线 `x_h_y(position, height)`；
/// * 否则：沿左法线（`normalize_or_zero` 后 `(-ty, tx)`）偏 `(width + 0.3) * 0.5` 正向走一遍、
///   再偏 `-0.5` 倍反向走回，最后闭合到首点；
/// * 颜色 `kind == 1` 为红色其余白色，线宽 = strength，`ui_pass = dashed = true`；
/// * `kind == 1` 追加第二条红色中心线（只取 positions/heights 的最短长度）。
pub fn stair_outline_lines(
    kind: u8,
    strength: f32,
    request: &PlayermadeStairsAssemblyParams,
) -> Vec<DebugLine> {
    let count = request
        .heights
        .len()
        .min(request.curve.points.len())
        .min(request.widths.len())
        .min(request.tangents.len());
    let centre = |i: usize| x_h_y(request.curve.points[i], request.heights[i]);
    let mut points = Vec::with_capacity(request.curve.points.len() * 2);
    if kind == 2 {
        points.extend((0..count).map(centre));
    } else {
        let side = |i: usize, factor: f32| {
            let width = request.widths[i] + STAIR_WIDTH_PAD;
            let base = centre(i);
            let tangent = request.tangents[i];
            let inv = 1.0 / (tangent.x * tangent.x + tangent.y * tangent.y).sqrt();
            let normal = if inv.is_finite() && inv > 0.0 {
                Vec2::new(-tangent.y * inv, tangent.x * inv)
            } else {
                Vec2::ZERO
            };
            Vec3::new(
                normal.x * width * factor + base.x,
                0.0 * width * factor + base.y,
                width * normal.y * factor + base.z,
            )
        };
        points.extend((0..count).map(|i| side(i, 0.5)));
        points.extend((0..count).rev().map(|i| side(i, -0.5)));
        let first = points[0];
        points.push(first);
    }
    let color = if kind == 1 { DebugColor::Red } else { DebugColor::White };
    let line = |points: Vec<Vec3>, color: DebugColor| DebugLine {
        points,
        colors: None,
        color,
        line_width: strength,
        ui_pass: true,
        dashed: true,
    };
    let mut lines = vec![line(points, color)];
    if kind == 1 {
        let n = request.curve.points.len().min(request.heights.len());
        lines.push(line((0..n).map(centre).collect(), DebugColor::Red));
    }
    lines
}

/// 楼梯事件分派（原版事件循环 0x14095a387）。
fn outline_stairs(
    commands: &mut EventReader<OutlineStairsCmd>,
    state: &StairsState,
    requests: &PlayermadeStairsAssemblyRequests,
    stream: &mut UiSignifierStream,
) {
    for cmd in commands.read() {
        if !(cmd.strength > 0.0) {
            continue;
        }
        let draw = |request: &PlayermadeStairsAssemblyParams, stream: &mut UiSignifierStream| {
            for line in stair_outline_lines(cmd.kind, cmd.strength, request) {
                line.show(stream);
            }
        };
        if !cmd.include_connected {
            if let Some(request) = requests.requests.iter().find(|r| r.segment == cmd.segment) {
                draw(request, stream);
            }
        } else {
            let nodes = state.get_all_connected_nodes(cmd.segment.nodes().0.0);
            for request in &requests.requests {
                let (a, b) = request.segment.nodes();
                if nodes.contains(&a.0) || nodes.contains(&b.0) {
                    draw(request, stream);
                }
            }
        }
    }
}

// ----------------------------------------------------------------------------- 墙/屋顶
/// 一次 `OutlineWallCmd` 的绘制上下文（原版把这些放在栈帧里）。
struct WallOutline<'a> {
    wall: WallId,
    strength: f32,
    frame: u32,
    state: &'a PublicWallState,
    roof: Option<&'a Roof>,
    terrain: &'a TerrainHeightsData<TerrainHeightSsbo>,
    camera: &'a MainCamera,
    window: &'a WindowSize,
    stream: &'a mut UiSignifierStream,
    /// `FUN_140960650` 的单元缓存（`auStack_4b4`，tag 1 = 已算）：墙与屋顶两处共用。
    extremes: Option<[Vec3; 2]>,
}

impl WallOutline<'_> {
    fn push(&mut self, base: &str, points: Vec<Vec3>, length: f32) {
        let name = suffixed_name(base, self.wall, self.frame);
        push_line(self.stream, &name, points, self.strength, length);
    }

    /// 两段式名字中不带 `{}_{}_{}` 后缀、直接 `"{prefix}{wall}_{frame}"` 的记录。
    fn push_direct(&mut self, prefix: &str, points: Vec<Vec3>, length: f32) {
        let name = format!("{prefix}{}_{}", self.wall, self.frame);
        push_line(self.stream, &name, points, self.strength, length);
    }

    fn expanded_shape(&self) -> ShapeParameters {
        ShapeParameters::from_snapshot(self.state.shape()).expand(WALL_EXPAND)
    }

    fn extremes(&mut self, circle: Circle2d) -> [Vec3; 2] {
        if let Some(cached) = self.extremes {
            return cached;
        }
        let first = Vec3::from(self.state.geometry.outline[0]);
        let projection = self.camera.read_matrix(0);
        let view = self.camera.read_matrix(0x80);
        let result = circle_screen_extremes(first, circle, &projection, &view, self.window.width as f32);
        self.extremes = Some(result);
        result
    }

    fn draw_wall(&mut self) {
        let state = self.state;
        let geometry = &state.geometry;
        let premade = state.shape().tag() < 2;
        let bottom = if premade {
            offset_wall_outline(&geometry.outline, &geometry.outline_u, geometry.mapped_curve_length)
        } else {
            geometry.outline.iter().copied().map(Vec3::from).collect()
        };
        let length = geometry.mapped_curve_length;
        self.push_direct("outline_wall_bottom_", bottom, length);
        if premade {
            self.draw_premade_wall_top();
        } else {
            self.draw_freeform_wall_top();
        }
    }

    fn draw_premade_wall_top(&mut self) {
        let shape = self.expanded_shape();
        let flat_rim = self
            .roof
            .is_some_and(|roof| roof.observed()[0x3d] != 0 && roof.height_01() <= 0.1);
        let max_y = self.state.max_y();
        let top_y = (if flat_rim { FLAT_RIM_TOP_ADJUST } else { 0.0 }) + max_y;
        let shape = shape.get_shape().expect("premade shape");
        let (curve, perimeter) = match shape {
            Shape::Circle(c) => (c.as_curve2(CIRCLE_MAX_STEP), c.perimeter()),
            Shape::Rectangle(r) => (r.as_curve2(), r.perimeter()),
        };
        let top = curve.points.iter().map(|&p| x_h_y(p, top_y)).collect();
        self.push_direct("outline_wall_top_", top, perimeter);
        match shape {
            Shape::Circle(c) => self.draw_circle_side_lines(c, top_y),
            Shape::Rectangle(r) => self.draw_rect_corner_lines(r, top_y),
        }
    }

    fn draw_circle_side_lines(&mut self, circle: Circle2d, top_y: f32) {
        let floor = terrain_floor_y(self.state);
        for pt in self.extremes(circle) {
            let xz = Vec2::new(pt.x, pt.z);
            let top = x_h_y(xz, top_y);
            let terrain_y = self.terrain.sample_at_world_xz(xz.x, xz.y);
            let ground = x_h_y(xz, ground_y(terrain_y, floor));
            let base = format!("outline_circle_side_{ground}");
            self.push(&base, vec![ground, top], top_y - pt.y);
        }
    }

    fn draw_rect_corner_lines(&mut self, rect: Rectangle2d, top_y: f32) {
        let state = self.state;
        let geometry = &state.geometry;
        let outline = &geometry.outline;
        let curve = Curve3XZ::from_cached_parts(
            outline.iter().copied().map(Vec3::from).collect(),
            geometry.outline_u.clone(),
            geometry.mapped_curve_length,
        );
        for j in 0..4usize {
            let corner = rect.get_point(j);
            let coord = curve.get_approx_coord_from_pos(corner);
            let (a, b) = (outline[coord.segment][1], outline[coord.segment + 1][1]);
            let ground = (1.0 - coord.t) * a + b * coord.t;
            let base = format!("outline_rect_corner_{j}");
            let line = vec![x_h_y(corner, ground), x_h_y(corner, top_y)];
            self.push(&base, line, top_y - ground);
        }
    }

    fn draw_freeform_wall_top(&mut self) {
        let state = self.state;
        let positions: &[[f32; 3]] = state
            .prepared_path
            .as_ref()
            .map_or(&[], |path| path.positions.as_slice());
        let heights = state.arch_segment_heights().unwrap_or(&[]);
        let top = positions
            .iter()
            .zip(heights)
            .map(|(p, &h)| x_h_y(Vec2::new(p[0], p[2]), h))
            .collect();
        let length = state.geometry.mapped_curve_length;
        self.push_direct("outline_wall_top_", top, length);
        let unwrap = "called `Option::unwrap()` on a `None` value";
        let first = (
            positions.first().expect(unwrap),
            heights.first().expect(unwrap),
        );
        let last = (positions.last().expect(unwrap), heights.last().expect(unwrap));
        for (side, (p, &h)) in [("outline_side_left", first), ("outline_side_right", last)] {
            let (a, b) = (Vec3::from(*p), x_h_y(Vec2::new(p[0], p[2]), h));
            self.push(side, vec![a, b], h - p[1]);
        }
    }

    /// 屋顶段。`wall_drawn == false`（`kind == 0`）时先补画墙顶线。
    fn draw_roof(&mut self, wall_drawn: bool) {
        let Some(roof) = self.roof else { return };
        if self.state.shape().tag() >= 2 {
            return;
        }
        let shape = self.expanded_shape();
        if !wall_drawn {
            self.draw_roof_only_wall_top(&shape);
        }
        match shape.get_shape().expect("premade shape") {
            Shape::Circle(c) => self.draw_roof_circle(c, roof),
            Shape::Rectangle(r) => self.draw_roof_rect(r, roof),
        }
    }

    /// `kind == 0` 补画：`ShapeParameters::as_curve2` 在 `max_y` 高度，长度 = 周长。
    fn draw_roof_only_wall_top(&mut self, shape: &ShapeParameters) {
        let max_y = self.state.max_y();
        let top = shape.as_curve2().points.iter().map(|&p| x_h_y(p, max_y)).collect();
        self.push_direct("outline_wall_top_", top, shape.perimeter());
    }

    fn draw_roof_circle(&mut self, circle: Circle2d, roof: &Roof) {
        let extremes = self.extremes(circle);
        let (floor, pivot) = (roof.floor_y(), roof.pivot_3d());
        let tip = pivot + Vec3::Y * roof.height();
        for pt in extremes {
            let xz = Vec2::new(pt.x, pt.z);
            let base_point = x_h_y(xz, floor);
            let profile = roof_profile(roof);
            let line = roof_profile_polyline(
                &profile,
                floor,
                roof.height(),
                xz,
                Vec2::new(pivot.x, pivot.z),
            );
            let (dx, dy, dz) = (base_point.x - tip.x, base_point.y - tip.y, base_point.z - tip.z);
            let length = (dz * dz + dy * dy + dx * dx).sqrt();
            let base = format!("outline_circle_roof_{pt}");
            self.push(&base, line, length);
        }
    }

    fn draw_roof_rect(&mut self, rect: Rectangle2d, roof: &Roof) {
        let RoofBoundsDims::Rectangle { bottom, top } = get_roof_expanded_top_and_bottom(roof)
        else {
            unreachable!()
        };
        let frame_of = |size: [f32; 2]| Rectangle2d {
            axis: rect.axis,
            center: rect.center,
            size: Vec2::from(size),
        };
        let (bottom, top) = (frame_of(bottom), frame_of(top));
        let floor = roof.floor_y();
        let height = roof.height();
        for j in 0..4usize {
            let (pb, pt) = (bottom.get_point(j), top.get_point(j));
            let (a1, a2) = (x_h_y(pb, floor), x_h_y(pt, floor + height));
            let (dx, dy, dz) = (a1.x - a2.x, a1.y - a2.y, a1.z - a2.z);
            let length = (dz * dz + dy * dy + dx * dx).sqrt();
            let line = roof_profile_polyline(&roof_profile(roof), floor, height, pb, pt);
            self.push(&format!("outline_roof_side_{j}"), line, length);
        }
        let p0 = top.get_point(0);
        let p1 = top.get_point(if roof.observed()[0x3c] == 0 { 3 } else { 1 });
        let ridge_y = floor + roof.height();
        let length = ((p0.y - p1.y) * (p0.y - p1.y) + (p0.x - p1.x) * (p0.x - p1.x)).sqrt();
        let line = vec![x_h_y(p0, ridge_y), x_h_y(p1, ridge_y)];
        self.push_direct("outline_roof_ridge_", line, length);
    }
}

/// `Roof::profile_curve_normalized`（0x1408e3420）：20 个归一化点组成的 `Curve2`。
fn roof_profile(roof: &Roof) -> Curve2 {
    let points = roof.profile_curve_normalized().iter().map(|&[u, v]| Vec2::new(u, v)).collect();
    Curve2::crashy_new_from_points(points, false)
}

/// `structure_outline`（B 0x140959fc0）。宿主注册：`add_wall_systems` set 28 chain 的末位。
#[allow(clippy::too_many_arguments)]
pub fn structure_outline(
    mut wall_commands: EventReader<OutlineWallCmd>,
    mut stairs_commands: EventReader<OutlineStairsCmd>,
    public_walls: Res<PublicWalls>,
    roofs: Query<&Roof>,
    mut stream: ResMut<UiSignifierStream>,
    time: Res<GameTime>,
    camera: Res<MainCamera>,
    window: Res<WindowSize>,
    terrain: Res<TerrainHeightsData<TerrainHeightSsbo>>,
    stairs: Res<StairsState>,
    stair_requests: Res<PlayermadeStairsAssemblyRequests>,
    menu: Res<EscMenuState>,
) {
    if !outline_enabled(&menu) {
        return;
    }
    outline_stairs(&mut stairs_commands, &stairs, &stair_requests, &mut stream);
    let Some(cmd) = wall_commands.read().last().copied() else {
        return;
    };
    let Some(state) = public_walls.get(cmd.wall) else {
        return;
    };
    let roof = public_walls.roof_component(cmd.wall, |entity| roofs.get(entity).ok());
    let mut outline = WallOutline {
        wall: cmd.wall,
        strength: cmd.strength,
        frame: time.frame_count,
        state,
        roof,
        terrain: &terrain,
        camera: &camera,
        window: &window,
        stream: &mut stream,
        extremes: None,
    };
    if cmd.draws_wall() {
        outline.draw_wall();
    }
    if cmd.draws_roof() {
        outline.draw_roof(cmd.draws_wall());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::roofs::Roof;
    use crate::resources::stairs::stairs_state::StairsState;
    use crate::resources::walls::decorator::DecoratorId;
    use crate::resources::walls::publication_fields::WallShapeSnapshot;
    use crate::resources::walls::publication_source::TerrainHeightGrid512;
    use crate::resources::walls::{InnerWalls, WallState};
    use crate::systems::ui::signifiers::signifier_create::UiSignifierEntry;
    use crate::systems::wall::wall_state::path_preparation::PathRasters512;
    use crate::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash;
    use crate::resources::walls::public_walls::WallPublicationDirty;
    use bevy_ecs::event::Events;

    fn seg(a: u32, b: u32) -> StairSegment {
        StairSegment::new(DecoratorId(a), DecoratorId(b))
    }

    fn request(segment: StairSegment) -> PlayermadeStairsAssemblyParams {
        PlayermadeStairsAssemblyParams {
            curve: utils::curve::Curve2::try_new_from_points(
                vec![Vec2::new(0.0, 0.0), Vec2::new(2.0, 0.0), Vec2::new(4.0, 0.0)], false,
            ).unwrap(),
            heights: vec![1.0, 2.0, 3.0],
            widths: vec![1.0, 1.0, 1.0],
            tangents: vec![Vec2::X; 3],
            auxiliary_samples: Vec::new(),
            interpolated_curve: None,
            nodes: [
                (crate::resources::stairs::stairs_assembly_params::StairNodeAnchor::Terrain { height: 1.0 }, 1),
                (crate::resources::stairs::stairs_assembly_params::StairNodeAnchor::Terrain { height: 3.0 }, 1),
            ],
            terrain_contacts: bitvec::bitvec![usize, bitvec::order::Lsb0; 0; 3],
            segment,
            sub_segments: smallvec::smallvec![
                crate::resources::stairs::stairs_assembly_params::StairSubSegment { kind: 2, flag: 1, count: 2 },
            ],
            dirty: false,
        }
    }

    fn entry_names(world: &mut World) -> Vec<String> {
        world
            .resource_mut::<UiSignifierStream>()
            .take_entries()
            .into_iter()
            .map(|entry| match entry {
                UiSignifierEntry::Factory(f) => f.name().unwrap().to_owned(),
                UiSignifierEntry::Raw(_) => panic!("raw entry"),
            })
            .collect()
    }

    fn base_world() -> World {
        let mut world = World::new();
        world.init_resource::<Events<OutlineWallCmd>>();
        world.init_resource::<Events<OutlineStairsCmd>>();
        world.init_resource::<PublicWalls>();
        world.init_resource::<UiSignifierStream>();
        world.init_resource::<GameTime>();
        world.init_resource::<MainCamera>();
        world.insert_resource(WindowSize::new(100, 100));
        world.init_resource::<TerrainHeightsData<TerrainHeightSsbo>>();
        world.insert_resource(StairsState::new());
        world.init_resource::<PlayermadeStairsAssemblyRequests>();
        world.init_resource::<EscMenuState>();
        world
    }

    fn run(world: &mut World) {
        let mut schedule = Schedule::default();
        schedule.add_systems(structure_outline);
        schedule.run(world);
    }

    fn terrain() -> TerrainHeightGrid512 {
        TerrainHeightGrid512 {
            encoded: vec![0u16; 512 * 512].into(),
            minimum_y: 0.0,
            maximum_y: 0.0,
            sampling_fix: false,
        }
    }

    fn publish(world: &mut World, shape: Option<[u8; 28]>, paths: bool) -> WallId {
        let curve = utils::curve::Curve2::try_new_from_points(vec![Vec2::ZERO, Vec2::new(4.0, 0.0)], false).unwrap();
        let curve = crate::resources::roofs::geometry::surface::Curve2::try_new(
            curve.points.iter().map(|p| [p.x, p.y]).collect(),
            false,
        )
        .unwrap();
        let mut state = WallState::new(curve, 4.0, 0, 0, false);
        if let Some(shape) = shape {
            state.snapshot.source.shape = WallShapeSnapshot::from_observed(shape);
        }
        let mut inner = InnerWalls::new();
        let id = inner.new_wall(state);
        let rasters = PathRasters512::from_encoded(std::array::from_fn(|_| vec![0u8; 512 * 512].into()), false).unwrap();
        world.resource_mut::<PublicWalls>().consume_with_paths(
            &mut inner,
            &terrain(),
            paths.then_some(&rasters),
            &WallPublicationDirty::default(),
            &WallSpatialHash::default(),
        );
        id
    }

    fn rect_shape() -> [u8; 28] {
        ShapeParameters::from_shape(Shape::Rectangle(Rectangle2d {
            axis: Vec2::X,
            center: Vec2::new(2.0, 0.0),
            size: Vec2::new(4.0, 4.0),
        }))
        .to_record()
    }

    fn circle_shape() -> [u8; 28] {
        ShapeParameters::from_shape(Shape::Circle(Circle2d {
            center: Vec2::new(2.0, 0.0),
            radius: 2.0,
            rotation: 0.0,
        }))
        .to_record()
    }

    fn roof(circle: bool) -> Roof {
        let mut bytes = [0u8; 88];
        let mut put = |offset: usize, value: f32| bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
        for (offset, value) in [(0x2c, 0.5), (0x30, 0.5), (0x34, 0.5), (0x38, 0.5), (0x14, 2.0), (0x1c, 4.0), (0x20, 4.0)] {
            put(offset, value);
        }
        bytes[8] = u8::from(!circle);
        Roof::from_observed(bytes)
    }

    #[test]
    fn gate_accepts_only_closed_menu_and_photomode_without_gallery() {
        assert!(outline_enabled(&EscMenuState::Closed));
        assert!(outline_enabled(&EscMenuState::Photomode(false)));
        assert!(!outline_enabled(&EscMenuState::Photomode(true)));
        assert!(!outline_enabled(&EscMenuState::Main));
    }

    #[test]
    fn wall_command_layout_matches_event_instance_offsets() {
        let cmd = OutlineWallCmd { wall: WallId(9), strength: 2.0, kind: 2 };
        assert!(cmd.draws_wall() && cmd.draws_roof());
        assert!(!OutlineWallCmd { kind: 0, ..cmd }.draws_wall());
        assert!(!OutlineWallCmd { kind: 1, ..cmd }.draws_roof());
    }

    #[test]
    fn stair_outline_kind_two_is_the_centre_line_only() {
        let lines = stair_outline_lines(2, 3.0, &request(seg(1, 2)));
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].points, vec![Vec3::new(0.0, 1.0, 0.0), Vec3::new(2.0, 2.0, 0.0), Vec3::new(4.0, 3.0, 0.0)]);
        assert_eq!((lines[0].color, lines[0].line_width, lines[0].ui_pass, lines[0].dashed), (DebugColor::White, 3.0, true, true));
    }

    #[test]
    fn stair_outline_ribbon_walks_left_then_right_and_closes() {
        let lines = stair_outline_lines(0, 1.0, &request(seg(1, 2)));
        assert_eq!(lines.len(), 1);
        let points = &lines[0].points;
        assert_eq!(points.len(), 7);
        // tangent +x => left normal (-ty, tx) = (0, 1) in xz; half width (1 + 0.3) * 0.5.
        assert!((points[0].z - 0.65).abs() < 1e-6 && (points[3].z + 0.65).abs() < 1e-6);
        assert_eq!(points[3].x, 4.0);
        assert_eq!(points[6], points[0]);
        let red = stair_outline_lines(1, 1.0, &request(seg(1, 2)));
        assert_eq!(red.len(), 2);
        assert!(red.iter().all(|l| l.color == DebugColor::Red));
        assert_eq!(red[1].points.len(), 3);
    }

    #[test]
    fn zero_tangent_collapses_the_ribbon_onto_the_centre_line() {
        let mut r = request(seg(1, 2));
        r.tangents = vec![Vec2::ZERO; 3];
        let points = &stair_outline_lines(0, 1.0, &r)[0].points;
        assert_eq!(points[0], Vec3::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn wall_bottom_is_pushed_out_along_the_left_xz_normal() {
        let outline = [[0.0, 1.0, 0.0], [4.0, 1.0, 0.0]];
        let points = offset_wall_outline(&outline, &[0.0, 1.0], 4.0);
        assert!((points[0].z - BOTTOM_OFFSET).abs() < 1e-6 && points[0].x == 0.0 && points[0].y == 1.0);
        assert!((points[1].z - BOTTOM_OFFSET).abs() < 1e-6 && points[1].x == 4.0);
    }

    #[test]
    fn circle_extremes_pick_the_screen_space_left_and_right_samples() {
        let circle = Circle2d { center: Vec2::new(10.0, 0.0), radius: 2.0, rotation: 0.0 };
        let [min, max] = circle_screen_extremes(Vec3::ZERO, circle, &Mat4::IDENTITY, &Mat4::IDENTITY, 100.0);
        assert!((min.x - 8.0).abs() < 0.05 && (max.x - 12.0).abs() < 0.05, "{min} {max}");
        // A degenerate projection (w = 0 -> NaN) never replaces the wall-outline start point.
        let first = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(circle_screen_extremes(first, circle, &Mat4::ZERO, &Mat4::IDENTITY, 100.0), [first, first]);
    }

    #[test]
    fn roof_profile_runs_from_the_eave_point_to_the_tip() {
        let r = roof(true);
        let line = roof_profile_polyline(&roof_profile(&r), 1.0, 5.0, Vec2::new(0.0, 0.0), Vec2::new(4.0, 2.0));
        assert_eq!(line.len(), 5);
        assert_eq!(line[0], Vec3::new(0.0, 1.0, 0.0));
        assert_eq!(line[4], Vec3::new(4.0, 6.0, 2.0));
        assert!(line.windows(2).all(|w| w[1].y >= w[0].y));
    }

    #[test]
    fn closed_gate_does_not_consume_events() {
        let mut world = base_world();
        world.insert_resource(EscMenuState::Main);
        world.resource_mut::<PlayermadeStairsAssemblyRequests>().requests.push(request(seg(1, 2)));
        world.resource_mut::<Events<OutlineStairsCmd>>().send(OutlineStairsCmd {
            segment: seg(1, 2), strength: 2.0, kind: 0, include_connected: false,
        });
        run(&mut world);
        assert!(entry_names(&mut world).is_empty());
        world.insert_resource(EscMenuState::Closed);
        run(&mut world);
        assert_eq!(entry_names(&mut world).len(), 1);
    }

    #[test]
    fn stairs_events_draw_first_match_or_every_connected_segment() {
        let mut world = base_world();
        {
            let mut state = world.resource_mut::<StairsState>();
            state.add_edge(1, 2);
            state.add_edge(2, 3);
        }
        world.resource_mut::<PlayermadeStairsAssemblyRequests>().requests.extend([
            request(seg(1, 2)), request(seg(2, 3)), request(seg(7, 8)),
        ]);
        let events = [
            OutlineStairsCmd { segment: seg(2, 3), strength: 1.0, kind: 0, include_connected: false },
            OutlineStairsCmd { segment: seg(2, 3), strength: 0.0, kind: 0, include_connected: false },
            OutlineStairsCmd { segment: seg(4, 5), strength: 1.0, kind: 0, include_connected: false },
            OutlineStairsCmd { segment: seg(1, 2), strength: 1.0, kind: 2, include_connected: true },
        ];
        for event in events {
            world.resource_mut::<Events<OutlineStairsCmd>>().send(event);
        }
        run(&mut world);
        // 1 (exact match) + 0 (strength 0) + 0 (no request) + 2 (connected 1-2, 2-3; not 7-8).
        assert_eq!(entry_names(&mut world).len(), 3);
    }

    #[test]
    fn rect_wall_draws_bottom_top_and_four_corner_lines() {
        let mut world = base_world();
        let id = publish(&mut world, Some(rect_shape()), false);
        world.resource_mut::<GameTime>().frame_count = 7;
        world.resource_mut::<Events<OutlineWallCmd>>().send(OutlineWallCmd { wall: id, strength: 1.5, kind: 1 });
        run(&mut world);
        let names = entry_names(&mut world);
        assert_eq!(names[0], "outline_wall_bottom_WallId(1)_7");
        assert_eq!(names[1], "outline_wall_top_WallId(1)_7");
        assert_eq!(names.len(), 6);
        assert_eq!(names[2], "outline_rect_corner_0_WallId(1)_7");
        assert_eq!(names[5], "outline_rect_corner_3_WallId(1)_7");
    }

    #[test]
    fn circle_wall_draws_two_side_lines_with_vec3_names() {
        let mut world = base_world();
        let id = publish(&mut world, Some(circle_shape()), false);
        world.resource_mut::<Events<OutlineWallCmd>>().send(OutlineWallCmd { wall: id, strength: 1.0, kind: 1 });
        run(&mut world);
        let names = entry_names(&mut world);
        assert_eq!(names.len(), 4);
        assert!(names[2].starts_with("outline_circle_side_[") && names[2].ends_with("_WallId(1)_0"));
    }

    #[test]
    fn roof_kinds_add_roof_lines_and_kind_zero_adds_the_wall_top() {
        let mut world = base_world();
        let id = publish(&mut world, Some(rect_shape()), false);
        let roof_entity = world.spawn(roof(false)).id();
        world.resource_mut::<PublicWalls>().associate_roof(id, roof_entity);
        world.resource_mut::<Events<OutlineWallCmd>>().send(OutlineWallCmd { wall: id, strength: 1.0, kind: 0 });
        run(&mut world);
        let names = entry_names(&mut world);
        // wall top + 4 roof sides + ridge.
        assert_eq!(names.len(), 6, "{names:?}");
        assert_eq!(names[0], "outline_wall_top_WallId(1)_0");
        assert_eq!(names[1], "outline_roof_side_0_WallId(1)_0");
        assert_eq!(names[5], "outline_roof_ridge_WallId(1)_0");
        // Only the newest wall command is used; kind 2 = wall + roof (no duplicate top).
        for kind in [1, 2] {
            world.resource_mut::<Events<OutlineWallCmd>>().send(OutlineWallCmd { wall: id, strength: 1.0, kind });
        }
        run(&mut world);
        assert_eq!(entry_names(&mut world).len(), 2 + 4 + 4 + 1);
    }

    #[test]
    fn circle_roof_lines_use_the_roof_profile_and_missing_walls_are_ignored() {
        let mut world = base_world();
        let id = publish(&mut world, Some(circle_shape()), false);
        let roof_entity = world.spawn(roof(true)).id();
        world.resource_mut::<PublicWalls>().associate_roof(id, roof_entity);
        world.resource_mut::<Events<OutlineWallCmd>>().send(OutlineWallCmd { wall: id, strength: 1.0, kind: 0 });
        run(&mut world);
        let names = entry_names(&mut world);
        assert_eq!(names.len(), 3, "{names:?}");
        assert!(names[1].starts_with("outline_circle_roof_["));
        world.resource_mut::<Events<OutlineWallCmd>>().send(OutlineWallCmd { wall: WallId(99), strength: 1.0, kind: 2 });
        run(&mut world);
        assert!(entry_names(&mut world).is_empty());
    }

    #[test]
    fn freeform_wall_draws_top_from_prepared_path_and_two_side_lines() {
        let mut world = base_world();
        let id = publish(&mut world, None, true);
        world.resource_mut::<Events<OutlineWallCmd>>().send(OutlineWallCmd { wall: id, strength: 1.0, kind: 1 });
        run(&mut world);
        let names = entry_names(&mut world);
        assert_eq!(names[0], "outline_wall_bottom_WallId(1)_0");
        assert_eq!(names[1], "outline_wall_top_WallId(1)_0");
        assert_eq!(names[2], "outline_side_left_WallId(1)_0");
        assert_eq!(names[3], "outline_side_right_WallId(1)_0");
        assert_eq!(names.len(), 4);
    }
}
