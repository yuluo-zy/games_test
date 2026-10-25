//! B 原 `systems/wall/shapes/shape_placement.rs`：圆形/矩形创建时的权威放置。
//!
//! 原 14097f970 全 PDB 范围 5171B；主控制流以原指令和 17 项跳表核验，
//! 不沿用反编译器越界的 switch。墙段候选、墙命中、屋顶脊、禁用吸附、
//! 冻结事件、提示及进入吸附声音都在这个原 owner 内处理。

use crate::resources::{
    cursor_terrain_raycast::CursorTerrainRaycast,
    mouse_terrain_raycast::CameraCursorRay,
    onboarding::NotifyHintSystemSnapping,
    roofs::roof_shape::Roof,
    walls::{
        WallId,
        public_walls::{PublicWallState, PublicWalls},
    },
};
use crate::systems::{
    audio::PlayWallUiSound,
    backend::input::mod_key::Modifiers,
    collision::world::{RaycastWorld, RaycastWorldResult},
    mode_manager::{AppMode, WallShape},
    ui::signifiers::signifier_create::UiSignifierStream,
    wall::wall_state::wall_spatial_hash::{WallSpatialHash, WallSpatialSegment},
};
use crate::utils::debug_viz::{DebugLine, DebugSphere};
use ::utils::geometry::Rectangle2d;
use ::utils::scalar::{clamp, clamp01, max_f32, smoothstep};
use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};

const SNAP_DISTANCE: f32 = 1.5;
const QUERY_HALF_EXTENT: f32 = 2.25;

/// 放置记录的安全观察层。既有 authoring 消费者使用原 80B 业务联合载荷；
/// 保留其调用面，以有界字节读写承载未命名字段，不模拟指针或 ECS ABI。
/// 未激活载荷和 padding 不参与业务判等；切换变体时只写原本激活的字段。
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct ShapePlacement {
    observed: [u8; 80],
}

impl Default for ShapePlacement {
    fn default() -> Self {
        // 原 add_wall_shape_ux：两个 tag=0、Option<bool>=None、位置/尾载荷=0。
        // 未写入的非激活载荷仅在 Rust 中确定性归零，不能推导出额外业务默认。
        let mut value = Self { observed: [0; 80] };
        value.observed[0x2c] = 2;
        value
    }
}

impl ShapePlacement {
    pub fn from_observed(observed: [u8; 80]) -> Self {
        Self { observed }
    }
    pub fn observed(&self) -> &[u8; 80] {
        &self.observed
    }
    pub fn height_type(&self) -> u32 {
        self.u32_at(0)
    }
    pub fn fixed_height(&self) -> f32 {
        self.f32_at(4)
    }
    pub fn surface(&self) -> u32 {
        self.u32_at(8)
    }
    pub fn position(&self) -> Vec3 {
        self.vec3_at(0x30)
    }
    pub fn normal(&self) -> Vec2 {
        Vec2::new(self.f32_at(0x18), self.f32_at(0x1c))
    }
    pub fn wall_id(&self) -> Option<WallId> {
        (self.surface() != 0).then(|| {
            WallId(u64::from_le_bytes(
                self.observed[0x20..0x28].try_into().unwrap(),
            ))
        })
    }
    pub fn curve_u(&self) -> f32 {
        self.f32_at(0x28)
    }
    /// 墙面是屋顶平顶与否；屋顶面保留原碰撞注册项 byte1，来源不能混同。
    pub fn flat_roof(&self) -> Option<bool> {
        (self.observed[0x2c] != 2).then(|| self.observed[0x2c] != 0)
    }
    /// 原放置面 +0x3c；与 calc_shape_height 之后写入的计算高度分开。
    pub fn elevation(&self) -> f32 {
        self.f32_at(0x3c)
    }
    pub fn height(&self) -> f32 {
        self.elevation()
    }
    pub fn computed_height(&self) -> f32 {
        self.f32_at(0x40)
    }
    pub fn set_computed_height(&mut self, value: f32) {
        self.put_f32(0x40, value);
    }
    pub fn base_position(&self) -> Vec3 {
        self.vec3_at(0x44)
    }
    pub fn new_unsnapped(position: Vec2, height: f32) -> Self {
        let mut value = Self::default();
        value.set_unsnapped(Vec3::new(position.x, height, position.y));
        value
    }
    fn u32_at(&self, at: usize) -> u32 {
        u32::from_le_bytes(self.observed[at..at + 4].try_into().unwrap())
    }
    fn f32_at(&self, at: usize) -> f32 {
        f32::from_bits(self.u32_at(at))
    }
    fn vec3_at(&self, at: usize) -> Vec3 {
        Vec3::new(self.f32_at(at), self.f32_at(at + 4), self.f32_at(at + 8))
    }
    fn put_u32(&mut self, at: usize, value: u32) {
        self.observed[at..at + 4].copy_from_slice(&value.to_le_bytes());
    }
    fn put_f32(&mut self, at: usize, value: f32) {
        self.put_u32(at, value.to_bits());
    }
    fn put_vec3(&mut self, at: usize, value: Vec3) {
        for (i, v) in value.to_array().into_iter().enumerate() {
            self.put_f32(at + i * 4, v);
        }
    }
    fn set_unsnapped(&mut self, position: Vec3) {
        self.put_u32(0, 0);
        self.put_u32(8, 0);
        self.observed[0x2c] = 2;
        self.put_vec3(0x30, position);
        self.observed[0x3c..0x50].fill(0);
    }
    fn set_wall(
        &mut self,
        proposal: SnapProposal,
        flat: bool,
        position: Vec3,
        height: f32,
        base: Vec3,
    ) {
        self.put_u32(0, 0);
        self.put_u32(8, 1);
        self.put_f32(0x18, proposal.normal.x);
        self.put_f32(0x1c, proposal.normal.y);
        self.observed[0x20..0x28].copy_from_slice(&proposal.wall.0.to_le_bytes());
        self.put_f32(0x28, proposal.u);
        self.observed[0x2c] = u8::from(flat);
        self.put_vec3(0x30, position);
        self.put_f32(0x3c, height);
        self.put_u32(0x40, 0);
        self.put_vec3(0x44, base);
    }
    fn set_roof(
        &mut self,
        proposal: SnapProposal,
        registry_flag: u8,
        position: Vec3,
        fixed_height: f32,
        elevation: f32,
    ) {
        self.set_wall(proposal, false, position, elevation, position);
        self.put_u32(0, 1);
        self.put_f32(4, fixed_height);
        self.put_u32(8, 2);
        self.put_vec3(0x0c, position);
        self.observed[0x2c] = registry_flag;
    }
}

