//! `systems/collision/world.rs` — rapier3d 碰撞世界（B 170 函数，核心查询族 O）。
//!
//! 证据：`collision-eco/ALGORITHMS.md`（机器码）：
//! - `RaycastWorld`：17 子世界槽 ×（脏标志@slot+0x388 → drain 0x200B@slot+0x188 →
//!   publish<DirtyTiles> → `QueryPipeline::update`(slot+0x0, slot+0x30)）；
//! - `RaycastWorld::raycast`（0x1408cc870）：17 槽 mask 遍历，每槽
//!   `QueryPipeline::cast_ray(pipeline, …, solid=1)`；
//! - `RaycastMeshData::new_convex`（0x1408cec10）：parry convex_hull；Some 写 tag=4、
//!   Box 0xd0@+0x20、vtable@+0x28、hash@+0x70；
//! - `SubworldColliderHandle::new`（0x1408cc620）：{u32@+0 index, u32@+4 gen, u8@+8 slot}，
//!   24 B 枚举。
//!
//! rapier 查询在 `native-physics` 特征下实现；非该特征用类型壳（查询不落地）。dirty 瓦片
//! 生命周期/QueryPipeline::update、collider upsert/remove 泛型族、parry 网格构造、以及
//! intersect_meshes/visualize_colliders 登记待续。
#[cfg(feature = "native-physics")]
use rapier3d::prelude::*;

use crate::systems::collision::{EMPTY_COLLIDER_HANDLE_TAG, SubworldIndex, SUBWORLD_SLOT_COUNT};

/// `SubworldColliderHandle`（O：0x1408cc620 写 {u32@+0 index, u32@+4 gen, u8@+8 slot}，
/// 24 B 枚举；空判别 0x11 = 槽数）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SubworldColliderHandle {
    pub index: u32,
    pub generation: u32,
    pub slot: u8,
}

impl Default for SubworldColliderHandle {
    fn default() -> Self {
        Self {
            index: u32::MAX,
            generation: u32::MAX,
            slot: EMPTY_COLLIDER_HANDLE_TAG,
        }
    }
}

impl SubworldColliderHandle {
    /// 0x1408cc620：`SubworldColliderHandle::new`。
    pub fn new(index: u32, generation: u32, slot: u8) -> Self {
        Self {
            index,
            generation,
            slot,
        }
    }
}

/// `RaycastMeshData`（O：0x1408cec10 new_convex —— parry convex_hull；Some 写 tag=4、
/// Box 0xd0@+0x20、vtable@+0x28、hash@+0x70）。头部布局 + 凸包句柄。
#[derive(Clone, Debug)]
pub struct RaycastMeshData {
    /// 判别 tag（O：Some → 4）。
    pub tag: u8,
    /// parry convex hull 句柄（native-physics；0xd0 B 载荷）。
    #[cfg(feature = "native-physics")]
    pub convex: Option<std::sync::Arc<SharedShape>>,
    /// 几何哈希（O：@+0x70）。
    pub hash: u64,
}

impl Default for RaycastMeshData {
    fn default() -> Self {
        Self {
            tag: 0,
            #[cfg(feature = "native-physics")]
            convex: None,
            hash: 0,
        }
    }
}

impl RaycastMeshData {
    /// `RaycastMeshData::new_convex`（0x1408cec10，O 结构）：从点集建 parry convex hull。
    #[cfg(feature = "native-physics")]
    pub fn new_convex(points: &[[f32; 3]]) -> Option<Self> {
        let points_parry: Vec<Point<f32>> = points
            .iter()
            .map(|p| Point::new(p[0], p[1], p[2]))
            .collect();
        let shape = SharedShape::convex_hull(&points_parry)?;
        Some(Self {
            tag: 4,
            convex: Some(std::sync::Arc::new(shape)),
            hash: 0,
        })
    }
}

/// `RaycastWorld`：17 子世界槽的查询管线（O 布局）。
pub struct RaycastWorld {
    /// 每槽 ColliderSet + QueryPipeline（O：slot+0x00 / slot+0x30）。
    #[cfg(feature = "native-physics")]
    slots: Vec<Subworld>,
    /// 脏标志位（O：槽@+0x388）。
    dirty: u64,
}

#[cfg(feature = "native-physics")]
struct Subworld {
    bodies: RigidBodySet,
    colliders: ColliderSet,
    pipeline: QueryPipeline,
    dirty_tiles: Vec<u32>,
}

