//! `roof_tile_ssbo.rs` — `RoofTileSsbo` 稀疏实例缓冲元素（B
//! `resources/roof_tile_ssbo.rs`；canonical 定义位于原 `resources/roof_tile_ssbo`；旧 roofs 路径仅重导出）。
//!
//! 证据（fresh 反编译/机器码，2026-10-17 restorer-render）：
//!  * `SparseInstanceBuffer::<RoofTileSsbo>::new_storage_buffer` 0x1412e4a90：
//!    `memset(byte_map, 0, 0x2000)` + `[0x2000] = 0x20` 哨兵 → **CHUNK_COUNT = 0x2000**；
//!    空闲栈 0x2000 个 u32 降序 `0x1fff..0`；影子 `Box<[T]>` 长度 **0x40000**
//!    （`in_RCX[9] = 0x40000`）= CHUNK_COUNT × 32；active/dirty 位图各 0x80 字。
//!  * `TypedStorageBuffer::new<RoofTileSsbo>` 0x1416191a0：`0x40000 < len` 越界检查 →
//!    **BUFFER_LEN = 0x40000**（元素数）。
//!  * `SparseInstanceBufferWriter::push<RoofTileSsbo>` 0x141a3c070：槽 stride **0x40**、
//!    整块 8 qword 拷贝 → **元素 64 B**。
//!  * 字段序 = 着色器 `RoofShingle`（`shaders/roof_tiles_gravity.cs.hlsl` 的字节载入/写回，
//!    偏移 0/12/16/24/32/40/44/48/52/56/60 与 push 的 qword 序逐一对上）。
//!  * 类型名串 `country_core::resources::roof_tile_ssbo::RoofTileSsbo`（0x35 B，机器断言）。
//!
//! 存储形态：原版 **`SparseInstanceBuffer<RoofTileSsbo>`**（`AssetSsboLibrary::get_mut<
//! RoofTileSsbo>` → `gpu_flush<RoofTileSsbo>` 0x141a3af80；`get<RoofTileSsbo>` 的 vtable
//! `+0x18` = `as_any`），与 `WoodenPlankSsbo`（`roof_buffers.rs`）同模板。
use glam::Vec3;

use crate::asset_libraries::ssbo_library::SsboType;
use crate::render::sparse_instance_buffer::{SparseInstanceBuffer, SparseSsbo, SLOTS_PER_CHUNK};
use crate::render::ssbo::{rhapsody_boundary::GraphicsDevice, StorageElement};

/// 原版完整类型名（机器码断言用；模块路径字符串固定，不随本 crate 目录调整）。
pub const TYPE_NAME: &str = "country_core::resources::roof_tile_ssbo::RoofTileSsbo";

/// `RoofTileSsbo` 元素（= 着色器 `RoofShingle`，64 B，`#[repr(C)]` 逐偏移对齐）。
///
/// | 偏移 | 字段 | 类型 |
/// |---|---|---|
/// | 0x00 | `base_position` | `Vec3`（12 B） |
/// | 0x0c | `current_height` | `f32` |
/// | 0x10 | `clipping_plane` | `[u32; 2]` |
/// | 0x18 | `scale_t` | `[u32; 2]` |
/// | 0x20 | `rotation_quat` | `[u32; 2]` |
/// | 0x28 | `roof_id` | `u32` |
/// | 0x2c | `velocity_rotation` | `u32` |
/// | 0x30 | `animation_t` | `f32` |
/// | 0x34 | `seed` | `i32` |
/// | 0x38 | `is_circle` | `i32` |
/// | 0x3c | `is_main_roof_body` | `i32` |
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RoofTileSsbo {
    /// +0x00：瓦片基准位置（着色器 `Load<float3>(i*64+0)`）。
    pub base_position: Vec3,
    /// +0x0c。
    pub current_height: f32,
    /// +0x10：裁剪平面（两个 u32 位模式）。
    pub clipping_plane: [u32; 2],
    /// +0x18：压缩的缩放/时间。
    pub scale_t: [u32; 2],
    /// +0x20：压缩的旋转四元数。
    pub rotation_quat: [u32; 2],
    /// +0x28。
    pub roof_id: u32,
    /// +0x2c：低 16 位速度、高 16 位旋转（半精度打包）。
    pub velocity_rotation: u32,
    /// +0x30：动画时间。
    pub animation_t: f32,
    /// +0x34：随机种子（落地延迟哈希）。
    pub seed: i32,
    /// +0x38。
    pub is_circle: i32,
    /// +0x3c。
    pub is_main_roof_body: i32,
}

