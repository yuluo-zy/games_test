//! `collision.rs` — 碰撞世界的已锚定常量（B `systems::collision::world`）。
//!
//! 证据（`collision-eco/ALGORITHMS.md`）：`SubworldIndex` 共 **17** 个槽（重建里的 16 个常量序错误），其中
//! Terrain 槽 = **5**（掩码 `0x20`、地形命中 `0xb`、映射表三方锚定）；`SubworldIndex::to_bitflag = 1 << 槽号`
//! 在 `iter_shape_overlaps` 族内联使用。`SubworldColliderHandle` 的空值判别是 `0x11`（= 槽数）。
//! 其余 16 个槽的名字与槽号对应关系未锚定（U），这里只提供不依赖名字的 [`SubworldIndex`]。
//!
//! `impl$0::build`（0x140b685e0）为 collision 模块插件 build：登记 `RaycastWorld` 与
//! `MeshIntersectionGraph` 资源（原 `bevy_app` 装配；core 不依赖 bevy_app，按
//! `resources/session.rs` 合同由宿主 App 调用）。

/// rapier 碰撞世界（B `systems::collision/world.rs`，核心查询族）。
pub mod world;

/// 网格对网格相交（B `systems/collision/intersect_meshes.rs`；本轮登记，见 REBUILD-REMAINDER.md）。
pub mod intersect_meshes;

/// collider 调试可视化（B `systems/collision/visualize_colliders.rs`；本轮登记）。
pub mod visualize_colliders;

/// 槽数（O）。
pub const SUBWORLD_SLOT_COUNT: u8 = 17;
/// `SubworldColliderHandle` 的"空"判别（O：0x11）。
pub const EMPTY_COLLIDER_HANDLE_TAG: u8 = 0x11;

/// 子世界槽号。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SubworldIndex(u8);

impl SubworldIndex {
    /// Terrain（O：槽 5，位标志 `0x20`）。
    pub const TERRAIN: Self = Self(5);

    /// 无检查直接包装（O：`SubworldIndex::from_usize_unchecked` 0x14067dd80，机器码
    /// `mov rax,rcx; ret` —— 无边界检查，usize→u8 截断）。
    pub fn from_usize_unchecked(slot: usize) -> Self {
        Self(slot as u8)
    }

    pub fn slot(self) -> u8 {
        self.0
    }

    /// `1 << 槽号`。
    pub fn to_bitflag(self) -> u32 {
        1 << self.0
    }
}

/// Window-side clutter selection recovered from B's six-case consumer table.
/// This semantic enum has no native ABI/layout promise. Only the selected
/// payload participates in equality; native padding is not represented.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindowAutoClutterKind {
    /// Native tag 0; a consumer exists, but its producer is still unresolved.
    WindowFlowerpot,
    /// Native tag 1, +08: index into the actual ClutterMeshes flowerbed Vec.
    WindowFlowerbed { mesh_index: usize },
    /// Native tag 2. The u32 is copied from DecoratorInfo +28; its business
    /// name remains unknown. Preserve every bit rather than infer an id.
    WindowShutter { side: bool, native_u32_04: u32 },
    /// Native tag 3.
    DoorBell,
    /// Native tag 4.
    DoorKrans,
    /// Native tag 5.
    DoorKransAutumn,
}

/// Shared blueprint-to-clutter value (B native stride 80, alignment 16).
/// These safe fields recover the value contract, not the native representation.
/// Resource/drop/clone evidence shows no element-owned Arc, String or Vec.
#[derive(Clone, Copy, Debug)]
pub struct WindowAutoClutterCandidate {
    pub kind: WindowAutoClutterKind,
    pub rotation: glam::Quat,
    pub translation: glam::Vec3,
    pub scale: glam::Vec3,
    /// Native +40: calculate_hash<(DecoratorId, &str)> result.
    pub key: u64,
    /// Native +48: the actual per-wall DecoratorStorage map key.
    pub wall: crate::resources::walls::WallId,
}

impl PartialEq for WindowAutoClutterCandidate {
    /// B equal@1412baca0: ordered float equality, then integer key,
    /// tag-selected payload, and wall. NaNs differ; signed zeros compare equal.
    fn eq(&self, other: &Self) -> bool {
        self.translation == other.translation
            && self.rotation == other.rotation
            && self.scale == other.scale
            && self.key == other.key
            && self.kind == other.kind
            && self.wall == other.wall
    }
}

/// The unique candidate Vec shared by decorator collection and window clutter.
/// B system_clutter::build@140652683..6ae inserts an initially empty Vec.
/// Registration belongs to the actual clutter owner, not CollisionPlugin.
/// Every eligible current blueprint contributes before the visual rebuild gate,
/// including an unchanged Arc; the actual clutter consumer drains this owner.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WindowAutoClutterCandidates(pub Vec<WindowAutoClutterCandidate>);

impl bevy_ecs::resource::Resource for WindowAutoClutterCandidates {}

