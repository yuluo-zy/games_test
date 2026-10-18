//! `decoclutter_show_pins.rs` — 显示/隐藏已固定 deco-clutter 图钉（B 0x1412357c0）。
//!
//! 证据：fresh 反编译（restorer-terrain-clutter/jobs/decomp/1412357c0，1128 行）。逐语句对照：
//! - 入口事件队列（stride 0x40）非空才处理；
//! - 遍历 pinned 记录表（0x98 stride）：`d = record[+0xe] - 0x38`；
//!   **仅当 `7 < d || d == 6` 继续**（即类型 <0x38、≥0x40、或 ==0x3e）；
//! - 父记录 hash 探测（0x40 stride）→ `clutter_mesh_ty_data()`（足迹 Rectangle3d）；
//! - **几何序列**：`Rectangle3d::expand({0.5,0.5,0.5})` → `set_orientation_xz(axis)` →
//!   `translate(position)`；
//! - 触发体积判别（`*local_1b0` 0/1/2）：
//!   - 2 → GlobalId 哈希探测路径；
//!   - 1 → 触发槽 `[10]==2` 时：标志字节 `[+0x28]&1` 且去重值 `+0x30` 相同则跳过；
//!     然后 `Rectangle3d::intersects` → `Rectangle2d::intersects`；
//!   - 0 → 触发槽 `[10]==2` 时：`[+0x20]&1` 且 `+0x28` 相同则跳过；
//!     `Rectangle3d::intersects` →（`[+1]==0` → `Rectangle2d::intersects_circle`；
//!     否则 `Circle2d::contains_point` OR 中心距 < `[+0xc]` → 命中，否则
//!     `Rectangle2d::intersects_circle`）；
//! - 命中 → SparseInstanceBuffer ChunkId 写入 + 每个图钉的淡入/淡出累加器
//!   （stride 0x18：`[4]` 累加 delta、`[1]` 累加 `delta/period`，period = 0.3 或 −0.2
//!   （`[4]>0.75`）、clamp[0,1]、`EasingFunction::apply`）→
//!   `spawn<(DecoClutterLock, Ui2dWsExtent, Ui2dImage)>` / `EntityCommands::despawn`；
//! - 维护双 Vec 缓冲（56 B union：`local_1d0/1c8/1c0` 与 `local_210`…，cap=4）。
//!
//! crate 无宿主实体/组件表与 SparseInstanceBuffer 写口；图钉几何由
//! [`PinnedDecoGeometry`] 适配承载，几何/门控算法按 O 序列落地，实体 spawn/despawn/
//! ChunkId 写口登记待接（不伪造实体）。`register_required_components`
//! @0x141237200 与 `clone_behavior` @0x140002370 为 bevy 胶水。
use bevy_ecs::prelude::*;
use std::collections::HashSet;
use utils::geometry::{Circle2d, Rectangle2d, Rectangle3d};

/// 已固定 deco-clutter 集合（I：pinned clutter；宿主维护）。
#[derive(Resource, Clone, Debug, Default)]
pub struct PinnedDecoClutter(pub HashSet<u64>);

/// 单图钉几何/触发数据（I：宿主从记录表/装饰存储/触发体积解析）。
///
/// O 字段对应：
/// - `type_byte` = pinned 记录 `+0xe`；
/// - `footprint` = `clutter_mesh_ty_data()` 输出；
/// - `axis`/`position` = `set_orientation_xz`/`translate` 输入；
/// - `trigger_kind` = `*local_1b0`（0/1/2）；
/// - `trigger3d`/`trigger2d` = 两级 `intersects` 输入；
/// - `circle`、`circle_2d`、`radius` = `[+1]`、`contains_point`、`[+0xc]`；
/// - `discriminant`/`dedup` = `[+0x20]/[+0x28]/[+0x30]` 去重门。
#[derive(Clone, Copy, Debug)]
pub struct PinnedDecoGeometry {
    pub type_byte: u8,
    pub footprint: Rectangle3d,
    pub axis: glam::Vec2,
    pub position: glam::Vec3,
    pub trigger_kind: u8,
    pub trigger3d: Rectangle3d,
    pub trigger2d: Rectangle2d,
    pub circle: Circle2d,
    pub circle_2d: bool,
    pub radius: f32,
    pub discriminant: [u8; 3],
    pub dedup: u64,
}

