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
    const NAME: &'static str = "TerrainRockMask";
    // B constructor0x141620415 allocates0x8000 bytes, or0x2000 u32 words.
    const BUFFER_LEN: usize = 0x2000;
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

/// `sample_at_world_xz`（0x1409ea800，O）：世界坐标 → mask 栅格 **位** 采样。
///
/// fresh 反编译（resources-0x1409ea800）：
/// - 映射 = `x / 130.0 + 0.5`（DIVSS 130.0，非 `x*(1/130)`），clamp[0,1]（NaN 保持 → 转型 0），
///   `* 512.0` 后 `(uint)(longlong)` 截断饱和，再 `min 0x1ff`；
/// - 索引 = `(z << 9 | x)`（`v*512 + u`）；
/// - **越界 = `index >> 5 < len` panic_bounds_check**（按 u32 词长；`Rust unwrap_or(0)` 系自造
///   静默返回）；
/// - 返回值 = `mask[index >> 5]` 的 **位** `index & 31`（mask 为位图，每 u32 词 = 32 格；
///   Rust 按整 u32 词返回系位/词语义错）。
pub fn sample_at_world_xz(mask: &[u32], x: f32, z: f32) -> u32 {
    // 原生 clamp 不用 `f32::clamp`（NaN 会 panic）：`<=0→0`、`1<=→1`，NaN 保持走 `as u32` 饱和。
    let mut u = x / 130.0 + 0.5;
    let mut v = z / 130.0 + 0.5;
    if u <= 0.0 {
        u = 0.0;
    }
    if 1.0 <= u {
        u = 1.0;
    }
    if v <= 0.0 {
        v = 0.0;
    }
    if 1.0 <= v {
        v = 1.0;
    }
    let xi = (u * 512.0) as u32;
    let zi = (v * 512.0) as u32;
    let xi = xi.min(0x1ff);
    let zi = zi.min(0x1ff);
    let index = (zi << 9) | xi;
    // 原生 `panic_bounds_check`（index>>5 < len），不静默返回。
    let word = index >> 5;
    assert!(
        word < mask.len() as u32,
        "terrain rock mask index out of bounds: word {word} >= len {}",
        mask.len()
    );
    (mask[word as usize] >> (index & 0x1f)) & 1
}


#[cfg(test)]
mod original_storage_contract_tests {
    use super::*;
    use crate::{asset_libraries::ssbo_library::AssetSsboLibrary, render::ssbo::tests::MockDevice};

    #[test]
    fn actual_library_constructor_matches_native_name_bytes_and_word_count() {
        let device = MockDevice::default();
        let mut library = AssetSsboLibrary::new();
        library.add_new::<TerrainRockMaskSsbo>(&device);
        let storage = library.get::<TerrainRockMaskSsbo>().unwrap();
        assert_eq!(storage.mapped.len(), 0x2000);
        assert_eq!(storage.buffer.byte_size, 0x8000);
        {
            let calls = device.created.borrow();
            assert_eq!(calls.len(), 1);
            let (desc, name, initial) = &calls[0];
            assert_eq!(name, "TerrainRockMask");
            assert_eq!((desc.size, desc.usage, desc.memory, desc.alignment), (0x8000, 0x22, 4, 4));
            let initial = initial.as_ref().unwrap();
            assert_eq!(initial.len(), 0x8000);
            assert!(initial.iter().all(|byte| *byte == 0));
        }
        library.get_mut::<TerrainRockMaskSsbo>().unwrap().mapped[0x1fff].0 = 1 << 31;
        let mask = library.get::<TerrainRockMaskSsbo>().unwrap();
        assert_eq!(sample_at_world_xz(bytemuck::cast_slice(&mask.mapped[..]), 65.0, 65.0), 1);
        assert_eq!(sample_at_world_xz(bytemuck::cast_slice(&mask.mapped[..]), -65.0, -65.0), 0);
    }
}
