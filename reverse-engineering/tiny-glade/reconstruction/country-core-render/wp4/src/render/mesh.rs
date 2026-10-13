//! `Mesh` / `VertexAttributeValues` — CPU mesh container (WP4-owned type).
//!
//! Skeleton bundle: `skeleton/B/crates/country-core/src/render/mesh.rs`
//! (source_sha256 `e8e64685…`; 30 procedures; modules 476/…/537/544/…).
//! Decompiles: `B_0x140a07390.c …`, `evidence/building-gates/B-mesh-extra/140a34c90.c`.
//!
//! `Mesh` is 0x78 = 120 bytes (O: `AssetMeshLibrary::add` box memcpy of 15 u64s).
//! `indices` lives at +0x00, the attribute map (RandomState hash map) starts at
//! +0x28 with buckets +0x38 / mask +0x40 / items +0x50. Fields +0x18/+0x20 and
//! +0x58..+0x70 are typed gaps.

use std::collections::HashMap;

use glam::{Vec2, Vec3, Vec4};

use crate::utils_contract::{Aabb3, Triangle};

/// Vertex attribute key used by the game's mesh loader for positions (O:
/// 15-char `"Vertex_Position"` compared bytewise in `positions`).
pub const ATTR_POSITION: &str = "Vertex_Position";
/// Inferred names (I) for the other standard attributes.
pub const ATTR_NORMAL: &str = "Vertex_Normal";
pub const ATTR_UV: &str = "Vertex_UV";
pub const ATTR_COLOR: &str = "Vertex_Color";

/// Per-vertex attribute payload. Enum discriminants observed: 0/1 = f32-ish,
/// 2 = `Vec<[f32;2]>` (uvs), 3 = `Vec<[f32;3]>` (positions/normals/colors),
/// 4 = `Vec<[f32;4]>`, i32 = `Vec<i32>` (I; exact order beyond 3 is a typed gap).
#[derive(Clone, Debug)]
pub enum VertexAttributeValues {
    Float(Vec<f32>),
    Float2(Vec<Vec2>),
    Float3(Vec<Vec3>),
    Float4(Vec<Vec4>),
    Int(Vec<i32>),
    /// Typed gap: additional variants exist in the original (O: `append`
    /// dispatches through a jump table with more than these arms).
    #[doc(hidden)]
    __GapUnlisted(Vec<u8>, usize),
}

impl VertexAttributeValues {
    /// `array_length` — element count of the backing vector (O: 0x140949170).
    pub fn array_length(&self) -> usize {
        match self {
            VertexAttributeValues::Float(v) => v.len(),
            VertexAttributeValues::Float2(v) => v.len(),
            VertexAttributeValues::Float3(v) => v.len(),
            VertexAttributeValues::Float4(v) => v.len(),
            VertexAttributeValues::Int(v) => v.len(),
            VertexAttributeValues::__GapUnlisted(v, _) => v.len(),
        }
    }

    /// `stride` — bytes per element (I from variant type).
    pub fn stride(&self) -> usize {
        match self {
            VertexAttributeValues::Float(_) => core::mem::size_of::<f32>(),
            VertexAttributeValues::Float2(_) => core::mem::size_of::<Vec2>(),
            VertexAttributeValues::Float3(_) => core::mem::size_of::<Vec3>(),
            VertexAttributeValues::Float4(_) => core::mem::size_of::<Vec4>(),
            VertexAttributeValues::Int(_) => core::mem::size_of::<i32>(),
            VertexAttributeValues::__GapUnlisted(_, s) => *s,
        }
    }

    /// `size_in_bytes` — total byte length (I).
    pub fn size_in_bytes(&self) -> usize {
        self.array_length().wrapping_mul(self.stride())
    }

    /// `component_count` (I).
    pub fn component_count(&self) -> usize {
        match self {
            VertexAttributeValues::Float(_) | VertexAttributeValues::Int(_) => 1,
            VertexAttributeValues::Float2(_) => 2,
            VertexAttributeValues::Float3(_) => 3,
            VertexAttributeValues::Float4(_) => 4,
            VertexAttributeValues::__GapUnlisted(_, _) => 0,
        }
    }

