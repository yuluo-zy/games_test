//! `MeshAtlas` — 把多个 `Mesh` 合并进一张大网格（layer A2）。
//!
//! 证据（O，逐条核对 `MeshAtlas::add` 0x140a36200，4040 B，见 `evidence/country-core-audit/L0_NOTES.md`）：
//!  * 布局：`mesh: Mesh` @0x00（0x78 B，合并后的网格）、`layouts: Vec<MeshAtlasPart>` @0x78（16 B 元素）、
//!    `parts: HashMap<AtlassedMeshName, u32>` @0x90（`items` @0xa8；`HashMap::insert<AtlassedMeshName,u32>`）。
//!    PDB 把 16 B 元素记作 `toolbar::mei::layout::ComputedLayout`，是链接器合并相同机器码所致，
//!    元素就是 `{index_offset, index_count, radius, 0}`。
//!  * 签名：`add(&mut self, name, mesh: &Mesh) -> Result<u32, anyhow::Error>`（返回的 u32 = 新 part 序号）。
//!    调用方 `MeshAtlasLibrary::add`（0x140a371d0）在 `Err` 上再包一层 "invalid mesh: " 上下文。
//!  * 执行顺序：
//!     1. `mesh.indices` 为空 → `panic!("invalid mesh: {:?}", name)`（`AtlassedMeshName` 的 Debug 是元组结构体）；
//!     2. 半径：属性表非空时取 `Vertex_Position`（缺失 → `expect("no entry found for key")`，判别式不是 Float3 →
//!        `panic!("incorrect attribute type")`），`sqrt(max_i (x²+y²)+z²)`，初值 0，`f32::max` 语义；表空 → 0；
//!     3. `assert!(mesh_idx_within_atlas)`：`parts.len() < 0x40`（一张图集最多 64 个 part）；
//!     4. 顶点基址 = 图集现有**任一**属性的元素个数（表空 → 0；各属性等长是不变量）；
//!     5. 图集索引为空（首个 mesh）→ `self.mesh = mesh.clone()`（含 `source_file`、`label`）；
//!        否则把 `mesh.indices` 逐个 `+ base_vertex`（wrapping）追加到图集索引，**然后**比较两边排序后的属性名
//!        列表（逐项 `String` 相等），不同 → `Err(anyhow!("Mesh attribute mismatch.\n    Existing: {:?}\n    Incoming: {:?}"))`
//!        （此时索引已被追加，不回滚），相同 → 逐属性 `VertexAttributeValues::append`；
//!     6. 追加布局 `{index_offset = 追加前的图集索引数, index_count = mesh.indices.len(), radius, 0}`；
//!     7. `parts.insert(name, part_idx)` 返回 `Some` → `panic!("Duplicate atlas mesh name: {:?}", name)`（布局已追加）；
//!     8. 返回 `Ok(part_idx)`。
//!
//! 偏离：原版属性表是 `RandomState` 的 `HashMap`，这里沿用 `render::mesh::Mesh` 的同一类型；
//! `parts` 的哈希器不可观察，用标准 `HashMap`。

use std::collections::HashMap;

use glam::Vec3;

use crate::render::mesh::{ATTR_POSITION, Mesh, VertexAttributeValues};
use crate::resources::render::mesh_atlas_library::AtlassedMeshName;

/// 一张图集最多容纳的 part 数（O：`parts.len() >= 0x40` → 断言失败）。
pub const MAX_PARTS: usize = 0x40;

/// 图集布局记录（16 B，O）：`{index_offset, index_count, radius, 0}`。
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MeshAtlasPart {
    /// 该 mesh 的索引在图集索引表中的起点（= 追加前的索引数）。
    pub base_index: u32,
    /// 该 mesh 的索引个数。
    pub count: u32,
    /// 包围半径（`sqrt(max |p|²)`）。
    pub radius: f32,
    pub pad: u32,
}

impl MeshAtlasPart {
    /// 空构造（0x140833250，3 B，O）。
    pub fn new() -> Self {
        Self::default()
    }
}

/// PDB 里的名字（O，ICF 别名）。
pub type ComputedLayout = MeshAtlasPart;

/// 网格图集。
#[derive(Debug)]
pub struct MeshAtlas {
    pub mesh: Mesh,
    pub layouts: Vec<MeshAtlasPart>,
    /// `crate` 内可见：`MeshAtlasLibrary::clear_atlas_by_name` / `clear_world_subset_atlas`
    /// 直接对这张表做 `remove` / `clear`（O：两处都是内联的 hashbrown 操作）。
    pub(crate) parts: HashMap<AtlassedMeshName, u32>,
}

