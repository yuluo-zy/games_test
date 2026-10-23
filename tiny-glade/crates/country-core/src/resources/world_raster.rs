//! world_raster.rs — 世界光栅缓冲族。
//!
//! 源 SHA256（B skeleton）`resources/world_raster.rs`；PDB 命名实体：
//! `WorldRaster` / `RasterBufferRef` / `DirtyTilePublisher` / `StrokeBlendMode` /
//! `UploadWorldRasterBuffersToStagingCmd`。
//!
//! `WorldRaster<512>` 的字段由 `new`（0x140a15530）、`copy_to_staging`（0x140a172d0）与
//! `DirtyTiles::retain<…>`（0x140adfcb0）逐条核对 EXE 字节得到，见 [`WorldRaster`]。
//! `rasterize_stroke`（0x140a16960）见 [`WorldRaster::rasterize_stroke`]；
//! `rasterize_world_space_stroke`（conservative_line_raster 依赖）与存档 serde 另行落码，这里不包含。
//! `RasterBufferRef<512>` 是 512×512 光栅的引用（O：preplaced_flower 反编译
//! `bilinear_sample_at_world_xz<resources::world_raster::RasterBufferRef<512>>`）。

use std::hash::{BuildHasher, Hasher};
use std::sync::Arc;

use bevy_ecs::resource::Resource;
use glam::Vec3;
use parking_lot::Mutex;
use tabloid::{Publisher, Subscriber};
use zeno::{Cap, Command, Join, Mask, Point, Stroke};

use crate::utils::dirty_tiles::{DirtyTiles, GRID_SIZE, TILE_SIZE};

/// Original event owner, also consumed by the world-raster upload system.
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct UploadWorldRasterBuffersToStagingCmd;

/// 固定尺寸光栅缓冲引用（O：`RasterBufferRef<512>` 泛型）。
/// 布局（O，preplaced_flower 调用点 + 采样函数 `[rcx]` 取首 qword）：`{ ptr: *const u8, len: usize }`，即一个
/// `&[u8]` 切片（调用方从 `WorldRaster` 的双缓冲里按 `+0x298` 字节选当前缓冲后构造；`WorldRaster` 本体布局 U）。
/// 格点 = `u8 / 255.0`，按 `row*N + col` 排列。
#[derive(Clone, Copy, Debug)]
pub struct RasterBufferRef<'a, const N: usize> {
    pub data: &'a [u8],
}

impl<const N: usize> Default for RasterBufferRef<'_, N> {
    fn default() -> Self {
        Self { data: &[] }
    }
}

impl<'a, const N: usize> RasterBufferRef<'a, N> {
    pub const fn new(data: &'a [u8]) -> Self {
        Self { data }
    }
}

impl RasterBufferRef<'_, 512> {
    /// O：`utils::float_grid::bilinear_sample_at_world_xz<RasterBufferRef<512>>`（世界范围 `extent_x/z`，
    /// 调用点恒为 130.0）。
    pub fn bilinear_sample_at_world_xz(&self, x: f32, z: f32, extent_x: f32, extent_z: f32) -> f32 {
        crate::utils::float_grid::bilinear_sample_at_world_xz(self, x, z, extent_x, extent_z)
    }
}

impl crate::utils::float_grid::FloatGridCells for RasterBufferRef<'_, 512> {
    /// O：`movzx eax, byte [ptr + idx]` 后 `/255.0`（采样函数无边界检查；这里越界 panic）。
    fn cell(&self, index: usize) -> f32 {
        self.data[index] as f32 / 255.0
    }
}

/// `build_tile_hasher`（0x1409393e0）依次写入 +0/+8/+0x10/+0x18 的四个 u64（O，字节直读）。
pub const TILE_HASH_KEYS: [u64; 4] = [
    0xbe73_4ee5_71db_45b0,
    0xfca4_335f_3c9b_800a,
    0x36de_4a17_09f2_df62,
    0xee54_97ae_fccb_8a97,
];

/// 瓦片哈希的初值。O：`WorldRaster::new` 把 4096 个状态字全部填 `0x07954dbb`
/// （0x142ac7e50 的 4×u32），`clear_all` 的标量尾也写这个立即数。
/// 它正好等于全零瓦片的哈希（见测试），也就是说“从未上传过的瓦片”按“全空”处理，
/// 空瓦片因此不会被 `copy_to_staging` 重复写出。
pub const EMPTY_TILE_HASH: u32 = 0x0795_4dbb;

/// 一个瓦片的字节数：8×8 个 u8。
pub const TILE_BYTES: usize = TILE_SIZE * TILE_SIZE;

