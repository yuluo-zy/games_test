//! terrain_height.rs — `TerrainHeightsData<T>`：512×512 的 u16 地形高度场的 CPU 镜像。
//!
//! 偏移（`sample_at_world_xz` 0x1409e9640 / `copy_from_gpu` 0x1409ea020）：
//!
//! | 偏移 | 字段 |
//! |---|---|
//! | +0x00/+0x08/+0x10 | 高度数据 `Vec<u16>`（cap/ptr/len），长度 0x40000 |
//! | +0x18 | `Publisher<DirtyTiles>`（56 B，`copy_from_gpu` 直接对它 `publish`） |
//! | +0x50 / +0x54 | `min_height` / `max_height`（f32），默认 -2.5 / 24.0 |
//!
//! 与 DIGEST 的两处不同（EXE 字节为准）：
//! 1. `sample_at_world_xz` 返回的是**双线性插值后的 f32 高度**，不是 u16 原始值：4 个角各自按
//!    `u16 / 65535` 解码成 `v * max + (1 - v) * min`（0x1408b54c0），再沿 z、再沿 x 做 `a + (b - a) * t`。
//!    居中/向量两条采样路径由全局字节 0x14362c308 指向的开关选择，与 `utils::float_grid` 共用。
//! 2. `intersect_ray` 是 32 步行进 + **7** 次二分 + 末次选择，不是 3 次。

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::neg_cmp_op_on_partial_ord)]

use std::marker::PhantomData;

use glam::Vec3;
use tabloid::{Publisher, Subscriber};
use utils::float_grid::{FloatGrid, FloatGridSampler, CENTERED_PATH, DEFAULT_WORLD_EXTENT};
use utils::geometry::{Aabb3, Plane, Ray, RayBoxIntersection};

use crate::utils::dirty_tiles::{DirtyTiles, GRID_SIZE, TILE_SIZE};

/// B readback SSBOs pack two u16 heights in each u32 (0x20000 elements).
/// They are also the distinct identity parameters of the two CPU height owners.
macro_rules! height_readback_ssbo {
    ($($name:ident),+ $(,)?) => {$(
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, Default)]
        pub struct $name(pub u32);
        unsafe impl bytemuck::Zeroable for $name {}
        unsafe impl bytemuck::Pod for $name {}
        impl crate::render::ssbo::StorageElement for $name {
            const NAME: &'static str = stringify!($name);
            const BUFFER_LEN: usize = 0x20000;
            const MAPPED_MEMORY: u8 = crate::render::ssbo::MAPPED_MEMORY_READBACK;
        }
        impl crate::asset_libraries::ssbo_library::SsboType for $name {
            type Storage = crate::render::ssbo::MappedStorageBuffer<'static, Self>;
            fn new_storage_buffer(
                device: &dyn crate::render::ssbo::rhapsody_boundary::GraphicsDevice,
                initial: Option<&[u8]>,
            ) -> Self::Storage {
                crate::render::ssbo::MappedStorageBuffer::new_storage_buffer(device, initial)
            }
        }
    )+};
}
height_readback_ssbo!(TerrainHeightSsbo, VisualTerrainHeightSsbo);

/// 网格边长（格）。
pub const GRID_SIDE: usize = 512;
/// 默认高度范围。O：0x1429df1c0 = -2.5、0x142934600 = 24.0，同一对值在 0x142a9f3f0。
pub const DEFAULT_MIN_HEIGHT: f32 = -2.5;
pub const DEFAULT_MAX_HEIGHT: f32 = 24.0;
/// `u16` 解码除数。O：0x142a7cd4c。
const U16_SCALE: f32 = 65535.0;
/// `intersect_ray` 的包围盒：x/z ∈ [-65, 65]（0x142ac4cb0 起的 -65/65），
/// y ∈ [min - 0.001, max + 0.001]（0x1429cd038 / 0x1429cd060）。
const WORLD_HALF_EXTENT: f32 = 65.0;
const AABB_Y_MARGIN: f32 = 0.001;
/// 行进步数 32，步长 `(t1 - t0) * 0.03125`（0x142ac4cc0）。
const MARCH_STEPS: u32 = 32;
const MARCH_STEP_SCALE: f32 = 0.03125;
/// 行进命中后的二分次数（7 个完整迭代，随后一个选择用的第 8 次采样）。
const BISECTION_STEPS: u32 = 7;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TerrainRayHit {
    pub t: f32,
    pub position: Vec3,
}

