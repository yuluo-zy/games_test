//! trimesh.rs — `Trimesh` / `TrimeshBuilder`（B 阶段按机器码重写，n_world）。
//!
//! 机器码锚点（B patched EXE cf048e27…）：
//! * `Trimesh::builder` 0x14129dbd0（3249B，O）；`TrimeshBuilder::positions` 0x14129e890（98B）；
//!   `TrimeshBuilder::attrib<Vec3>` 0x140905080 / 0x1412a90c0（476B，两份 ICF）；
//!   `TrimeshBuilder::build` 0x14129e900（2816B）；`TrimeshChunk::index_count` 0x14129dbc0（5B：`movzx eax, word [rcx+8]`）。
//!
//! 旧骨架的更正（均由机器码证实）：
//! 1. `attrib` 的 0x18B 记录是 `{ data: &[u8]（ptr + byte_len）, elem_size: u16, align: u8 }`，**没有属性名**；
//!    `byte` @0x12 是元素**对齐**（`Vec3` 为 4），不是类型枚举；属性必须按**对齐降序**加入。
//! 2. `TrimeshBuilder` 是 0x50B：`positions_index: Option<usize>`(+0)、`chunks: Vec<_>`(+0x10)、`attribs: Vec<_>`(+0x28)、
//!    `input_max_index: isize`(+0x40)、`vertex_count: usize`(+0x48)。`positions` 记录的是 positions 属性在
//!    `attribs` 里的**下标**（`[rdx+8] = [rdx+0x38]` = 当前 `attribs.len()`），不是 bool。
//! 3. `Trimesh::builder(indices)` 把索引缓冲切成每块最多 64 个三角形的 meshlet（`TrimeshChunk`），块内顶点去重成
//!    局部 u8 下标；这是旧骨架完全缺失的主体。`build` 把每块的“三角形 + 逐顶点属性”打包进一个 GPU 缓冲，并算包围球。
//! 4. builder 全部按值传递（`mov rcx, out; rdx = &self`，返回新值）。

#![allow(dead_code, unused_imports)]

use alloc::vec::Vec;
use core::mem::{align_of, size_of};
use std::collections::HashMap;

use glam::Vec3;

use crate::render::ssbo::rhapsody_boundary::{BackendError, Buffer, BufferDesc, GraphicsDevice};

/// 每个 meshlet 最多 64 个三角形（O：`cmp dword [rbp+0x28], 0x3f; ja` 与 `Vec::with_capacity(0x40)` u32）。
pub const TRIANGLES_PER_CHUNK: usize = 64;
/// 每块顶点容量（O：`Vec::with_capacity(0xc0)` u32 = 64 × 3）。
pub const VERTICES_PER_CHUNK_CAPACITY: usize = TRIANGLES_PER_CHUNK * 3;
/// `BufferDesc::new_gpu_only` 的 usage（O：`mov r8d, 0x20020`）。
pub const TRIMESH_BUFFER_USAGE: u32 = 0x20020;

/// 属性记录（0x18B，O）：`{ data: ptr+len (0x10B), elem_size: u16 @0x10, align: u8 @0x12 }`。
#[derive(Clone, Copy, Debug)]
struct Attrib<'a> {
    /// 调用方提供的整条属性数据（`len * size_of::<T>()` 字节；O：`[rax+8*r15] = ptr`，`+8 = len*12`）。
    data: &'a [u8],
    /// 元素大小（`Vec3` = 0xc）。
    elem_size: u16,
    /// 元素对齐（`Vec3` = 4）。
    align: u8,
}

/// builder 内部的 meshlet（0x30B，O）：`Vec<u32>` 三角形（每个三角形把 3 个局部下标打包成 `l0 | l1 << 8 | l2 << 16`）
/// + `Vec<u32>` 全局顶点下标（局部下标 → 全局下标）。
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BuilderChunk {
    pub triangles: Vec<u32>,
    pub vertices: Vec<u32>,
}