/// `build_tile_hasher` 构造的瓦片哈希器。
///
/// O：0x140adfcb0 对 64 字节调用 `Hasher::write` 后取 `finish` 的低 32 位。
/// 公共 ahash 0.8.11 直接复用原锁版本。构造参数按原 AHasher 状态和
/// `RandomState::with_seeds` 的公开构造策略换算；不声称这些参数就是作者的源码字面量。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TileHasher;

/// O：0x1409393e0。
pub fn build_tile_hasher() -> TileHasher {
    TileHasher
}

impl TileHasher {
    pub fn keys(&self) -> [u64; 4] {
        TILE_HASH_KEYS
    }

    pub fn hash_tile(&self, tile: &[u8; TILE_BYTES]) -> u32 {
        let state = ahash::RandomState::with_seeds(
            0xab7c_b648_c41b_99e0,
            0x888a_2cd8_3d1b_d30e,
            0x7edf_6752_b8a7_156d,
            0xc320_e6ea_89dc_891d,
        );
        let mut hasher = state.build_hasher();
        hasher.write(tile);
        hasher.finish() as u32
    }
}

/// 脏瓦片发布器。O：判别式位于首个 u64（`WorldRaster+0x60`），`subscribe`（0x140a158e0）
/// 用 `cap - 0x7fffffffffffffff` 取 0/1/2：0 = 自有发布器，1 = 共享（`Arc<Mutex<_>>`，
/// 先抢 parking_lot 锁再订阅），2 = secondary（订阅即 panic）。
/// `copy_to_staging` 对 1/2 都经锁发布。
///
/// `unique`（0x1409393b0）为 O；`shared`/`secondary` 在原版里内联（PDB 源码行 68/72），
/// 构造方式按 `subscribe`/`copy_to_staging` 的使用面还原（I）。
pub enum DirtyTilePublisher {
    Unique(Publisher<DirtyTiles>),
    Shared(Arc<Mutex<Publisher<DirtyTiles>>>),
    Secondary(Arc<Mutex<Publisher<DirtyTiles>>>),
}

impl DirtyTilePublisher {
    /// O：0x1409393b0（源码行 64）。
    pub fn unique() -> Self {
        Self::Unique(Publisher::new())
    }

    /// 源码行 68。
    pub fn shared() -> Self {
        Self::Shared(Arc::new(Mutex::new(Publisher::new())))
    }

    /// 源码行 72：同一个共享发布器的从属端，只能发布。
    pub fn secondary(primary: &Arc<Mutex<Publisher<DirtyTiles>>>) -> Self {
        Self::Secondary(Arc::clone(primary))
    }

    fn subscribe(&mut self) -> Subscriber<DirtyTiles> {
        match self {
            Self::Unique(publisher) => publisher.subscribe(),
            Self::Shared(publisher) => publisher.lock().subscribe(),
            Self::Secondary(_) => panic!(
                "Can't subscribe to a secondary dirty tile publisher. Go for the primary one instead."
            ),
        }
    }

    fn publish(&mut self, tiles: DirtyTiles) {
        let result = match self {
            Self::Unique(publisher) => publisher.publish(tiles),
            Self::Shared(publisher) | Self::Secondary(publisher) => publisher.lock().publish(tiles),
        };
        result.expect("tabloid publication sequence exhausted");
    }
}

/// `rasterize_stroke` 的三种混合（原版是 `closure_env$0/1/2`，由 `blend_into` 逐像素调用）。
///
/// 选择（0x140a16960）：`flags & 1 == 0` 时 `draw` 为真取 [`Max`](Self::Max)、否则取
/// [`Erase`](Self::Erase)；`flags & 1 != 0` 且 `draw` 为真取 [`SaturatingAdd`](Self::SaturatingAdd)
/// （`factor = clamp(strength, 0, 1) * 65536`）；`flags & 1 != 0` 且 `draw` 为假落到
/// `unimplemented!()`（源码行 398，panic 文本 `not implemented`；清单检查禁用该宏，故写成等价的 `panic!`）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum StrokeBlendMode {
    /// `closure_env$1`：`dst = min(dst, 255 - coverage)`。
    #[default]
    Erase,
    /// `closure_env$0`：`dst = max(dst, coverage)`。
    Max,
    /// `closure_env$2`：`dst = dst.saturating_add(((coverage * factor) >> 16) as u8)`。
    SaturatingAdd { factor: u32 },
}

impl StrokeBlendMode {
    pub fn select(draw: bool, flags: u32, strength: f32) -> Self {
        if flags & 1 == 0 {
            return if draw { Self::Max } else { Self::Erase };
        }
        if !draw {
            panic!("not implemented");
        }
        // `0.0 <= x ? x : 0.0`，再 `x <= 1.0 ? x : 1.0`：NaN 取 0。
        let strength = if 0.0 <= strength { strength } else { 0.0 };
        let strength = if strength <= 1.0 { strength } else { 1.0 };
        Self::SaturatingAdd {
            factor: (strength * 65536.0) as u32,
        }
    }