/// 原持久资源：0=没有吸附，1/2=吸附；进入 1/2 时只播放一次声音。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SnapMode(pub u8);

/// 原创建交互在拖拽中冻结放置；读取后一次性消费全部未读事件。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct DontRecalculateShapePlacement;

#[derive(Clone, Copy, Debug)]
struct SnapProposal {
    wall: WallId,
    normal: Vec2,
    u: f32,
    point: Vec3,
}

/// 原闭包 140b15f80：资格判据来自 Public +0x280（来源 terrain_floor），
/// 最近点取整个 mapped outline；段仅提供圆形/自由形态的方向。
fn proposal_for_segment(
    state: &PublicWallState,
    segment: &WallSpatialSegment,
    cursor: Vec3,
) -> Option<SnapProposal> {
    if state.authority.elevation() > SNAP_DISTANCE {
        return None;
    }
    let points = state.geometry.outline.as_slice();
    let c = ::utils::curve::approx_curve_coord(points, Vec2::new(cursor.x, cursor.z), |p| {
        Vec2::new(p[0], p[2])
    });
    let a = Vec3::from_array(points[c.segment]);
    let b = Vec3::from_array(points[c.segment + 1]);
    let complement = 1.0 - c.t;
    // 原 x/y 为 t*b+(1-t)*a，z 的加数次序相反，保留原指令舍入顺序。
    let point = Vec3::new(
        c.t * b.x + complement * a.x,
        c.t * b.y + complement * a.y,
        a.z * complement + b.z * c.t,
    );
    let normal = if let Some(rectangle) = state.is_rectangle() {
        rectangle.axis
    } else {
        let d = Vec2::from_array(segment.endpoints[1]) - Vec2::from_array(segment.endpoints[0]);
        d / (d.y * d.y + d.x * d.x).sqrt()
    };
    let dx = point.x - cursor.x;
    let dz = point.z - cursor.z;
    if (dz * dz + dx * dx).sqrt() > SNAP_DISTANCE {
        return None;
    }
    assert!(c.t >= 0.0 && c.t <= 1.0, "原候选坐标必须位于段内");
    let us = &state.geometry.outline_u;
    let u = ::utils::scalar::lerp(us[c.segment], us[c.segment + 1], c.t);
    Some(SnapProposal {
        wall: segment.wall,
        normal,
        u,
        point,
    })
}

fn distance_xz(a: Vec3, b: Vec3) -> f32 {
    let dx = a.x - b.x;
    let dz = a.z - b.z;
    (dz * dz + dx * dx).sqrt()
}

fn nearest_proposal(
    public: &PublicWalls,
    spatial: &WallSpatialHash,
    cursor: Vec3,
) -> Option<SnapProposal> {
    let segments = spatial.query_segments_with_aabb_clamped(
        [cursor.x - QUERY_HALF_EXTENT, cursor.z - QUERY_HALF_EXTENT],
        [cursor.x + QUERY_HALF_EXTENT, cursor.z + QUERY_HALF_EXTENT],
    );
    let mut proposals: Vec<_> = segments
        .into_iter()
        .filter_map(|segment| {
            let state = public.get(segment.wall).expect("原空间段必须有有效公共墙");
            proposal_for_segment(state, segment, cursor)
        })
        .collect();
    // 原稳定 sort_by，等距保留查询顺序，NaN 比较按原 partial_cmp().unwrap() 失败。
    proposals.sort_by(|a, b| {
        distance_xz(a.point, cursor)
            .partial_cmp(&distance_xz(b.point, cursor))
            .unwrap()
    });
    proposals.first().copied()
}

