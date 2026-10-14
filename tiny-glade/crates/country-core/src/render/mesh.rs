//! `Mesh` / `VertexAttributeValues` — CPU mesh container (layer A1).
//!
//! 证据（O，逐条核对机器码；详见 `evidence/country-core-audit/L0_NOTES.md`）：
//!  * `Mesh` 0x78 B：`indices: Vec<u32>` @0x00、`source_file: Option<(String, 8 B)>` @0x18（None = cap 取 `i64::MIN`，
//!    32 B）、`attributes: HashMap<String, VertexAttributeValues, RandomState>` @0x38（hashbrown 表 0x38..0x58 +
//!    SipHash 密钥 0x58..0x68，桶 0x38 B：key String @0 + 值 @0x18）、`label: &'static str` @0x68（I）。
//!    `Mesh::new` 0x140a07390 取 `(label)`，表为空、密钥来自线程局部 `RandomState`。
//!  * `VertexAttributeValues`：判别式 **Int=0, Float=1, Float2=2, Float3=3, Float4=4**（`set_attribute<Vec<T>>`
//!    的写入立即数、`append` 跳转表、`build_vao` 的元素大小 4/4/8/12/16 三处互证）；Vec 在 +0x08（cap/ptr/len）。
//!    旧版 `__GapUnlisted` 变体不存在。
//!  * `From<Vec<[f32;2]>>`/`From<Vec<[f32;3]>>`（impl$6/impl$7 0x140a0db00/0x140a0db90）：`assert!(!v.is_empty())`。
//!    `set_attribute<Vec<T>>` 内联了同一断言；`set_attribute<VertexAttributeValues>` 不检查。
//!  * 取属性的方法一律：表为空/键缺失 → `Option::unwrap` panic；判别式不符 → panic `"incorrect attribute type"`
//!    （`crates/country-core/src/render/mesh.rs`）。旧版把缺失当作空切片，是错的。

use std::collections::HashMap;

use glam::{Vec2, Vec3, Vec4};

use crate::utils_contract::{Aabb3, Rectangle2d, Triangle};

/// 原版字符串（O：`positions` 比较 15 字节 `"Vertex_Position"`；`normals…` 13 字节；`uvs…` 9 字节；
/// `add_color` 写入 12 字节 `"Vertex_Color"`）。
pub const ATTR_POSITION: &str = "Vertex_Position";
pub const ATTR_NORMAL: &str = "Vertex_Normal";
pub const ATTR_UV: &str = "Vertex_UV";
pub const ATTR_COLOR: &str = "Vertex_Color";

/// 单个顶点属性的数据（判别式见模块注释）。
#[derive(Clone, Debug, PartialEq)]
pub enum VertexAttributeValues {
    Int(Vec<i32>),
    Float(Vec<f32>),
    Float2(Vec<Vec2>),
    Float3(Vec<Vec3>),
    Float4(Vec<Vec4>),
}

impl VertexAttributeValues {
    /// 元素个数（O：所有变体的 len 都在 +0x18）。
    pub fn array_length(&self) -> usize {
        match self {
            Self::Int(v) => v.len(),
            Self::Float(v) => v.len(),
            Self::Float2(v) => v.len(),
            Self::Float3(v) => v.len(),
            Self::Float4(v) => v.len(),
        }
    }

    /// 每元素字节数（O：4/4/8/12/16）。
    pub fn stride(&self) -> usize {
        match self {
            Self::Int(_) | Self::Float(_) => 4,
            Self::Float2(_) => 8,
            Self::Float3(_) => 12,
            Self::Float4(_) => 16,
        }
    }

    pub fn size_in_bytes(&self) -> usize {
        self.array_length() * self.stride()
    }

