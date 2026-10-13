//! rocky_terrain.rs — 岩质地层/高度导数数据（WP8 恢复）。
//!
//! 源：B skeleton `resources/rocky_terrain.rs`（pdb_sha256 2d9686b8…）。
//! 命名实体（PDB structures）：`RockyTerrainMesh`（资源）、`LoadedRockyTerrainVertex` /
//! `LoadedRockyTerrainTriangle`（JSON 中间结构）、`RockyTerrainVertexSSBO` / `RockyTerrainTriangleSSBO`
//! （SSBO 标记）、`RockyTerrainMeshVertCount`、`TerrainEntities`。
//! 反编译：`RockyTerrainMesh::load`@0x1408bbf50（行 34：读文件→JSON→两个 SSBO `add_new_with_data`）；
//! `displace_rocky_terrain`@0x140a3ee80（W4：写两个 SSBO + 5 纹理 + dispatch tri=uVar2/3）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::resource::Resource;
use serde::{Deserialize, Serialize};

use crate::typed_gap::Gap;

/// 岩体顶点 SSBO 标记类型（O：`displace_rocky_terrain` `write_storage_buffer`）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct RockyTerrainVertexSSBO;

/// 岩体三角形 SSBO 标记类型（O：`displace_rocky_terrain` `write_storage_buffer`）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct RockyTerrainTriangleSSBO;

/// 岩体顶点数 newtype（O 名）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RockyTerrainMeshVertCount(pub u32);

/// JSON 加载的岩体顶点（O 名；字段布局 U）。源记录 0x24B（反编译观察）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LoadedRockyTerrainVertex {
    /// typed gap：{位置, 法线, …} 字段 U。
    pub _gap: [f32; 9],
}

/// JSON 加载的岩体三角形（O 名；字段布局 U）。源记录 0x10B（反编译观察）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LoadedRockyTerrainTriangle {
    /// typed gap：{顶点索引} 字段 U。
    pub _gap: [u32; 4],
}

/// 岩质地层网格资源（O 名；`RockyTerrainMesh`）。
/// 布局 U → typed gap。load 后持顶点/三角形数据（O 流程）。
#[derive(Resource, Clone, Debug, Default)]
pub struct RockyTerrainMesh {
    /// typed gap：{顶点列表, 三角形列表, 顶点计数} 字段 U。
    pub _gap: Gap,
}

/// 地形实体查询集合（O 名；`read_terrain_data` 绘制地形/岩体用）。内部 U。
#[derive(Clone, Copy, Debug, Default)]
pub struct TerrainEntities(pub Gap);