/// `build` 产出的 meshlet 描述（0x20B，O：`ebc1`–`ebd9` 的写入偏移）：
/// `address` @0（先写缓冲内偏移，创建缓冲后 += `device_address`）、`index_count` @8（u16，= 3 × 三角形数）、
/// `vertex_count` @0xa（u16）、`attributes_offset` @0xc（u32：属性区相对块起点的偏移）、包围球 @0x10。
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TrimeshChunk {
    pub address: u64,
    pub index_count: u16,
    pub vertex_count: u16,
    pub attributes_offset: u32,
    pub sphere_center: Vec3,
    pub sphere_radius: f32,
}

impl TrimeshChunk {
    /// `TrimeshChunk::index_count`（0x14129dbc0，O）：`movzx eax, word ptr [rcx + 8]`。
    pub fn index_count(&self) -> u16 {
        self.index_count
    }
}

/// 构建好的三角网格：meshlet 列表 + 持有全部数据的 GPU 缓冲（O：`Result` Ok 标签 0xb，`+8` Vec、`+0x20` Buffer 40B）。
pub struct Trimesh {
    pub chunks: Vec<TrimeshChunk>,
    pub buffer: Buffer,
}

/// 三角网格构建器（O，0x50B）。
pub struct TrimeshBuilder<'a> {
    /// +0x00：`positions()` 记录的 positions 属性下标（`Some` = 标签 1）。
    positions_index: Option<usize>,
    /// +0x10：meshlet 列表。
    chunks: Vec<BuilderChunk>,
    /// +0x28：属性列表。
    attribs: Vec<Attrib<'a>>,
    /// +0x40：输入索引里出现过的最大值（空为 -1；O：`mov r14, -1` + `cmovg`）。
    input_max_index: isize,
    /// +0x48：顶点数（0 = 未定；首个 attrib 设值，之后必须相等）。
    vertex_count: usize,
}

impl Trimesh {
    /// `Trimesh::builder`（0x14129dbd0，O）：把平铺的三角形索引缓冲切成 meshlet。
    /// * 只处理完整三元组（`len - len % 3`）；`chunks` 预留 `ceil(tri_count / 64)`；
    /// * 每块：最多 64 个三角形；块内用 `HashMap<u32, u8>`（RandomState）把全局下标映射到局部下标，
    ///   新顶点 `vertices.push(global)`，局部下标 = 映射当前元素数；三角形打包 `l0 | l1 << 8 | l2 << 16`；
    /// * 每个索引都并入 `input_max_index`（有符号最大值，初值 -1）；
    /// * 块结束后清空映射（复用同一个 map，Vec 则每块重新分配）；映射为空时结束。
    pub fn builder<'a>(indices: &[u32]) -> TrimeshBuilder<'a> {
        let tri_count = indices.len() / 3;
        let mut chunks: Vec<BuilderChunk> =
            Vec::with_capacity(tri_count.div_ceil(TRIANGLES_PER_CHUNK));
        let mut input_max_index: isize = -1;
        let mut map: HashMap<u32, u8> = HashMap::new();
        let mut triangles_in = indices.chunks_exact(3);
        loop {
            let mut triangles: Vec<u32> = Vec::with_capacity(TRIANGLES_PER_CHUNK);
            let mut vertices: Vec<u32> = Vec::with_capacity(VERTICES_PER_CHUNK_CAPACITY);
            let mut taken = 0usize;
            while taken < TRIANGLES_PER_CHUNK {
                let Some(tri) = triangles_in.next() else {
                    break;
                };
                taken += 1;
                let mut packed = 0u32;
                for (slot, &index) in tri.iter().enumerate() {
                    input_max_index = input_max_index.max(index as isize);
                    let next_local = map.len() as u8;
                    let local = *map.entry(index).or_insert_with(|| {
                        vertices.push(index);
                        next_local
                    });
                    packed |= u32::from(local) << (8 * slot);
                }
                triangles.push(packed);
            }
            if map.is_empty() {
                break;
            }
            chunks.push(BuilderChunk {
                triangles,
                vertices,
            });
            map.clear();
        }
        TrimeshBuilder {
            positions_index: None,
            chunks,
            attribs: Vec::new(),
            input_max_index,
            vertex_count: 0,
        }
    }
}

fn next_multiple_of_16(v: usize) -> usize {
    v.next_multiple_of(16)
}