pub struct TerrainHeightsData<T = TerrainHeightSsbo> {
    heights: Vec<u16>,
    publisher: Publisher<DirtyTiles>,
    min_height: f32,
    max_height: f32,
    marker: PhantomData<T>,
}
impl<T: Send + Sync + 'static> bevy_ecs::resource::Resource for TerrainHeightsData<T> {}

impl<T> TerrainHeightsData<T> {
    /// 全零网格 + 默认高度范围。原版的构造点未恢复（U）；这里只提供一个可用起点。
    pub fn new() -> Self {
        Self {
            heights: vec![0; GRID_SIDE * GRID_SIDE],
            publisher: Publisher::new(),
            min_height: DEFAULT_MIN_HEIGHT,
            max_height: DEFAULT_MAX_HEIGHT,
            marker: PhantomData,
        }
    }

    pub fn heights(&self) -> &[u16] {
        &self.heights
    }

    pub fn heights_mut(&mut self) -> &mut [u16] {
        &mut self.heights
    }

    pub fn min_height(&self) -> f32 {
        self.min_height
    }

    pub fn max_height(&self) -> f32 {
        self.max_height
    }

    pub fn set_height_range(&mut self, min_height: f32, max_height: f32) {
        self.min_height = min_height;
        self.max_height = max_height;
    }

    pub fn subscribe(&mut self) -> Subscriber<DirtyTiles> {
        self.publisher.subscribe()
    }

    /// O：0x1408b54c0。`v = u16 / 65535`，高度 = `v * max + (1 - v) * min`。
    pub fn load_index_unchecked(&self, index: usize) -> f32 {
        let v = f32::from(self.heights[index]) / U16_SCALE;
        v * self.max_height + (1.0 - v) * self.min_height
    }

    /// O：0x1409e9640。路径开关取 [`CENTERED_PATH`]。
    pub fn sample_at_world_xz(&self, x: f32, z: f32) -> f32 {
        self.sample_at_world_xz_with_mode(
            x,
            z,
            CENTERED_PATH.load(std::sync::atomic::Ordering::Relaxed),
        )
    }

    /// `centered`：居中采样（原版 0x14362c308 指向的字节非零）。
    pub fn sample_at_world_xz_with_mode(&self, x: f32, z: f32, centered: bool) -> f32 {
        let extent = glam::Vec2::splat(DEFAULT_WORLD_EXTENT);
        View(self).bilinear_sample_at_world_xz(glam::Vec2::new(x, z), extent, centered)
    }

    /// O：0x1409ea020。
    ///
    /// 位图先清零；`min/max` 与默认值任一不等（NaN 也算不等）→ 复位并把位图 `memset(0xff)`、标记已变化。
    /// 随后按 64 个瓦片行 × 8 个像素行 × 64 个瓦片列，以 16 字节（一个瓦片内的一行 8 个 u16）为单位比较，
    /// 不同则拷贝并在该瓦片行的位图字里置第 `tx` 位。只要有变化就向订阅者发布位图。
    pub fn copy_from_gpu(&mut self, gpu: &[u16]) {
        assert_eq!(gpu.len(), GRID_SIDE * GRID_SIDE, "gpu height grid extent");
        let mut changed_tiles = DirtyTiles::new_clear();
        let mut changed = false;
        if self.min_height != DEFAULT_MIN_HEIGHT || self.max_height != DEFAULT_MAX_HEIGHT {
            self.min_height = DEFAULT_MIN_HEIGHT;
            self.max_height = DEFAULT_MAX_HEIGHT;
            changed_tiles.set_all();
            changed = true;
        }
        let mut rows = *changed_tiles.rows();
        for (tile_row, row_bits) in rows.iter_mut().enumerate() {
            for sub_row in 0..TILE_SIZE {
                let pixel_row = (tile_row * TILE_SIZE + sub_row) * GRID_SIDE;
                for tile_column in 0..GRID_SIZE {
                    let start = pixel_row + tile_column * TILE_SIZE;
                    let range = start..start + TILE_SIZE;
                    if gpu[range.clone()] != self.heights[range.clone()] {
                        self.heights[range.clone()].copy_from_slice(&gpu[range]);
                        *row_bits |= 1u64 << tile_column;
                        changed = true;
                    }
                }
            }
        }
        if changed {
            self.publisher
                .publish(DirtyTiles::from_rows(rows))
                .expect("tabloid publication sequence exhausted");
        }
    }