/// 借用现有曲线缓存，复用 Curve::get_coord_at_u 的合同，避免每帧/每候选复制 Vec。
fn outline_coord(state: &PublicWallState, u: f32) -> ::utils::curve::CurveInternalCoord {
    assert!(u.is_finite());
    let us = &state.geometry.outline_u;
    if u >= 1.0 {
        return ::utils::curve::CurveInternalCoord {
            segment: us.len() - 2,
            t: 1.0,
        };
    }
    if u <= 0.0 {
        return ::utils::curve::CurveInternalCoord { segment: 0, t: 0.0 };
    }
    let next = us.partition_point(|&value| value <= u);
    assert!(next > 0 && next < us.len());
    let a = us[next - 1];
    let b = us[next];
    assert!(a != b);
    ::utils::curve::CurveInternalCoord {
        segment: next - 1,
        t: clamp01((u - a) / (b - a)),
    }
}
fn outline_position(state: &PublicWallState, u: f32) -> Vec3 {
    let c = outline_coord(state, u);
    Vec3::from_array(state.geometry.outline[c.segment + 1]) * c.t
        + Vec3::from_array(state.geometry.outline[c.segment]) * (1.0 - c.t)
}
fn wall_coordinates(state: &PublicWallState, p: Vec3) -> (f32, f32) {
    let xz = Vec2::new(p.x, p.z);
    let mapped = clamp01(::utils::curve::approx_curve_u(
        &state.geometry.outline,
        &state.geometry.outline_u,
        xz,
        |p| Vec2::new(p[0], p[2]),
    ));
    let source = state.simplified_curve_2d();
    let authored = clamp01(::utils::curve::approx_curve_u(
        &source.points,
        &source.points_u,
        xz,
        |p| Vec2::from_array(*p),
    ));
    (mapped, authored)
}
fn wall_normal(state: &PublicWallState, mapped: f32, authored: f32) -> Vec2 {
    if let Some(circle) = state.is_circle() {
        return circle.get_normal_at_u(authored);
    }
    if let Some(rectangle) = state.is_rectangle() {
        return rectangle.axis;
    }
    let c = outline_coord(state, mapped);
    let d = Vec3::from_array(state.geometry.outline[c.segment + 1])
        - Vec3::from_array(state.geometry.outline[c.segment]);
    let tangent = ::utils::curve::CurvePoint::normalized(d);
    Vec2::new(tangent.x, tangent.z)
}
fn roof_flat(public: &PublicWalls, roofs: &Query<&Roof>, wall: WallId) -> bool {
    public
        .roof_entity(wall)
        .and_then(|entity| roofs.get(entity).ok())
        .is_some_and(|roof| roof.height_01() <= 0.1)
}

/// 生产系统共用的几何步骤；测试可在真实 World Query 中注入物理查询的已知结果，
/// 比较原指令业务，既不复制候选实现，也不向生产系统加入测试输入资源。
fn resolve_placement(
    mode: AppMode,
    cursor: Vec3,
    public: &PublicWalls,
    spatial: &WallSpatialHash,
    roofs: &Query<&Roof>,
    hit: Option<&RaycastWorldResult>,
    placement: &mut ShapePlacement,
    stream: &mut UiSignifierStream,
) -> Option<u8> {
    let new_snap = match hit.map(|hit| (hit.handle[0] & 0xff) as u8) {
        Some(0 | 9) => {
            let hit = hit.unwrap();
            let wall = WallId(hit.handle[1]);
            let state = public.get(wall).expect("原墙碰撞必须有公共墙");
            let mut point = hit.position;
            // 原 MINSS 的第二操作数为 60；unordered 也选 60。
            point.y = if point.y < 60.0 { point.y } else { 60.0 };
            let (mapped, authored) = wall_coordinates(state, point);
            let normal = wall_normal(state, mapped, authored);
            let mut height = point.y;
            if height < SNAP_DISTANCE {
                point = outline_position(state, mapped);
                height = 0.0;
            }
            let outline = outline_position(state, mapped);
            // 原 xvy 只写 base 载荷；可见放置仍保留碰撞 XZ，除非低墙/向内分支改写。
            let base = Vec3::new(outline.x, point.y, outline.z);
            if height > 0.0 && matches!(mode, AppMode::Wall(WallShape::Rectangle)) {
                let inner = if let Some(mut circle) = state.is_circle() {
                    circle.radius -= 0.925;
                    Some(circle.get_pos_at_u(authored))
                } else if let Some(mut rectangle) = state.is_rectangle() {
                    rectangle.size -= Vec2::splat(1.25);
                    Some(rectangle.get_pos_at_u(authored))
                } else {
                    None
                };
                if let Some(inner) = inner {
                    point.x = inner.x;
                    point.z = inner.y;
                }
            }
            placement.set_wall(
                SnapProposal {
                    wall,
                    normal,
                    u: mapped,
                    point,
                },
                roof_flat(public, roofs, wall),
                point,
                height,
                base,
            );
            2
        }
        Some(2) => {
            let hit = hit.unwrap();
            let wall = WallId(hit.handle[1]);
            let state = public.get(wall).expect("原屋顶碰撞必须有公共墙");
            assert!(state.is_premade_shape(), "原屋顶吸附要求规则墙形状");
            let (mapped, authored) = wall_coordinates(state, hit.position);
            let normal = wall_normal(state, mapped, authored);
            let offset = if matches!(mode, AppMode::Wall(WallShape::Rectangle)) {
                -6.0
            } else {
                -8.0
            };
            let entity = public.roof_entity(wall).expect("原屋顶碰撞必须有屋顶实体");
            let roof = roofs.get(entity).expect("原屋顶实体必须有 Roof");
            let (point, snapped) = match snap_to_roof_ridge(roof, hit.position, 0.5) {
                RoofRidgeSnap::None => (hit.position, false),
                RoofRidgeSnap::Point(point) => (point, true),
                RoofRidgeSnap::Segment { a, b, t } => {
                    DebugLine::new(vec![
                        a + Vec3::new(0.0, 0.2, 0.0),
                        b + Vec3::new(0.0, 0.2, 0.0),
                    ])
                    .line_width(3.0)
                    .dashed()
                    .show(stream);
                    (b * t + a * (1.0 - t), true)
                }
            };
            if snapped {
                DebugSphere::new(point).radius(0.3).show(stream);
                DebugLine::new(vec![
                    point,
                    Vec3::new(point.x, state.authority.elevation(), point.z),
                ])
                .line_width(3.0)
                .dashed()
                .show(stream);
            }
            placement.set_roof(
                SnapProposal {
                    wall,
                    normal,
                    u: mapped,
                    point,
                },
                (hit.handle[0] >> 8) as u8,
                point,
                hit.position.y + offset + 2.5,
                state.authority.elevation(),
            );
            if snapped { 2 } else { 0 }
        }
        Some(3 | 4 | 5 | 15 | 16) => panic!("原形状放置收到非预期碰撞类别"),
        _ => {
            let Some(proposal) = nearest_proposal(public, spatial, cursor) else {
                placement.set_unsnapped(cursor);
                return None;
            };
            placement.set_wall(
                proposal,
                roof_flat(public, roofs, proposal.wall),
                proposal.point,
                0.0,
                proposal.point,
            );
            2
        }
    };
    Some(new_snap)
}

