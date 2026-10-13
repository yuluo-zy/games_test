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

/// 激活实例组件（O：maintain 的 `Insert<NaniActiveInstance>` 28B command 负载 16B）。
/// 布局（O 反编译命令写入）：`{ chunk_id: u32, instance_index: u32, kind: u8,
/// sort_key_f16: u16, flags: u8, reserved: u8 }`（13B 有效 + 3B 填充 = 16B）。
/// chunk_id 是否 u64（对应 InstanceKey.chunk_id）I。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct NaniActiveInstance {
    /// 块/高层键（O 命令 +0x18；u32 低 32 位）。
    pub chunk_id: u32,
    /// 实例索引（O 命令 +0x1c）。
    pub instance_index: u32,
    /// subset kind（O 命令 +0x20，u8）。
    pub kind: u8,
    /// 排序键 f16（O：`half::f16::from_f32` → 命令 +0x21，2B）。
    pub sort_key_f16: u16,
    /// 标志（O 命令 +0x23；typed gap）。
    pub flags: u8,
    /// 保留（O 命令 +0x24 = 0）。
    pub reserved: u8,
}

impl NaniActiveInstance {
    /// 由 `InstanceKey` + kind 构造（O：maintain 建实例后的命令负载；U：chunk_id 语义）。
    pub fn from_key(_key: InstanceKey, _kind: crate::render::nani::gpu_world::GpuWorldSubsetKind) -> Self {
        todo!("recover NaniActiveInstance packing")
    }

    /// 转回 `InstanceKey`（O：update_instance_data 的 fast-path key；sort_key f16 解码 I）。
    pub fn into_key(&self) -> InstanceKey {
        todo!("recover InstanceKey from active instance (chunk_id + index + f16 sort_key)")
    }

    /// `InstanceHandle`（O：`{kind, index}`；maintain Local HashMap 键）。
    pub fn handle(&self) -> InstanceHandle {
        InstanceHandle {
            kind: self.kind_as_kind(),
            index: self.instance_index,
        }
    }

    fn kind_as_kind(&self) -> crate::render::nani::gpu_world::GpuWorldSubsetKind {
        if self.kind < 27 {
            unsafe { core::mem::transmute::<u8, crate::render::nani::gpu_world::GpuWorldSubsetKind>(self.kind) }
        } else {
            todo!("invalid kind u8 {}", self.kind)
        }
    }
}

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