    /// `append`（O 0x140a08440）：判别式相同 → `extend_from_slice`；不同 →
    /// `panic!("vertex attribute mismatch: self: {:?}, src: {:?}", discriminant(self), discriminant(src))`。
    pub fn append(&mut self, src: &VertexAttributeValues) {
        match (&mut *self, src) {
            (Self::Int(a), Self::Int(b)) => a.extend_from_slice(b),
            (Self::Float(a), Self::Float(b)) => a.extend_from_slice(b),
            (Self::Float2(a), Self::Float2(b)) => a.extend_from_slice(b),
            (Self::Float3(a), Self::Float3(b)) => a.extend_from_slice(b),
            (Self::Float4(a), Self::Float4(b)) => a.extend_from_slice(b),
            _ => panic!(
                "vertex attribute mismatch: self: {:?}, src: {:?}",
                core::mem::discriminant(self),
                core::mem::discriminant(src)
            ),
        }
    }

    /// `retain_by_index`（O 0x140a36110）：对各变体做 `Vec::retain`，谓词收到从 0 起的元素序号。
    pub fn retain_by_index(&mut self, mut f: impl FnMut(usize) -> bool) {
        fn go<T>(v: &mut Vec<T>, f: &mut impl FnMut(usize) -> bool) {
            let mut i = 0usize;
            v.retain(|_| {
                let keep = f(i);
                i += 1;
                keep
            });
        }
        match self {
            Self::Int(v) => go(v, &mut f),
            Self::Float(v) => go(v, &mut f),
            Self::Float2(v) => go(v, &mut f),
            Self::Float3(v) => go(v, &mut f),
            Self::Float4(v) => go(v, &mut f),
        }
    }
}

macro_rules! impl_from_vec {
    ($($ty:ty => $var:ident),* $(,)?) => {$(
        impl From<Vec<$ty>> for VertexAttributeValues {
            /// 仅 Float2/Float3 有独立符号（O），其余在 `set_attribute<Vec<T>>` 内联；断言一致。
            fn from(v: Vec<$ty>) -> Self {
                assert!(!v.is_empty(), "assertion failed: !v.is_empty()");
                Self::$var(v)
            }
        }
    )*};
}
impl_from_vec! { i32 => Int, f32 => Float, Vec2 => Float2, Vec3 => Float3, Vec4 => Float4 }

/// `source_file` 的载荷（O：32 B，首字为 String cap）。后 8 字节含义 U（I：文件修改时间）。
#[derive(Clone, Debug, PartialEq)]
pub struct SourceFile {
    pub path: String,
    pub stamp: u64,
}

/// `Mesh::collision_bbx` 的返回值（O：`Rectangle2d`(0x18 B) + `min.y` @0x18 + `max.y` @0x1c）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CollisionBbx {
    pub rect: Rectangle2d,
    pub min_y: f32,
    pub max_y: f32,
}

/// CPU 网格（O：`MeshAtlas::add` 首个 mesh 走整体克隆，含属性表 `hashbrown::clone` 与 `source_file`）。
#[derive(Debug, Clone)]
pub struct Mesh {
    pub indices: Vec<u32>,
    pub source_file: Option<SourceFile>,
    pub attributes: HashMap<String, VertexAttributeValues>,
    /// +0x68：`Mesh::new` 的参数（I：调试标签）。
    pub label: &'static str,
}

impl Default for Mesh {
    fn default() -> Self {
        Mesh::new("")
    }
}

impl Mesh {
    /// `Mesh::new`（O 0x140a07390）：空索引、`source_file = None`、空属性表。
    pub fn new(label: &'static str) -> Self {
        Mesh {
            indices: Vec::new(),
            source_file: None,
            attributes: HashMap::new(),
            label,
        }
    }

    /// `set_source_file`（O 0x140a07440）：释放旧值后整体覆盖。
    pub fn set_source_file(&mut self, file: SourceFile) {
        self.source_file = Some(file);
    }

    pub fn has_attribute(&self, name: &str) -> bool {
        self.attributes.contains_key(name)
    }

    /// `set_indices`（O 0x140a07490）：释放旧缓冲并接管新 Vec。
    pub fn set_indices(&mut self, indices: Vec<u32>) {
        self.indices = indices;
    }

