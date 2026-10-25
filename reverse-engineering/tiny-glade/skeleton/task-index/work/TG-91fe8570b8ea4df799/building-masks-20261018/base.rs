//! `garden_mask.rs` — 花园 mask 回读 SSBO（B：`GardenMaskSsbo`，O 名）。
//!
//! 证据：read_terrain_data.rs "Read garden" pass `write_storage_buffer<GardenMaskSsbo>`
//! （0x14098c680 族）。SSBO 载体实现与 `TerrainHeightSsbo` 同族。
use crate::asset_libraries::ssbo_library::SsboType;
use crate::render::ssbo::{MappedStorageBuffer, StorageElement, MAPPED_MEMORY_READBACK};

/// 花园 mask 回读 SSBO（O：`GardenMaskSsbo`，0x2000 线程回读）。
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default)]
pub struct GardenMaskSsbo(pub u32);

unsafe impl bytemuck::Zeroable for GardenMaskSsbo {}
unsafe impl bytemuck::Pod for GardenMaskSsbo {}

impl StorageElement for GardenMaskSsbo {
    const NAME: &'static str = "GardenMaskSsbo";
    const BUFFER_LEN: usize = 0x8000;
    const MAPPED_MEMORY: u8 = MAPPED_MEMORY_READBACK;
}

impl SsboType for GardenMaskSsbo {
    type Storage = MappedStorageBuffer<'static, Self>;
    fn new_storage_buffer(
        device: &dyn crate::render::ssbo::rhapsody_boundary::GraphicsDevice,
        initial: Option<&[u8]>,
    ) -> Self::Storage {
        MappedStorageBuffer::new_storage_buffer(device, initial)
    }
}
