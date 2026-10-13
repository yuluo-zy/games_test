//! mesh_atlas.rs — 网格图集（WP3）。
//!
//! 机器码锚点（B patched EXE）：
//! - `MeshAtlas::add` 0x140a36200（行 75，3289B）。
//! - `MeshAtlasPart::new` 0x140833250（3B）；`impl$10::clone_behavior` 0x140002370。
//! 完整契约见 contracts/mesh_atlas.md。

#![allow(dead_code, unused_imports)]

use glam::Vec3;

use crate::render::mesh::Mesh;
use crate::render::mesh_atlas::AtlassedMeshName;

/// No-op hasher for pre-hashed `AtlassedMeshName` keys (observed `nohash_hasher` usage;
/// nohash-hasher is an offline-cache env gap — local no-op stand-in).
#[derive(Default, Clone, Copy)]
pub struct AtlasNameHasher(u64);
impl core::hash::Hasher for AtlasNameHasher {
    fn finish(&self) -> u64 {
        self.0
    }
    fn write(&mut self, _bytes: &[u8]) {
        // Unused for integer keys.
    }
    fn write_u64(&mut self, i: u64) {
        self.0 = i;
    }
}

/// 图集 part（16B，O）：`{ base_index: u32, count: u32, radius: f32, pad: u32 }`。
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct MeshAtlasPart {
    /// 顶点基址/part 索引（O：`MeshAtlas::add` 写 part_index）。
    pub base_index: u32,
    /// 顶点数（O：add 写 count）。
    pub count: u32,
    /// 包围半径（O：`sqrt(max|p|²)`）。
    pub radius: f32,
    /// 0。
    pub pad: u32,
}

impl MeshAtlasPart {
    /// 空构造（0x140833250，3B，O）。
    pub fn new() -> Self {
        Self::default()
    }
}

/// 网格图集：`HashMap<AtlassedMeshName, MeshAtlasPart>`（`NoHashHasher`，O）+ 属性布局。
#[derive(Default)]
pub struct MeshAtlas {
    /// 名字 → part（O：`HashMap::insert<AtlassedMeshName, MeshAtlasPart>` + "Duplicate atlas mesh name" panic）。
    parts: hashbrown::HashMap<
        AtlassedMeshName,
        MeshAtlasPart,
        core::hash::BuildHasherDefault<AtlasNameHasher>,
    >,
    /// 属性布局（排序后的属性名列表；O：`sort::driftsort_main`/`insertion_sort_shift_left`）。
    attributes: Vec<alloc::string::String>,
    /// 已注册 part 计数 / ComputedLayout 记录（O：`grow_one<ComputedLayout>`；语义 U）。
    layout: Vec<ComputedLayout>,
}

/// 布局记录（O 存在，字段 U）。
pub struct ComputedLayout {
    pub part: MeshAtlasPart,
}

impl MeshAtlas {
    /// 加一个 mesh（0x140a36200，O）：
    /// 1. 空 positions → panic "invalid mesh"。
    /// 2. 计算包围半径（遍历 position 三元组 sqrt(max²)）。
    /// 3. 属性名集与已有布局比对（`memcmp` 排序列表）；不一致 → `Err(anyhow!("Mesh attribute mismatch: Existing ..."))`。
    /// 4. 索引 `+ base_vertex` 偏移；push part 记录；`HashMap::insert`（重复名 → panic "Duplicate atlas mesh name"）。
    pub fn add(&mut self, name: AtlassedMeshName, mesh: Mesh) -> Result<(), anyhow::Error> {
        let _ = (name, mesh);
        todo!("recover MeshAtlas::add; see contracts/mesh_atlas.md §2")
    }

    /// 半径辅助（O：`radius = sqrt(max(x²+y²+z²))`，空=0）。
    fn compute_radius(positions: &[Vec3]) -> f32 {
        positions
            .iter()
            .fold(0.0f32, |acc, p| acc.max(p.length_squared()))
            .sqrt()
    }
}

/// 每 atlas part 的 mesh 信息（`TypedStorageBuffer` 元素；GPU `atlas_part_info` SSBO 布局；字段 U）。
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct NaniMeshAtlasPartInfo {
    /// typed gap：{base_vertex, vertex_count, index_offset, ...} 字段 U。
    pub _gap: [u32; 8],
}