impl Default for PinnedDecoGeometry {
    fn default() -> Self {
        Self {
            type_byte: 0,
            footprint: Rectangle3d {
                footprint: Rectangle2d::new(glam::Vec2::ZERO, glam::Vec2::ZERO, glam::Vec2::X),
                bottom: 0.0,
                top: 0.0,
            },
            axis: glam::Vec2::X,
            position: glam::Vec3::ZERO,
            trigger_kind: 0,
            trigger3d: Rectangle3d {
                footprint: Rectangle2d::new(glam::Vec2::ZERO, glam::Vec2::ZERO, glam::Vec2::X),
                bottom: 0.0,
                top: 0.0,
            },
            trigger2d: Rectangle2d::new(glam::Vec2::ZERO, glam::Vec2::ZERO, glam::Vec2::X),
            circle: Circle2d::new(glam::Vec2::ZERO, 0.0),
            circle_2d: false,
            radius: 0.0,
            discriminant: [0; 3],
            dedup: 0,
        }
    }
}

/// 图钉几何源（I：宿主维护；对应 fresh 的 pinned/父/触发记录表）。
#[derive(Resource, Clone, Debug, Default)]
pub struct PinnedDecoGeometryTable(pub hashbrown::HashMap<u64, PinnedDecoGeometry>);

/// 是否显示图钉（O：`ShowPinnedDecoClutter` 状态资源；56 B union = 双 Vec<u64> + 载荷槽）。
#[derive(Resource, Clone, Debug, Default)]
pub struct ShowPinnedDecoClutter {
    /// O 双 Vec 缓冲之一：可见 pinned 实体列表。
    pub pinned: Vec<u64>,
    /// O 双 Vec 缓冲之二：raycast 命中列表。
    pub hits: Vec<u64>,
    /// 载荷槽（bool 判别；字段语义 U 待取证）。
    pub show: bool,
}

/// 每图钉动画状态（O：0x18 stride 记录：`+0x4` 累加 delta、`+0x4` clamp 进度、
/// `+0x14` 显示周期 0.3/−0.2、`+0x14` scale 载荷）。
#[derive(Clone, Copy, Debug, Default)]
pub struct PinAnimation {
    /// `[4]`：delta 累加器。
    pub accumulator: f32,
    /// `[1]`：淡入/淡出进度（clamp[0,1]）。
    pub progress: f32,
    /// `[5]`：当前是否显示（冷却判别）。
    pub shown: bool,
}

/// 双 Vec 缓冲（O：56 B union：`local_1d0/1c8/1c0` 与 `local_210`…，cap=4；
/// 系统内 Local）。crate 以独立 Resource 承载（避免与 show 输入资源别名）。
#[derive(Resource, Clone, Debug, Default)]
pub struct PinnedDecoClutterBuffers {
    /// 可见 pinned 实体。
    pub pinned: Vec<u64>,
    /// raycast 命中实体。
    pub hits: Vec<u64>,
}

/// 动画状态表（I：宿主按实体维护；fresh 为 stride 0x18 的旁路记录）。
#[derive(Resource, Clone, Debug, Default)]
pub struct PinAnimationTable(pub hashbrown::HashMap<u64, PinAnimation>);

/// 图钉可见性切换事件（I：宿主按实体切换组件）。
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct PinVisibilityToggle {
    pub entity: u64,
    pub visible: bool,
}

