//! `terrain_rock_mask.rs` — 岩石 mask 回读资源（B 0x1409ea800 `sample_at_world_xz`，O 结构）。
//!
//! 证据：`chapters/resources/terrain_rock_mask.rs.md`、re-analyst-interaction（卡7）：
//! - `TerrainRockMaskSsbo` 名字断言 = `"country_core::resources::terrain_rock_mask::TerrainRockMaskSsbo"`
//!   （0x3f 字节，见 render/nani/instance_updates.rs）；
//! - `sample_at_world_xz`（0x1409ea800）：世界坐标 → mask 栅格采样。
//!
//! SSBO 载体实现与 `TerrainHeightSsbo` 同族（StorageElement NAME=stringify、BUFFER_LEN、
//! MAPPED_MEMORY_READBACK + SsboType）。
use crate::asset_libraries::ssbo_library::SsboType;
use crate::render::ssbo::{MappedStorageBuffer, StorageElement, MAPPED_MEMORY_READBACK};

/// 岩石 mask 回读 SSBO（O：`TerrainRockMaskSsbo`，0x2000 线程回读）。
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default)]
pub struct TerrainRockMaskSsbo(pub u32);

unsafe impl bytemuck::Zeroable for TerrainRockMaskSsbo {}
unsafe impl bytemuck::Pod for TerrainRockMaskSsbo {}

impl StorageElement for TerrainRockMaskSsbo {
    const NAME: &'static str = "TerrainRockMaskSsbo";
    const BUFFER_LEN: usize = 0x8000;
    const MAPPED_MEMORY: u8 = MAPPED_MEMORY_READBACK;
}

impl SsboType for TerrainRockMaskSsbo {
    type Storage = MappedStorageBuffer<'static, Self>;
    fn new_storage_buffer(
        device: &dyn crate::render::ssbo::rhapsody_boundary::GraphicsDevice,
        initial: Option<&[u8]>,
    ) -> Self::Storage {
        MappedStorageBuffer::new_storage_buffer(device, initial)
    }
}

/// 岩石 mask 栅格边长（I：512，与地形高度同族）。
pub const ROCK_MASK_SIDE: usize = 512;

/// `sample_at_world_xz`（0x1409ea800，O 结构）：世界坐标 → mask 栅格采样。
/// 栅格映射 = 与地形高度同族（`world_xz * 1/130 + 0.5`，130.0 世界半宽/栅格）。
pub fn sample_at_world_xz(mask: &[u32], x: f32, z: f32) -> u32 {
    let side = ROCK_MASK_SIDE as f32;
    let u = (x * (1.0 / 130.0) + 0.5).clamp(0.0, 1.0) * side;
    let v = (z * (1.0 / 130.0) + 0.5).clamp(0.0, 1.0) * side;
    let xi = (u as usize).min(side as usize - 1);
    let zi = (v as usize).min(side as usize - 1);
    mask.get(zi * ROCK_MASK_SIDE + xi).copied().unwrap_or(0)
}