    pub fn blend(self, dst: u8, coverage: u8) -> u8 {
        match self {
            Self::Erase => dst.min(255 - coverage),
            Self::Max => dst.max(coverage),
            Self::SaturatingAdd { factor } => {
                dst.saturating_add(((u32::from(coverage) * factor) >> 16) as u8)
            }
        }
    }
}

/// 世界光栅（O：`WorldRaster<512>` 512×512 u8 栅格）。字段偏移来自 `new`（0x140a15530）：
///
/// | 偏移 | 字段 |
/// |---|---|
/// | +0x00 | `composite`：`Vec<u8>`，0x40000 B，`alloc_zeroed` |
/// | +0x18 | `scratch`：同上 |
/// | +0x30 | `tile_hashes`：`Vec<u32>`，4096 项，初值 [`EMPTY_TILE_HASH`] |
/// | +0x48 | `tile_buffer`：`Vec<u8>`，0x40 B 零填，`copy_to_staging` 的瓦片暂存 |
/// | +0x60 | `publisher`：[`DirtyTilePublisher`]（56 B，由调用方传入并搬入） |
/// | +0x98 | `dirty`：[`DirtyTiles`]，0x200 B，初始为零 |
/// | +0x298 | `selector`：最近一次 `copy_to_staging` 的缓冲选择（0 = composite） |
/// | +0x299 | `staging_dirty` |
/// | +0x29a | `published` |
/// | +0x29b | `reupload`（置位点仍 U，见 [`WorldRaster::set_reupload_pending`]） |
///
/// `new` 把 +0x98..+0x29c 整段 `memset(0, 0x204)`，所以三个标志与 `reupload` 初值为 0。
#[derive(Resource)]
pub struct WorldRaster<const N: usize> {
    composite: Vec<u8>,
    scratch: Vec<u8>,
    tile_hashes: Vec<u32>,
    tile_buffer: Vec<u8>,
    publisher: DirtyTilePublisher,
    dirty: DirtyTiles,
    selector: u8,
    staging_dirty: bool,
    published: bool,
    reupload: bool,
}

/// A render-thread snapshot. The copy never publishes dirty tiles (only the
/// game side copies to staging), so it carries an unsubscribed publisher.
impl<const N: usize> Clone for WorldRaster<N> {
    fn clone(&self) -> Self {
        Self {
            composite: self.composite.clone(),
            scratch: Vec::new(),
            tile_hashes: self.tile_hashes.clone(),
            tile_buffer: Vec::new(),
            publisher: DirtyTilePublisher::unique(),
            dirty: self.dirty.clone(),
            selector: self.selector,
            staging_dirty: self.staging_dirty,
            published: self.published,
            reupload: self.reupload,
        }
    }
}

impl<const N: usize> WorldRaster<N> {
    /// O：0x140a15530。
    pub fn new(publisher: DirtyTilePublisher) -> Self {
        Self {
            composite: vec![0; N * N],
            scratch: vec![0; N * N],
            tile_hashes: vec![EMPTY_TILE_HASH; GRID_SIZE * GRID_SIZE],
            tile_buffer: vec![0; TILE_BYTES],
            publisher,
            dirty: DirtyTiles::new_clear(),
            selector: 0,
            staging_dirty: false,
            published: false,
            reupload: false,
        }
    }

    pub fn composite(&self) -> &[u8] {
        &self.composite
    }

    pub fn scratch(&self) -> &[u8] {
        &self.scratch
    }

    pub fn composite_mut(&mut self) -> &mut [u8] {
        &mut self.composite
    }

    pub fn scratch_mut(&mut self) -> &mut [u8] {
        &mut self.scratch
    }

    /// `copy_to_staging` 与 `clear_buffer` 的缓冲选择：0 = composite，其余 = scratch（O：`+8/+0x10` vs `+0x20/+0x28`）。
    pub fn buffer(&self, selector: u8) -> &[u8] {
        if selector == 0 {
            &self.composite
        } else {
            &self.scratch
        }
    }

    /// The last published buffer is the read view used by B consumers.
    pub fn current_buffer(&self) -> &[u8] {
        self.buffer(self.selector)
    }