impl Default for MeshAtlas {
    fn default() -> Self {
        // O：原版没有独立的 `Default` 符号；`MeshAtlasLibrary` 的 `from_iter` fold（0x141478760）和
        // `add` 的“缺失则补”分支（0x140a371d0）都内联同一份构造：`Mesh::new("MeshAtlas")`
        // （label 指针 → "MeshAtlas"，长度 9）、空 layouts、空 parts。
        MeshAtlas {
            mesh: Mesh::new("MeshAtlas"),
            layouts: Vec::new(),
            parts: HashMap::new(),
        }
    }
}

impl MeshAtlas {
    /// 已登记的 part 数（O：`parts.items`）。
    pub fn len(&self) -> usize {
        self.parts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.parts.is_empty()
    }

    /// 按名字取 part 序号（I：`parts` 的只读访问）。
    pub fn part_index(&self, name: AtlassedMeshName) -> Option<u32> {
        self.parts.get(&name).copied()
    }

    /// O 0x140a36200，执行顺序见模块注释。
    pub fn add(&mut self, name: AtlassedMeshName, mesh: &Mesh) -> Result<u32, anyhow::Error> {
        if mesh.indices.is_empty() {
            panic!("invalid mesh: {:?}", name);
        }

        let radius = if mesh.attributes.is_empty() {
            0.0
        } else {
            Self::compute_radius(Self::positions_of(mesh))
        };

        let part_idx = self.parts.len();
        let mesh_idx_within_atlas = part_idx < MAX_PARTS;
        assert!(
            mesh_idx_within_atlas,
            "assertion failed: mesh_idx_within_atlas"
        );

        let base_vertex = self
            .mesh
            .attributes
            .values()
            .next()
            .map(VertexAttributeValues::array_length)
            .unwrap_or(0) as u32;

        let index_offset = self.mesh.indices.len() as u32;
        if self.mesh.indices.is_empty() {
            self.mesh = mesh.clone();
        } else {
            self.mesh
                .indices
                .extend(mesh.indices.iter().map(|&i| i.wrapping_add(base_vertex)));

            let existing = Self::sorted_names(&self.mesh);
            let incoming = Self::sorted_names(mesh);
            if existing != incoming {
                return Err(anyhow::anyhow!(
                    "Mesh attribute mismatch.\n    Existing: {:?}\n    Incoming: {:?}",
                    existing,
                    incoming
                ));
            }
            for (key, values) in self.mesh.attributes.iter_mut() {
                let src = mesh.attributes.get(key).expect("no entry found for key");
                values.append(src);
            }
        }

        self.layouts.push(MeshAtlasPart {
            base_index: index_offset,
            count: mesh.indices.len() as u32,
            radius,
            pad: 0,
        });

        if self.parts.insert(name, part_idx as u32).is_some() {
            panic!("Duplicate atlas mesh name: {:?}", name);
        }
        Ok(part_idx as u32)
    }

    fn positions_of(mesh: &Mesh) -> &[Vec3] {
        match mesh
            .attributes
            .get(ATTR_POSITION)
            .expect("no entry found for key")
        {
            VertexAttributeValues::Float3(v) => v,
            _ => panic!("incorrect attribute type"),
        }
    }

    /// 排序后的属性名（O：`Vec<&String>` + 稳定排序；这里取同样的字符串序）。
    fn sorted_names(mesh: &Mesh) -> Vec<&String> {
        let mut v: Vec<&String> = mesh.attributes.keys().collect();
        v.sort();
        v
    }

