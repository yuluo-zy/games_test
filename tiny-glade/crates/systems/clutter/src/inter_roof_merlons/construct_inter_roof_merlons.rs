//! construct_inter_roof_merlons（B 主基线；0x1422d4870，size 7070，源 L39–215+）。
//!
//! 证据（clutter-extra-analysis.md §host_geometry）：
//! - 输入：宿主几何集合 Vec（stride 0x38）——每元素 `+0x10` i32 判别（0=Circle2d、
//!   非 0=Rectangle2d）、`+0x14/+0x1c` u64×2、`+0x24` u64、`+0x34` float（宽度基准）。
//! - 城垛门控（local_f1）：`PublicWalls::get` → `WallStyle::is_halftimbered()`（半木墙）
//!   或 `PublicWalls::roof_entity()` + `Roof::is_flat_w_no_rim()`（平顶无沿）→ f1=1 才生成；
//!   随后 `f1 ^= 1` 取反。
//! - 数量：`ceilf(曲线长 / 0.46)`（0x3eeb851f）；`长*1000.0`（0x447a0000）截断整型。
//! - `utils::random_splits_into()` 按均匀 u 切段；每段 `fastrand::Rng::f32()` 抖动：
//!   `((r*0.5-(1-r)*0.5)*0.21*0.8+0.368)*0.5 + (宽基+0.23)`。
//! - `Curve::get_coord_at_u` ×2 取段两端坐标 → 线性插值 + 垂直方向归一化（1/SQRT）→
//!   swizzle x0y/xvy。
//! - 实体产出：每段 `SparseInstanceBufferWriter::push<BrickTransformSsbo>()` 一个城垛砖
//!   实例（记录含段 u×曲线长、形状原点、(-100,-100) 常量、砖种类 4、缩放）。
//!   城垛 = 沿屋顶形状曲线等距（0.46）的 BrickTransformSsbo 砖实例。

use bevy_ecs::prelude::*;
use country_core::resources::roofs::roof_shape::Roof;
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::resources::walls::WallId;
use country_core::systems::wall::valid_enclosure::ShapeParameters;
use fastrand::Rng;
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

/// construct_inter_roof_merlons（0x1422d4870）。
#[allow(clippy::too_many_arguments)]
pub fn construct_inter_roof_merlons(
    walls: Res<PublicWalls>,
    mut rng: ResMut<MerlonRng>,
    mut bricks: ResMut<MerlonBricks>,
    mut geometry: ResMut<MerlonHostGeometryList>,
    mut roof_query: Query<&Roof>,
) {
    let r = &mut rng.0.lock().unwrap();
    for host in geometry.0.iter() {
        // 城垛门控（O）：半木墙 或 平顶无沿屋顶。
        let halftimbered = walls
            .get(host.wall)
            .map_or(false, |w| wall_is_halftimbered(w));
        let flat_no_rim = walls
            .roof_entity(host.wall)
            .and_then(|e| roof_query.get(e).ok())
            .map_or(false, |roof| roof.is_flat_w_no_rim());
        let mut gate = halftimbered || flat_no_rim;
        gate = !gate; // 随后取反（O：local_f1 ^= 1）。

        // 数量：ceilf(周长 / 0.46)。
        let length = host.shape.perimeter();
        let count = (length / 0.46).ceil() as usize;
        // 段切分（O：utils::random_splits_into 均匀 u 切段；每段 jitter）。
        let mut bricks_out = Vec::with_capacity(count);
        for i in 0..count {
            let u0 = i as f32 / count.max(1) as f32;
            let u1 = (i + 1) as f32 / count.max(1) as f32;
            let p0 = host.shape.get_pos_at_u(u0);
            let p1 = host.shape.get_pos_at_u(u1);
            let rnd = r.f32();
            let jitter = ((rnd * 0.5 - (1.0 - rnd) * 0.5) * 0.21 * 0.8 + 0.368) * 0.5
                + (host.width_base + 0.23);
            let mid = p0.lerp(p1, 0.5);
            let dir = (p1 - p0).normalize_or_zero();
            let normal = Vec2::new(-dir.y, dir.x);
            let pos = Vec3::new(mid.x + normal.x * jitter, 0.0, mid.y + normal.y * jitter);
            if gate {
                bricks_out.push(MerlonBrick {
                    position: pos,
                    rotation: normal.y.atan2(normal.x),
                    kind: 4,
                    scale: 1.0,
                });
            }
        }
        bricks.bricks.extend(bricks_out);
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