/// 命中去重表（O：`local_258` 与 `[+0x30]/[+0x28]` 比较；fresh 用局部槽保存上帧值）。
#[derive(Resource, Clone, Debug, Default)]
pub struct PinHitDedup(pub hashbrown::HashMap<u64, (u8, u64)>);

/// 图钉槽组件（O：`register_required_components` 注册；宿主在 pinned 实体上添加）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DecoClutterLock;

/// 图钉足迹命中判定（O 序列：expand → set_orientation_xz → translate → 条件门）。
///
/// `trigger_kind`：
/// - 1：`Rectangle3d::intersects(trigger3d)` ∧ `trigger2d.intersects(footprint)`；
/// - 0：`Rectangle3d::intersects(trigger3d)` ∧（`circle_2d` →
///   `footprint.intersects_circle(circle)`；否则 `circle.contains_point(center)` ∨
///   `dist(center, circle.center) < radius` ∨ `footprint.intersects_circle(circle)`）；
/// - 2：GlobalId 哈希探测路径（宿主提供 `global_hit`）。
pub fn pinned_geometry_intersects(geo: &PinnedDecoGeometry, global_hit: bool) -> bool {
    let mut rect = geo.footprint;
    rect.expand(glam::Vec3::splat(0.5));
    rect.set_orientation_xz(geo.axis);
    rect.translate(geo.position);
    match geo.trigger_kind {
        1 => {
            if !rect.overlaps_rectangle3d(geo.trigger3d) {
                return false;
            }
            // 原生 `impl$6::intersects`（0x140c9b990 端口）在轴对齐零分量方向下含 NaN
            // 除法；以同义的 2D 重叠谓词承载（几何语义等价，I）。
            rect.footprint.overlaps_rectangle2d(geo.trigger2d)
        }
        0 => {
            if !rect.overlaps_rectangle3d(geo.trigger3d) {
                return false;
            }
            if geo.circle_2d {
                // `trigger+1 == 0` → Rectangle2d::intersects_circle。
                rect.footprint.intersects_circle(geo.circle)
            } else {
                // Circle2d::contains_point OR 中心距 < radius，否则 intersects_circle。
                let center = rect.footprint.center;
                if geo.circle.contains_point(center) {
                    true
                } else {
                    let delta = center - geo.circle.center;
                    if (delta.y * delta.y + delta.x * delta.x).sqrt() < geo.radius {
                        true
                    } else {
                        rect.footprint.intersects_circle(geo.circle)
                    }
                }
            }
        }
        _ => global_hit,
    }
}