    /// O：0x1409e9a20。`t_max` 为射线最大距离。
    pub fn intersect_ray(&self, ray: Ray, t_max: f32) -> Option<TerrainRayHit> {
        self.intersect_ray_with_mode(
            ray,
            t_max,
            CENTERED_PATH.load(std::sync::atomic::Ordering::Relaxed),
        )
    }

    pub fn intersect_ray_with_mode(
        &self,
        ray: Ray,
        t_max: f32,
        centered: bool,
    ) -> Option<TerrainRayHit> {
        let terrain = self.march_terrain(ray, t_max, centered);
        // 0x1412803f0：与 y = 0 平面求交（min_t = 0）；`t <= t_max` 且命中点落在 ±65 之外才算数。
        // `ucomiss t_max, t; jb` 在无序（NaN）时同样跳过，所以这里用有序的 `t_max >= t`。
        let plane = Plane {
            point: Vec3::ZERO,
            normal: Vec3::Y,
        }
        .intersect_ray(ray, 0.0)
        .filter(|&(t, position)| {
            t_max >= t
                && (position.x.abs() > WORLD_HALF_EXTENT || position.z.abs() > WORLD_HALF_EXTENT)
        })
        .map(|(t, position)| TerrainRayHit { t, position });
        match (terrain, plane) {
            // `ucomiss plane_t, terrain_t; jbe` → 平面不比地形远（含 NaN）就取平面。
            (Some(terrain), Some(plane)) => Some(if plane.t > terrain.t { terrain } else { plane }),
            (terrain, plane) => terrain.or(plane),
        }
    }

    fn march_terrain(&self, ray: Ray, t_max: f32, centered: bool) -> Option<TerrainRayHit> {
        let aabb = Aabb3 {
            min: Vec3::new(
                -WORLD_HALF_EXTENT,
                self.min_height + -AABB_Y_MARGIN,
                -WORLD_HALF_EXTENT,
            ),
            max: Vec3::new(
                WORLD_HALF_EXTENT,
                self.max_height + AABB_Y_MARGIN,
                WORLD_HALF_EXTENT,
            ),
        };
        let (enter, exit) = match aabb.classify_ray_intersection(ray) {
            RayBoxIntersection::NoHit => return None,
            // `maxss enter, 0`：NaN 取 0。
            RayBoxIntersection::Inside { enter, exit } => {
                (if enter > 0.0 { enter } else { 0.0 }, exit)
            }
            RayBoxIntersection::Hit { enter, exit } => (enter, exit),
        };
        if enter > t_max {
            return None;
        }
        // `minss t_max, exit`，exit 为 NaN 时取 t_max。
        let end = if exit.is_nan() || t_max < exit {
            t_max
        } else {
            exit
        };
        let step = (end - enter) * MARCH_STEP_SCALE;
        let (origin, direction) = (ray.origin, ray.direction);
        let height_above = |t: f32| {
            let y = direction.y * t + origin.y;
            let sampled = self.sample_at_world_xz_with_mode(
                direction.x * t + origin.x,
                direction.z * t + origin.z,
                centered,
            );
            (y, sampled)
        };

        let mut lo = enter;
        let mut hi = None;
        for _ in 0..MARCH_STEPS {
            let t = step + lo;
            let (y, sampled) = height_above(t);
            if sampled > y {
                hi = Some(t);
                break;
            }
            lo = t;
        }
        let mut hi = hi?;
        if !(hi > lo) {
            return None;
        }
        for _ in 0..BISECTION_STEPS {
            let mid = (lo + hi) * 0.5;
            let (y, sampled) = height_above(mid);
            if y < sampled {
                hi = mid;
            }
            if !(sampled > y) {
                lo = mid;
            }
        }
        let mid = (hi + lo) * 0.5;
        let (y, sampled) = height_above(mid);
        let t = if y < sampled { lo } else { mid };
        Some(TerrainRayHit {
            t,
            position: Vec3::new(
                origin.x + direction.x * t,
                origin.y + direction.y * t,
                direction.z * t + origin.z,
            ),
        })
    }
}