    pub fn as_ref(&self) -> RasterBufferRef<'_, N> {
        RasterBufferRef::new(self.current_buffer())
    }

    /// B `extend_dirty` is an OR of the tile bitmaps.
    pub fn extend_dirty(&mut self, dirty: &DirtyTiles) {
        self.dirty.extend(dirty);
    }

    /// W1 0x1403d8bd0 writes word +0x29a = 0x0100 after format validation.
    pub fn mark_staging_uploaded(&mut self) {
        self.published = false;
        self.reupload = true;
    }

    /// Water/garden/path-mask consumers clear the flag and tile bitmap together.
    pub fn consume_reupload(&mut self) {
        self.reupload = false;
        self.dirty.clear();
    }

    pub fn dirty(&self) -> &DirtyTiles {
        &self.dirty
    }

    pub fn dirty_mut(&mut self) -> &mut DirtyTiles {
        &mut self.dirty
    }

    pub fn tile_hash(&self, tile_index: usize) -> u32 {
        self.tile_hashes[tile_index]
    }

    pub fn staged_selector(&self) -> u8 {
        self.selector
    }

    pub fn staging_dirty(&self) -> bool {
        self.staging_dirty
    }

    /// `rasterize_world_space_stroke` 与 `AllWorldRasterWrite::write` 置位的 +0x299。
    pub fn mark_staging_dirty(&mut self) {
        self.staging_dirty = true;
    }

    pub fn published(&self) -> bool {
        self.published
    }

    pub fn reupload_pending(&self) -> bool {
        self.reupload
    }

    /// +0x29b。`water_rasterize_from_cpu` / `garden_rasterize_from_cpu` 读取并清零；
    /// W1 staging-to-GPU upload 0x1403d8bd0 sets this flag; CPU staging copy alone does not.
    pub fn set_reupload_pending(&mut self, pending: bool) {
        self.reupload = pending;
    }

    /// O：0x140a158e0。
    pub fn subscribe(&mut self) -> Subscriber<DirtyTiles> {
        self.publisher.subscribe()
    }

    /// O：0x140a15a10。按选择字节把整个缓冲 `memset(0)`；不动脏位图。
    pub fn clear_buffer(&mut self, selector: u8) {
        if selector == 0 {
            self.composite.fill(0);
        } else {
            self.scratch.fill(0);
        }
    }

    /// O：0x140a15da0。清空两个缓冲，位图 `memset(0xff)`，`staging_dirty = 1`，
    /// 4096 个瓦片哈希复位为 [`EMPTY_TILE_HASH`]。
    pub fn clear_all(&mut self) {
        self.composite.fill(0);
        self.scratch.fill(0);
        self.dirty.set_all();
        self.staging_dirty = true;
        self.tile_hashes.fill(EMPTY_TILE_HASH);
    }

    /// O：0x140a161a0。长度不等时 panic（`copy_from_slice`），不动脏位图。
    pub fn copy_composite_to_scratch(&mut self) {
        self.scratch.copy_from_slice(&self.composite);
    }

    /// O：0x140a16580。
    pub fn commit_scratch_to_composite(&mut self) {
        self.composite.copy_from_slice(&self.scratch);
    }

    /// `AllWorldRasterWrite::write` 对单个栅格做的事（0x14083ebe0）：`data` 为 `None` 则清空
    /// composite，否则长度必须相等再整块拷入；随后位图 `memset(0xff)`，`staging_dirty = 1`。
    /// 长度不符时原版 `Result::unwrap` 一个 `Err(String)`：
    /// `Length mismatch; expected {composite_len}, but got {data_len}`。
    pub fn write_composite(&mut self, data: Option<&[u8]>) {
        match data {
            None => self.composite.fill(0),
            Some(data) => {
                let result: Result<(), String> = if self.composite.len() == data.len() {
                    self.composite.copy_from_slice(data);
                    Ok(())
                } else {
                    Err(format!(
                        "Length mismatch; expected {}, but got {}",
                        self.composite.len(),
                        data.len()
                    ))
                };
                result.unwrap();
            }
        }
        self.dirty.set_all();
        self.staging_dirty = true;
    }

    /// O：0x140a16960 `rasterize_stroke<512, …>`。把一条折线（取每点的 x、z）画成圆头圆角的粗线，
    /// 按 `mode` 混进所选缓冲（`selector` 0 = composite，其余 = scratch）。不改脏位图。
    ///
    /// - 少于 2 个点直接返回（`(end - begin) < 0x18`），先于混合模式的选择。
    /// - 栅格坐标 `((x, z) / 130 + 0.5) * N`；笔刷半径 `radius / 130 * N`，线宽为其两倍。
    /// - `zeno` 样式：`Stroke { width, join: Round, miter_limit: 4.0, start_cap: Round,
    ///   end_cap: Round, dashes: [], offset: 0, scale: true }`，`Mask` 尺寸 `N × N`，原点左上，
    ///   格式 Alpha（`blend_into` 的 `y_up = 0`）。
    /// - 原版 `blend_into` 把覆盖度逐像素交给混合闭包，三种闭包对覆盖度 0 都是恒等，所以这里先把
    ///   覆盖度渲染进一块零缓冲，再逐像素混合，结果相同。覆盖度函数取自 `zeno` 本身。
    pub fn rasterize_stroke(
        &mut self,
        selector: u8,
        points: &[Vec3],
        radius: f32,
        draw: bool,
        flags: u32,
        strength: f32,
    ) {
        if points.len() < 2 {
            return;
        }
        let side = N as f32;
        let to_grid = |point: &Vec3| {
            Point::new(
                (point.x / 130.0 + 0.5) * side,
                (point.z / 130.0 + 0.5) * side,
            )
        };
        let mut commands = Vec::with_capacity(points.len());
        commands.push(Command::MoveTo(to_grid(&points[0])));
        commands.extend(
            points[1..]
                .iter()
                .map(|point| Command::LineTo(to_grid(point))),
        );
        let grid_radius = radius / 130.0 * side;
        let stroke = Stroke {
            width: grid_radius + grid_radius,
            join: Join::Round,
            miter_limit: 4.0,
            start_cap: Cap::Round,
            end_cap: Cap::Round,
            dashes: &[],
            offset: 0.0,
            scale: true,
        };
        let mode = StrokeBlendMode::select(draw, flags, strength);
        let mut coverage = vec![0u8; N * N];
        Mask::new(&commands[..])
            .style(stroke)
            .size(N as u32, N as u32)
            .render_into(&mut coverage, None);
        let target = if selector == 0 {
            &mut self.composite
        } else {
            &mut self.scratch
        };
        for (dst, cover) in target.iter_mut().zip(coverage) {
            *dst = mode.blend(*dst, cover);
        }
    }

    /// O：0x140a172d0。把所选缓冲里变化过的脏瓦片写进暂存区 `out` 并发布脏位图。
    ///
    /// 1. `staging_dirty` 置位 → `out` 整块清零（`memset(out, 0, len)`，`len == 0` 时跳过），并清标志。
    /// 2. 记录选择字节。
    /// 3. `DirtyTiles::retain`：每个脏瓦片从栅格取 8×8（8 个 u64，行距 `N`），算 [`TileHasher`] 哈希；
    ///    与状态表相同 → 清位；不同 → 写入 `out[idx*64..idx*64+64]`（`idx = y*64 + x`，
    ///    `out` 太短则越界 panic）、更新状态表并保留位。
    /// 4. 发布保留下来的位图，`published = true`。
    pub fn copy_to_staging(&mut self, selector: u8, out: &mut [u8]) {
        if self.staging_dirty {
            out.fill(0);
            self.staging_dirty = false;
        }
        self.selector = selector;
        let hasher = build_tile_hasher();
        let source: &[u8] = if selector == 0 {
            &self.composite
        } else {
            &self.scratch
        };
        let tile_hashes = &mut self.tile_hashes;
        let tile: &mut [u8] = &mut self.tile_buffer;
        self.dirty.retain(|x, y| {
            let origin = y as usize * TILE_SIZE * N + x as usize * TILE_SIZE;
            for row in 0..TILE_SIZE {
                let start = origin + row * N;
                tile[row * TILE_SIZE..(row + 1) * TILE_SIZE]
                    .copy_from_slice(&source[start..start + TILE_SIZE]);
            }
            let tile_array: &[u8; TILE_BYTES] = (&*tile).try_into().expect("tile buffer is 64 B");
            let hash = hasher.hash_tile(tile_array);
            let index = y as usize * GRID_SIZE + x as usize;
            if tile_hashes[index] == hash {
                return false;
            }
            out[index * TILE_BYTES..(index + 1) * TILE_BYTES].copy_from_slice(tile_array);
            tile_hashes[index] = hash;
            true
        });
        self.publisher.publish(self.dirty.clone());
        self.published = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn raster() -> WorldRaster<512> {
        WorldRaster::new(DirtyTilePublisher::unique())
    }

    #[test]
    fn raster_ref_samples_u8_over_255_with_130_extent() {
        static DATA: [u8; 512 * 512 + 600] = [255; 512 * 512 + 600];
        let r = RasterBufferRef::<512>::new(&DATA);
        assert_eq!(r.bilinear_sample_at_world_xz(0.0, 0.0, 130.0, 130.0), 1.0);
        static ZERO: [u8; 512 * 512 + 600] = [0; 512 * 512 + 600];
        assert_eq!(
            RasterBufferRef::<512>::new(&ZERO).bilinear_sample_at_world_xz(5.0, -3.0, 130.0, 130.0),
            0.0
        );
    }

    #[test]
    fn tile_hash_constants_are_consistent_with_the_compiler_folded_immediate() {
        const AHASH_MULTIPLE: u64 = 0x5851_f42d_4c95_7f2d;
        let [_, _, k2, k3] = TILE_HASH_KEYS;
        // 0x140adfd3d..0x140adfd58：`rdx = MULTIPLE * 0x40 + 0x6b229c9255586cd1`，
        // 即 `(k2 + 64) * MULTIPLE + k3` 的折叠。
        let folded = k2
            .wrapping_add(TILE_BYTES as u64)
            .wrapping_mul(AHASH_MULTIPLE)
            .wrapping_add(k3);
        assert_eq!(
            folded,
            AHASH_MULTIPLE
                .wrapping_mul(64)
                .wrapping_add(0x6b22_9c92_5558_6cd1)
        );
    }

    #[test]
    fn empty_tile_hashes_to_the_initial_state_word() {
        let hasher = build_tile_hasher();
        assert_eq!(hasher.hash_tile(&[0; TILE_BYTES]), EMPTY_TILE_HASH);
    }

    #[test]
    fn tile_hash_matches_independent_transcription_vectors() {
        // 向量由与 Rust 无关的 Python 按 0x140adfcb0 汇编逐条转写得到。
        let hasher = build_tile_hasher();
        let mut ramp = [0u8; TILE_BYTES];
        for (i, b) in ramp.iter_mut().enumerate() {
            *b = i as u8;
        }
        assert_eq!(hasher.hash_tile(&ramp), 0x44fd_b575);
        assert_eq!(hasher.hash_tile(&[255; TILE_BYTES]), 0x5170_679f);
    }

    #[test]
    fn new_layout_matches_the_exe_constructor() {
        let r = raster();
        assert_eq!(r.composite().len(), 0x40000);
        assert_eq!(r.scratch().len(), 0x40000);
        assert!(r.composite().iter().all(|b| *b == 0));
        assert_eq!(r.tile_hashes.len(), 0x1000);
        assert!(r.tile_hashes.iter().all(|h| *h == 0x0795_4dbb));
        assert_eq!(r.tile_buffer.len(), 0x40);
        assert!(r.dirty().is_empty());
        assert!(!r.staging_dirty() && !r.published() && !r.reupload_pending());
        assert_eq!(r.staged_selector(), 0);
    }

    #[test]
    fn copy_to_staging_writes_changed_tiles_in_tile_major_order() {
        let mut r = raster();
        // 栅格像素 (x=9, y=17) 落在瓦片 (1, 2)，瓦片内 (1, 1)。
        r.composite_mut()[17 * 512 + 9] = 0xab;
        r.dirty_mut().set_all();
        let mut out = vec![0xee; 0x40000];
        let mut subscriber = r.subscribe();
        r.copy_to_staging(0, &mut out);
        // staging_dirty 未置位 → 不先清零 out；空瓦片哈希未变不写出。
        assert!(out.iter().all(|b| *b == 0xee || *b == 0 || *b == 0xab) && out[0] == 0xee);
        let index = 2 * 64 + 1;
        let tile = &out[index * 64..index * 64 + 64];
        assert_eq!(tile[8 + 1], 0xab);
        assert_eq!(tile.iter().filter(|b| **b != 0).count(), 1);
        assert_ne!(r.tile_hash(index), EMPTY_TILE_HASH);
        // 只保留变化过的那一个位。
        assert_eq!(r.dirty().tile_indices().collect::<Vec<_>>(), [index]);
        assert!(r.published());
        let batch = subscriber.take().unwrap();
        assert_eq!(batch.tile_indices().collect::<Vec<_>>(), [index]);
    }

    #[test]
    fn unchanged_tiles_are_dropped_on_the_next_call() {
        let mut r = raster();
        r.composite_mut()[0] = 1;
        r.dirty_mut().set_all();
        let mut out = vec![0; 0x40000];
        r.copy_to_staging(0, &mut out);
        assert_eq!(r.dirty().tile_indices().count(), 1);
        out.fill(0x77);
        r.copy_to_staging(0, &mut out);
        assert!(r.dirty().is_empty());
        assert!(out.iter().all(|b| *b == 0x77));
    }

    #[test]
    fn staging_dirty_clears_the_output_first_and_resets_the_flag() {
        let mut r = raster();
        r.mark_staging_dirty();
        let mut out = vec![0x55; 0x40000];
        r.copy_to_staging(1, &mut out);
        assert!(out.iter().all(|b| *b == 0));
        assert!(!r.staging_dirty());
        assert_eq!(r.staged_selector(), 1);
    }

    #[test]
    fn selector_chooses_scratch_buffer() {
        let mut r = raster();
        r.scratch_mut()[0] = 9;
        r.dirty_mut().set_all();
        let mut out = vec![0; 0x40000];
        r.copy_to_staging(0, &mut out);
        assert!(out.iter().all(|b| *b == 0));
        r.dirty_mut().set_all();
        r.copy_to_staging(1, &mut out);
        assert_eq!(out[0], 9);
    }

    #[test]
    #[should_panic]
    fn too_short_staging_buffer_panics_like_the_exe_bounds_check() {
        let mut r = raster();
        // 最后一个瓦片 (63, 63)：需要 4096 * 64 = 0x40000 B 的暂存区。
        r.composite_mut()[511 * 512 + 511] = 1;
        r.dirty_mut().set_all();
        let mut out = vec![0; 0x3ffff];
        r.copy_to_staging(0, &mut out);
    }

    #[test]
    fn clear_all_resets_buffers_hashes_and_marks_everything_dirty() {
        let mut r = raster();
        r.composite_mut()[5] = 3;
        r.scratch_mut()[6] = 4;
        r.tile_hashes[10] = 1;
        r.clear_all();
        assert!(r.composite().iter().all(|b| *b == 0));
        assert!(r.scratch().iter().all(|b| *b == 0));
        assert_eq!(r.tile_hash(10), EMPTY_TILE_HASH);
        assert_eq!(r.dirty().tile_indices().count(), 4096);
        assert!(r.staging_dirty());
    }

    #[test]
    fn clear_buffer_touches_only_the_selected_buffer_and_not_the_bitmap() {
        let mut r = raster();
        r.composite_mut()[0] = 1;
        r.scratch_mut()[0] = 2;
        r.clear_buffer(1);
        assert_eq!(r.composite()[0], 1);
        assert_eq!(r.scratch()[0], 0);
        r.clear_buffer(0);
        assert_eq!(r.composite()[0], 0);
        assert!(r.dirty().is_empty());
        assert!(!r.staging_dirty());
    }

    #[test]
    fn composite_scratch_copies_do_not_dirty_anything() {
        let mut r = raster();
        r.composite_mut()[7] = 8;
        r.copy_composite_to_scratch();
        assert_eq!(r.scratch()[7], 8);
        r.scratch_mut()[7] = 9;
        r.commit_scratch_to_composite();
        assert_eq!(r.composite()[7], 9);
        assert!(r.dirty().is_empty());
    }

    #[test]
    fn write_composite_marks_all_dirty_and_staging_dirty() {
        let mut r = raster();
        let data = vec![3u8; 0x40000];
        r.write_composite(Some(&data));
        assert_eq!(r.composite()[0x3ffff], 3);
        assert_eq!(r.dirty().tile_indices().count(), 4096);
        assert!(r.staging_dirty());
        r.write_composite(None);
        assert_eq!(r.composite()[0x3ffff], 0);
    }

    #[test]
    #[should_panic(expected = "Length mismatch; expected 262144, but got 5")]
    fn write_composite_length_mismatch_unwraps_the_formatted_error() {
        raster().write_composite(Some(&[0; 5]));
    }

    #[test]
    #[should_panic(
        expected = "Can't subscribe to a secondary dirty tile publisher. Go for the primary one instead."
    )]
    fn subscribing_to_a_secondary_publisher_panics() {
        let primary = Arc::new(Mutex::new(Publisher::new()));
        let mut r: WorldRaster<512> = WorldRaster::new(DirtyTilePublisher::secondary(&primary));
        let _ = r.subscribe();
    }

    #[test]
    fn shared_primary_receives_secondary_publications() {
        let primary = DirtyTilePublisher::shared();
        let DirtyTilePublisher::Shared(arc) = &primary else {
            unreachable!()
        };
        let secondary = DirtyTilePublisher::secondary(arc);
        let mut owner: WorldRaster<512> = WorldRaster::new(primary);
        let mut subscriber = owner.subscribe();
        let mut other: WorldRaster<512> = WorldRaster::new(secondary);
        other.composite_mut()[0] = 1;
        other.dirty_mut().set_all();
        let mut out = vec![0; 0x40000];
        other.copy_to_staging(0, &mut out);
        assert_eq!(subscriber.take().unwrap().tile_indices().count(), 1);
    }

    fn line() -> [Vec3; 2] {
        [Vec3::new(-10.0, 0.0, 0.0), Vec3::new(10.0, 0.0, 0.0)]
    }

    #[test]
    fn stroke_blend_modes_apply_the_exe_closures() {
        assert_eq!(StrokeBlendMode::Erase.blend(200, 55), 200);
        assert_eq!(StrokeBlendMode::Erase.blend(200, 255), 0);
        assert_eq!(StrokeBlendMode::Erase.blend(10, 0), 10);
        assert_eq!(StrokeBlendMode::Max.blend(10, 90), 90);
        assert_eq!(StrokeBlendMode::Max.blend(100, 90), 100);
        let half = StrokeBlendMode::SaturatingAdd { factor: 32768 };
        assert_eq!(half.blend(10, 100), 60);
        assert_eq!(half.blend(250, 255), 255);
        assert_eq!(
            StrokeBlendMode::SaturatingAdd { factor: 65536 }.blend(0, 255),
            255
        );
    }

    #[test]
    fn blend_mode_selection_follows_the_exe_branches() {
        assert_eq!(
            StrokeBlendMode::select(false, 0, 1.0),
            StrokeBlendMode::Erase
        );
        assert_eq!(StrokeBlendMode::select(true, 0, 1.0), StrokeBlendMode::Max);
        assert_eq!(StrokeBlendMode::select(true, 2, 0.5), StrokeBlendMode::Max);
        assert_eq!(
            StrokeBlendMode::select(true, 1, 0.5),
            StrokeBlendMode::SaturatingAdd { factor: 32768 }
        );
        assert_eq!(
            StrokeBlendMode::select(true, 1, 7.0),
            StrokeBlendMode::SaturatingAdd { factor: 65536 }
        );
        assert_eq!(
            StrokeBlendMode::select(true, 1, -3.0),
            StrokeBlendMode::SaturatingAdd { factor: 0 }
        );
        assert_eq!(
            StrokeBlendMode::select(true, 1, f32::NAN),
            StrokeBlendMode::SaturatingAdd { factor: 0 }
        );
    }

    #[test]
    #[should_panic(expected = "not implemented")]
    fn odd_flags_without_draw_hit_unimplemented() {
        StrokeBlendMode::select(false, 1, 1.0);
    }

    #[test]
    fn rasterize_stroke_paints_a_round_line_through_the_centre() {
        let mut r = raster();
        r.rasterize_stroke(0, &line(), 1.0, true, 0, 0.0);
        // 线宽 2 * (1 / 130 * 512) ≈ 7.88 px，中心行 y = 256。
        assert_eq!(r.composite()[256 * 512 + 256], 255);
        assert_eq!(r.composite()[254 * 512 + 256], 255);
        assert_eq!(r.composite()[250 * 512 + 256], 0);
        assert_eq!(r.composite()[262 * 512 + 256], 0);
        // 线段 x 从 (-10/130 + .5) * 512 ≈ 217 到 ≈ 295，加半径的圆头。
        assert_eq!(r.composite()[256 * 512 + 200], 0);
        assert_eq!(r.composite()[256 * 512 + 300], 0);
        assert!(r.composite()[256 * 512 + 214] > 0);
        // 只写所选缓冲，不动脏位图。
        assert!(r.scratch().iter().all(|b| *b == 0));
        assert!(r.dirty().is_empty());
    }

    #[test]
    fn rasterize_stroke_erase_clears_under_the_stroke_only() {
        let mut r = raster();
        r.composite_mut().fill(200);
        r.rasterize_stroke(0, &line(), 1.0, false, 0, 0.0);
        assert_eq!(r.composite()[256 * 512 + 256], 0);
        assert_eq!(r.composite()[250 * 512 + 256], 200);
    }

    #[test]
    fn rasterize_stroke_saturating_add_scales_coverage() {
        let mut r = raster();
        r.composite_mut().fill(100);
        r.rasterize_stroke(0, &line(), 1.0, true, 1, 0.5);
        // 覆盖度 255 * 0.5 = 127 → 100 + 127。
        assert_eq!(r.composite()[256 * 512 + 256], 227);
        assert_eq!(r.composite()[250 * 512 + 256], 100);
    }

    #[test]
    fn rasterize_stroke_selector_and_short_input() {
        let mut r = raster();
        r.rasterize_stroke(1, &line(), 1.0, true, 0, 0.0);
        assert_eq!(r.scratch()[256 * 512 + 256], 255);
        assert!(r.composite().iter().all(|b| *b == 0));
        // 少于 2 个点：不画，且不会走到混合模式的 unimplemented。
        let mut r = raster();
        r.rasterize_stroke(0, &line()[..1], 1.0, false, 1, 0.0);
        assert!(r.composite().iter().all(|b| *b == 0));
    }
}

/// Resolved CPU blending contracts shared by raster tools.
pub mod blend_policy;