impl<'a> TrimeshBuilder<'a> {
    /// meshlet 列表（O：`+0x10`）。
    pub fn chunks(&self) -> &[BuilderChunk] {
        &self.chunks
    }

    /// 输入索引的最大值（O：`+0x40`）。
    pub fn input_max_index(&self) -> isize {
        self.input_max_index
    }

    /// 已登记属性数（O：`+0x38`）。
    pub fn attrib_count(&self) -> usize {
        self.attribs.len()
    }

    /// 写入 positions 属性（0x14129e890，O）：`positions_index = Some(attribs.len())`，再 `attrib::<Vec3>`。
    pub fn positions(mut self, positions: &'a [Vec3]) -> Self {
        self.positions_index = Some(self.attribs.len());
        self.attrib(positions)
    }

    /// 写一个属性（0x140905080 / 0x1412a90c0，O），顺序（行号来自 panic 位置）：
    /// 1. `assert!(!data.is_empty())`（trimesh.rs:112:9）；
    /// 2. `assert!(self.input_max_index < data.len() as isize)`（:114:9）；
    /// 3. `vertex_count == 0` → 置为 `data.len()`，否则 `assert_eq!(self.vertex_count, data.len())`（:118:13）；
    /// 4. 上一个属性的对齐必须 ≥ 本属性对齐，否则 panic "Attributes must be added in the order of descending alignment"（:124:9）；
    /// 5. push `{ data 字节视图, size_of::<T>() as u16, align_of::<T>() as u8 }`。
    ///
    /// `T` 必须没有填充字节（原版只实例化 `Vec3`，O）。
    pub fn attrib<T: bytemuck::NoUninit>(mut self, data: &'a [T]) -> Self {
        assert!(!data.is_empty(), "assertion failed: !data.is_empty()");
        assert!(
            self.input_max_index < data.len() as isize,
            "assertion failed: self.input_max_index < data.len() as isize"
        );
        if self.vertex_count == 0 {
            self.vertex_count = data.len();
        } else {
            assert_eq!(self.vertex_count, data.len());
        }
        if let Some(last) = self.attribs.last() {
            if usize::from(last.align) < align_of::<T>() {
                panic!("Attributes must be added in the order of descending alignment");
            }
        }
        // 无填充字节由 `T: NoUninit` 保证（原版只实例化 `Vec3`）；视图只读且与 `data` 同寿命。
        let bytes: &'a [u8] = bytemuck::cast_slice(data);
        self.attribs.push(Attrib {
            data: bytes,
            elem_size: size_of::<T>() as u16,
            align: align_of::<T>() as u8,
        });
        self
    }

    /// 包围球（O：`ea5f`–`eb64`）：中心 = 块内顶点位置的平均（`1/n` 乘法），半径 = 到中心最大距离平方的平方根
    /// （`maxss` 忽略 NaN，初值 0）。越界 → `panic_bounds_check`（trimesh.rs:167:35 / :173:39）。
    fn bounding_sphere(positions: &[Vec3], vertices: &[u32]) -> (Vec3, f32) {
        let inv = 1.0f32 / vertices.len() as f32;
        let mut sum = Vec3::ZERO;
        for &i in vertices {
            sum += positions[i as usize];
        }
        let center = if vertices.is_empty() {
            Vec3::splat(0.0 * inv)
        } else {
            sum * inv
        };
        let mut max_d2 = 0.0f32;
        for &i in vertices {
            let d = center - positions[i as usize];
            let d2 = d.z * d.z + (d.x * d.x + d.y * d.y);
            max_d2 = max_d2.max(d2);
        }
        (center, max_d2.sqrt())
    }

    /// `positions` 属性的 `&[Vec3]` 视图（O：真实 `bytemuck::cast_slice::<u8, Vec3>`，
    /// 失败走 `something_went_wrong<PodCastError>("cast_slice", err)` → panic `cast_slice>{err:?}`）。
    fn positions_view(&self, index: usize) -> &[Vec3] {
        bytemuck::cast_slice::<u8, Vec3>(self.attribs[index].data)
    }