    /// `arrayed_indices`（O 0x140a074d0）：`chunks_exact(3)`，尾部不足 3 个的被丢弃。
    pub fn arrayed_indices(&self) -> Vec<[u32; 3]> {
        self.indices
            .chunks_exact(3)
            .map(|c| [c[0], c[1], c[2]])
            .collect()
    }

    /// `set_attribute`（O 0x140a34170 …共 8 个单态化）：键已存在 → 原位替换值（旧 Vec 释放，键不重建）；
    /// 否则复制键并插入。`Vec<T>` 入口带非空断言（见 `From`），`VertexAttributeValues` 入口不带。
    pub fn set_attribute(&mut self, name: &str, values: impl Into<VertexAttributeValues>) {
        let values = values.into();
        match self.attributes.get_mut(name) {
            Some(slot) => *slot = values,
            None => {
                self.attributes.insert(name.to_owned(), values);
            }
        }
    }

    fn attr(&self, name: &str) -> &VertexAttributeValues {
        self.attributes
            .get(name)
            .expect("called `Option::unwrap()` on a `None` value")
    }

    fn attr_f3(&self, name: &str) -> &[Vec3] {
        match self.attr(name) {
            VertexAttributeValues::Float3(v) => v,
            _ => panic!("incorrect attribute type"),
        }
    }

    /// `positions`（O 0x140a07b30）：`(索引, 位置)`；缺失 → unwrap panic，类型不符 → "incorrect attribute type"。
    pub fn positions(&self) -> (&[u32], &[Vec3]) {
        (&self.indices, self.attr_f3(ATTR_POSITION))
    }

    /// O 0x140a07ca0。
    pub fn positions_without_indices(&self) -> &[Vec3] {
        self.attr_f3(ATTR_POSITION)
    }

    /// O 0x140a07e00（键 `Vertex_Normal`，Float3）。
    pub fn normals_without_indices(&self) -> &[Vec3] {
        self.attr_f3(ATTR_NORMAL)
    }

    /// O 0x140a07f50（键 `Vertex_UV`，Float2）。
    pub fn uvs_without_indices(&self) -> &[Vec2] {
        match self.attr(ATTR_UV) {
            VertexAttributeValues::Float2(v) => v,
            _ => panic!("incorrect attribute type"),
        }
    }

    /// `add_color`（O 0x140a07540）：`n` = 位置属性的元素个数（**不检查判别式**），
    /// `set_attribute("Vertex_Color", vec![color; n])`；n == 0 时由 `From` 的非空断言 panic。
    pub fn add_color(&mut self, color: Vec3) {
        let n = self.attr(ATTR_POSITION).array_length();
        self.set_attribute(ATTR_COLOR, vec![color; n]);
    }

    /// `add_color_self`（O 0x140a07680）：按值接收并返回 `Mesh` 的 builder 形态 —— 做 `add_color` 后
    /// 整体（0x78 B）搬出。旧版把它当成“按位置着色”是错的。
    pub fn add_color_self(mut self, color: Vec3) -> Self {
        self.add_color(color);
        self
    }

    /// `aabb_bounds`（O 0x140a07870）：位置为空 → `{min: +MAX, max: -MAX}`；否则以首点为初值，
    /// 其余点 `expand_to_contain_point`。
    pub fn aabb_bounds(&self) -> Aabb3 {
        let pos = self.attr_f3(ATTR_POSITION);
        let Some((&first, rest)) = pos.split_first() else {
            return Aabb3::from_points([]);
        };
        let mut b = Aabb3 {
            min: first,
            max: first,
        };
        for &p in rest {
            b.expand_to_contain_point(p);
        }
        b
    }

    /// `collision_bbx`（O 0x140a07a70）：`aabb_bounds` 的 xz 投影矩形（宽 = extent.x、长 = extent.z、
    /// 中心 = (center.x, center.z)，轴对齐）加上 y 范围。
    pub fn collision_bbx(&self) -> CollisionBbx {
        let b = self.aabb_bounds();
        let (e, c) = (b.extent(), b.center());
        CollisionBbx {
            rect: Rectangle2d::new_axis_aligned(Vec2::new(c.x, c.z), Vec2::new(e.x, e.z)),
            min_y: b.min.y,
            max_y: b.max.y,
        }
    }