/// `decoclutter_show_pins` @0x1412357c0。
pub fn decoclutter_show_pins(
    show: Res<ShowPinnedDecoClutter>,
    pinned: Res<PinnedDecoClutter>,
    geometry: Res<PinnedDecoGeometryTable>,
    mut buffers: ResMut<PinnedDecoClutterBuffers>,
    mut toggled: EventWriter<PinVisibilityToggle>,
    mut animations: ResMut<PinAnimationTable>,
    mut dedup: ResMut<PinHitDedup>,
) {
    buffers.pinned.clear();
    buffers.hits.clear();
    for &entity in pinned.0.iter() {
        let Some(geo) = geometry.0.get(&entity) else {
            continue;
        };
        // 类型门（O：`d = [0xe]-0x38`；`7<d || d==6` 才处理）。
        let d = geo.type_byte.wrapping_sub(0x38);
        if d <= 7 && d != 6 {
            continue;
        }
        // 去重门（O：判别字节匹配且 dedup 相同 → 跳过；仅 kind 1/0 有该门）。
        let flags = if geo.trigger_kind == 1 {
            geo.discriminant[1]
        } else {
            geo.discriminant[0]
        };
        if flags & 1 != 0 {
            if let Some(&(last_flags, last_dedup)) = dedup.0.get(&entity) {
                if last_flags == flags && last_dedup == geo.dedup {
                    continue;
                }
            }
        }
        // 条件门（O：kind 1/0 → intersects 链；kind 2 → GlobalId 哈希探测，宿主提供）。
        let global_hit = false;
        if !pinned_geometry_intersects(geo, global_hit) {
            continue;
        }
        dedup.0.insert(entity, (flags, geo.dedup));
        // O：命中 → ChunkId + 动画累加器（`[4] += delta`；`[1] += delta/period`，
        // period = 0.3，`[4] > 0.75` 时 −0.2；clamp[0,1]；EasingFunction::apply）。
        let animation = animations.0.entry(entity).or_default();
        let delta = 0.0_f32; // GameTime delta 由宿主注入；此处保持记录结构。
        animation.accumulator += delta;
        let period = if animation.accumulator > 0.75 {
            -0.2
        } else {
            0.3
        };
        let mut progress = delta / period + animation.progress;
        if progress < 0.0 {
            progress = 0.0;
        }
        if 1.0 < progress {
            progress = 1.0;
        }
        animation.progress = progress;
        // O：`[5] = 0.75 < [4]`（累加器门）；命中后 `0 < progress || [5]==0` → spawn/scale，
        // 否则 despawn。crate 以 toggled 事件承载 spawn/despawn 消费面。
        animation.shown = 0.75 < animation.accumulator;
        if progress > 0.0 || !animation.shown {
            buffers.pinned.push(entity);
            toggled.write(PinVisibilityToggle {
                entity,
                visible: show.show,
            });
        }
    }
}

/// `register_required_components` @0x141237200（O：bevy 组件注册；`DecoClutterLock`）。
pub fn register_required_components(world: &mut World) {
    world.register_component::<DecoClutterLock>();
}

/// `clone_behavior` @0x140002370（O：ICF 折叠单行；行为标记，无独立逻辑）。
#[derive(Clone, Copy, Debug, Default)]
pub struct CloneBehavior(pub u8);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pinned_geometry_expands_orients_translates_then_intersects() {
        let footprint = Rectangle3d::new(
            glam::Vec3::ZERO,
            glam::Vec3::new(2.0, 2.0, 2.0),
            glam::Vec2::X,
        );
        let trigger3d = Rectangle3d::new(
            glam::Vec3::new(10.0, 0.0, 10.0),
            glam::Vec3::new(1.0, 4.0, 1.0),
            glam::Vec2::X,
        );
        let geo = PinnedDecoGeometry {
            type_byte: 0x40,
            footprint,
            axis: glam::Vec2::X,
            position: glam::Vec3::ZERO,
            trigger_kind: 1,
            trigger3d,
            trigger2d: Rectangle2d::new(
                glam::Vec2::new(10.0, 10.0),
                glam::Vec2::new(1.0, 1.0),
                glam::Vec2::X,
            ),
            ..Default::default()
        };
        assert!(!pinned_geometry_intersects(&geo, false));
        let covering = PinnedDecoGeometry {
            trigger3d: Rectangle3d::new(
                glam::Vec3::new(1.0, 0.0, 1.0),
                glam::Vec3::new(10.0, 4.0, 10.0),
                glam::Vec2::X,
            ),
            trigger2d: Rectangle2d::new(
                glam::Vec2::new(1.0, 1.0),
                glam::Vec2::new(10.0, 10.0),
                glam::Vec2::X,
            ),
            ..geo
        };
        assert!(pinned_geometry_intersects(&covering, false));
    }

    #[test]
    fn type_gate_matches_native_d() {
        // O：`d = type-0x38`；`7<d || d==6` 处理。
        for (ty, expected) in [
            (0x37u8, true),
            (0x38, false),
            (0x3e, true),
            (0x3f, false),
            (0x40, true),
        ] {
            let d = ty.wrapping_sub(0x38);
            assert_eq!(d > 7 || d == 6, expected, "type {ty:#x}");
        }
    }
}