/// 真实 14 参数系统。UiSignifierStream 在当前重构中为显式可变队列，
/// 因而用 ResMut 替代原内部可变 Res；其余 producer/consumer 合同保持不变。
pub fn shape_placement_params(
    mode: Res<AppMode>,
    cursor: Res<CursorTerrainRaycast>,
    public: Res<PublicWalls>,
    spatial: Res<WallSpatialHash>,
    roofs: Query<&Roof>,
    mut sounds: EventWriter<PlayWallUiSound>,
    mut placement: ResMut<ShapePlacement>,
    mut snap_mode: ResMut<SnapMode>,
    modifiers: Res<Modifiers>,
    raycast: Option<Res<RaycastWorld>>,
    camera_ray: Res<CameraCursorRay>,
    mut hints: EventWriter<NotifyHintSystemSnapping>,
    mut frozen: EventReader<DontRecalculateShapePlacement>,
    mut stream: ResMut<UiSignifierStream>,
) {
    if !matches!(
        *mode,
        AppMode::Wall(WallShape::Circle | WallShape::Rectangle)
    ) {
        return;
    }
    if !frozen.is_empty() {
        frozen.clear();
        return;
    }
    let cursor = if cursor.tag & 1 != 0 {
        cursor.pos
    } else {
        Vec3::ZERO
    };
    let hit = raycast
        .as_ref()
        .map(|world| world.raycast(camera_ray.origin, camera_ray.dir, 0xfb));
    let hit = hit.filter(|hit| !hit.is_miss());
    let Some(new_snap) = resolve_placement(
        *mode,
        cursor,
        &public,
        &spatial,
        &roofs,
        hit.as_ref(),
        &mut placement,
        &mut stream,
    ) else {
        if modifiers.flags as u8 != 1 {
            snap_mode.0 = 0;
        }
        return;
    };
    finish_placement(
        cursor,
        modifiers.flags as u8,
        new_snap,
        &mut placement,
        &mut snap_mode,
        &mut hints,
        &mut sounds,
    );
}

fn finish_placement(
    cursor: Vec3,
    modifier: u8,
    new_snap: u8,
    placement: &mut ShapePlacement,
    snap_mode: &mut SnapMode,
    hints: &mut EventWriter<NotifyHintSystemSnapping>,
    sounds: &mut EventWriter<PlayWallUiSound>,
) {
    if modifier != 0 {
        hints.write(NotifyHintSystemSnapping(true));
        placement.set_unsnapped(cursor);
        return;
    }
    hints.write(NotifyHintSystemSnapping(false));
    if (1..=2).contains(&new_snap) && snap_mode.0 == 0 {
        sounds.write(PlayWallUiSound {
            kind: 3,
            selector: 0,
            value: 0.0,
            position: placement.position(),
        });
    }
    snap_mode.0 = new_snap;
}

/// `snap_to_roof_ridge` 返回载荷（原生 sret 判别 +0：0=None / 1=Point /
/// 2=RidgeSegment）。tag 2 载荷 = `{t f32@+4, a Vec3@+8, b Vec3@+0x14}`。
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RoofRidgeSnap {
    None,
    Point(glam::Vec3),
    Segment {
        a: glam::Vec3,
        b: glam::Vec3,
        t: f32,
    },
}

/// `max_num`: NaN operand selects the other value (std `f32::max`).
fn max_num(a: f32, b: f32) -> f32 {
    max_f32(b, a)
}