    /// 计算每块的缓冲布局（O：`build` 第一遍）：块 = `[三角形 u32 × n][填充到 16][属性0 × 顶点数][属性1 × …]`，
    /// 每个属性起点必须已按其对齐对齐（`assert_eq!`，:196:21），下一块紧接着上一块的属性区末尾（不再按 16 对齐）。
    /// 返回 `(每块 (起始偏移, 属性区起点), 总长)`，总长 = 最后偏移向上取整到 16。
    fn layout(&self) -> (Vec<(usize, usize)>, usize) {
        let mut offset = 0usize;
        let mut placed = Vec::with_capacity(self.chunks.len());
        for chunk in &self.chunks {
            let attr_start = next_multiple_of_16(offset + 4 * chunk.triangles.len());
            placed.push((offset, attr_start));
            offset = attr_start;
            for attr in &self.attribs {
                let align = usize::from(attr.align);
                let padded = offset.next_multiple_of(align);
                assert_eq!(offset, padded);
                offset += usize::from(attr.elem_size) * chunk.vertices.len();
            }
        }
        (placed, next_multiple_of_16(offset))
    }

    /// `TrimeshBuilder::build`（0x14129e900，O）：
    /// 1. `positions_index.expect("Don't forget to call positions()")`（:147:14），取 positions 视图；
    /// 2. 第一遍：逐块算包围球、写 `TrimeshChunk`（`address` = 块起点偏移）并累计布局；
    /// 3. 分配零初始化的字节缓冲（总长向上取整 16），第二遍：逐块拷贝三角形、再按属性顺序把块内每个顶点的元素
    ///    从属性数组 gather 出来（切片越界 → :232/:233 的 `slice_end_index_len_fail`；对齐 `assert_eq!` :224:17；
    ///    末尾 `assert_eq!(buffer.len(), 对齐后的偏移)` :243:9）；
    /// 4. `create_buffer(BufferDesc::new_gpu_only(len, 0x20020), name, Some(bytes))`；失败原样返回错误；
    /// 5. 成功后每个 `TrimeshChunk.address += buffer.device_address()`。
    pub fn build<D: GraphicsDevice>(self, device: &D, name: &str) -> Result<Trimesh, BackendError> {
        let positions_index = self
            .positions_index
            .expect("Don't forget to call positions()");
        let positions = self.positions_view(positions_index);

        let (placed, total) = self.layout();
        let mut chunks = Vec::with_capacity(self.chunks.len());
        for (chunk, &(start, attr_start)) in self.chunks.iter().zip(&placed) {
            let (center, radius) = Self::bounding_sphere(positions, &chunk.vertices);
            chunks.push(TrimeshChunk {
                address: start as u64,
                index_count: (3 * chunk.triangles.len()) as u16,
                vertex_count: chunk.vertices.len() as u16,
                attributes_offset: (attr_start - start) as u32,
                sphere_center: center,
                sphere_radius: radius,
            });
        }

        let mut bytes = vec![0u8; total];
        let mut offset = 0usize;
        for chunk in &self.chunks {
            let tri_bytes = 4 * chunk.triangles.len();
            bytes[offset..offset + tri_bytes].copy_from_slice(unsafe {
                core::slice::from_raw_parts(chunk.triangles.as_ptr() as *const u8, tri_bytes)
            });
            offset = next_multiple_of_16(offset + tri_bytes);
            for attr in &self.attribs {
                let align = usize::from(attr.align);
                assert_eq!(offset, offset.next_multiple_of(align));
                let size = usize::from(attr.elem_size);
                for &vertex in &chunk.vertices {
                    let src = vertex as usize * size;
                    bytes[offset..offset + size].copy_from_slice(&attr.data[src..src + size]);
                    offset += size;
                }
            }
        }
        assert_eq!(bytes.len(), next_multiple_of_16(offset));

        let desc = BufferDesc::new_gpu_only(bytes.len(), TRIMESH_BUFFER_USAGE);
        let buffer = device.create_buffer(&desc, name, Some(&bytes))?;
        let base = device.device_address(&buffer);
        for chunk in &mut chunks {
            chunk.address += base;
        }
        Ok(Trimesh { chunks, buffer })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::ssbo::tests::MockDevice;

    struct Dev(MockDevice);
    impl GraphicsDevice for Dev {
        fn create_buffer(
            &self,
            desc: &BufferDesc,
            name: &str,
            data: Option<&[u8]>,
        ) -> Result<Buffer, BackendError> {
            self.0.create_buffer(desc, name, data)
        }
        fn update_buffer_bytes(
            &self,
            b: &Buffer,
            data: &[u8],
            ranges: &[core::ops::Range<usize>],
            offset: usize,
        ) -> Result<(), BackendError> {
            self.0.update_buffer_bytes(b, data, ranges, offset)
        }
        fn device_address(&self, _buffer: &Buffer) -> u64 {
            0x1000_0000
        }
    }

    #[repr(C, align(16))]
    #[derive(Clone, Copy)]
    struct Aligned16([f32; 4]);
    // SAFETY: 16 字节、对齐 16、无填充字节。
    unsafe impl bytemuck::NoUninit for Aligned16 {}

    fn quad() -> (Vec<u32>, Vec<Vec3>) {
        (
            vec![0, 1, 2, 2, 1, 3],
            vec![
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(2.0, 0.0, 0.0),
                Vec3::new(0.0, 2.0, 0.0),
                Vec3::new(2.0, 2.0, 0.0),
            ],
        )
    }

    #[test]
    fn builder_dedups_vertices_into_local_indices() {
        let (idx, _) = quad();
        let b = Trimesh::builder(&idx);
        assert_eq!(b.chunks().len(), 1);
        let c = &b.chunks()[0];
        assert_eq!(c.vertices, vec![0, 1, 2, 3]);
        // 三角形 (0,1,2) → l = 0,1,2；(2,1,3) → l = 2,1,3；打包 l0 | l1<<8 | l2<<16。
        assert_eq!(c.triangles, vec![0x0002_0100, 0x0003_0102]);
        assert_eq!(b.input_max_index(), 3);
    }

    #[test]
    fn builder_splits_at_64_triangles_and_drops_remainder() {
        // 130 个三角形 → 64 + 64 + 2；末尾多出 2 个索引（不足一个三角形）被忽略。
        let mut idx: Vec<u32> = (0..130 * 3).collect();
        idx.extend([7, 8]);
        let b = Trimesh::builder(&idx);
        assert_eq!(
            b.chunks()
                .iter()
                .map(|c| c.triangles.len())
                .collect::<Vec<_>>(),
            vec![64, 64, 2]
        );
        assert_eq!(b.chunks()[0].vertices.len(), 192);
        assert_eq!(b.input_max_index(), 389);
        // 局部下标按块重新编号：第二块第一个三角形仍是 0,1,2。
        assert_eq!(b.chunks()[1].triangles[0], 0x0002_0100);
        assert_eq!(b.chunks()[1].vertices[0], 192);
    }

    #[test]
    fn builder_empty_input() {
        let b = Trimesh::builder(&[]);
        assert!(b.chunks().is_empty());
        assert_eq!(b.input_max_index(), -1);
        let b = Trimesh::builder(&[1, 2]);
        assert!(b.chunks().is_empty());
        assert_eq!(b.input_max_index(), -1, "不足一个三角形时不读取索引");
    }

    #[test]
    fn positions_records_attrib_index() {
        let (idx, pos) = quad();
        let b = Trimesh::builder(&idx).positions(&pos);
        assert_eq!(b.positions_index, Some(0));
        assert_eq!(b.attrib_count(), 1);
        let a = &b.attribs[0];
        assert_eq!((a.elem_size, a.align, a.data.len()), (12, 4, 48));
    }

    #[test]
    #[should_panic(expected = "assertion failed: !data.is_empty()")]
    fn attrib_rejects_empty() {
        let _ = Trimesh::builder(&[]).attrib::<Vec3>(&[]);
    }

    #[test]
    #[should_panic(expected = "assertion failed: self.input_max_index < data.len() as isize")]
    fn attrib_requires_covering_the_max_index() {
        let (idx, pos) = quad();
        let _ = Trimesh::builder(&idx).positions(&pos[..3]); // 最大下标 3，只给 3 个顶点
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn attrib_vertex_count_must_match() {
        let (idx, pos) = quad();
        let extra = [1.0f32; 5];
        let _ = Trimesh::builder(&idx).positions(&pos).attrib(&extra);
    }

    #[test]
    #[should_panic(expected = "Attributes must be added in the order of descending alignment")]
    fn attrib_alignment_must_descend() {
        let (idx, pos) = quad();
        let wide = [Aligned16([0.0; 4]); 4];
        let _ = Trimesh::builder(&idx).positions(&pos).attrib(&wide);
    }

    #[test]
    fn attrib_descending_alignment_is_allowed() {
        let (idx, pos) = quad();
        let wide = [Aligned16([0.0; 4]); 4];
        let b = Trimesh::builder(&idx).attrib(&wide).positions(&pos);
        assert_eq!(b.positions_index, Some(1));
    }

    #[test]
    #[should_panic(expected = "Don't forget to call positions()")]
    fn build_requires_positions() {
        let (idx, _) = quad();
        let dev = Dev(MockDevice::default());
        let _ = Trimesh::builder(&idx).build(&dev, "t");
    }

    #[test]
    fn build_packs_triangles_then_gathered_attributes() {
        let (idx, pos) = quad();
        let dev = Dev(MockDevice::default());
        let mesh = Trimesh::builder(&idx)
            .positions(&pos)
            .build(&dev, "quad")
            .unwrap();
        let created = dev.0.created.borrow();
        let (desc, name, data) = &created[0];
        assert_eq!(name, "quad");
        assert_eq!(desc.usage, 0x20020);
        assert!(desc.gpu_only);
        let bytes = data.as_ref().unwrap();
        // 2 个三角形 = 8B，对齐到 16；4 个顶点 × 12B = 48B；总长 64。
        assert_eq!(bytes.len(), 64);
        assert_eq!(desc.size, 64);
        assert_eq!(
            u32::from_le_bytes(bytes[0..4].try_into().unwrap()),
            0x0002_0100
        );
        assert_eq!(
            u32::from_le_bytes(bytes[4..8].try_into().unwrap()),
            0x0003_0102
        );
        // 属性区从 16 开始：顶点 1（局部 1）= (2,0,0)。
        let x = f32::from_le_bytes(bytes[16 + 12..16 + 16].try_into().unwrap());
        assert_eq!(x, 2.0);
        // 块描述：偏移加上设备地址；index_count = 6，vertex_count = 4，属性区偏移 16。
        assert_eq!(mesh.chunks.len(), 1);
        let c = &mesh.chunks[0];
        assert_eq!(c.address, 0x1000_0000);
        assert_eq!(
            (c.index_count(), c.vertex_count, c.attributes_offset),
            (6, 4, 16)
        );
        // 包围球：中心 (1,1,0)，半径 √2。
        assert_eq!(c.sphere_center, Vec3::new(1.0, 1.0, 0.0));
        assert!((c.sphere_radius - 2.0f32.sqrt()).abs() < 1e-6);
        assert_eq!(core::mem::size_of::<TrimeshChunk>(), 0x20);
    }

    #[test]
    fn build_chunk_offsets_are_back_to_back() {
        // 两块：第一块 64 个三角形（192 顶点），第二块 1 个三角形。
        let idx: Vec<u32> = (0..65 * 3).collect();
        let pos: Vec<Vec3> = (0..65 * 3).map(|i| Vec3::new(i as f32, 0.0, 0.0)).collect();
        let dev = Dev(MockDevice::default());
        let mesh = Trimesh::builder(&idx)
            .positions(&pos)
            .build(&dev, "m")
            .unwrap();
        assert_eq!(mesh.chunks.len(), 2);
        // 第一块：三角形 256B（已按 16 对齐），属性 192 × 12 = 2304B。
        assert_eq!(mesh.chunks[0].attributes_offset, 256);
        // 下一块紧接上一块属性区末尾：256 + 2304 = 2560（不再对齐到 16 的倍数以外的位置）。
        assert_eq!(mesh.chunks[1].address - 0x1000_0000, 2560);
        // 第二块：4B 三角形 → 属性区起点 +16；3 个顶点 × 12。
        assert_eq!(mesh.chunks[1].attributes_offset, 16);
        let created = dev.0.created.borrow();
        assert_eq!(
            created[0].0.size,
            (2560 + 16 + 36usize).next_multiple_of(16)
        );
    }
}