    /// `sqrt(max_i ((x²+y²)+z²))`，初值 0，`f32::max`（O：`maxss` + `cmpunordss` 的 NaN 处理等价于 `f32::max`）。
    pub fn compute_radius(positions: &[Vec3]) -> f32 {
        positions
            .iter()
            .fold(0.0f32, |acc, p| {
                acc.max((p.x * p.x + p.y * p.y) + p.z * p.z)
            })
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::name_hash::NameHash;

    /// 测试用：用原始 u64 哈希构造名字。
    fn name(n: u64) -> AtlassedMeshName {
        AtlassedMeshName(NameHash(n))
    }

    fn tri(offset: f32, extra: bool) -> Mesh {
        let mut m = Mesh::new("t");
        m.set_attribute(
            ATTR_POSITION,
            vec![
                Vec3::new(offset, 0.0, 0.0),
                Vec3::new(offset, 3.0, 0.0),
                Vec3::new(offset, 0.0, 4.0),
            ],
        );
        if extra {
            m.set_attribute("Vertex_Color", vec![Vec3::ONE; 3]);
        }
        m.set_indices(vec![0, 1, 2]);
        m
    }

    #[test]
    fn first_mesh_is_cloned_and_gets_part_zero() {
        let mut a = MeshAtlas::default();
        let idx = a.add(name(10), &tri(0.0, false)).unwrap();
        assert_eq!(idx, 0);
        assert_eq!(a.mesh.indices, vec![0, 1, 2]);
        assert_eq!(
            a.layouts,
            vec![MeshAtlasPart {
                base_index: 0,
                count: 3,
                radius: 4.0,
                pad: 0
            }]
        );
        assert_eq!(a.part_index(name(10)), Some(0));
    }

    #[test]
    fn second_mesh_offsets_indices_by_vertex_count_and_appends_attributes() {
        let mut a = MeshAtlas::default();
        a.add(name(1), &tri(0.0, true)).unwrap();
        let idx = a.add(name(2), &tri(5.0, true)).unwrap();
        assert_eq!(idx, 1);
        assert_eq!(a.mesh.indices, vec![0, 1, 2, 3, 4, 5]);
        assert_eq!(a.layouts[1].base_index, 3);
        assert_eq!(a.layouts[1].count, 3);
        // 半径 = sqrt(max(25 + 0 + 0, 25 + 9, 25 + 16)) = sqrt(41)
        assert!((a.layouts[1].radius - 41.0f32.sqrt()).abs() < 1e-6);
        assert_eq!(a.mesh.attributes["Vertex_Position"].array_length(), 6);
        assert_eq!(a.mesh.attributes["Vertex_Color"].array_length(), 6);
        assert_eq!(a.len(), 2);
    }

    #[test]
    fn attribute_mismatch_errors_after_indices_were_already_extended() {
        let mut a = MeshAtlas::default();
        a.add(name(1), &tri(0.0, true)).unwrap();
        let err = a.add(name(2), &tri(1.0, false)).unwrap_err().to_string();
        assert!(
            err.starts_with("Mesh attribute mismatch.\n    Existing: "),
            "{err}"
        );
        assert!(
            err.contains("\n    Incoming: [\"Vertex_Position\"]"),
            "{err}"
        );
        // O：索引先追加、后校验，不回滚；布局与名字未登记。
        assert_eq!(a.mesh.indices.len(), 6);
        assert_eq!(a.layouts.len(), 1);
        assert_eq!(a.len(), 1);
    }

    #[test]
    #[should_panic(expected = "invalid mesh: AtlassedMeshName(NameHash { hash: 7 })")]
    fn empty_indices_panic_with_debug_name() {
        let mut m = tri(0.0, false);
        m.set_indices(Vec::new());
        MeshAtlas::default().add(name(7), &m).unwrap();
    }

    #[test]
    #[should_panic(expected = "Duplicate atlas mesh name: AtlassedMeshName(NameHash { hash: 1 })")]
    fn duplicate_name_panics() {
        let mut a = MeshAtlas::default();
        a.add(name(1), &tri(0.0, false)).unwrap();
        a.add(name(1), &tri(0.0, false)).unwrap();
    }

    #[test]
    #[should_panic(expected = "mesh_idx_within_atlas")]
    fn sixty_fifth_part_is_rejected() {
        let mut a = MeshAtlas::default();
        for i in 0..=MAX_PARTS as u64 {
            a.add(name(i), &tri(0.0, false)).unwrap();
        }
    }

    #[test]
    #[should_panic(expected = "incorrect attribute type")]
    fn non_float3_position_panics() {
        let mut m = Mesh::new("bad");
        m.set_attribute(ATTR_POSITION, vec![glam::Vec2::ZERO; 3]);
        m.set_indices(vec![0, 1, 2]);
        MeshAtlas::default().add(name(1), &m).unwrap();
    }

    #[test]
    fn radius_ignores_nan_like_f32_max() {
        let r =
            MeshAtlas::compute_radius(&[Vec3::new(f32::NAN, 0.0, 0.0), Vec3::new(3.0, 4.0, 0.0)]);
        assert_eq!(r, 5.0);
        assert_eq!(MeshAtlas::compute_radius(&[]), 0.0);
    }
}
