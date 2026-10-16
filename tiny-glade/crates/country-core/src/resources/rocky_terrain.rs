//! `rocky_terrain.rs` — 岩质地层网格资源（B 0x1408bbf50 `RockyTerrainMesh::load`，O）。
//!
//! 证据：`chapters/resources/rocky_terrain.rs.md`、`terrain-water/DIGEST.md`：
//! `load` = `fs::read_to_string` → `serde_json` 解析 → `add_new_with_data<VertexSSBO/
//! TriangleSSBO>`（顶点源记录 0x24 B→0x30 B、三角形 0x10 B→0x20 B GPU 填充）。
//! SSBO 名（O，SsboId 立即数）：顶点 "Rocky Terrain Vertices"、三角形 "Rocky Terrain
//! Triangles"；`displace_rocky_terrain` 每帧 GPU 变形。
use std::path::Path;

use bevy_ecs::prelude::*;
use serde::Deserialize;

use crate::asset_libraries::ssbo_library::{AssetSsboLibrary, SsboType};
use crate::render::ssbo::{StorageElement, MAPPED_MEMORY_READBACK};

/// JSON 中间顶点（O：source 0x24 B → GPU 0x30 B；字段语义 I）。
#[derive(Clone, Copy, Debug, Deserialize)]
pub struct LoadedRockyTerrainVertex {
    /// 位置（+0）。
    pub position: [f32; 3],
    /// 法线/UV 等剩余字段（I：JSON 字段名待取证，先按位型记录）。
    pub extra: [f32; 6],
}

/// JSON 中间三角形（O：source 0x10 B → GPU 0x20 B；三个顶点索引）。
#[derive(Clone, Copy, Debug, Deserialize)]
pub struct LoadedRockyTerrainTriangle {
    pub indices: [u32; 4],
}

/// 顶点 SSBO 元素（O：GPU 0x30 B）。
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default)]
pub struct RockyTerrainVertexSsbo(pub [u32; 12]);

unsafe impl bytemuck::Zeroable for RockyTerrainVertexSsbo {}
unsafe impl bytemuck::Pod for RockyTerrainVertexSsbo {}

impl StorageElement for RockyTerrainVertexSsbo {
    const NAME: &'static str = "Rocky Terrain Vertices";
    const BUFFER_LEN: usize = 0;
    const MAPPED_MEMORY: u8 = MAPPED_MEMORY_READBACK;
}

impl SsboType for RockyTerrainVertexSsbo {
    type Storage = crate::render::ssbo::TypedStorageBuffer<Self>;
    fn new_storage_buffer(
        device: &dyn crate::render::ssbo::rhapsody_boundary::GraphicsDevice,
        initial: Option<&[u8]>,
    ) -> Self::Storage {
        crate::render::ssbo::TypedStorageBuffer::new_storage_buffer(device, initial)
    }
}

/// 三角形 SSBO 元素（O：GPU 0x20 B）。
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default)]
pub struct RockyTerrainTriangleSsbo(pub [u32; 8]);

unsafe impl bytemuck::Zeroable for RockyTerrainTriangleSsbo {}
unsafe impl bytemuck::Pod for RockyTerrainTriangleSsbo {}

impl StorageElement for RockyTerrainTriangleSsbo {
    const NAME: &'static str = "Rocky Terrain Triangles";
    const BUFFER_LEN: usize = 0;
    const MAPPED_MEMORY: u8 = MAPPED_MEMORY_READBACK;
}

impl SsboType for RockyTerrainTriangleSsbo {
    type Storage = crate::render::ssbo::TypedStorageBuffer<Self>;
    fn new_storage_buffer(
        device: &dyn crate::render::ssbo::rhapsody_boundary::GraphicsDevice,
        initial: Option<&[u8]>,
    ) -> Self::Storage {
        crate::render::ssbo::TypedStorageBuffer::new_storage_buffer(device, initial)
    }
}

/// 岩质地层网格资源（O：`RockyTerrainMesh`，首 qword = 三角形数 / 3）。
#[derive(Resource, Clone, Debug, Default)]
pub struct RockyTerrainMesh {
    /// 三角形组数（O：`displace_rocky_terrain` 首 qword / 3）。
    pub triangle_groups: u32,
    /// 顶点缓冲（GPU 0x30 B）。
    pub vertices: Vec<RockyTerrainVertexSsbo>,
    /// 三角形缓冲（GPU 0x20 B）。
    pub triangles: Vec<RockyTerrainTriangleSsbo>,
}

impl RockyTerrainMesh {
    /// `RockyTerrainMesh::load`（0x1408bbf50，O 结构）：读 JSON → serde 解析 →
    /// 转换 GPU 记录 → `add_new_with_data` 注册两 SSBO。
    pub fn load(
        path: &Path,
        library: &AssetSsboLibrary,
        device: &dyn crate::render::ssbo::rhapsody_boundary::GraphicsDevice,
    ) -> Result<Self, std::io::Error> {
        let text = std::fs::read_to_string(path)?;
        let data: RockyTerrainJson = serde_json::from_str(&text)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        let vertices: Vec<RockyTerrainVertexSsbo> = data
            .vertices
            .iter()
            .map(|v| RockyTerrainVertexSsbo(packed_vertex(v)))
            .collect();
        let triangles: Vec<RockyTerrainTriangleSsbo> = data
            .triangles
            .iter()
            .map(|t| RockyTerrainTriangleSsbo(packed_triangle(t)))
            .collect();
        library.add_new_with_data(device, &vertices);
        library.add_new_with_data(device, &triangles);
        Ok(Self {
            triangle_groups: (triangles.len() / 3) as u32,
            vertices,
            triangles,
        })
    }
}

/// JSON 根结构（I：字段名待取证，按章节字段名登记）。
#[derive(Clone, Debug, Deserialize)]
pub struct RockyTerrainJson {
    pub vertices: Vec<LoadedRockyTerrainVertex>,
    pub triangles: Vec<LoadedRockyTerrainTriangle>,
}

/// 顶点 0x24 B → GPU 0x30 B 打包（I：源布局待取证，前 12 项透传）。
fn packed_vertex(v: &LoadedRockyTerrainVertex) -> [u32; 12] {
    let mut out = [0u32; 12];
    for (slot, value) in v.position.iter().chain(v.extra.iter()).enumerate() {
        if slot < 12 {
            out[slot] = f32::to_bits(*value);
        }
    }
    out
}

/// 三角形 0x10 B → GPU 0x20 B 打包（I：3 索引 + pad）。
fn packed_triangle(t: &LoadedRockyTerrainTriangle) -> [u32; 8] {
    let mut out = [0u32; 8];
    for (slot, value) in t.indices.iter().take(3).enumerate() {
        out[slot] = *value;
    }
    out
}