    /// `collision_radius_xz`（O 0x140a081e0）：遍历**索引**（不是顶点）取 `sqrt(x²+z²)` 的最大值，初值 0；
    /// 索引越界 panic（`panic_bounds_check`）。
    pub fn collision_radius_xz(&self) -> f32 {
        let (indices, pos) = self.positions();
        indices.iter().fold(0.0f32, |m, &i| {
            let p = pos[i as usize];
            m.max((p.x * p.x + p.z * p.z).sqrt())
        })
    }

    /// `as_triangles`（O 0x140a08280）：`chunks_exact(3)`，每组按位置表取三点（越界 panic）。
    pub fn as_triangles(&self) -> Vec<Triangle> {
        let (indices, pos) = self.positions();
        indices
            .chunks_exact(3)
            .map(|c| Triangle::new(pos[c[0] as usize], pos[c[1] as usize], pos[c[2] as usize]))
            .collect()
    }

    /// `retain_vertices`（O 0x140a08090）。`flags[v] == 1` 保留顶点 v，其余删除。顺序：
    ///  1. `indices.retain(|i| flags[i] == 1)`（越界 panic）；
    ///  2. 每个属性 `retain_by_index(|k| flags[k] == 1)`（越界 panic）；
    ///  3. 把 `flags` 就地改成“新顶点序号，被删除者为 -1”；
    ///  4. 索引经 `flags` 重映射，结果为 -1 → `assert!(remapped != u32::MAX)`。
    pub fn retain_vertices(&mut self, flags: &mut [i32]) {
        self.indices.retain(|&i| flags[i as usize] == 1);
        for v in self.attributes.values_mut() {
            v.retain_by_index(|k| flags[k] == 1);
        }
        let mut next = 0i32;
        for f in flags.iter_mut() {
            let keep = *f == 1;
            *f = if keep { next } else { -1 };
            next += keep as i32;
        }
        for i in self.indices.iter_mut() {
            let remapped = flags[*i as usize] as u32;
            assert!(
                remapped != u32::MAX,
                "assertion failed: remapped != u32::MAX"
            );
            *i = remapped;
        }
    }

    /// `translate_hack_slow_dont_use`（PDB 名为 O，无独立符号 → 已内联；行为 I：平移全部位置）。
    pub fn translate_hack_slow_dont_use(&mut self, offset: Vec3) {
        if let Some(VertexAttributeValues::Float3(v)) = self.attributes.get_mut(ATTR_POSITION) {
            v.iter_mut().for_each(|p| *p += offset);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn quad() -> Mesh {
        let mut m = Mesh::new("quad");
        m.set_attribute(
            ATTR_POSITION,
            vec![
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(2.0, 3.0, 0.0),
                Vec3::new(2.0, 5.0, 4.0),
                Vec3::new(0.0, 0.0, 4.0),
            ],
        );
        m.set_indices(vec![0, 1, 2, 0, 2, 3]);
        m
    }

    #[test]
    fn tags_and_strides() {
        use VertexAttributeValues as V;
        let d = |v: &V| format!("{:?}", core::mem::discriminant(v));
        assert_eq!(d(&V::Int(vec![1])), "Discriminant(0)");
        assert_eq!(d(&V::Float4(vec![Vec4::ONE])), "Discriminant(4)");
        assert_eq!(V::Float3(vec![Vec3::ONE; 2]).size_in_bytes(), 24);
    }

    #[test]
    #[should_panic(expected = "!v.is_empty()")]
    fn from_vec_rejects_empty() {
        let _ = VertexAttributeValues::from(Vec::<Vec3>::new());
    }

    #[test]
    #[should_panic(
        expected = "vertex attribute mismatch: self: Discriminant(2), src: Discriminant(3)"
    )]
    fn append_mismatch_panics_with_discriminants() {
        let mut a = VertexAttributeValues::Float2(vec![Vec2::ZERO]);
        a.append(&VertexAttributeValues::Float3(vec![Vec3::ZERO]));
    }