/// B 0x140980db0（fresh 反编译 + 316 指令 disasm 2026-10-17）：屋顶脊吸附。
///
/// 两条主分支按 `roof[8] & 1`（圆 / 矩形）：
/// - 圆：平顶（height_01 ≤ 0.1）→ 脊点 = `(center.x, position.y, center.z)`；
///   否则按 `((handle − 0.90000004)/7.1)/0.4` 缓动求脊高（低值 8.0）→
///   `(0·h + center.x, h + floor, center.z + 0)`。
/// - 矩形：`RoofRidgeDims::calc` 同构的尺寸对 → `Rectangle2d`（axis=roof+0xc,
///   center=roof+0x14, size=尺寸对，另一轴取 0.01/0.1）→ 角点 0/2 经 `xvy`
///   取两点；y = 平顶 ? position.y : floor + ridge height。
/// - 命中（到点/线段距离 < threshold）：
///   - 圆 → tag 1 Point；
///   - 矩形 → tag 2 Segment，`t` 缺省 0.5，否则按 0.3/len 边界夹取
///     （`max(dims) ≤ 0.5` 时恒 0.5）。
///
/// 常量（fresh read_memory 2026-10-17）：1e-20 / 0.3 / 0.4 / 12.0 / 8.0 /
/// 9.0 / -2.25 / 12.75 / -0.90000004 / 7.1 / 0.1 / 0.01 / 3.0。
/// 高度夹取 `assert!(0 ≤ limit)`（原生 `f32::clamp` 运行时 panic 路径）。
pub fn snap_to_roof_ridge(roof: &Roof, position: glam::Vec3, threshold: f32) -> RoofRidgeSnap {
    let observed = roof.observed();
    let at = |o: usize| f32::from_le_bytes(observed[o..o + 4].try_into().unwrap());
    let height_01 = at(0x30);
    let floor_y = at(0x4c);
    if observed[8] & 1 == 0 {
        // 圆
        let point = if height_01 <= 0.1 {
            // 平顶：xvy(center_xz, position.y)
            glam::Vec3::new(at(0xc), position.y, at(0x10))
        } else {
            let ratio = ((at(0x14) - f32::from_bits(0x3f666667)) / 7.1) / 0.4;
            let t = clamp01(ratio);
            let eased = smoothstep(t);
            // smooth·12.0 + (1 − smooth)·8.0
            let limit = eased * 12.0 + (1.0 - eased) * 8.0;
            // 原生：f32::clamp(min=0,max=limit) 的 `assert!(min <= max)` 运行时 panic。
            assert!(0.0 <= limit, "invalid roof ridge height limit");
            let requested = (1.0 - height_01) * 0.4 + height_01 * 12.0;
            let ridge_height = clamp(requested, 0.0, limit);
            glam::Vec3::new(
                0.0 * ridge_height + at(0xc),
                ridge_height + floor_y,
                at(0x10) + 0.0,
            )
        };
        let delta = position - point;
        return if delta.length() < threshold {
            RoofRidgeSnap::Point(point)
        } else {
            RoofRidgeSnap::None
        };
    }
    // 矩形
    let ridge_length_01 = at(0x34);
    let ridge_dir = observed[0x3c];
    let (dim0, dim1) = (at(0x1c), at(0x20));
    let calculated = (1.0 - ridge_length_01) * 0.1
        + ridge_length_01 * at(if ridge_dir == 0 { 0x1c } else { 0x20 });
    let (f3, f2) = if ridge_dir == 0 {
        (calculated, 0.1)
    } else {
        (0.1, calculated)
    };
    let mut f4 = 0.01;
    if f3 <= f2 {
        f4 = f2;
    }
    // CMPLTSS(f2,f3) 选择：f2 < f3 → f3；否则 0.01。
    let f3 = if f2 < f3 { f3 } else { 0.01 };
    let ridge_height = if 0.1 < height_01 {
        let ratio =
            (max_num(dim0, dim1) + f32::from_bits(0xc0100000)) / f32::from_bits(0x414c0000) / 0.4;
        let t = clamp01(ratio);
        let eased = smoothstep(t);
        let limit = eased * 12.0 + (1.0 - eased) * 9.0;
        assert!(0.0 <= limit, "invalid roof ridge height limit");
        let requested = (1.0 - height_01) * 0.4 + height_01 * 12.0;
        clamp(requested, 0.0, limit)
    } else {
        0.0
    };
    let ridge_rect = Rectangle2d {
        axis: Vec2::new(at(0xc), at(0x10)),
        center: Vec2::new(at(0x14), at(0x18)),
        size: Vec2::new(f3, f4),
    };
    let point_y = if 0.1 < height_01 {
        floor_y + ridge_height
    } else {
        position.y
    };
    let a_xz = ridge_rect.get_point(0);
    let b_xz = ridge_rect.get_point(2);
    let a = glam::Vec3::new(a_xz.x, point_y, a_xz.y);
    let b = glam::Vec3::new(b_xz.x, point_y, b_xz.y);
    let ab = b - a;
    let length_squared = ab.length_squared();
    // MAXSS len², 1e-20（dst>src ? dst : src；NaN 取 1e-20）。
    let denominator = if length_squared > 1e-20 {
        length_squared
    } else {
        1e-20
    };
    let t = clamp01((position - a).dot(ab) / denominator);
    // 原生 lerp = a·(1−t) + b·t（保持加乘次序）。
    let closest = a * (1.0 - t) + b * t;
    if (position - closest).length() < threshold {
        let length = length_squared.sqrt();
        let boundary = 0.3 / length;
        // UCOMISS/JA/JBE 有序语义：boundary>t → 0；t>1−boundary → 1；否则 t。
        let keep_t = !(t > (1.0 - boundary));
        let t_out = if 0.5 > max_num(f3, f4) {
            0.5
        } else if boundary > t {
            0.0
        } else if keep_t {
            t
        } else {
            1.0
        };
        RoofRidgeSnap::Segment { a, b, t: t_out }
    } else {
        RoofRidgeSnap::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_placement_branches_match_original_instruction_fixture() {
        use crate::resources::roofs::geometry::surface::Curve2;
        use crate::resources::walls::publication_fields::WallShapeSnapshot;
        use crate::resources::walls::{
            InnerWalls, WallState, publication_source::TerrainHeightGrid512,
        };
        use bevy_ecs::system::RunSystemOnce;
        use std::sync::Arc;
        let rows: Vec<serde_json::Value> = serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/fixtures/native/toolbar-shape-placement.json"
        )))
        .unwrap();
        for (index, row) in rows.into_iter().enumerate() {
            let input = &row["input"];
            let number =
                |key: &str, default: f32| input[key].as_f64().map(|v| v as f32).unwrap_or(default);
            let integer = |key: &str, default: u64| input[key].as_u64().unwrap_or(default);
            let vector = |key: &str, default: Vec3| {
                if let Some(v) = input[format!("{key}_bits")].as_array() {
                    return Vec3::new(
                        f32::from_bits(v[0].as_u64().unwrap() as u32),
                        f32::from_bits(v[1].as_u64().unwrap() as u32),
                        f32::from_bits(v[2].as_u64().unwrap() as u32),
                    );
                }
                input[key]
                    .as_array()
                    .map(|v| {
                        Vec3::new(
                            v[0].as_f64().unwrap() as f32,
                            v[1].as_f64().unwrap() as f32,
                            v[2].as_f64().unwrap() as f32,
                        )
                    })
                    .unwrap_or(default)
            };
            let shape = integer("shape", 1) as u32;
            let mut raw_shape = [0; 28];
            raw_shape[..4].copy_from_slice(&shape.to_le_bytes());
            let fields: Vec<f32> = if shape == 1 {
                vec![1., 0., 4., 0., 8., 6.]
            } else {
                vec![4., 0., 4., 0.]
            };
            for (i, v) in fields.iter().enumerate() {
                raw_shape[4 + i * 4..8 + i * 4].copy_from_slice(&v.to_le_bytes());
            }
            let mut authority = WallState::new(
                Curve2 {
                    points: vec![[0., 0.], [4., 0.], [8., 0.]],
                    points_u: vec![0., 0.5, 1.],
                    length: 8.,
                },
                3.,
                0,
                0,
                false,
            )
            .snapshot()
            .clone();
            authority.source.shape = WallShapeSnapshot::from_observed(raw_shape);
            authority.source.terrain_floor = number("elevation", 0.);
            authority.source.observed_path_scalar = number("arch", 2.8);
            let mut inner = InnerWalls::new();
            inner.new_wall_with_id(WallId(42), WallState::from_snapshot(authority, 0));
            let terrain = TerrainHeightGrid512 {
                encoded: vec![0; 512 * 512].into(),
                minimum_y: 0.,
                maximum_y: 0.,
                sampling_fix: false,
            };
            let mut public = PublicWalls::default();
            public.consume_with_paths(&mut inner, &terrain, None, &crate::resources::walls::public_walls::WallPublicationDirty::default(), &crate::resources::walls::public_walls::WallSpatialHash::default());
            let state = public.get_mut(WallId(42)).unwrap();
            state.geometry.outline = Arc::new(vec![[0., 0., 0.], [4., 1., 0.], [8., 0., 0.]]);
            state.geometry.outline_u = vec![0., 0.5, 1.];
            let projection = state.building_projection();
            assert_eq!(
                projection.float_0x288().to_bits(),
                number("arch", 2.8).to_bits()
            );
            assert_eq!(
                &projection.observed_bytes()[0x280..0x284],
                &number("elevation", 0.).to_le_bytes()
            );
            let mut spatial = WallSpatialHash::default();
            if input["candidate"].as_bool() == Some(true) {
                let segments: Vec<[[f32; 2]; 2]> = input
                    .get("segments")
                    .map(|v| serde_json::from_value(v.clone()).unwrap())
                    .unwrap_or_else(|| vec![[[0., 0.], [4., 0.]], [[4., 0.], [4., 4.]]]);
                for endpoints in segments {
                    spatial.add_segment(WallSpatialSegment {
                        endpoints,
                        source_u: [0., 1.],
                        relative_heights: [0.; 2],
                        terrain_heights: [0.; 2],
                        wall: WallId(42),
                    });
                }
            }
            let mut world = World::new();
            let mut roof_bytes = [0; 88];
            for (i, v) in row["roof"].as_array().unwrap().iter().enumerate() {
                roof_bytes[i] = v.as_u64().unwrap() as u8;
            }
            if input["roof"].as_bool() == Some(true) {
                let entity = world.spawn(Roof::from_observed(roof_bytes)).id();
                public.associate_roof(WallId(42), entity);
            }
            world.insert_resource(ShapePlacement::from_observed([0xa5; 80]));
            world.insert_resource(SnapMode(integer("snap", 0) as u8));
            world.init_resource::<UiSignifierStream>();
            world.init_resource::<Events<PlayWallUiSound>>();
            world.init_resource::<Events<NotifyHintSystemSnapping>>();
            let cursor = if integer("cursor_tag", 1) & 1 != 0 {
                vector("cursor", Vec3::new(2., 0., 1.))
            } else {
                Vec3::ZERO
            };
            let modifier = integer("modifier", 0) as u8;
            let selected = integer("tool", 1);
            let mode = AppMode::Wall(if selected == 2 {
                WallShape::Rectangle
            } else {
                WallShape::Circle
            });
            let active = integer("mode", 6) == 6
                && (selected == 1 || selected == 2)
                && input["frozen"].as_bool() != Some(true);
            let hit = input["hit_kind"].as_u64().map(|kind| RaycastWorldResult {
                handle: [kind | (integer("registry_flag", 0) << 8), 42, 0],
                position: vector("hit", Vec3::new(2., 3., 0.25)),
                normal: Vec3::Y,
                toi: 1.,
                discriminant: 0,
                extra: [0.; 2],
            });
            let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                world
                    .run_system_once(
                        move |roofs: Query<&Roof>,
                              mut stream: ResMut<UiSignifierStream>,
                              mut placement: ResMut<ShapePlacement>,
                              mut snap: ResMut<SnapMode>,
                              mut hints: EventWriter<NotifyHintSystemSnapping>,
                              mut sounds: EventWriter<PlayWallUiSound>| {
                            if !active {
                                return;
                            }
                            if let Some(new_snap) = resolve_placement(
                                mode,
                                cursor,
                                &public,
                                &spatial,
                                &roofs,
                                hit.as_ref(),
                                &mut placement,
                                &mut stream,
                            ) {
                                finish_placement(
                                    cursor,
                                    modifier,
                                    new_snap,
                                    &mut placement,
                                    &mut snap,
                                    &mut hints,
                                    &mut sounds,
                                );
                            } else if modifier != 1 {
                                snap.0 = 0;
                            }
                        },
                    )
                    .unwrap();
            }));
            if !row["panic"].is_null() {
                assert!(outcome.is_err(), "native panic case {index}: {input}");
                continue;
            }
            assert!(outcome.is_ok(), "candidate panic case {index}: {input}");
            let actual = world.resource::<ShapePlacement>().observed();
            let expected: Vec<u8> = row["placement"]
                .as_array()
                .unwrap()
                .iter()
                .flat_map(|v| (v.as_u64().unwrap() as u32).to_le_bytes())
                .collect();
            let surface = u32::from_le_bytes(actual[8..12].try_into().unwrap());
            let mask: Vec<usize> = if !active {
                (0..80).collect()
            } else {
                (0..4)
                    .chain(8..12)
                    .chain(if surface == 2 { 4..24 } else { 0..0 })
                    .chain(if surface != 0 { 24..45 } else { 44..45 })
                    .chain(48..80)
                    .collect()
            };
            for byte in mask {
                let word = byte / 4 * 4;
                if [4, 12, 16, 20, 24, 28, 40, 48, 52, 56, 60, 64, 68, 72, 76].contains(&word)
                    && f32::from_le_bytes(actual[word..word + 4].try_into().unwrap()).is_nan()
                    && f32::from_le_bytes(expected[word..word + 4].try_into().unwrap()).is_nan()
                {
                    continue;
                }
                assert_eq!(
                    actual[byte], expected[byte],
                    "case {index} byte {byte:#x}: {input}"
                );
            }
            assert_eq!(
                world.resource::<SnapMode>().0,
                row["snap"].as_u64().unwrap() as u8,
                "snap {index}: {input}"
            );
            let hints: Vec<_> = world
                .resource::<Events<NotifyHintSystemSnapping>>()
                .iter_current_update_events()
                .map(|e| u8::from(e.0))
                .collect();
            let expected_hints: Vec<_> = row["hint"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_u64().unwrap() as u8)
                .collect();
            assert_eq!(hints, expected_hints, "hint {index}");
            let sounds: Vec<_> = world
                .resource::<Events<PlayWallUiSound>>()
                .iter_current_update_events()
                .map(|e| {
                    assert_eq!(e.kind, 3);
                    e.position.to_array().map(f32::to_bits)
                })
                .collect();
            let expected_sounds: Vec<[u32; 3]> = row["audio"]
                .as_array()
                .unwrap()
                .iter()
                .map(|r| std::array::from_fn(|i| r[i].as_u64().unwrap() as u32))
                .collect();
            assert_eq!(sounds, expected_sounds, "sound {index}");
            let entries = world.resource_mut::<UiSignifierStream>().take_entries();
            assert_eq!(
                entries.len(),
                row["draws"].as_array().unwrap().len(),
                "draw requests {index}"
            );
            assert!(
                entries.iter().all(|entry| matches!(
                    entry,
                    crate::systems::ui::signifiers::signifier_create::UiSignifierEntry::Factory(_)
                )),
                "原绘制必须创建真实factory"
            );
        }
    }

    #[test]
    fn real_ecs_placement_freeze_toggle_repeat_and_wall_removal_lifecycle() {
        use crate::resources::roofs::geometry::surface::Curve2;
        use crate::resources::walls::{
            InnerWalls, WallState, publication_source::TerrainHeightGrid512,
        };
        let mut inner = InnerWalls::new();
        inner.new_wall_with_id(
            WallId(42),
            WallState::new(
                Curve2::try_new(vec![[0., 0.], [8., 0.]], false).unwrap(),
                3.,
                0,
                0,
                false,
            ),
        );
        let terrain = TerrainHeightGrid512 {
            encoded: vec![0; 512 * 512].into(),
            minimum_y: 0.,
            maximum_y: 0.,
            sampling_fix: false,
        };
        let mut public = PublicWalls::default();
        public.consume_with_paths(&mut inner, &terrain, None, &crate::resources::walls::public_walls::WallPublicationDirty::default(), &crate::resources::walls::public_walls::WallSpatialHash::default());
        let mut spatial = WallSpatialHash::default();
        spatial.add_segment(WallSpatialSegment {
            endpoints: [[0., 0.], [8., 0.]],
            source_u: [0., 1.],
            relative_heights: [0.; 2],
            terrain_heights: [0.; 2],
            wall: WallId(42),
        });
        let mut world = World::new();
        world.insert_resource(AppMode::Wall(WallShape::Circle));
        world.insert_resource(public);
        world.insert_resource(spatial);
        world.insert_resource(CursorTerrainRaycast {
            tag: 1,
            pos: Vec3::new(2., 0., 1.),
            ..Default::default()
        });
        world.init_resource::<ShapePlacement>();
        world.init_resource::<SnapMode>();
        world.init_resource::<Modifiers>();
        world.init_resource::<CameraCursorRay>();
        world.init_resource::<UiSignifierStream>();
        world.init_resource::<Events<PlayWallUiSound>>();
        world.init_resource::<Events<NotifyHintSystemSnapping>>();
        world.init_resource::<Events<DontRecalculateShapePlacement>>();
        let mut schedule = Schedule::default();
        schedule.add_systems(shape_placement_params);
        schedule.run(&mut world);
        assert_eq!(
            world.resource::<ShapePlacement>().wall_id(),
            Some(WallId(42))
        );
        assert_eq!(world.resource::<SnapMode>().0, 2);
        assert_eq!(world.resource::<Events<PlayWallUiSound>>().len(), 1);
        schedule.run(&mut world);
        assert_eq!(
            world.resource::<Events<PlayWallUiSound>>().len(),
            1,
            "持续吸附不重复发进入音效"
        );
        world.resource_mut::<Modifiers>().flags = 1;
        schedule.run(&mut world);
        assert_eq!(world.resource::<ShapePlacement>().surface(), 0);
        assert_eq!(
            world.resource::<SnapMode>().0,
            2,
            "按键关闭保留前一 SnapMode"
        );
        assert!(
            world
                .resource::<Events<NotifyHintSystemSnapping>>()
                .iter_current_update_events()
                .last()
                .unwrap()
                .0
        );
        // 模式门禁先于 freeze reader；退出工具时不能提前消费冻结事件。
        world
            .resource_mut::<Events<DontRecalculateShapePlacement>>()
            .send(DontRecalculateShapePlacement);
        *world.resource_mut::<AppMode>() = AppMode::Path;
        schedule.run(&mut world);
        world.resource_mut::<Modifiers>().flags = 0;
        *world.resource_mut::<AppMode>() = AppMode::Wall(WallShape::Circle);
        schedule.run(&mut world);
        assert_eq!(
            world.resource::<ShapePlacement>().surface(),
            0,
            "第一个回工具帧仍冻结"
        );
        schedule.run(&mut world);
        assert_eq!(
            world.resource::<ShapePlacement>().surface(),
            1,
            "冻结只消费一次"
        );
        inner.remove(WallId(42));
        world
            .resource_mut::<PublicWalls>()
            .consume_with_paths(&mut inner, &terrain, None, &crate::resources::walls::public_walls::WallPublicationDirty::default(), &crate::resources::walls::public_walls::WallSpatialHash::default());
        world
            .resource_mut::<WallSpatialHash>()
            .remove_wall_id(WallId(42));
        schedule.run(&mut world);
        assert_eq!(world.resource::<ShapePlacement>().surface(), 0);
        assert_eq!(world.resource::<ShapePlacement>().wall_id(), None);
        assert_eq!(world.resource::<SnapMode>().0, 0);
        assert_eq!(
            world.resource::<ShapePlacement>().position(),
            Vec3::new(2., 0., 1.)
        );
        world.remove_resource::<ShapePlacement>();
        world.remove_resource::<SnapMode>();
        world.init_resource::<ShapePlacement>();
        world.init_resource::<SnapMode>();
        assert_eq!(
            world.resource::<ShapePlacement>().observed(),
            ShapePlacement::default().observed()
        );
        assert_eq!(world.resource::<SnapMode>().0, 0);
    }

    fn flat_circle_roof(center: [f32; 2], floor: f32) -> Roof {
        let mut bytes = [0u8; 88];
        bytes[8..12].copy_from_slice(&0u32.to_le_bytes()); // circle tag
        bytes[0x0c..0x10].copy_from_slice(&center[0].to_le_bytes());
        bytes[0x10..0x14].copy_from_slice(&center[1].to_le_bytes());
        bytes[0x30..0x34].copy_from_slice(&0.0f32.to_le_bytes()); // height_01 = 0
        bytes[0x4c..0x50].copy_from_slice(&floor.to_le_bytes());
        Roof::from_observed(bytes)
    }

    #[test]
    fn flat_circle_ridge_uses_cursor_height_and_respects_threshold() {
        let roof = flat_circle_roof([2.0, 4.0], 0.7);
        assert_eq!(
            snap_to_roof_ridge(&roof, glam::Vec3::new(2.0, 1.5, 4.0), 0.5),
            RoofRidgeSnap::Point(glam::Vec3::new(2.0, 1.5, 4.0))
        );
        assert_eq!(
            snap_to_roof_ridge(&roof, glam::Vec3::new(10.0, 1.5, 10.0), 0.5),
            RoofRidgeSnap::None
        );
    }

    #[test]
    fn pitched_circle_ridge_is_floor_plus_eased_height() {
        let mut bytes = [0u8; 88];
        bytes[8..12].copy_from_slice(&0u32.to_le_bytes());
        bytes[0x0c..0x10].copy_from_slice(&1.0f32.to_le_bytes());
        bytes[0x10..0x14].copy_from_slice(&2.0f32.to_le_bytes());
        bytes[0x14..0x18].copy_from_slice(&1.0f32.to_le_bytes()); // handle
        bytes[0x30..0x34].copy_from_slice(&1.0f32.to_le_bytes()); // height_01
        bytes[0x4c..0x50].copy_from_slice(&0.25f32.to_le_bytes()); // floor
        let roof = Roof::from_observed(bytes);
        let t = (((1.0f32 - f32::from_bits(0x3f666667)) / 7.1) / 0.4).clamp(0.0, 1.0);
        let eased = (3.0 - (t + t)) * t * t;
        let limit = eased * 12.0 + (1.0 - eased) * 8.0;
        let expected = glam::Vec3::new(1.0, limit + 0.25, 2.0);
        assert_eq!(
            snap_to_roof_ridge(&roof, expected, 0.5),
            RoofRidgeSnap::Point(expected)
        );
    }

    #[test]
    fn rectangle_ridge_returns_corner_segment_with_default_half() {
        let mut bytes = [0u8; 88];
        bytes[8] = 1; // rectangle
        bytes[0x0c..0x10].copy_from_slice(&1.0f32.to_le_bytes()); // axis = X
        bytes[0x10..0x14].copy_from_slice(&0.0f32.to_le_bytes());
        bytes[0x1c..0x20].copy_from_slice(&4.0f32.to_le_bytes());
        bytes[0x20..0x24].copy_from_slice(&2.0f32.to_le_bytes());
        bytes[0x30..0x34].copy_from_slice(&0.0f32.to_le_bytes()); // flat
        bytes[0x34..0x38].copy_from_slice(&0.5f32.to_le_bytes()); // ridge_length_01
        bytes[0x3c] = 0; // ridge dir
        let roof = Roof::from_observed(bytes);
        let position = glam::Vec3::new(0.0, 0.5, 0.0);
        match snap_to_roof_ridge(&roof, position, 0.5) {
            RoofRidgeSnap::Segment { a, b, t } => {
                // dir=0 → size = (calculated=2.05, 其余轴 0.01)；角点 0/2 = ±half。
                assert!((a.x + 1.025).abs() < 1e-6 && (a.z + 0.005).abs() < 1e-6);
                assert!((b.x - 1.025).abs() < 1e-6 && (b.z - 0.005).abs() < 1e-6);
                assert_eq!(a.y, 0.5);
                assert_eq!(b.y, 0.5);
                assert_eq!(t, 0.5); // t=0.5 且不在 0.3/len 边界内 → 保留投影参数
            }
            other => panic!("expected ridge segment, got {other:?}"),
        }
    }
}
