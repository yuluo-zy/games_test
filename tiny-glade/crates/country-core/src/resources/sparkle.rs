//! `resources::sparkle` — 火花粒子稀疏实例元素（`SparkleSsbo`）。
//!
//! Env36（0x1409b7fa0）的稀疏实例缓冲元素（O，2026-10-17 impl-render-ssbo fresh 反编译）：
//! * `new_storage_buffer<SparkleSsbo>` 0x1412e5f50：byte_map 长 0x301（`[0x300] = 0x20` 哨兵 →
//!   `CHUNK_COUNT = 0x300`）、影子 0xc0000 B / 0x6000 元素（→ `size_of::<Self> = 32`）、
//!   free 栈 `[0x2ff .. 0]` 降序、active/dirty 各 0xc 字（0x300 / 64）。
//! * `push<SparkleSsbo>` 0x14120c710：整块拷贝 4 个 qword = 32 B。
//! * Env36 的 `FrozenMap::get` 键 = 0x142c2e3f0 处 u64 0x2bf3b1a4ac675864 = `xxh3_64("SparkleSsbo")`；
//!   Env36 经 `as_any`（Ssbo 虚表 +0x18）取 `SparseInstanceBuffer<SparkleSsbo>`，调
//!   `to_draw_multi_instanced`（0x1412e4250，与 `<RoofTileSsbo>` 同形，ICF 合并为同名符号）每帧现算 ranges。
//! * 字段布局 = 重建 HLSL `shaders/sparkles.raster.hlsl` 的 `SparkleInstanceData`
//!   （`StructuredBuffer<SparkleInstanceData> instanced_data : register(t14)`；32 B 与机器码吻合）：
//!   `position: float3 @0`、`scale: float @0xc`、`time_offset: float @0x10`、
//!   `speed_mult: float @0x14`、`color_id: int @0x18`、`pad1: float @0x1c`。
//! * 写面（火花粒子系统，critters_particles 域，库内未建）不在本文件：本文件只落地元素类型 +
//!   `SparseSsbo`/`SsboType` 合同，供 `draw_callbacks_b::env36` 经 `AssetSsboLibrary::get` 取缓冲、
//!   每帧 `to_draw_multi_instanced` 现算绘制 ranges（替代原先的 `SparkleDrawRangesView` 预计算视图）。

use glam::Vec3;

use crate::asset_libraries::ssbo_library::SsboType;
use crate::render::sparse_instance_buffer::{SLOTS_PER_CHUNK, SparseInstanceBuffer, SparseSsbo};
use crate::render::ssbo::{StorageElement, rhapsody_boundary::GraphicsDevice};

/// 火花粒子实例（O：32 B；`SparkleInstanceData`，字段偏移见模块注释）。
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SparkleSsbo {
    /// +0x00。
    pub position: Vec3,
    /// +0x0c。
    pub scale: f32,
    /// +0x10。
    pub time_offset: f32,
    /// +0x14。
    pub speed_mult: f32,
    /// +0x18。
    pub color_id: i32,
    /// +0x1c。
    pub pad1: f32,
}

impl StorageElement for SparkleSsbo {
    const NAME: &'static str = "SparkleSsbo";
    /// O：`CHUNK_COUNT * 32` = 0x300 * 0x20 = 0x6000（影子 0xc0000 B / 0x6000 元素）。
    const BUFFER_LEN: usize = 0x300 * SLOTS_PER_CHUNK;
}

// SAFETY: 全零是合法实例（稀疏影子数组 memset 零初始化；`SparseSsbo` 契约）。
unsafe impl SparseSsbo for SparkleSsbo {
    /// O：`new_storage_buffer<SparkleSsbo>` 的 byte_map 长 0x301（哨兵 @0x300）。
    const CHUNK_COUNT: usize = 0x300;
}

impl SsboType for SparkleSsbo {
    type Storage = SparseInstanceBuffer<Self>;
    fn new_storage_buffer(
        device: &dyn GraphicsDevice,
        initial: Option<&[u8]>,
    ) -> Self::Storage {
        SparseInstanceBuffer::<Self>::new_storage_buffer(device, initial)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::test_device::TestDevice;
    use core::sync::atomic::Ordering;
    use xxhash_rust::xxh3::xxh3_64;

    #[test]
    fn constants_match_machine_code() {
        assert_eq!(core::mem::size_of::<SparkleSsbo>(), 32);
        assert_eq!(core::mem::offset_of!(SparkleSsbo, position), 0x00);
        assert_eq!(core::mem::offset_of!(SparkleSsbo, scale), 0x0c);
        assert_eq!(core::mem::offset_of!(SparkleSsbo, time_offset), 0x10);
        assert_eq!(core::mem::offset_of!(SparkleSsbo, speed_mult), 0x14);
        assert_eq!(core::mem::offset_of!(SparkleSsbo, color_id), 0x18);
        assert_eq!(core::mem::offset_of!(SparkleSsbo, pad1), 0x1c);
        assert_eq!(xxh3_64(b"SparkleSsbo"), 0x2bf3b1a4ac675864);
        assert_eq!(SparkleSsbo::BUFFER_LEN, 0x6000);
        assert_eq!(SparkleSsbo::CHUNK_COUNT, 0x300);
    }

    #[test]
    fn sparse_buffer_layout_matches_the_machine_code() {
        let dev = TestDevice(1);
        let b = SparseInstanceBuffer::<SparkleSsbo>::new_storage_buffer(&dev, None);
        assert_eq!(b.byte_map.len(), 0x301);
        assert_eq!(b.byte_map[0x300], 0x20, "哨兵 chunk 恒满");
        assert!(b.byte_map[..0x300].iter().all(|&x| x == 0));
        assert_eq!(b.data.len(), 0x6000);
        assert_eq!(b.active_bitset.len(), 0xc);
        assert_eq!(b.dirty_bitset.len(), 0xc);
        let free = b.free_chunks.read();
        assert_eq!(free.len(), 0x300);
        assert_eq!((free[0], free[0x2ff]), (0x2ff, 0), "降序，栈顶 = 0");
    }

    #[test]
    fn writer_push_and_draw_ranges_are_8_byte_pairs() {
        let dev = TestDevice(1);
        let mut b = SparseInstanceBuffer::<SparkleSsbo>::new_storage_buffer(&dev, None);
        let a = b.new_allocation();
        {
            let mut w = b.writer(&a);
            for _ in 0..35 {
                w.push(&SparkleSsbo::default()).unwrap();
            }
        }
        assert_eq!(b.byte_map[0], 32);
        assert_eq!(b.byte_map[1], 3);
        assert_eq!(b.active_bitset[0].load(Ordering::Relaxed), 0b11);
        assert_eq!(b.to_draw_multi_instanced(), vec![(0, 35)]);
        assert_eq!(b.allocation_to_draw_multi_instanced(&a), vec![(0, 35)]);
    }
}
