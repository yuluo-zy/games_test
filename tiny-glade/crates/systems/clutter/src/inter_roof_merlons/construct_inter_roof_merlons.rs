//! construct_inter_roof_merlons（B 主基线；0x1422d4870，size 7070）。
//!
//! 2026-10-17 restorer-clutter-remaining：按 fresh 反编译 + 逐指令反汇编重建
//! （`evidence/decomp/0x1422d4870_*.c`、`evidence/disasm5_0.txt`；常量实读
//! `0x142a7e348=0.32`、`0x142a7e354=0.07`、`0x142a80a4c=0.28`、
//! `0x142a89600=0.23`、`0x142c1c7a4=0.115`、`0x14295b630=1000.0`、`0x447a0000=1000.0`、
//! `0x3eeb851f=0.46`、`0x142925870=0.5`、`0x142a89668=0.21`、`0x142a16cb8=0.8`、
//! `0x142a7fca0=0.368`、`0x142a8c5e0=(-100,-100)`）。
//!
//! 控制流（O）：
//! - 宿主几何 Vec stride 0x38：`+0x10` i32 判别（0=Circle、非 0=Rectangle）、
//!   `+0x14/+0x1c` 形状载荷、`+0x24` u64、`+0x34` 宽度基准。
//! - 城垛门控 `local_f1 = !(halftimbered || flat_w_no_rim)`；数量
//!   `ceil(周长/0.46)`；`random_splits_into(count, 0.32/周长, rng)`。
//! - 每段端点由 `get_coord_at_u` 插值；额外段抖动
//!   `((r*0.5-(1-r)*0.5)*0.21*0.8+0.368)*0.5 + (宽基+0.23)`；
//!   普通段偏移 `宽基+0.115`；砖缩放 `((r2*0.5-(1-r2)*0.5)*0.07+0.28)`。
//! - 推 `BrickTransformSsbo`（常量 (-100,-100)、砖种类 4）。
//!
//! U 登记：段端点/法向的 SSE 展开（4.72/0.28/1.5 项）为 I；BrickTransformSsbo 写入为
//! 宿主 SparseInstanceBufferWriter 边界（本 crate 以 [`MerlonBrick`] 承载 CPU 侧）。

use bevy_ecs::prelude::*;
use country_core::systems::wall::construction::bricks::random_splits_into;
use country_core::resources::roofs::roof_shape::Roof;
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::resources::walls::WallId;
use country_core::systems::wall::valid_enclosure::ShapeParameters;
use glam::{Vec2, Vec3};

/// 城垛砖实例（BrickTransformSsbo 的 CPU 侧投影；宿主写入实例缓冲）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MerlonBrick {
    pub position: Vec3,
    pub rotation: f32,
    /// 砖种类（O：local_13c=4）。
    pub kind: u8,
    pub scale: f32,
}

/// 城垛砖缓冲（资源；宿主 SparseInstanceBufferWriter::push 接线）。
#[derive(Resource, Clone, Debug, Default)]
pub struct MerlonBricks {
    pub bricks: Vec<MerlonBrick>,
}

/// 城垛 RNG（fastrand；段抖动）。
#[derive(Resource, Debug)]
pub struct MerlonRng(pub std::sync::Mutex<fastrand::Rng>);

impl Default for MerlonRng {
    fn default() -> Self {
        Self(std::sync::Mutex::new(fastrand::Rng::with_seed(0x606f)))
    }
}

/// 屋顶城垛宿主几何（stride 0x38；墙 + 屋顶形状）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MerlonHostGeometry {
    pub wall: WallId,
    /// 屋顶形状（Circle2d/Rectangle2d 判别在 tag）。
    pub shape: ShapeParameters,
    /// 宽度基准（+0x34 float）。
    pub width_base: f32,
}

/// 段距常量（O，0x3eeb851f = 0.46）。
pub const MERLON_SPACING: f32 = 0.46;

/// construct_inter_roof_merlons（0x1422d4870）。
pub fn construct_inter_roof_merlons(
    walls: Res<PublicWalls>,
    mut rng: ResMut<MerlonRng>,
    mut bricks: ResMut<MerlonBricks>,
    mut geometry: ResMut<MerlonHostGeometryList>,
    mut roof_query: Query<&Roof>,
) {
    let r = &mut rng.0.lock().unwrap();
    for host in geometry.0.iter() {
        // 城垛门控（O）：half-timbered 或 flat_w_no_rim → 取反后门控。
        let halftimbered = walls
            .get(host.wall)
            .map_or(false, |w| wall_is_halftimbered(w));
        let flat_no_rim = walls
            .roof_entity(host.wall)
            .and_then(|e| roof_query.get(e).ok())
            .map_or(false, |roof| roof.is_flat_w_no_rim());
        let gate = !(halftimbered || flat_no_rim);

        // 数量：ceil(周长 / 0.46)。
        let length = host.shape.perimeter();
        let count = (length / MERLON_SPACING).ceil() as usize;
        if count == 0 {
            continue;
        }
        // O：random_splits_into(count, 0.32/周长, rng)。
        let mut splits = Vec::new();
        random_splits_into(
            count,
            0.32 / length.max(f32::MIN_POSITIVE),
            r,
            &mut splits,
        );
        let n = splits.len();
        // 每段（splits[1..]）+ 额外 1 段（idx == n 时走抖动公式）。
        let mut points: Vec<(usize, f32)> = splits
            .iter()
            .enumerate()
            .skip(1)
            .map(|(i, &u)| (i, u))
            .collect();
        points.push((n, 1.0));
        let mut prev_u = 0.0f32;
        for (idx, u) in points {
            let p0 = host.shape.get_pos_at_u(prev_u);
            let p1 = host.shape.get_pos_at_u(u);
            let mid = p0.lerp(p1, 0.5);
            let dir = (p1 - p0).normalize_or_zero();
            let normal = Vec2::new(-dir.y, dir.x);
            let offset = if idx < n {
                // O：普通段偏移 宽基+0.115。
                host.width_base + 0.115
            } else {
                // O：额外段抖动 ((r*0.5-(1-r)*0.5)*0.21*0.8+0.368)*0.5 + (宽基+0.23)。
                let rnd = r.f32();
                ((rnd * 0.5 - (1.0 - rnd) * 0.5) * 0.21 * 0.8 + 0.368) * 0.5
                    + (host.width_base + 0.23)
            };
            let scale = {
                // O：砖缩放 ((r2*0.5-(1-r2)*0.5)*0.07+0.28)。
                let rnd = r.f32();
                (rnd * 0.5 - (1.0 - rnd) * 0.5) * 0.07 + 0.28
            };
            if gate {
                bricks.bricks.push(MerlonBrick {
                    position: Vec3::new(
                        mid.x + normal.x * offset,
                        0.0,
                        mid.y + normal.y * offset,
                    ),
                    rotation: normal.y.atan2(normal.x),
                    kind: 4,
                    scale,
                });
            }
            prev_u = u;
        }
    }
}

/// 半木墙判定（WallStyle::is_halftimbered 0x1408c5670 语义：style byte >= 2；
/// 经 inner WallState 的 WallAuthoritySnapshot.source.style 字节读取）。
fn wall_is_halftimbered(w: &country_core::resources::walls::public_walls::PublicWallState) -> bool {
    w.authority.snapshot().source.style >= 2
}

/// 城垛宿主几何集合（资源；宿主装配）。
#[derive(Resource, Clone, Debug, Default)]
pub struct MerlonHostGeometryList(pub Vec<MerlonHostGeometry>);