impl<T> Default for TerrainHeightsData<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// `utils::float_grid` 的 `FloatGrid` 视图：解码后的 f32 高度。
struct View<'a, T>(&'a TerrainHeightsData<T>);

impl<T> FloatGrid for View<'_, T> {
    fn dimensions(&self) -> [usize; 2] {
        [GRID_SIDE, GRID_SIDE]
    }

    fn load_index_unchecked(&self, index: usize) -> f32 {
        self.0.load_index_unchecked(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn flat(raw: u16) -> TerrainHeightsData {
        let mut data = TerrainHeightsData::new();
        data.heights_mut().fill(raw);
        data
    }

    #[test]
    fn defaults_and_decode_endpoints_match_the_exe() {
        let mut data = TerrainHeightsData::<TerrainHeightSsbo>::new();
        assert_eq!(data.min_height(), -2.5);
        assert_eq!(data.max_height(), 24.0);
        data.heights_mut()[0] = 0;
        data.heights_mut()[1] = 65535;
        assert_eq!(data.load_index_unchecked(0), -2.5);
        assert_eq!(data.load_index_unchecked(1), 24.0);
    }

    #[test]
    fn decode_uses_v_times_max_plus_one_minus_v_times_min() {
        let mut data = TerrainHeightsData::<TerrainHeightSsbo>::new();
        data.set_height_range(1.0, 5.0);
        data.heights_mut()[7] = 32768;
        let v = 32768.0_f32 / 65535.0;
        assert_eq!(data.load_index_unchecked(7), v * 5.0 + (1.0 - v) * 1.0);
    }

    #[test]
    fn flat_terrain_samples_the_decoded_height_on_both_paths() {
        let data = flat(65535);
        for centered in [false, true] {
            assert_eq!(data.sample_at_world_xz_with_mode(3.0, -7.0, centered), 24.0);
            // 世界之外的坐标被钳制，仍是同一个高度。
            assert_eq!(data.sample_at_world_xz_with_mode(1e9, -1e9, centered), 24.0);
        }
    }

    #[test]
    fn sample_interpolates_between_adjacent_columns() {
        let mut data = TerrainHeightsData::<TerrainHeightSsbo>::new();
        data.set_height_range(0.0, 65535.0);
        for z in 0..GRID_SIDE {
            for x in 0..GRID_SIDE {
                data.heights_mut()[z * GRID_SIDE + x] = x as u16 * 100;
            }
        }
        // 世界 x 使居中路径的 texel = 256.5：x = (256.5 + 0.5) / 512 - 0.5) * 130
        let x = ((257.0_f32 / 512.0) - 0.5) * 130.0;
        let h = data.sample_at_world_xz_with_mode(x, 0.0, true);
        assert!((h - 25650.0).abs() < 0.5, "{h}");
    }

    #[test]
    fn copy_from_gpu_flags_only_the_changed_tiles_and_publishes() {
        let mut data = TerrainHeightsData::<TerrainHeightSsbo>::new();
        let mut subscriber = data.subscribe();
        let mut gpu = vec![0u16; GRID_SIDE * GRID_SIDE];
        // 像素 (x = 9, y = 17) → 瓦片 (1, 2)。
        gpu[17 * GRID_SIDE + 9] = 1234;
        data.copy_from_gpu(&gpu);
        assert_eq!(data.heights()[17 * GRID_SIDE + 9], 1234);
        let batch = subscriber.take().unwrap();
        assert_eq!(batch.tile_indices().collect::<Vec<_>>(), [2 * 64 + 1]);
        // 再拷一次：没有变化，不发布。
        data.copy_from_gpu(&gpu);
        assert!(subscriber.take().is_none());
    }

    #[test]
    fn copy_from_gpu_with_non_default_range_resets_it_and_dirties_everything() {
        let mut data = TerrainHeightsData::<TerrainHeightSsbo>::new();
        data.set_height_range(0.0, 10.0);
        let mut subscriber = data.subscribe();
        data.copy_from_gpu(&vec![0u16; GRID_SIDE * GRID_SIDE]);
        assert_eq!(data.min_height(), -2.5);
        assert_eq!(data.max_height(), 24.0);
        assert_eq!(subscriber.take().unwrap().tile_indices().count(), 4096);
    }

    #[test]
    fn nan_height_range_counts_as_non_default() {
        let mut data = TerrainHeightsData::<TerrainHeightSsbo>::new();
        data.set_height_range(f32::NAN, DEFAULT_MAX_HEIGHT);
        let mut subscriber = data.subscribe();
        data.copy_from_gpu(&vec![0u16; GRID_SIDE * GRID_SIDE]);
        assert_eq!(data.min_height(), -2.5);
        assert!(subscriber.take().is_some());
    }

    #[test]
    fn straight_down_ray_hits_flat_terrain_at_its_height() {
        // raw 0 → 高度 -2.5。
        let data = flat(0);
        let ray = Ray {
            origin: Vec3::new(1.0, 10.0, 2.0),
            direction: Vec3::NEG_Y,
        };
        let hit = data.intersect_ray_with_mode(ray, 100.0, false).unwrap();
        assert!((hit.position.y - -2.5).abs() < 1e-3, "{hit:?}");
        assert!((hit.position.x - 1.0).abs() < 1e-6 && (hit.position.z - 2.0).abs() < 1e-6);
        assert!((hit.t - 12.5).abs() < 1e-3);
    }

    #[test]
    fn upward_ray_misses_and_short_t_max_misses() {
        let data = flat(0);
        let up = Ray {
            origin: Vec3::new(0.0, 10.0, 0.0),
            direction: Vec3::Y,
        };
        assert_eq!(data.intersect_ray_with_mode(up, 100.0, false), None);
        let down = Ray {
            origin: Vec3::new(0.0, 10.0, 0.0),
            direction: Vec3::NEG_Y,
        };
        assert_eq!(data.intersect_ray_with_mode(down, 5.0, false), None);
    }

    #[test]
    fn ray_outside_the_box_falls_back_to_the_y_zero_plane() {
        let data = flat(0);
        // 起点在 x = 100，已在 ±65 之外；斜向下，落在 y = 0 平面上也在 ±65 之外。
        let ray = Ray {
            origin: Vec3::new(100.0, 10.0, 0.0),
            direction: Vec3::new(0.0, -1.0, 0.0),
        };
        let hit = data.intersect_ray_with_mode(ray, 100.0, false).unwrap();
        assert_eq!(hit.t, 10.0);
        assert_eq!(hit.position, Vec3::new(100.0, 0.0, 0.0));
    }

    #[test]
    fn nan_t_max_skips_the_plane_hit_like_ucomiss_jb() {
        let data = flat(0);
        let ray = Ray {
            origin: Vec3::new(100.0, 10.0, 0.0),
            direction: Vec3::NEG_Y,
        };
        assert_eq!(data.intersect_ray_with_mode(ray, f32::NAN, false), None);
    }

    #[test]
    fn plane_hit_inside_the_box_is_ignored() {
        // 高地：raw 全满 → 24；射线从 30 高度下探，终点会停在地形上方而不是 y=0 平面。
        let data = flat(65535);
        let ray = Ray {
            origin: Vec3::new(0.0, 30.0, 0.0),
            direction: Vec3::NEG_Y,
        };
        let hit = data.intersect_ray_with_mode(ray, 100.0, false).unwrap();
        assert!((hit.position.y - 24.0).abs() < 1e-3, "{hit:?}");
    }
}