    /// `append` — extend with `other` (O: 0x140a08440; different variants → panic).
    pub fn append(&mut self, other: &VertexAttributeValues) {
        let same_variant = core::mem::discriminant(self) == core::mem::discriminant(other);
        if !same_variant {
            panic!("cannot append different attribute types");
        }
        match (self, other) {
            (VertexAttributeValues::Float(a), VertexAttributeValues::Float(b)) => {
                a.extend_from_slice(b)
            }
            (VertexAttributeValues::Float2(a), VertexAttributeValues::Float2(b)) => {
                a.extend_from_slice(b)
            }
            (VertexAttributeValues::Float3(a), VertexAttributeValues::Float3(b)) => {
                a.extend_from_slice(b)
            }
            (VertexAttributeValues::Float4(a), VertexAttributeValues::Float4(b)) => {
                a.extend_from_slice(b)
            }
            (VertexAttributeValues::Int(a), VertexAttributeValues::Int(b)) => {
                a.extend_from_slice(b)
            }
            _ => {}
        }
    }

    /// `retain_by_index` — keep elements where `f(i)` holds (O: 0x140a36110,
    /// used by `Mesh::retain_vertices`).
    pub fn retain_by_index<F>(&mut self, mut f: F)
    where
        F: FnMut(usize) -> bool,
    {
        match self {
            VertexAttributeValues::Float(v) => {
                for i in (0..v.len()).rev() {
                    if !f(i) {
                        v.remove(i);
                    }
                }
            }
            VertexAttributeValues::Float2(v) => {
                for i in (0..v.len()).rev() {
                    if !f(i) {
                        v.remove(i);
                    }
                }
            }
            VertexAttributeValues::Float3(v) => {
                for i in (0..v.len()).rev() {
                    if !f(i) {
                        v.remove(i);
                    }
                }
            }
            VertexAttributeValues::Float4(v) => {
                for i in (0..v.len()).rev() {
                    if !f(i) {
                        v.remove(i);
                    }
                }
            }
            VertexAttributeValues::Int(v) => {
                for i in (0..v.len()).rev() {
                    if !f(i) {
                        v.remove(i);
                    }
                }
            }
            VertexAttributeValues::__GapUnlisted(_v, _) => {
                let _ = f;
                // U: unknown variant payload semantics.
            }
        }
    }

    pub fn as_f32(&self) -> &[f32] {
        match self {
            VertexAttributeValues::Float(v) => v,
            _ => panic!("attribute is not f32"),
        }
    }
    pub fn as_f32x2(&self) -> &[Vec2] {
        match self {
            VertexAttributeValues::Float2(v) => v,
            _ => panic!("attribute is not f32x2"),
        }
    }
    pub fn as_f32x3(&self) -> &[Vec3] {
        match self {
            VertexAttributeValues::Float3(v) => v,
            _ => panic!("attribute is not f32x3"),
        }
    }
    pub fn as_f32x4(&self) -> &[Vec4] {
        match self {
            VertexAttributeValues::Float4(v) => v,
            _ => panic!("attribute is not f32x4"),
        }
    }
    pub fn as_i32(&self) -> &[i32] {
        match self {
            VertexAttributeValues::Int(v) => v,
            _ => panic!("attribute is not i32"),
        }
    }
}

macro_rules! impl_from_vec {
    ($($ty:ident => $var:ident),* $(,)?) => {
        $(
            impl From<Vec<$ty>> for VertexAttributeValues {
                fn from(v: Vec<$ty>) -> Self {
                    VertexAttributeValues::$var(v)
                }
            }
        )*
    };
}
impl_from_vec! {
    f32 => Float,
    Vec2 => Float2,
    Vec3 => Float3,
    Vec4 => Float4,
    i32 => Int,
}

