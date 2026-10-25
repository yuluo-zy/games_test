//! `resources::vegetation` — garden/grass/植被 CPU 数据（帧 pass 消费的生产者）。
//!
//! 接线（WP5 消费侧 + 本包生产侧，I/O 见契约 `vegetation-systems.md`）：
//!   - `TerrainHeightsData<T>` → `bind_terrain_heightmap`：grass_compute(W29)、garden_spawn(W16)、
//!     spawn_vegetation_around_buildings(W14)、water_spawn(W13)、path_pebbles(W17)。
//!   - `GardenMaskSsbo`/`GardenRaster` → garden_rasterize_from_cpu(W12)、garden_spawn_vegetation(W16)。
//!   - `WaterExcludeClutterMaskSsbo` → foliage exclusion(W7)/水植被（I）。
//!   - `LoadedParticleEmitterCount` + `ParticleEmitterSsbo` → leaf_particles_compute(W18)。
//!   - 灌木/花/草 SSBO 为 nani 实例数据域（wp3 `PlantInstanceData`），引用不重定义。

#![allow(dead_code, unused_imports)]

use bevy_ecs::resource::Resource;
use glam::Vec3;

use crate::typed_gap::Gap;

/// 地形高度数据（O 名/方法；骨架 `resources/terrain_height.rs`）。
/// 泛型 T = SSBO 载体（`TerrainHeightSsbo`/`VisualTerrainHeightSsbo`）。字段布局 U（typed gap）。
#[derive(Resource)]
pub struct TerrainHeightsData<T> {
    /// SSBO 载体句柄（布局 U）。
    pub ssbo: T,
    /// CPU 副本（布局 U）。
    pub _gap: Gap,
}

impl<T: Default> Default for TerrainHeightsData<T> {
    fn default() -> Self {
        TerrainHeightsData {
            ssbo: T::default(),
            _gap: Gap,
        }
    }
}

impl<T> TerrainHeightsData<T> {
    /// `TerrainHeightsData::sample_at_world_xz`（O 名；双线性采样地形高度）。
    pub fn sample_at_world_xz(&self, _x: f32, _z: f32) -> f32 {
        0.0
    }

    /// `TerrainHeightsData::heightfield_intersect_ray`（O 名；射线求交高度场）。
    pub fn heightfield_intersect_ray(&self, _origin: Vec3, _dir: Vec3) -> Option<f32> {
        None
    }

    /// `TerrainHeightsData::copy_from_gpu<T>`（O 名；GPU 回读 CPU）。
    pub fn copy_from_gpu(&mut self) {
        // rhapsody 回读面（复用边界）；内部实现 U。
    }

    /// `TerrainHeightsData::subscribe`（O 名；地形变化订阅）。
    pub fn subscribe(&mut self) {
    }

    /// `TerrainHeightsData::height_range` / `as_bytes` / `as_raw` / `as_mut_raw`（O 名；U 实现）。
    pub fn height_range(&self) -> (f32, f32) {
        (0.0, 0.0)
    }
}

/// 花园掩码 SSBO（O 名；`resources/garden_mask.rs`，512 掩码，布局 U）。
#[derive(Clone, Copy, Debug, Default)]
pub struct GardenMaskSsbo(pub Gap);

/// 水面排除灌木掩码 SSBO（O 名；`resources/water_exclude_clutter_mask.rs`，布局 U）。
#[derive(Clone, Copy, Debug, Default)]
pub struct WaterExcludeClutterMaskSsbo(pub Gap);

/// 花园 CPU raster（O 名；`resources/world_raster.rs::GardenRaster`，WP6 边界；布局 U）。
#[derive(Clone, Copy, Debug, Default)]
pub struct GardenRaster(pub Gap);

/// 灌木 SSBO（nani 实例数据域，O 名；布局 U，不重定义 wp3 PlantInstanceData）。
#[derive(Clone, Copy, Debug, Default)]
pub struct BushSsbo(pub Gap);

/// 花 SSBO（O 名；布局 U）。
#[derive(Clone, Copy, Debug, Default)]
pub struct FlowerSsbo(pub Gap);

/// 小型花 SSBO（O 名；布局 U）。
#[derive(Clone, Copy, Debug, Default)]
pub struct Flower03Ssbo(pub Gap);

/// 花园石 SSBO（O 名；布局 U）。
#[derive(Clone, Copy, Debug, Default)]
pub struct GardenStoneSsbo(pub Gap);

/// 灌木 CPU 回读 SSBO（O 名；`read_bush_positions_for_cpu.rs`（W15），布局 U）。
#[derive(Clone, Copy, Debug, Default)]
pub struct BushCpuSsbo(pub Gap);

/// 树叶粒子 SSBO（O 名；`leaf_particles.rs`（W18），布局 U）。
#[derive(Clone, Copy, Debug, Default)]
pub struct LeafParticleSsbo(pub Gap);

/// 粒子发射器 SSBO（O 名；布局 U）。
#[derive(Clone, Copy, Debug, Default)]
pub struct ParticleEmitterSsbo(pub Gap);

/// 已加载粒子发射器计数 CPU 资源（O 名；定义于 `startup/mod.rs`；`leaf_particles_compute` 经
/// `utils::get_res` 读取）。
#[derive(Resource, Default)]
pub struct LoadedParticleEmitterCount {
    pub count: u64,
}

/// 草叶数据 SSBO（O 名；`grass.rs`（W29），布局 U）。
#[derive(Clone, Copy, Debug, Default)]
pub struct GrassBladeDataSsbo(pub Gap);

/// 草 push 常量（O 名；`object_grass_push.rs`（W9）+ grass 共享，布局 U）。
#[derive(Clone, Copy, Debug, Default)]
pub struct ObjectGrassPush(pub Gap);