impl Default for RaycastWorld {
    fn default() -> Self {
        Self::new()
    }
}

impl RaycastWorld {
    /// 新建（O：17 槽 + 脏标志复位）。
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "native-physics")]
            slots: (0..SUBWORLD_SLOT_COUNT).map(|_| Subworld {
                bodies: RigidBodySet::new(),
                colliders: ColliderSet::new(),
                pipeline: QueryPipeline::new(),
                dirty_tiles: Vec::new(),
            }).collect(),
            dirty: 0,
        }
    }

    /// `RaycastWorld::raycast`（0x1408cc870，O 结构）：17 槽 mask 遍历，
    /// 每槽 `QueryPipeline::cast_ray_and_get_normal`，最近命中。
    #[cfg(feature = "native-physics")]
    pub fn raycast(&self, origin: glam::Vec3, dir: glam::Vec3, mask: u64) -> Option<(f32, u32)> {
        let ray = Ray::new(
            Point::new(origin.x, origin.y, origin.z),
            Vector::new(dir.x, dir.y, dir.z),
        );
        let mut best: Option<(f32, u32)> = None;
        for (slot_index, subworld) in self.slots.iter().enumerate() {
            if (mask >> slot_index) & 1 == 0 {
                continue;
            }
            let hit = subworld.pipeline.cast_ray_and_get_normal(
                &subworld.bodies,
                &subworld.colliders,
                &ray,
                f32::MAX,
                true,
                QueryFilter::default(),
            );
            if let Some((_handle, intersection)) = hit {
                let toi = intersection.time_of_impact;
                if best.map_or(true, |(t, _)| toi < t) {
                    best = Some((toi, slot_index as u32));
                }
            }
        }
        best
    }

    /// `RaycastWorld::legacy_raycast`（0x1408cc680）：mask=0 的 rapier raycast
    /// （无命中语义，遗留）。
    #[cfg(feature = "native-physics")]
    pub fn legacy_raycast(&self, _origin: glam::Vec3, _dir: glam::Vec3) -> Option<f32> {
        None
    }

    /// `mesh_intersect_collider_diff_publisher`（0x1408cc670）：返回 &RaycastWorld+0x3cb0
    /// tabloid Publisher 字段（O 布局；发布接线待续）。
    pub fn mesh_intersect_collider_diff_publisher(&self) -> u64 {
        0
    }
}

/// `Raycasting`（O：旧包装，转发 `RaycastWorld`）。
pub struct Raycasting {
    world: RaycastWorld,
}

impl Raycasting {
    /// `Raycasting::raycast`（0x1408cc2e0）：转发 `raycast_w_terrain`。
    #[cfg(feature = "native-physics")]
    pub fn raycast(&self, origin: glam::Vec3, dir: glam::Vec3, mask: u64) -> Option<(f32, u32)> {
        self.world.raycast(origin, dir, mask)
    }

    /// `Raycasting::raycast`（非 native-physics 壳：查询不落地）。
    #[cfg(not(feature = "native-physics"))]
    pub fn raycast(&self, _origin: glam::Vec3, _dir: glam::Vec3, _mask: u64) -> Option<(f32, u32)> {
        None
    }

    /// `Raycasting::intersects_any`（0x1408cc320，O：旧包装，掩码=0/语义待核）。
    /// 掩码=0 时 17 槽全跳过 → 无命中；返回是否命中任意碰撞体。
    #[cfg(feature = "native-physics")]
    pub fn intersects_any(&self, origin: glam::Vec3, dir: glam::Vec3, mask: u64) -> bool {
        self.raycast(origin, dir, mask).is_some()
    }

    /// `Raycasting::intersects_any`（非 native-physics 壳）。
    #[cfg(not(feature = "native-physics"))]
    pub fn intersects_any(&self, _origin: glam::Vec3, _dir: glam::Vec3, _mask: u64) -> bool {
        false
    }

    /// `Raycasting::spherecast`（0x1408cc390）：转发球体扫掠。
    /// PounceLight rapier fork 的 QueryPipeline 球体扫掠 API 未定（cast_sphere 名待核对），
    /// 先登记为不落地桩，待取证后补。
    #[cfg(feature = "native-physics")]
    pub fn spherecast(&self, _origin: glam::Vec3, _dir: glam::Vec3, _radius: f32, _mask: u64) -> Option<(f32, u32)> {
        None
    }
}