/// CPU mesh container (O: 0x78 bytes; indices at +0x00, attribute map at +0x28).
#[derive(Default, Debug)]
pub struct Mesh {
    pub indices: Vec<u32>,
    /// +0x18/+0x20 in the original layout. Semantics U (typed gap).
    _opaque_a: u64,
    _opaque_b: u64,
    /// Name-keyed attributes (`HashMap<String, VertexAttributeValues>`).
    pub attributes: HashMap<String, VertexAttributeValues>,
    /// +0x58..+0x70 in the original layout. Semantics U (typed gap).
    _opaque_c: u64,
    _opaque_d: u64,
    _opaque_e: u64,
}

impl Mesh {
    /// `Mesh::new` — empty mesh (O: 0x140a07390; empty indices + empty attributes).
    pub fn new(_source_file: Option<String>) -> Self {
        Mesh::default()
    }

    /// `set_source_file` (O: 0x140a07440).
    pub fn set_source_file(&mut self, _path: String) {
        // O: trivial store into the +0x68/+0x70 region (typed gap).
    }

    /// `is_valid` (I).
    pub fn is_valid(&self) -> bool {
        !self.indices.is_empty() && self.has_attribute(ATTR_POSITION)
    }

    /// `has_attribute` (O name; U body).
    pub fn has_attribute(&self, name: &str) -> bool {
        self.attributes.contains_key(name)
    }

    /// `set_indices` — replaces the index buffer (O: 0x140a07490).
    pub fn set_indices(&mut self, indices: Vec<u32>) {
        self.indices = indices;
    }

    /// `arrayed_indices` — triples the flat index list (O: 0x140a074d0).
    pub fn arrayed_indices(&self) -> Vec<[u32; 3]> {
        self.indices
            .chunks_exact(3)
            .map(|c| [c[0], c[1], c[2]])
            .collect()
    }

    /// `vertex_count` (I).
    pub fn vertex_count(&self) -> usize {
        self.positions().1.len()
    }

    /// `positions` — `(&indices-slice-range, &[[f32;3]])` (O: 0x140a07b30).
    /// Missing/incorrect `Vertex_Position` attribute → panic (`unwrap_failed`).
    pub fn positions(&self) -> (&[u32], &[Vec3]) {
        let pos = self
            .attributes
            .get(ATTR_POSITION)
            .unwrap_or_else(|| panic!("Mesh has no {} attribute", ATTR_POSITION));
        match pos {
            VertexAttributeValues::Float3(v) => (&self.indices, v),
            _ => panic!("{} attribute is not f32x3", ATTR_POSITION),
        }
    }

    /// `positions_without_indices` (O: 0x140a07ca0).
    pub fn positions_without_indices(&self) -> &[Vec3] {
        self.positions().1
    }

    /// `normals_without_indices` (O: 0x140a07e00; name inferred).
    pub fn normals_without_indices(&self) -> &[Vec3] {
        match self.attributes.get(ATTR_NORMAL) {
            Some(VertexAttributeValues::Float3(v)) => v,
            _ => &[],
        }
    }

    /// `uvs_without_indices` (O: 0x140a07f50; name inferred).
    pub fn uvs_without_indices(&self) -> &[Vec2] {
        match self.attributes.get(ATTR_UV) {
            Some(VertexAttributeValues::Float2(v)) => v,
            _ => &[],
        }
    }

    /// `set_attribute` — insert/replace a named attribute (O: 0x140a34170 &
    /// variants). Replaces in place when the name already exists (deallocates the
    /// old value and writes the new discriminant + Vec), else inserts a clone of
    /// the name with the value.
    pub fn set_attribute(&mut self, name: &str, values: impl Into<VertexAttributeValues>) {
        let values = values.into();
        match self.attributes.get_mut(name) {
            Some(slot) => *slot = values,
            None => {
                self.attributes.insert(name.to_owned(), values);
            }
        }
    }

