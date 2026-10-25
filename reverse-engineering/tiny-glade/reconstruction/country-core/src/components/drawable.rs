//! drawable.rs — nani 可绘制组件族（WP6 恢复；systems.rs 消费的集合）。
//!
//! 源 SHA256（B skeleton）`components/drawable.rs`；PDB 命名实体 15 个。
//! 标注：**O**=反编译/PDB 直接支持；**I**=推断；**U**=未恢复（typed gap）。
//!
//! 消费者：`render/nani/systems.rs`（maintain 插 `NaniActiveInstance`、更新
//! `NaniInstanceDesc`；`nani_update_*` 读 `NaniSolidTexture`/`PrevTransform`/`AlbedoMap`）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::component::Component;

use crate::render::nani::gpu_world::{InstanceHandle, InstanceKey};

/// 网格选择器（O：B public-symbols `NaniInstanceMeshDesc::from<MeshAtlasPart>`；I 布局）。
/// 与 `tg-building-runtime` 候选同形；B 行 stride 证据对 16B 布局有歧义（见契约）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NaniInstanceMeshDesc {
    /// 命名网格：`NameHash`（u64；O：maintain `get_by_name<NameHash>`）。
    Named(u64),
    /// 图集 part 索引（u32；O：maintain `*local_190 & 1` 分支的 atlassed 路径）。
    AtlasPart(u32),
}

impl From<crate::render::nani::mesh_atlas::MeshAtlasPart> for NaniInstanceMeshDesc {
    /// O：B public-symbols `NaniInstanceMeshDesc::from<MeshAtlasPart>`（part → AtlasPart）。
    fn from(_part: crate::render::nani::mesh_atlas::MeshAtlasPart) -> Self {
        NaniInstanceMeshDesc::AtlasPart(0)
    }
}

/// 实例描述（O：ECS `&NaniInstanceDesc`；maintain 读 mesh selector + subset_kind）。
/// 布局 I：`{ mesh: NaniInstanceMeshDesc, subset_kind: u8 }`（候选 24B；B 行 stride
/// 0x18 对更紧凑布局有证据——见 contracts/components.md §2）。
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct NaniInstanceDesc {
    pub mesh: NaniInstanceMeshDesc,
    pub subset_kind: u8,
}

impl Default for NaniInstanceDesc {
    fn default() -> Self {
        Self {
            mesh: NaniInstanceMeshDesc::AtlasPart(0),
            subset_kind: 9,
        }
    }
}

impl NaniInstanceDesc {
    /// 命名网格的 `NameHash`（O：maintain `MeshAtlasLibrary::get_by_name<NameHash>`）。
    pub fn mesh_name_hash(&self) -> u64 {
        match self.mesh {
            NaniInstanceMeshDesc::Named(h) => h,
            NaniInstanceMeshDesc::AtlasPart(_) => todo!("named-only; atlassed desc path"),
        }
    }
}

/// 激活实例组件（O：maintain 的 `Insert<NaniActiveInstance>` 命令负载 = `extend_no_data` 回调给出的 12B 记录
/// `{id: u32, index: u32, kind: u8, pad[3]}`，各 `nani_update_*` 以 `&NaniActiveInstance` 取它，再原样传给
/// `update_instance_data<T>`）。与 `GpuWorld` 的 12B `InstanceKey` 是同一个类型，故这里是别名
/// （旧版 16B 的 `{chunk_id, instance_index, kind, sort_key_f16, flags, reserved}` 与机器码不符，已删除；
/// `from_key`/`into_key`/`handle` 随之取消）。
pub use crate::render::nani::gpu_world::InstanceKey as NaniActiveInstance;

/// 纯色 albedo 贴图选择（O：ECS `&NaniSolidTexture`；nani_update_solid_albedo_map 读）。
/// 字段 U（typed gap；纹理句柄/索引）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct NaniSolidTexture(pub crate::Gap);

/// 上一帧变换（O：ECS `&PrevTransform`；velocity_only/wren 读；布局 U）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct PrevTransform(pub crate::Gap);

/// albedo 贴图组件（O：ECS `Option<&AlbedoMap>`；uber/window_glass 读；布局 U）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct AlbedoMap(pub crate::Gap);

/// alpha 贴图组件（O：drawable 命名实体；本组系统未消费）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct AlphaMap(pub crate::Gap);

/// 绘制 bundle（O：B `NaniBundle<SolidVertexColorInstanceData>` insert 命令；
/// `{ descriptor, transform, marker }`，I）。
#[derive(Component, Clone, Copy, Debug)]
pub struct NaniBundle<T> {
    pub descriptor: NaniInstanceDesc,
    pub transform: crate::utils::components::transform::Transform,
    pub marker: crate::render::nani::gpu_world::InstanceDataMarker<T>,
}