/// 原 64 字节鸟巢候选：碰撞折线共享切片、全局身份、目的信息和位姿。
/// B 0x1412f9800 复制并增加 Arc 引用，0x1422c9c70 消费对应字段。
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BirdNestCandidateLocation {
    /// +0：共享折线切片（胖指针，长度位于 +8）。
    pub points: std::sync::Arc<[glam::Vec3]>,
    /// +0x10：用于自动装饰身份的 GlobalId 原始值。
    pub global_id: u64,
    /// +0x18：OnBuilding 目标的 WallId 原始值。
    pub destination_wall: u64,
    /// +0x20：原字段未闭合，逐字节保留。
    pub unknown_20: u64,
    /// +0x28：候选位置。
    pub position: glam::Vec3,
    /// +0x34：候选方向。
    pub forward: glam::Vec3,
}

/// `BirdNestCandidateLocations`（O：`Res<BirdNestCandidateLocations>` 资源，`insert_resource`
/// 0x140ee9270 / `init_state` 0x141375610；`spec_extend` 0x1412f9800 把边载荷的
/// `Vec<BirdNestCandidateLocation>` 展平收集进该资源）。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BirdNestCandidateLocations(pub Vec<BirdNestCandidateLocation>);

impl bevy_ecs::resource::Resource for BirdNestCandidateLocations {}

/// `PendingMeshIntersection`（O：`intersect_meshes` 临时相交记录，**0x40 B**）。
///
/// fresh 锚定（`intersect_newly_added_colliders::closure_env$0` 0x1417d1440 的 0x40 B
/// push 序列，2026-10-17）——精确字段：
///
/// | +0x00 | +0x08 | +0x10 | +0x18 | +0x20 | +0x28 | +0x34 |
/// |---|---|---|---|---|---|---|
/// | Vec<Polyline> cap | Vec<Polyline> ptr | Vec<Polyline> len | mesh_ref_a | mesh_ref_b | handle a @+0x28 | handle b @+0x34 |
///
/// - `Vec<Polyline>`（0x18 B，`merge_segments_into_polylines` 输出）；
/// - `mesh_ref_a/b` = 两个 collider 注册表条目 `+0x08` 的 qword（mesh 侧引用，语义未锚定）；
/// - `a` = 新增句柄 {index@+0x28, generation@+0x2c, slot@+0x30}；
/// - `b` = 候选句柄 {index@+0x34, generation@+0x38, slot@+0x3c}。
///
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PendingMeshIntersection {
    /// 折线（O：+0x00，`Vec<Polyline>`）。
    pub polylines: Vec<intersect_meshes::Polyline>,
    /// 左侧 collider 注册表条目的 `+0x08` qword（O：+0x18）。
    pub mesh_ref_a: u64,
    /// 右侧 collider 注册表条目的 `+0x08` qword（O：+0x20）。
    pub mesh_ref_b: u64,
    /// 左侧 collider 句柄（O：+0x28，新增/变更句柄）。
    pub a: world::SubworldColliderHandle,
    /// 右侧 collider 句柄（O：+0x34，候选句柄）。
    pub b: world::SubworldColliderHandle,
}

/// 原 GraphMap 边持有候选和真实砖实例分配；删除边时分配的 Drop 退役。
#[derive(Clone, Debug)]
pub struct MeshIntersectionArtifacts {
    pub bird_nests: Vec<BirdNestCandidateLocation>,
    pub bricks: crate::render::sparse_instance_buffer::SparseInstanceBufferAllocation<crate::geometry::instanced_wall::BrickTransformSsbo>,
}

/// `MeshIntersectionGraph`（O：`petgraph::graphmap::GraphMap<SubworldColliderHandle,
/// MeshIntersectionArtifacts, Undirected, RandomState>`；`impl$0::build` 0x140b685e0
/// `default<...>` 后 `insert_resource`）。
#[derive(Clone, Debug, Default)]
pub struct MeshIntersectionGraph(
    pub petgraph::graphmap::GraphMap<
        world::SubworldColliderHandle,
        MeshIntersectionArtifacts,
        petgraph::Undirected,
        std::collections::hash_map::RandomState,
    >,
);

impl bevy_ecs::resource::Resource for MeshIntersectionGraph {}

impl MeshIntersectionGraph {
    /// 底层 GraphMap 引用。
    pub fn inner(&self) -> &petgraph::graphmap::GraphMap<
        world::SubworldColliderHandle,
        MeshIntersectionArtifacts,
        petgraph::Undirected,
        std::collections::hash_map::RandomState,
    > {
        &self.0
    }

    /// 底层 GraphMap 可变引用（O：`find_mesh_mesh_intersections` 0x140a97b70 的
    /// `GraphMap::remove_node` / `add_node` / `add_edge` 装配面）。
    pub fn inner_mut(&mut self) -> &mut petgraph::graphmap::GraphMap<
        world::SubworldColliderHandle,
        MeshIntersectionArtifacts,
        petgraph::Undirected,
        std::collections::hash_map::RandomState,
    > {
        &mut self.0
    }
}

