//! render/mesh_atlas_library.rs — mesh 图集库资源（WP6 恢复）。
//!
//! 源 SHA256（B skeleton）`resources/render/mesh_atlas_library.rs`；PDB 命名实体：
//! `MeshAtlasLibrary` / `AtlassedMeshName` / `WorldSubsetMeshAtlas` /
//! `PrefabMeshAtlasPart` / `MeshAtlasesModifiedCmd`。
//! `MeshAtlasLibrary::get_by_name<NameHash>`（O）：`nani_maintain_instances` 查 part。

#![allow(dead_code, unused_imports)]

use bevy_ecs::resource::Resource;

use crate::render::nani::mesh_atlas::MeshAtlasPart;
use crate::typed_gap::Gap;

/// 网格图集库（O：`Res<MeshAtlasLibrary>`；maintain 读）。
/// 布局 U（typed gap；内部 `FrozenMap<NameHash, part>`）。
#[derive(Resource, Clone, Debug, Default)]
pub struct MeshAtlasLibrary(pub Gap);

impl MeshAtlasLibrary {
    /// 按 `NameHash` 查 part（O：`get_by_name<utils::name_hash::NameHash>`；
    /// 缺失 → maintain panic）。
    pub fn get_by_name_hash(&self, _hash: u64) -> MeshAtlasPart {
        todo!("recover atlas part lookup")
    }
}

/// 图集化网格名（O：命名实体；`FrozenMap` 键）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct AtlassedMeshName(pub u64);
