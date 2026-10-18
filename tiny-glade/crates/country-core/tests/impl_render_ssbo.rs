//! impl-render-ssbo 验证探针（2026-10-17）。
//!
//! Env36（0x1409b7fa0）的 ranges 数据流：`AssetSsboLibrary::get::<SparkleSsbo>` →
//! `SparseInstanceBuffer<SparkleSsbo>::to_draw_multi_instanced`（每帧现算 `(first_instance, instance_count)`）
//! → 映射成 `[u32; 2]` 交给 `draw_multi_instanced`。本文件只经公开 API 验证该组合 + 元素类型合同；
//! 常量/布局单测在 `resources::sparkle::tests`。

use core::ops::Range;

use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::render::sparse_instance_buffer::{SparseInstanceBuffer, SparseSsbo};
use country_core::render::ssbo::rhapsody_boundary::{self, Buffer, BufferDesc, GraphicsDevice};
use country_core::render::ssbo::StorageElement;
use country_core::resources::sparkle::SparkleSsbo;

/// `TestDevice` 是 `#[cfg(test)]` 门控的，集成测试里用本地最小 mock（只覆盖创建路径）。
#[derive(Default)]
struct Dev;

impl GraphicsDevice for Dev {
    fn create_buffer(
        &self,
        desc: &BufferDesc,
        _name: &str,
        _data: Option<&[u8]>,
    ) -> Result<Buffer, rhapsody_boundary::BackendError> {
        Ok(Buffer {
            byte_size: desc.size as u64,
            ..Buffer::default()
        })
    }
    fn update_buffer_bytes(
        &self,
        _b: &Buffer,
        _d: &[u8],
        _r: &[Range<usize>],
        _o: usize,
    ) -> Result<(), rhapsody_boundary::BackendError> {
        Ok(())
    }
}

#[test]
fn sparkle_element_contract_matches_machine_code() {
    // O：影子 0xc0000 B / 0x6000 元素 → 32 B；byte_map 长 0x301（哨兵 @0x300=0x20）→ CHUNK_COUNT 0x300。
    assert_eq!(core::mem::size_of::<SparkleSsbo>(), 32);
    assert_eq!(SparkleSsbo::BUFFER_LEN, 0x6000);
    assert_eq!(SparkleSsbo::CHUNK_COUNT, 0x300);
    // 字段布局 = `sparkles.raster.hlsl` 的 SparkleInstanceData。
    assert_eq!(core::mem::offset_of!(SparkleSsbo, position), 0x00);
    assert_eq!(core::mem::offset_of!(SparkleSsbo, scale), 0x0c);
    assert_eq!(core::mem::offset_of!(SparkleSsbo, time_offset), 0x10);
    assert_eq!(core::mem::offset_of!(SparkleSsbo, speed_mult), 0x14);
    assert_eq!(core::mem::offset_of!(SparkleSsbo, color_id), 0x18);
    assert_eq!(core::mem::offset_of!(SparkleSsbo, pad1), 0x1c);
    // SsboId = xxh3_64("SparkleSsbo") = Env36 FrozenMap::get 键（0x142c2e3f0）。
    assert_eq!(country_core::asset_libraries::ssbo_library::SsboId::of::<SparkleSsbo>().0 .0, 0x2bf3b1a4ac675864);
}

#[test]
fn sparse_buffer_layout_matches_the_machine_code() {
    let dev = Dev;
    let b = SparseInstanceBuffer::<SparkleSsbo>::new_storage_buffer(&dev, None);
    assert_eq!(b.byte_map.len(), 0x301);
    assert_eq!(b.byte_map[0x300], 0x20);
    assert!(b.byte_map[..0x300].iter().all(|&x| x == 0));
    assert_eq!(b.data.len(), 0x6000);
    assert_eq!(b.active_bitset.len(), 0xc);
    assert_eq!(b.dirty_bitset.len(), 0xc);
    let free = b.free_chunks.read();
    assert_eq!(free.len(), 0x300);
    assert_eq!((free[0], free[0x2ff]), (0x2ff, 0), "降序，栈顶 = 0");
}

#[test]
fn env36_ranges_come_from_the_sparkle_sparse_buffer_each_frame() {
    let mut lib = AssetSsboLibrary::new();
    lib.add_new::<SparkleSsbo>(&Dev);
    // 写 35 个实例（chunk 0 满 + chunk 1 3 个）。分配销毁只进 waste，active 位图保留到 take_out_waste。
    let b = lib.get_mut::<SparkleSsbo>().unwrap();
    let a = b.new_allocation();
    {
        let mut wr = b.writer(&a);
        for _ in 0..35 {
            wr.push(&SparkleSsbo::default()).unwrap();
        }
    }
    drop(a);
    // env36 的现算路径。
    let ranges = lib.get::<SparkleSsbo>().unwrap().to_draw_multi_instanced();
    let ranges: Vec<[u32; 2]> = ranges.into_iter().map(|(first, count)| [first, count]).collect();
    assert_eq!(ranges, vec![[0, 35]]);
}
