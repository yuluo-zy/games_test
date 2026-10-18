//! `drawable.rs` — nani 可绘制组件族里已恢复的部分（B `components::drawable`）。
//!
//! - `NaniInstanceMeshDesc::from<MeshAtlasPart>`（`impl$1::from`，0x14082bd60，机器码 `mov [rcx+4], edx; mov dword [rcx], 1`）：
//!   判别 `1`（AtlasPart）+ 值 `edx`（part 索引**原样保留**）。此前重建里的 `AtlasPart(0)` 与证据不符。
//!   另一个变体 `Named(NameHash)` 由 `From<&str>` 之类内联构造（判别 0，I）。
//! - `DrawableMeshBundle::build`（`impl$2::build`，0x14082bd70）：`mov rcx, rdx; jmp 0x140b66340`，纯尾调用到共享的
//!   Bundle build，没有自己的逻辑。
//!
//! 本文件还是两个 `#[derive(Bundle)]` 的归属（PDB 全名
//! `country_core::components::drawable::{NaniBundle, DrawableMeshBundle}`）：
//!
//! * `NaniBundle<T>`：`component_ids<NaniBundle<UberInstanceData>>`（0x14267a9e0，fresh 2026-10-17）
//!   依次注册 `NaniInstanceDesc`、`Transform`、`InstanceDataMarker<T>`；字段声明序即此序。
//!   因 `repr(Rust)` 的字段重排（`Transform` 对齐 16），实际内存布局
//!   `{ Transform @0x00 (0x30), NaniInstanceDesc @0x30 (0x18), InstanceDataMarker<T> @0x48 (ZST) }`，
//!   总大小 0x48 —— `write_components<NaniBundle<StoneFloorInstanceData>>`（0x141a07f20）的两段
//!   拷贝分别读 bundle `+0x30..0x48`（desc）与 `+0x00..0x30`（Transform），第三段 0 长度（marker），O。
//!   `WindowVisualEntityBuilder::build` 的 insert 闭包（0x1412ab270）以 `+0x08` 起写入同一布局，O。
//! * `DrawableMeshBundle`：`component_ids`（0x14267c2a0，fresh）依次注册
//!   `Handle<Mesh>`、`Handle<RasterShaderProgram>`、`Transform`；`write_components`（0x141fe7aa0）
//!   读 `Transform` @0x00..0x30、两个 8B 句柄 @0x30/@0x38，总大小 0x40，O。
//!
//! 未做：`cache_prev_object_transforms`（0x14082bd80，依赖稀疏实例缓冲两种布局与 `PrevTransform` 槽，见
//! `components-geometry/ALGORITHMS.md` §3）；`NaniActiveInstance`、`AlbedoMap`、`AlphaMap`、`NaniSolidTexture`、
//! `PrevTransform` 的字段。
use bevy_ecs::bundle::Bundle;
use bevy_ecs::component::Component;

use crate::asset_libraries::Handle;
use crate::name_hash::NameHash;
use crate::render::mesh::Mesh;
use crate::render::nani::gpu_world::InstanceDataMarker;
use crate::render::shader::RasterShaderProgram;
use crate::utils::components::transform::Transform;

/// 原始类型路径是 `country_core::components::drawable::NaniInstanceDesc`
/// （`component_ids` 里 `register_component` 的 PDB 名，O）；当前实现位于
/// `render::nani::systems`，此处按原模块路径重导出（类型同一，非新类型）。
pub use crate::render::nani::systems::NaniInstanceDesc;

/// 图集里的 part 索引（O：`From<MeshAtlasPart>` 存 `edx`，一个 u32）。
///
/// 与 `render::nani::MeshAtlasPart`（16 B 的图集布局记录）不是同一个东西。
pub type AtlasPartIndex = u32;

/// 网格描述：按名字或按图集 part 选择（O：`u32` 判别 @0）。
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub enum NaniInstanceMeshDesc {
    /// 判别 0（I）。
    Named(NameHash),
    /// 判别 1（O）。
    AtlasPart(AtlasPartIndex),
}

impl NaniInstanceMeshDesc {
    /// 判别值（O）。
    pub fn tag(self) -> u32 {
        match self {
            Self::Named(_) => 0,
            Self::AtlasPart(_) => 1,
        }
    }
}

impl From<AtlasPartIndex> for NaniInstanceMeshDesc {
    /// 0x14082bd60。
    fn from(part: AtlasPartIndex) -> Self {
        Self::AtlasPart(part)
    }
}