impl StorageElement for RoofTileSsbo {
    const NAME: &'static str = "RoofTileSsbo";
    /// O：`TypedStorageBuffer::new<RoofTileSsbo>` 的 `0x40000` 元素上限。
    const BUFFER_LEN: usize = 0x2000 * SLOTS_PER_CHUNK;
}

// SAFETY: 全零是合法实例（sparse 影子数组按 `SparseSsbo` 契约零初始化）。
unsafe impl SparseSsbo for RoofTileSsbo {
    /// O：0x1412e4a90 的 `memset` 长度 0x2000 + 哨兵。
    const CHUNK_COUNT: usize = 0x2000;
}

impl SsboType for RoofTileSsbo {
    type Storage = SparseInstanceBuffer<Self>;

    fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage {
        SparseInstanceBuffer::<Self>::new_storage_buffer(device, initial)
    }
}

/// Native typed ctor141616860:1000 records×32B, independent InstanceBuffer
/// count. Shader SPIR-V member offsets0/16/20/24/28 are retained exactly.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RoofSpireSsbo {
    pub position: glam::Vec4,
    pub roof_animation_t: f32,
    pub roof_profile: f32,
    pub radius: f32,
    pub pad1: i32,
}
impl StorageElement for RoofSpireSsbo {
    const NAME: &'static str = "RoofSpearTransforms";
    const BUFFER_LEN: usize = 1000;
}
impl SsboType for RoofSpireSsbo {
    type Storage = crate::render::ssbo::InstanceBuffer<Self>;
    fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage {
        crate::resources::roofs::roof_buffers::create_roof_instance_storage::<Self>(
            device, initial, 16,
        )
    }
}
impl crate::render::pass_builder::pass_resource_access::SsboType for RoofSpireSsbo {
    fn rg_buffer_hash() -> u64 {
        crate::asset_libraries::ssbo_library::SsboId::of::<Self>()
            .0
             .0
    }
}