    /// `add_color` — paint every vertex a uniform color (O: 0x140a07540):
    /// reads `Vertex_Position` count, builds `from_elem(color, n)`, sets
    /// `Vertex_Color`.
    pub fn add_color(&mut self, color: Vec3) {
        let n = self.positions().1.len();
        self.set_attribute(ATTR_COLOR, vec![color; n]);
    }

    /// `add_color_self` — per-vertex color derived from position (O:
    /// 0x140a07680; exact formula U).
    pub fn add_color_self(&mut self) {
        let positions = self.positions().1.to_vec();
        let colors: Vec<Vec3> = positions
            .iter()
            .map(|p| Vec3::new(p.x, p.y, p.z)) // U: color function not recovered
            .collect();
        self.set_attribute(ATTR_COLOR, colors);
    }

    /// `aabb_bounds` — axis-aligned bounding box of positions (O: 0x140a07870).
    /// Empty → `Aabb3 { min: +FLT_MAX, max: -FLT_MAX }`.
    pub fn aabb_bounds(&self) -> Aabb3 {
        let positions = self.positions().1;
        let mut bounds = Aabb3::empty();
        for p in positions {
            bounds.expand_to_contain_point(*p);
        }
        bounds
    }

    /// `collision_bbx` — collision bounding box (O: 0x140a07a70; xz-extended
    /// variant used by the game's physics queries).
    pub fn collision_bbx(&self) -> Aabb3 {
        self.aabb_bounds()
    }

    /// `collision_radius_xz` — max `sqrt(x²+z²)` over vertices (O: 0x140a081e0).
    pub fn collision_radius_xz(&self) -> f32 {
        let (indices, positions) = self.positions();
        let mut max_r = 0.0f32;
        for &i in indices {
            let p = positions[i as usize];
            max_r = max_r.max((p.x * p.x + p.z * p.z).sqrt());
        }
        max_r
    }

    /// `as_triangles` — collect index triples as `Triangle`s (O: 0x140a08280).
    pub fn as_triangles(&self) -> Vec<Triangle> {
        let (indices, positions) = self.positions();
        indices
            .chunks_exact(3)
            .map(|c| {
                Triangle::new(
                    positions[c[0] as usize],
                    positions[c[1] as usize],
                    positions[c[2] as usize],
                )
            })
            .collect()
    }

    /// `retain_vertices` — compact vertices (O: 0x140a08090).
    ///
    /// `vertex_flags` is `&mut [i32]` where 1 = keep, 0 = drop. On return each
    /// entry is the new vertex index (kept) or -1 (dropped); indices are
    /// rewritten to the compacted numbering (panic if an index pointed at a
    /// dropped vertex).
    pub fn retain_vertices(&mut self, vertex_flags: &mut [i32]) {
        // 1. Keep only indices that reference kept vertices.
        self.indices
            .retain(|&i| vertex_flags.get(i as usize).copied().unwrap_or(0) == 1);

        // 2. Compact each attribute by the keep mask.
        let attr_names: Vec<String> = self.attributes.keys().cloned().collect();
        for name in attr_names {
            if let Some(values) = self.attributes.get_mut(&name) {
                let flags = vertex_flags.to_vec();
                values.retain_by_index(|i| flags.get(i).copied().unwrap_or(0) == 1);
            }
        }

        // 3. Rewrite flags → new index (or -1).
        let mut next = 0i32;
        for f in vertex_flags.iter_mut() {
            if *f == 1 {
                *f = next;
                next += 1;
            } else {
                *f = -1;
            }
        }

        // 4. Rewrite indices through the remap.
        for idx in self.indices.iter_mut() {
            let new = vertex_flags[*idx as usize];
            assert!(new >= 0, "index references a removed vertex");
            *idx = new as u32;
        }
    }

    /// `clear` (O name).
    pub fn clear(&mut self) {
        self.indices.clear();
        self.attributes.clear();
    }

    /// `translate_hack_slow_dont_use` (O name; U body).
    pub fn translate_hack_slow_dont_use(&mut self, _offset: Vec3) {
        // U: not recovered (inline).
    }
}
