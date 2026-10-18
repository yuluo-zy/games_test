//! `water_exclude_clutter_mask.rs` — 水位排除杂物 mask 回读 SSBO（B：`WaterExcludeClutterMaskSsbo`，O 名）。
//!
//! 证据：read_terrain_data.rs "Read water" pass
//! `write_storage_buffer<WaterExcludeClutterMaskSsbo>`（与 WaterMaskSsbo 同一次回读双写）。
//! SSBO 载体实现与 `TerrainHeightSsbo` 同族。
use crate::asset_libraries::ssbo_library::SsboType;
use crate::render::ssbo::{MAPPED_MEMORY_READBACK, MappedStorageBuffer, StorageElement};

/// 水位排除杂物 mask 回读 SSBO（O：`WaterExcludeClutterMaskSsbo`，0x2000 线程回读）。
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default)]
pub struct WaterExcludeClutterMaskSsbo(pub u32);

unsafe impl bytemuck::Zeroable for WaterExcludeClutterMaskSsbo {}
unsafe impl bytemuck::Pod for WaterExcludeClutterMaskSsbo {}

impl StorageElement for WaterExcludeClutterMaskSsbo {
    const NAME: &'static str = "WaterExcludeClutterMask";
    const BUFFER_LEN: usize = 0x2000;
    const MAPPED_MEMORY: u8 = MAPPED_MEMORY_READBACK;
}

impl SsboType for WaterExcludeClutterMaskSsbo {
    type Storage = MappedStorageBuffer<'static, Self>;
    fn new_storage_buffer(
        device: &dyn crate::render::ssbo::rhapsody_boundary::GraphicsDevice,
        initial: Option<&[u8]>,
    ) -> Self::Storage {
        MappedStorageBuffer::new_storage_buffer(device, initial)
    }
}

impl crate::render::pass_builder::pass_resource_access::SsboType for WaterExcludeClutterMaskSsbo {
    fn rg_buffer_hash() -> u64 {
        crate::name_hash::NameHash::compute(<Self as StorageElement>::NAME).0
    }
}