    #[test]
    fn append_same_kind() {
        let mut a = VertexAttributeValues::Float(vec![1.0]);
        a.append(&VertexAttributeValues::Float(vec![2.0, 3.0]));
        assert_eq!(a, VertexAttributeValues::Float(vec![1.0, 2.0, 3.0]));
    }

    #[test]
    fn missing_or_wrong_attribute_panics() {
        let m = Mesh::new("x");
        assert!(
            std::panic::catch_unwind(|| Mesh::new("x").positions_without_indices().len()).is_err()
        );
        assert!(!m.has_attribute(ATTR_UV));
        let mut m = quad();
        m.set_attribute(ATTR_UV, VertexAttributeValues::Float3(vec![Vec3::ZERO]));
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            m.uvs_without_indices().len()
        }));
        assert!(r.is_err());
    }

    #[test]
    fn add_color_and_builder() {
        let m = quad().add_color_self(Vec3::new(0.1, 0.2, 0.3));
        match &m.attributes[ATTR_COLOR] {
            VertexAttributeValues::Float3(c) => {
                assert_eq!((c.len(), c[3]), (4, Vec3::new(0.1, 0.2, 0.3)))
            }
            _ => panic!(),
        }
    }

    #[test]
    fn set_attribute_replaces_in_place() {
        let mut m = quad();
        m.set_attribute(ATTR_POSITION, vec![Vec3::X]);
        assert_eq!(m.attributes.len(), 1);
        assert_eq!(m.positions_without_indices(), &[Vec3::X]);
    }

    #[test]
    fn bounds_and_collision() {
        let m = quad();
        let b = m.aabb_bounds();
        assert_eq!(
            (b.min, b.max),
            (Vec3::new(0.0, 0.0, 0.0), Vec3::new(2.0, 5.0, 4.0))
        );
        let c = m.collision_bbx();
        assert_eq!(
            (c.rect.size.x, c.rect.size.y, c.rect.center),
            (2.0, 4.0, Vec2::new(1.0, 2.0))
        );
        assert_eq!((c.min_y, c.max_y), (0.0, 5.0));
        assert_eq!(c.rect.axis, Vec2::X);
        assert_eq!(m.collision_radius_xz(), (2.0f32 * 2.0 + 4.0 * 4.0).sqrt());
    }

    #[test]
    fn empty_positions_give_inverted_box() {
        let mut m = Mesh::new("e");
        m.attributes
            .insert(ATTR_POSITION.into(), VertexAttributeValues::Float3(vec![]));
        assert_eq!(m.aabb_bounds(), Aabb3::from_points([]));
    }

    #[test]
    fn triangles_and_arrayed_indices_drop_remainder() {
        let mut m = quad();
        m.indices.push(9); // 7 个索引，最后一个落在不足 3 个的尾部
        assert_eq!(m.arrayed_indices(), vec![[0, 1, 2], [0, 2, 3]]);
        assert_eq!(m.as_triangles().len(), 2);
    }

    #[test]
    fn retain_vertices_remaps_flags_and_indices() {
        let mut m = quad();
        m.set_attribute(ATTR_COLOR, vec![Vec3::ONE; 4]);
        m.set_indices(vec![0, 2, 3]);
        let mut flags = [1, 0, 1, 1];
        m.retain_vertices(&mut flags);
        assert_eq!(flags, [0, -1, 1, 2]);
        assert_eq!(m.indices, vec![0, 1, 2]);
        assert_eq!(m.positions_without_indices().len(), 3);
        assert_eq!(m.attributes[ATTR_COLOR].array_length(), 3);
    }

    #[test]
    #[should_panic]
    fn retain_vertices_out_of_range_index_panics() {
        let mut m = quad();
        m.set_indices(vec![0, 7, 1]);
        m.retain_vertices(&mut [1, 1, 1, 1]);
    }
}
