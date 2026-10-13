//! terrain_height.rs — 权威地形高度场资源（WP8 恢复）。
//!
//! 源：B skeleton `resources/terrain_height.rs`（pdb_sha256 2d9686b8…，source_sha256 1227dfdd…）。
//! 命名实体（PDB）：`TerrainHeightsData<T>`（泛型资源，两次实例化：`TerrainHeightSsbo` 权威 /
//! `VisualTerrainHeightSsbo` 视觉）、`TerrainHeightFloatGrid`（`utils::float_grid` 网格）。
//! 方法行号（PDB inlinee）：`impl$4/5::default`@96/107、`sample_at_world_xz`@118、`intersect_ray`@126、
//! `heightfield_intersect_ray`@161、`subscribe`@243、`copy_from_gpu`@247、`as_raw`@321、`as_mut_raw`@325、
//! `as_bytes`@330、`height_range`@334、`impl$7::load_index_unchecked`@347。
//! 反编译证据：`insert_terrain_resources`@0x1409457c0 插入两个实例；`mouse_terrain_raycast`@0x1408ef320
//! 调用 `intersect_ray<TerrainHeightSsbo>`；`delayed_terrain_changed_msg`@0x140aabdd0 调用 `copy_from_gpu`。
//! 业务边界：512×512 网格、高度范围 -2.5..24.0（prior `tg-terrain::schema`，O 原机器码）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::resource::Resource;
use glam::Vec3;
use std::marker::PhantomData;

use crate::typed_gap::Gap;

/// 权威高度场 SSBO 标记类型（O：`insert_resource<TerrainHeightsData<TerrainHeightSsbo>>`）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct TerrainHeightSsbo;

/// 视觉高度场 SSBO 标记类型（O：`insert_resource<TerrainHeightsData<VisualTerrainHeightSsbo>>`）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct VisualTerrainHeightSsbo;

/// 高度场浮点网格（O：`utils::float_grid::bilinear_sample_at_world_xz<TerrainHeightFloatGrid>`）。
/// 布局 U（数据指针/范围/格式）→ typed gap。
#[derive(Clone, Debug, Default)]
pub struct TerrainHeightFloatGrid {
    /// typed gap：{数据指针/网格, 世界范围, 尺寸} 字段 U。
    pub _gap: Gap,
}

/// 地形高度数据资源（O 名；泛型 `T` 区分权威/视觉 SSBO 类型）。
/// 字段布局 U（PDB `name_and_relations_only`）→ typed gap。
#[derive(Resource)]
pub struct TerrainHeightsData<T> {
    /// typed gap：{浮点网格, GPU SSBO 句柄, 订阅表, 脏范围, RT 三角形元数据} 字段 U。
    pub _gap: Gap,
    pub _marker: PhantomData<fn() -> T>,
}

impl<T> Default for TerrainHeightsData<T> {
    fn default() -> Self {
        Self {
            _gap: Gap,
            _marker: PhantomData,
        }
    }
}

impl<T> TerrainHeightsData<T> {
    /// 世界 XZ → 高度采样（O 名；`stairs_assemble` 等消费者；语义 `utils::float_grid` 双线性采样，I）。
    pub fn sample_at_world_xz(&self, _x: f32, _z: f32) -> f32 {
        unimplemented!("recover sample_at_world_xz via utils::float_grid")
    }

    /// 高度场射线求交（O 名；`mouse_terrain_raycast` 调用，命中→{pos, dist}，I）。
    pub fn intersect_ray(&self, _origin: Vec3, _dir: Vec3) -> Option<(Vec3, f32)> {
        unimplemented!("recover intersect_ray")
    }

    /// 高度场求交核心（O 名；`intersect_ray` 内部）。
    pub fn heightfield_intersect_ray(&self, _origin: Vec3, _dir: Vec3) -> Option<(Vec3, f32)> {
        unimplemented!("recover heightfield_intersect_ray")
    }

    /// 订阅地形变化（O 名；`system_wall_constructor::construct_elevation_supports` 消费）。
    /// 返回订阅句柄语义 U。
    pub fn subscribe(&self) -> Gap {
        Gap
    }

    /// GPU → CPU 高度回拷（O 名；`delayed_terrain_changed_msg` 调用）。
    /// prior 语义：512×512 u16 编码 + 最小/最大高度；脏块 8×8 位图（I）。
    pub fn copy_from_gpu(&mut self, _readback: &[u16]) -> Gap {
        Gap
    }

    /// 原始字节视图（O 名；save snapshot load 消费）。
    pub fn as_raw(&self) -> &[u8] {
        &[]
    }

    /// 原始可变字节视图（O 名；save snapshot load 消费）。
    pub fn as_mut_raw(&mut self) -> &mut [u8] {
        &mut []
    }

    /// 字节视图（O 名；save snapshot capture 消费）。
    pub fn as_bytes(&self) -> &[u8] {
        &[]
    }

    /// 高度范围 [min_y, max_y]（O 名；save snapshot capture 消费）。
    pub fn height_range(&self) -> (f32, f32) {
        (-2.5, 24.0)
    }
}

/// `TerrainHeightFloatGrid` 索引访问（O 名，`impl$7::load_index_unchecked`@347）。
impl TerrainHeightFloatGrid {
    pub fn load_index_unchecked(&self, _index: usize) -> f32 {
        unimplemented!("recover load_index_unchecked")
    }
}