#[cfg(test)]
mod native_instance_tests {
    use super::*;
    use crate::render::ssbo::rhapsody_boundary::{BackendError, Buffer, BufferDesc};
    use crate::{
        asset_libraries::ssbo_library::AssetSsboLibrary,
        resources::roofs::roof_buffers::RoofSupportBeamSsbo,
    };
    use core::ops::Range;
    use std::cell::RefCell;
    #[derive(Default)]
    struct Recorder(RefCell<Vec<(String, BufferDesc, Option<usize>)>>);
    impl GraphicsDevice for Recorder {
        fn create_buffer(
            &self,
            d: &BufferDesc,
            n: &str,
            b: Option<&[u8]>,
        ) -> Result<Buffer, BackendError> {
            self.0.borrow_mut().push((n.into(), *d, b.map(<[u8]>::len)));
            Ok(Buffer {
                byte_size: d.size as u64,
                ..Default::default()
            })
        }
        fn update_buffer_bytes(
            &self,
            _: &Buffer,
            _: &[u8],
            _: &[Range<usize>],
            _: usize,
        ) -> Result<(), BackendError> {
            unreachable!()
        }
    }
    #[test]
    fn native_roof_instance_constructors_preserve_empty_counts_and_no_upload() {
        let library = AssetSsboLibrary::new();
        let device = Recorder::default();
        library.add_new::<RoofSpireSsbo>(&device);
        library.add_new::<RoofSupportBeamSsbo>(&device);
        assert_eq!(library.instance_count::<RoofSpireSsbo>(), 0);
        assert_eq!(library.instance_count::<RoofSupportBeamSsbo>(), 0);
        let calls = device.0.borrow();
        assert_eq!(
            (
                calls[0].0.as_str(),
                calls[0].1.size,
                calls[0].1.usage,
                calls[0].1.memory,
                calls[0].1.alignment,
                calls[0].2
            ),
            ("RoofSpearTransforms", 32000, 0x22, 1, 16, None)
        );
        assert_eq!(
            (
                calls[1].0.as_str(),
                calls[1].1.size,
                calls[1].1.usage,
                calls[1].1.memory,
                calls[1].1.alignment,
                calls[1].2
            ),
            ("RoofSupportBeams", 0xa00000, 0x22, 1, 4, None)
        );
        assert_eq!(
            crate::asset_libraries::ssbo_library::SsboId::of::<RoofSpireSsbo>()
                .0
                 .0,
            0x3d1661290bab42e1
        );
        assert_eq!(
            crate::asset_libraries::ssbo_library::SsboId::of::<RoofSupportBeamSsbo>()
                .0
                 .0,
            0x83773598f5ecfead
        );
        assert_eq!(core::mem::size_of::<RoofSpireSsbo>(), 32);
        assert_eq!(core::mem::size_of::<RoofSupportBeamSsbo>(), 80);
        assert_eq!(
            core::any::type_name::<RoofTileSsbo>(),
            "country_core::resources::roof_tile_ssbo::RoofTileSsbo"
        );
        assert_eq!(
            core::any::TypeId::of::<RoofTileSsbo>(),
            core::any::TypeId::of::<crate::resources::roofs::roof_tile_ssbo::RoofTileSsbo>()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::ssbo::rhapsody_boundary::{Buffer, BufferDesc};
    use core::ops::Range;

    #[derive(Default)]
    struct Dev;

    impl GraphicsDevice for Dev {
        fn create_buffer(
            &self,
            desc: &BufferDesc,
            _name: &str,
            _data: Option<&[u8]>,
        ) -> Result<Buffer, crate::render::ssbo::rhapsody_boundary::BackendError> {
            Ok(Buffer {
                byte_size: desc.size as u64,
                ..Buffer::default()
            })
        }

        fn update_buffer_bytes(
            &self,
            _buffer: &Buffer,
            _data: &[u8],
            _ranges: &[Range<usize>],
            _offset: usize,
        ) -> Result<(), crate::render::ssbo::rhapsody_boundary::BackendError> {
            Ok(())
        }
    }

    #[test]
    fn element_is_64_bytes_with_the_shader_offsets() {
        assert_eq!(core::mem::size_of::<RoofTileSsbo>(), 0x40);
        assert_eq!(core::mem::offset_of!(RoofTileSsbo, base_position), 0x00);
        assert_eq!(core::mem::offset_of!(RoofTileSsbo, current_height), 0x0c);
        assert_eq!(core::mem::offset_of!(RoofTileSsbo, clipping_plane), 0x10);
        assert_eq!(core::mem::offset_of!(RoofTileSsbo, scale_t), 0x18);
        assert_eq!(core::mem::offset_of!(RoofTileSsbo, rotation_quat), 0x20);
        assert_eq!(core::mem::offset_of!(RoofTileSsbo, roof_id), 0x28);
        assert_eq!(core::mem::offset_of!(RoofTileSsbo, velocity_rotation), 0x2c);
        assert_eq!(core::mem::offset_of!(RoofTileSsbo, animation_t), 0x30);
        assert_eq!(core::mem::offset_of!(RoofTileSsbo, seed), 0x34);
        assert_eq!(core::mem::offset_of!(RoofTileSsbo, is_circle), 0x38);
        assert_eq!(core::mem::offset_of!(RoofTileSsbo, is_main_roof_body), 0x3c);
    }

    #[test]
    fn sparse_constants_match_the_machine_code() {
        assert_eq!(RoofTileSsbo::NAME, "RoofTileSsbo");
        assert_eq!(RoofTileSsbo::CHUNK_COUNT, 0x2000);
        assert_eq!(RoofTileSsbo::BUFFER_LEN, 0x40000);
        assert_eq!(TYPE_NAME.len(), 0x35);
        // 原版 SsboId = xxh3_64("RoofTileSsbo")。
        assert_eq!(
            crate::asset_libraries::ssbo_library::SsboId::of::<RoofTileSsbo>(),
            crate::asset_libraries::ssbo_library::SsboId(crate::name_hash::NameHash::compute(
                "RoofTileSsbo"
            ))
        );
    }

    #[test]
    fn sparse_storage_layout_matches_new_storage_buffer() {
        let dev = Dev;
        let b = SparseInstanceBuffer::<RoofTileSsbo>::new_storage_buffer(&dev, None);
        assert_eq!(b.byte_map.len(), 0x2001);
        assert_eq!(b.byte_map[0x2000], SLOTS_PER_CHUNK as u8);
        assert_eq!(b.data.len(), 0x40000);
        assert_eq!(b.active_word_count(), 0x80);
        assert_eq!(b.dirty_bitset.len(), 0x80);
        let free = b.free_chunks.read();
        assert_eq!(free.len(), 0x2000);
        assert_eq!((free[0], free[0x1fff]), (0x1fff, 0));
    }
}