/// nani 实例实体 bundle（B `components::drawable::NaniBundle<T>`，O）。
///
/// 字段**声明序**（`component_ids<NaniBundle<UberInstanceData>>` 0x14267a9e0 的注册序，O）：
/// `NaniInstanceDesc` → `Transform` → `InstanceDataMarker<T>`。
///
/// 内存布局（O，`write_components<NaniBundle<StoneFloorInstanceData>>` 0x141a07f20 +
/// `WindowVisualEntityBuilder::build` insert 闭包 0x1412ab270 的写偏移）：
///
/// | 偏移 | 字段 | 大小 |
/// |---|---|---|
/// | 0x00 | `transform`（`utils::components::transform::Transform`） | 0x30 |
/// | 0x30 | `NaniInstanceDesc`（16B 网格选择器 + 1B subset kind） | 0x18 |
/// | 0x48 | `InstanceDataMarker<T>`（ZST） | 0 |
///
/// 这是 `repr(Rust)` 的字段重排结果（`Transform` 对齐 16 排在最前），不是声明序，
/// 所以本结构**不能**加 `#[repr(C)]`（那会把 desc 移到 +0x00，与机器码不符）。
///
/// 字段名是 U（机器码只给布局）；类型与偏移是 O。
/// `T: Send + Sync + 'static` 是 `InstanceDataMarker<T>: Component` 的隐含约束
/// （所有实例数据类型都是纯数据；原版 derive 同样需要它，I）。
#[derive(Bundle)]
pub struct NaniBundle<T: Send + Sync + 'static> {
    /// `NaniInstanceDesc`（+0x30，O）。
    pub desc: NaniInstanceDesc,
    /// 游戏自有 `Transform`（+0x00，O）。
    pub transform: Transform,
    /// 实例数据类型的 ECS marker（+0x48，ZST，O）。
    pub marker: InstanceDataMarker<T>,
}

/// 普通网格可绘制 bundle（B `components::drawable::DrawableMeshBundle`，O）。
///
/// 字段**声明序**（`component_ids` 0x14267c2a0 的注册序，O）：
/// `Handle<Mesh>` → `Handle<RasterShaderProgram>` → `Transform`。
///
/// 内存布局（O，`write_components<DrawableMeshBundle>` 0x141fe7aa0）：
///
/// | 偏移 | 字段 | 大小 |
/// |---|---|---|
/// | 0x00 | `transform` | 0x30 |
/// | 0x30 | `mesh`（`Handle<Mesh>`） | 8 |
/// | 0x38 | `shader`（`Handle<RasterShaderProgram>`） | 8 |
///
/// 同上：`repr(Rust)` 把对齐 16 的 `Transform` 排到最前；总大小 0x40。
/// 没有 bool 字段——早期证据里“DrawableMeshBundle 内一个 bool”位于 insert 闭包
/// （命令/错误处理状态，闭包 +0x48/0x58），不在 bundle 载荷内（fresh 复核）。
///
/// 字段名是 U；类型与偏移是 O。
#[derive(Bundle)]
pub struct DrawableMeshBundle {
    /// 网格句柄（+0x30，O）。
    pub mesh: Handle<Mesh>,
    /// raster shader 句柄（+0x38，O）。
    pub shader: Handle<RasterShaderProgram>,
    /// 游戏自有 `Transform`（+0x00，O）。
    pub transform: Transform,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atlas_part_index_is_preserved_not_zeroed() {
        let desc = NaniInstanceMeshDesc::from(42);
        assert_eq!(desc, NaniInstanceMeshDesc::AtlasPart(42));
        assert_eq!(desc.tag(), 1);
    }

    #[test]
    fn named_descriptors_use_tag_zero() {
        assert_eq!(NaniInstanceMeshDesc::Named(NameHash(7)).tag(), 0);
    }

    /// O：`component_ids` 0x14267a9e0 的三个组件 + `write_components` 0x141a07f20 的读偏移。
    #[test]
    fn nani_bundle_layout_matches_write_components() {
        use crate::render::nani::instance_data::StoneFloorInstanceData;
        use core::mem::{align_of, offset_of, size_of};

        assert_eq!(offset_of!(NaniBundle<StoneFloorInstanceData>, transform), 0x00);
        assert_eq!(offset_of!(NaniBundle<StoneFloorInstanceData>, desc), 0x30);
        assert_eq!(offset_of!(NaniBundle<StoneFloorInstanceData>, marker), 0x48);
        // `Transform` 对齐 16 → 结构体尾部补齐到 16 的倍数：0x48 补齐为 0x50。
        // `write_components` 只读 +0x30..0x48（desc），尾部 padding 不参与拷贝。
        assert_eq!(size_of::<NaniBundle<StoneFloorInstanceData>>(), 0x50);
        assert_eq!(size_of::<NaniInstanceDesc>(), 0x18);
        assert_eq!(size_of::<Transform>(), 0x30);
        assert_eq!(
            align_of::<NaniBundle<StoneFloorInstanceData>>(),
            align_of::<Transform>()
        );
    }

    /// O：`component_ids` 0x14267c2a0 的三个组件 + `write_components` 0x141fe7aa0 的读偏移。
    #[test]
    fn drawable_mesh_bundle_layout_matches_write_components() {
        use core::mem::{offset_of, size_of};

        assert_eq!(offset_of!(DrawableMeshBundle, transform), 0x00);
        assert_eq!(offset_of!(DrawableMeshBundle, mesh), 0x30);
        assert_eq!(offset_of!(DrawableMeshBundle, shader), 0x38);
        assert_eq!(size_of::<DrawableMeshBundle>(), 0x40);
        assert_eq!(size_of::<Handle<Mesh>>(), 8);
        assert_eq!(size_of::<Handle<RasterShaderProgram>>(), 8);
    }
}