/// `impl$0::build`（0x140b685e0）：collision 模块插件 build —— 登记 `RaycastWorld`（17 槽 +
/// Publisher）与 `MeshIntersectionGraph` 资源。
/// country-core 不依赖 bevy_app；`build` 登记资源插入合同，由宿主 App 装配。
/// 系统装配（Startup `fn(ResMut<RaycastWorld>, Commands)` 等）依赖的初始化系统属
/// LOGIC.md 未实现登记，此处不注册。
#[derive(Clone, Copy, Debug, Default)]
pub struct CollisionPlugin;

impl CollisionPlugin {
    /// 原 `build`：`insert_resource<RaycastWorld>`（RapierSubWorld×17 + Publisher）+
    /// `insert_resource<MeshIntersectionGraph>`（GraphMap default）。
    pub fn build(&self, world: &mut bevy_ecs::world::World) {
        world.insert_resource(world::RaycastWorld::new());
        world.insert_resource(MeshIntersectionGraph::default());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn window_candidate_equality_uses_ordered_float_values() {
        let candidate = WindowAutoClutterCandidate {
            kind: WindowAutoClutterKind::DoorBell,
            rotation: glam::Quat::IDENTITY,
            translation: glam::Vec3::ZERO,
            scale: glam::Vec3::ONE,
            key: 0x1234_5678_9abc_def0,
            wall: crate::resources::walls::WallId(0xfedc_ba98_7654_3210),
        };
        let mut other = candidate;
        other.translation.x = -0.0;
        assert_eq!(candidate, other);
        other.translation.x = f32::NAN;
        assert_ne!(other, other);
        other = candidate;
        other.rotation = glam::Quat::from_array([0.0, 0.0, 0.0, f32::NAN]);
        assert_ne!(other, other);
        other = candidate;
        other.scale.z = f32::NAN;
        assert_ne!(other, other);
    }

    #[test]
    fn window_candidate_vector_keeps_unknown_shutter_payload_and_full_wall() {
        let candidate = WindowAutoClutterCandidate {
            kind: WindowAutoClutterKind::WindowShutter {
                side: true,
                native_u32_04: 0x8000_0001,
            },
            rotation: glam::Quat::IDENTITY,
            translation: glam::Vec3::new(1.0, 2.0, 3.0),
            scale: glam::Vec3::ONE,
            key: 9,
            wall: crate::resources::walls::WallId(0x1_0000_0001),
        };
        let mut values = WindowAutoClutterCandidates::default();
        assert!(values.0.is_empty());
        values.0.push(candidate);
        assert_eq!(values, values.clone());
        let mut changed = values.clone();
        changed.0[0].kind = WindowAutoClutterKind::WindowShutter {
            side: true,
            native_u32_04: 1,
        };
        assert_ne!(values, changed);
        changed = values.clone();
        changed.0[0].wall = crate::resources::walls::WallId(1);
        assert_ne!(values, changed);
    }

    #[test]
    fn terrain_is_slot_five_with_mask_0x20() {
        assert_eq!(SubworldIndex::TERRAIN.slot(), 5);
        assert_eq!(SubworldIndex::TERRAIN.to_bitflag(), 0x20);
    }

    #[test]
    fn there_are_seventeen_slots_and_the_empty_tag_is_the_slot_count() {
        assert_eq!(EMPTY_COLLIDER_HANDLE_TAG, SUBWORLD_SLOT_COUNT);
        // 无检查构造：16 槽合法，17 槽也截断包装（O：`mov rax,rcx; ret` 无检查）。
        assert_eq!(SubworldIndex::from_usize_unchecked(16).to_bitflag(), 1 << 16);
        assert_eq!(SubworldIndex::from_usize_unchecked(17).slot(), 17);
    }

    #[test]
    fn collision_plugin_build_inserts_both_resources() {
        let mut world = bevy_ecs::world::World::new();
        CollisionPlugin.build(&mut world);
        assert!(world.get_resource::<world::RaycastWorld>().is_some());
        assert!(world.get_resource::<MeshIntersectionGraph>().is_some());
    }

    /// O：`spec_extend<BirdNestCandidateLocation,…>` 0x1412f9800 逐元素拷贝 stride = 0x40，
    /// 且 +0x00 为 `Arc`（refcount ++）。
    #[test]
    fn bird_nest_candidate_location_is_forty_bytes_with_arc_first() {
        assert_eq!(core::mem::size_of::<BirdNestCandidateLocation>(), 0x40);
        assert_eq!(core::mem::offset_of!(BirdNestCandidateLocation, points), 0x00);
        assert_eq!(core::mem::offset_of!(BirdNestCandidateLocation, global_id), 0x10);
        assert_eq!(core::mem::offset_of!(BirdNestCandidateLocation, destination_wall), 0x18);
        assert_eq!(core::mem::offset_of!(BirdNestCandidateLocation, unknown_20), 0x20);
        assert_eq!(core::mem::offset_of!(BirdNestCandidateLocation, position), 0x28);
        assert_eq!(core::mem::offset_of!(BirdNestCandidateLocation, forward), 0x34);
    }
}

pub mod query_policy;
