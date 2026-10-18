//! 岩质地层网格的 JSON 加载、GPU 记录和共享顶点数资源。
//!
//! B 原证：load 0x1408bbf50（1292B），Vertex/Triangle typed buffer
//! constructors 0x141611ba0（579B）/0x1416144a0（531B）。源 JSON 记录
//! 分别为 36B/16B，GPU 记录为 48B/32B；保留当前类型化库和 load API。
use std::path::Path;

use bevy_ecs::prelude::*;
use serde::Deserialize;

use crate::asset_libraries::ssbo_library::{AssetSsboLibrary, SsboType};
use crate::render::ssbo::StorageElement;

/// Fixed capacities in the B typed buffer constructors.
pub const ROCKY_TERRAIN_VERTEX_CAPACITY: usize = 148_794;
pub const ROCKY_TERRAIN_TRIANGLE_CAPACITY: usize = 49_598;

/// Original JSON fields; source 36 bytes become a 48-byte GPU record.
#[derive(Clone, Copy, Debug, Deserialize)]
pub struct LoadedRockyTerrainVertex {
    /// 位置（+0）。
    pub position: [f32; 3],
    pub cell_bby: f32,
    pub cell_id: i32,
    pub dir_to_centroid: [f32; 3],
    pub is_corner: i32,
}

/// Original triangle adjacency fields, including signed boundary neighbours.
#[derive(Clone, Copy, Debug, Deserialize)]
pub struct LoadedRockyTerrainTriangle {
    pub neighbours: [i32; 3],
    pub is_top: i32,
}

/// 顶点 SSBO 元素（O：GPU 0x30 B）。
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default)]
pub struct RockyTerrainVertexSsbo(pub [u32; 12]);

unsafe impl bytemuck::Zeroable for RockyTerrainVertexSsbo {}
unsafe impl bytemuck::Pod for RockyTerrainVertexSsbo {}

impl StorageElement for RockyTerrainVertexSsbo {
    const NAME: &'static str = "Rocky Terrain Vertices";
    const BUFFER_LEN: usize = ROCKY_TERRAIN_VERTEX_CAPACITY;
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
    const BUFFER_LEN: usize = ROCKY_TERRAIN_TRIANGLE_CAPACITY;
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

/// Actual startup metadata consumed by both rocky displacement and drawing.
/// B Commands payload is one 64-bit vertex count, not the CPU mesh's layout.
/// Construct this from a successfully loaded mesh; no startup default is inferred.
#[repr(transparent)]
#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq)]
pub struct RockyTerrainMeshVertCount(pub usize);

/// The two live entities spawned by startup_terrain and borrowed by rendering.
/// Their identities come from the actual spawn results; there is no default.
#[derive(Resource, Clone, Copy, Debug)]
pub struct TerrainEntities {
    pub floor: Entity,
    pub rocky: Entity,
}

/// Keep the current CPU mesh/API for callers that inspect loaded data.
/// The original startup retains only RockyTerrainMeshVertCount after upload.
#[derive(Resource, Clone, Debug, Default)]
pub struct RockyTerrainMesh {
    /// Compatibility derived dispatch groups: loaded vertex count / 3.
    /// This field is not the original VertCount ECS resource.
    pub triangle_groups: u32,
    /// 顶点缓冲（GPU 0x30 B）。
    pub vertices: Vec<RockyTerrainVertexSsbo>,
    /// 三角形缓冲（GPU 0x20 B）。
    pub triangles: Vec<RockyTerrainTriangleSsbo>,
}

impl RockyTerrainMesh {
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Original startup keeps this metadata after the two successful uploads.
    /// Consuming the current CPU mesh drops its retained inspection vectors.
    pub fn into_vertex_count(self) -> RockyTerrainMeshVertCount {
        RockyTerrainMeshVertCount(self.vertices.len())
    }

