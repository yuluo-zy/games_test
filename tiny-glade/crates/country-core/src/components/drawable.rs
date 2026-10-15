//! `drawable.rs` — nani 可绘制组件族里已恢复的部分（B `components::drawable`）。
//!
//! - `NaniInstanceMeshDesc::from<MeshAtlasPart>`（`impl$1::from`，0x14082bd60，机器码 `mov [rcx+4], edx; mov dword [rcx], 1`）：
//!   判别 `1`（AtlasPart）+ 值 `edx`（part 索引**原样保留**）。此前重建里的 `AtlasPart(0)` 与证据不符。
//!   另一个变体 `Named(NameHash)` 由 `From<&str>` 之类内联构造（判别 0，I）。
//! - `DrawableMeshBundle::build`（`impl$2::build`，0x14082bd70）：`mov rcx, rdx; jmp 0x140b66340`，纯尾调用到共享的
//!   Bundle build，没有自己的逻辑。
//!
//! 未做：`cache_prev_object_transforms`（0x14082bd80，依赖稀疏实例缓冲两种布局与 `PrevTransform` 槽，见
//! `components-geometry/ALGORITHMS.md` §3）；`NaniActiveInstance`、`AlbedoMap`、`AlphaMap`、`NaniSolidTexture`、
//! `PrevTransform` 的字段。
use bevy_ecs::component::Component;

use crate::name_hash::NameHash;

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
}
