//! 可绘制实体的网格描述与显式组件组合。
//!
//! NaniBundle 用于实例化渲染，DrawableMeshBundle 用于普通网格绘制。
//! 使用 Bevy Bundle 派生完成插入，不另实现原作的库转发胶水。
//! 原生字段复制范围与本机 Rust 分配大小分别记录在布局测试中。
//!
//! Previous-object transforms use the canonical component below; material families
//! 详见 recovery/components-2026-10-18.md。
use bevy_ecs::bundle::Bundle;

use crate::asset_libraries::Handle;
use crate::name_hash::NameHash;
use crate::render::mesh::Mesh;
use crate::render::nani::gpu_world::InstanceDataMarker;
use crate::render::shader::RasterShaderProgram;
use crate::utils::components::transform::Transform;

/// 共享实例描述组件的原模块入口，保持同一 ECS 类型身份。
pub use crate::render::nani::systems::NaniInstanceDesc;

/// 图集里的 part 索引（O：`From<MeshAtlasPart>` 存 `edx`，一个 u32）。
///
/// 与 `render::nani::MeshAtlasPart`（16 B 的图集布局记录）不是同一个东西。
pub type AtlasPartIndex = u32;

/// 网格描述：按名字或按图集 part 选择（O：`u32` 判别 @0）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

/// 实例渲染实体：网格描述、世界变换和实例数据类型标记。
///
/// B component_ids 注册序为 desc → transform → marker；字段物理顺序由 Rust
/// 布局决定。本机 transform@0、desc@0x30、marker@0x48，尾部对齐后大小 0x50。
/// 原作写入器的有效复制范围至 0x48；不能将其当作本机分配大小。
#[derive(Bundle)]
pub struct NaniBundle<T: Send + Sync + 'static> {
    /// `NaniInstanceDesc`（+0x30，O）。
    pub desc: NaniInstanceDesc,
    /// 游戏自有 `Transform`（+0x00，O）。
    pub transform: Transform,
    /// 实例数据类型的 ECS marker（+0x48，ZST，O）。
    pub marker: InstanceDataMarker<T>,
}

/// 普通网格实体：网格句柄、光栅着色器句柄和世界变换。
///
/// B component_ids 注册序为 mesh → shader → transform；本机物理偏移见测试。
/// Bundle 载荷不含 bool，旧证据中的 bool 属于外层命令闭包。
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

        assert_eq!(
            offset_of!(NaniBundle<StoneFloorInstanceData>, transform),
            0x00
        );
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

/// Native material disk-texture name hash, eight bytes. The draw callback
/// reads this same component; no parallel render-only view is constructed.
#[derive(bevy_ecs::component::Component, Debug, PartialEq, Eq)]
pub struct AlbedoMap(pub u64);
/// Native alpha-map disk-texture name hash.
#[derive(bevy_ecs::component::Component, Debug, PartialEq, Eq)]
pub struct AlphaMap(pub u64);

/// One canonical previous object Transform, read by actual sheep/duck draw callbacks.
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, PartialEq)]
pub struct PrevTransform {
    pub rotation: glam::Quat,
    pub translation: glam::Vec3,
    pub scale: glam::Vec3,
}
impl Default for PrevTransform {
    fn default() -> Self {
        Self::from(Transform::identity())
    }
}
impl From<Transform> for PrevTransform {
    fn from(value: Transform) -> Self {
        Self {
            rotation: value.rotation,
            translation: value.translation,
            scale: value.scale,
        }
    }
}

/// B14082bd80/628B, PDB FunctionSystem140b51320: unfiltered
/// Query<(&Transform, &mut PrevTransform)>. Bevy owns changed ticks.
pub fn cache_prev_object_transforms(
    mut transforms: bevy_ecs::system::Query<(&Transform, &mut PrevTransform)>,
) {
    for (current, mut previous) in &mut transforms {
        *previous = PrevTransform::from(*current);
    }
}

/// Canonical solid-texture selector observed as one component byte in B
/// clone1412efd00 and updates142072010/142072b50. PDB does not recover the
/// original Rust declaration or variant names; this is a checked opaque host
/// representation of the nine observed codes. Code9 is builder None, not a
/// component value. No default selector is supplied.
#[repr(transparent)]
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct NaniSolidTexture(u8);

impl NaniSolidTexture {
    pub const fn encoded(self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for NaniSolidTexture {
    type Error = u8;

    fn try_from(encoded: u8) -> Result<Self, Self::Error> {
        if encoded < 9 {
            Ok(Self(encoded))
        } else {
            Err(encoded)
        }
    }
}

#[cfg(test)]
mod solid_texture_tests {
    use super::NaniSolidTexture;

    #[test]
    fn explicit_codes_round_trip_and_none_niche_is_not_a_component() {
        assert_eq!(core::mem::size_of::<NaniSolidTexture>(), 1);
        for code in 0..9 {
            assert_eq!(NaniSolidTexture::try_from(code).unwrap().encoded(), code);
        }
        assert_eq!(NaniSolidTexture::try_from(9), Err(9));
        assert_eq!(NaniSolidTexture::try_from(255), Err(255));
    }
}