    /// `RockyTerrainMesh::load`（0x1408bbf50，O 结构）：读 JSON → serde 解析 →
    /// 转换 GPU 记录 → `add_new_with_data` 注册两 SSBO。
    ///
    /// 错误类型按 fresh 0x1408bbf50 对齐：`anyhow::error::impl$1::from<
    /// std::io::error::Error>`（行 28）与 `from<serde_json::error::Error>`（行 33）——
    /// 原版 `anyhow::Result`，非 Rust 此前 `Result<_, io::Error>`（判等附注，已改）。
    pub fn load(
        path: &Path,
        library: &AssetSsboLibrary,
        device: &dyn crate::render::ssbo::rhapsody_boundary::GraphicsDevice,
    ) -> anyhow::Result<Self> {
        let text = std::fs::read_to_string(path)?;
        let data: RockyTerrainJson = serde_json::from_str(&text)?;
        let vertices: Vec<RockyTerrainVertexSsbo> = data
            .vertices
            .iter()
            .map(|v| RockyTerrainVertexSsbo(packed_vertex(v)))
            .collect();
        // Original publishes vertices before constructing/uploading triangles.
        library.add_new_with_data(device, &vertices);
        let triangles: Vec<RockyTerrainTriangleSsbo> = data
            .triangles
            .iter()
            .map(|t| RockyTerrainTriangleSsbo(packed_triangle(t)))
            .collect();
        library.add_new_with_data(device, &triangles);
        Ok(Self {
            triangle_groups: (vertices.len() / 3) as u32,
            vertices,
            triangles,
        })
    }
}

/// Root fields of the actual B assets/data/rocky_terrain.json.
#[derive(Clone, Debug, Deserialize)]
pub struct RockyTerrainJson {
    pub vertices: Vec<LoadedRockyTerrainVertex>,
    pub triangles: Vec<LoadedRockyTerrainTriangle>,
}

/// B load1408bc090: duplicate position; integer fields remain raw words.
fn packed_vertex(v: &LoadedRockyTerrainVertex) -> [u32; 12] {
    [
        v.position[0].to_bits(),
        v.position[1].to_bits(),
        v.position[2].to_bits(),
        v.cell_bby.to_bits(),
        v.position[0].to_bits(),
        v.position[1].to_bits(),
        v.position[2].to_bits(),
        v.cell_id as u32,
        v.dir_to_centroid[0].to_bits(),
        v.dir_to_centroid[1].to_bits(),
        v.dir_to_centroid[2].to_bits(),
        v.is_corner as u32,
    ]
}

/// B load1408bc1b0/214: four meaningful words then explicit 16-byte zero pad.
fn packed_triangle(t: &LoadedRockyTerrainTriangle) -> [u32; 8] {
    [
        t.neighbours[0] as u32,
        t.neighbours[1] as u32,
        t.neighbours[2] as u32,
        t.is_top as u32,
        0,
        0,
        0,
        0,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asset_libraries::ssbo_library::SsboId;
    use crate::render::ssbo::{rhapsody_boundary::BufferDesc, tests::MockDevice};
    use std::panic::{AssertUnwindSafe, catch_unwind};

    #[test]
    fn native_pack_duplicates_position_and_preserves_integer_and_float_bits() {
        let v = LoadedRockyTerrainVertex {
            position: [1.0, -0.0, f32::from_bits(0x7fc12345)],
            cell_bby: 2.0,
            cell_id: 0xfedcba98u32 as i32,
            dir_to_centroid: [-1.0, 0.0, 0.5],
            is_corner: 0x12345678,
        };
        assert_eq!(
            packed_vertex(&v),
            [
                0x3f800000, 0x80000000, 0x7fc12345, 0x40000000, 0x3f800000, 0x80000000, 0x7fc12345,
                0xfedcba98, 0xbf800000, 0, 0x3f000000, 0x12345678
            ]
        );
        assert_eq!(
            packed_triangle(&LoadedRockyTerrainTriangle {
                neighbours: [-1, i32::MIN, 7],
                is_top: 9,
            }),
            [0xffffffff, 0x80000000, 7, 9, 0, 0, 0, 0]
        );
    }

    #[test]
    fn json_integer_fields_use_native_signed_i32_and_pack_raw_words() {
        let vertex: LoadedRockyTerrainVertex = serde_json::from_str(
            r#"{"is_corner":-2147483648,"dir_to_centroid":[4,5,6],"cell_id":-1,"position":[1,2,3],"cell_bby":7}"#,
        ).unwrap();
        assert_eq!(vertex.cell_id, -1);
        assert_eq!(vertex.is_corner, i32::MIN);
        assert_eq!(packed_vertex(&vertex)[7], u32::MAX);
        assert_eq!(packed_vertex(&vertex)[11], 0x80000000);
        let triangle: LoadedRockyTerrainTriangle =
            serde_json::from_str(r#"{"is_top":-1,"neighbours":[-2147483648,0,2147483647]}"#)
                .unwrap();
        assert_eq!(
            packed_triangle(&triangle),
            [0x80000000, 0, 0x7fffffff, u32::MAX, 0, 0, 0, 0]
        );
        for out_of_range in [2147483648_i64, 4294967295_i64] {
            for field in ["cell_id", "is_corner"] {
                let mut invalid = serde_json::json!({"position":[0,0,0],"cell_bby":0,
                    "cell_id":0,"dir_to_centroid":[0,0,0],"is_corner":0});
                invalid[field] = serde_json::json!(out_of_range);
                assert!(serde_json::from_value::<LoadedRockyTerrainVertex>(invalid).is_err());
            }
            let invalid = serde_json::json!({"neighbours":[0,0,0],"is_top":out_of_range});
            assert!(serde_json::from_value::<LoadedRockyTerrainTriangle>(invalid).is_err());
        }
    }

    #[test]
    fn canonical_typed_buffers_keep_fixed_capacity_and_no_initial_upload() {
        let device = MockDevice::default();
        let vertex = RockyTerrainVertexSsbo::new_storage_buffer(&device, None);
        let triangle = RockyTerrainTriangleSsbo::new_storage_buffer(&device, None);
        assert_eq!(core::mem::size_of::<RockyTerrainVertexSsbo>(), 48);
        assert_eq!(core::mem::size_of::<RockyTerrainTriangleSsbo>(), 32);
        assert_eq!(core::mem::align_of::<RockyTerrainVertexSsbo>(), 4);
        assert_eq!(core::mem::align_of::<RockyTerrainTriangleSsbo>(), 4);
        let calls = device.created.borrow();
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0].0, BufferDesc::new(0x6cfae0, 0x22, 1).alignment(4));
        assert_eq!(calls[1].0, BufferDesc::new(0x1837c0, 0x22, 1).alignment(4));
        assert_eq!(calls[0].1, "Rocky Terrain Vertices");
        assert_eq!(calls[1].1, "Rocky Terrain Triangles");
        assert!(calls.iter().all(|call| call.2.is_none()));
        assert_eq!(vertex.buffer.byte_size, 0x6cfae0);
        assert_eq!(triangle.buffer.byte_size, 0x1837c0);
        assert_eq!(
            SsboId::of::<RockyTerrainVertexSsbo>().0.0,
            0x1c5c4204bf6436bb
        );
        assert_eq!(
            SsboId::of::<RockyTerrainTriangleSsbo>().0.0,
            0x2a397285cdc5c9db
        );
    }

    #[test]
    fn typed_initial_validation_happens_before_device_creation() {
        let device = MockDevice::default();
        let bad_vertex = [0u32; 13];
        let bad_triangle = [0u32; 9];
        let unaligned = [0u32; 24];
        let unaligned = &bytemuck::cast_slice(&unaligned)[1..49];
        for (vertex, bytes) in [
            (true, bytemuck::cast_slice(&bad_vertex)),
            (false, bytemuck::cast_slice(&bad_triangle)),
            (true, unaligned),
        ] {
            let failure = catch_unwind(AssertUnwindSafe(|| {
                if vertex {
                    let _ = RockyTerrainVertexSsbo::new_storage_buffer(&device, Some(bytes));
                } else {
                    let _ = RockyTerrainTriangleSsbo::new_storage_buffer(&device, Some(bytes));
                }
            }));
            assert!(failure.is_err());
            assert!(device.created.borrow().is_empty());
        }
        let too_many = vec![RockyTerrainVertexSsbo::default(); ROCKY_TERRAIN_VERTEX_CAPACITY + 1];
        assert!(
            catch_unwind(AssertUnwindSafe(|| {
                RockyTerrainVertexSsbo::new_storage_buffer(
                    &device,
                    Some(bytemuck::cast_slice(&too_many)),
                )
            }))
            .is_err()
        );
        let too_many =
            vec![RockyTerrainTriangleSsbo::default(); ROCKY_TERRAIN_TRIANGLE_CAPACITY + 1];
        assert!(
            catch_unwind(AssertUnwindSafe(|| {
                RockyTerrainTriangleSsbo::new_storage_buffer(
                    &device,
                    Some(bytemuck::cast_slice(&too_many)),
                )
            }))
            .is_err()
        );
        assert!(device.created.borrow().is_empty());
    }

    #[test]
    fn actual_b_asset_loads_through_the_real_library_and_native_record_layouts() {
        // This recovery regression intentionally reads the separately hash-bound
        // original B asset, not a fabricated DTO fixture or a GPU simulation.
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../../ljxsj_92385/Tiny Glade/assets/data/rocky_terrain.json");
        let library = AssetSsboLibrary::new();
        let device = MockDevice::default();
        let mesh = RockyTerrainMesh::load(&path, &library, &device).unwrap();
        assert_eq!(mesh.vertex_count(), 148794);
        assert_eq!(mesh.triangles.len(), 49598);
        assert_eq!(mesh.triangle_groups, 49598);
        assert_eq!(
            mesh.vertices[0].0,
            [
                1047544600, 1027906470, 1052294028, 1065353216, 1047544600, 1027906470, 1052294028,
                0, 1057341816, 0, 3210364733, 0
            ]
        );
        assert_eq!(
            mesh.vertices[1].0,
            [
                1047047523, 1027906470, 1052734662, 1065353216, 1047047523, 1027906470, 1052734662,
                0, 1057206576, 0, 3210446730, 1
            ]
        );
        assert_eq!(
            mesh.vertices[74397].0,
            [
                1062477650, 0, 1062806502, 0, 1062477650, 0, 1062806502, 337, 1058707185, 0,
                3209432533, 0
            ]
        );
        assert_eq!(
            mesh.vertices[148793].0,
            [
                1061393090, 1027906470, 1064637947, 1065353216, 1061393090, 1027906470, 1064637947,
                608, 3199635257, 0, 1064250481, 0
            ]
        );
        assert_eq!(mesh.triangles[0].0, [21, 38, 1, 1, 0, 0, 0, 0]);
        assert_eq!(mesh.triangles[1].0, [30, 0, 25, 1, 0, 0, 0, 0]);
        assert_eq!(
            mesh.triangles[24799].0,
            [24817, 24760, 24818, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            mesh.triangles[49597].0,
            [46639, 49562, 49580, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            mesh.triangles[36760].0,
            [4294967295, 36759, 36751, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            library
                .get::<RockyTerrainVertexSsbo>()
                .unwrap()
                .buffer
                .byte_size,
            0x6cfae0
        );
        assert_eq!(
            library
                .get::<RockyTerrainTriangleSsbo>()
                .unwrap()
                .buffer
                .byte_size,
            0x1837c0
        );
        let calls = device.created.borrow();
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0].1, "Rocky Terrain Vertices");
        assert_eq!(calls[1].1, "Rocky Terrain Triangles");
        assert_eq!(
            calls[0].2.as_deref().unwrap(),
            bytemuck::cast_slice::<_, u8>(&mesh.vertices)
        );
        assert_eq!(
            calls[1].2.as_deref().unwrap(),
            bytemuck::cast_slice::<_, u8>(&mesh.triangles)
        );
        assert_eq!(calls[0].0, BufferDesc::new(0x6cfae0, 0x22, 1).alignment(4));
        assert_eq!(calls[1].0, BufferDesc::new(0x1837c0, 0x22, 1).alignment(4));
        assert_eq!(mesh.into_vertex_count(), RockyTerrainMeshVertCount(148794));
        assert_eq!(core::mem::size_of::<RockyTerrainMeshVertCount>(), 8);
    }
}
