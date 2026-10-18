//! `indirect_params.rs` — GPU indirect-args 缓冲类型（`country_core::systems::render_loop::indirect_params`）。
//!
//! 原模块 **没有独立函数**（PDB functions=0），只有 4 个 SSBO 元素类型作为泛型实参出现在
//! `AssetSsboLibrary::{add_new,get_rg_buffer}<T>`、`render::ssbo::impl$0::new_storage_buffer<T>` 与
//! `ComputePass::{read,write}_storage_buffer[_concurrent]<T>` 里（O：procs.tsv 各 4 个实例）。
//!
//! 每个类型的 `NAME` 与缓冲规格由 `new_storage_buffer<T>` 与 `get_rg_buffer<T>` 机器码核对（O）：
//!
//! | 类型 | `get_rg_buffer` 的 SsboId 立即数 | = xxh3_64(NAME) | 缓冲字节（`BufferDesc::new`） | 元素 × 个数 |
//! |---|---|---|---|---|
//! | `AtomicCounterBuffer` (0x1412cb1e0) | 0x564703083181a121 | "AtomicCounterBuffer" | 8（0x1416148e0） | 4 B × 2 |
//! | `DrawIndirectBuffer` (0x1412d6130) | 0x324747c4b06c07f2 | "DrawIndirectBuffer" | 0x10（0x141610820） | 16 B × 1 |
//! | `DispatchIndirectBuffer` (0x1412d2170) | 0xfb06797fe67b2c94 | "DispatchIndirectBuffer" | 0x80（0x141613db0） | 16 B × 8 |
//! | `DrawIndexedIndirectBuffer` (0x1412d5ad0) | 0xf4a562f8a5a7ece7 | "DrawIndexedIndirectBuffer" | 0x1e0（0x141614d30） | 20 B × 24 |
//!
//! 用法/usage 常量 `0x122`（STORAGE|INDIRECT|TRANSFER_DST，I）、内存类型 1、对齐 4 在四者中一致（O）。
//! 初始内容见 `compute_passes::reset_indirect_args`（每帧由暂存区 copy 进来）。
//!
//! 与 `crate::resources::render_ssbo` 里同名的 `consume::buffers::*` 是两份类型（后者是旧占位，布局为 `Gap`）；
//! 集成者应把 `render_ssbo` 的同名类型改成 `pub use` 本模块（见 `evidence/country-core-audit/B_c3.md`）。
//!
//! `DrawIndirectBucket` / `DrawIndexedIndirectBucket`：PDB 里没有任何 `add_new/get_rg_buffer/new_storage_buffer`
//! 实例（O：procs.tsv 无），因此它们不是库 SSBO；保持旧占位（U）。

use crate::asset_libraries::ssbo_library::SsboType as LibrarySsbo;
use crate::render::ssbo::rhapsody_boundary::{BufferDesc, GraphicsDevice, slice_as_bytes};
use crate::render::ssbo::{StorageElement, TypedStorageBuffer};

/// Original producer usage, not the generic `TypedStorageBuffer::new` policy.
pub const INDIRECT_USAGE: u32 = 0x122;

fn create_indirect<T: StorageElement>(
    device: &dyn GraphicsDevice,
    initial: Option<&[u8]>,
    defaults: impl FnOnce() -> Option<Vec<u32>>,
) -> TypedStorageBuffer<T> {
    let element_size = core::mem::size_of::<T>();
    let size = element_size * T::BUFFER_LEN;
    if let Some(bytes) = initial {
        // Native order: element-byte divisibility, pointer alignment, capacity.
        assert_eq!(bytes.len() % element_size, 0);
        assert_eq!(bytes.as_ptr() as usize % 4, 0);
        assert!(
            bytes.len() / element_size <= T::BUFFER_LEN,
            "Excessive data size supplied for buffer {}",
            core::any::type_name::<T>()
        );
    }
    let owned;
    let data = match initial {
        Some(bytes) => Some(bytes),
        None => {
            owned = defaults();
            owned.as_deref().map(slice_as_bytes)
        }
    };
    let desc = BufferDesc::new(size, INDIRECT_USAGE, 1).alignment(4);
    let buffer = device
        .create_buffer(&desc, T::NAME, data)
        .expect("called `Result::unwrap()` on an `Err` value: create_buffer");
    TypedStorageBuffer::from_created_buffer(buffer)
}

macro_rules! indirect_ssbo {
    ($(#[$m:meta])* $name:ident($words:literal), $short:literal, $len:literal, $defaults:expr) => {
        $(#[$m])*
        #[repr(C)]
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
        pub struct $name(pub [u32; $words]);

        impl StorageElement for $name {
            const NAME: &'static str = $short;
            const BUFFER_LEN: usize = $len;
        }
        impl LibrarySsbo for $name {
            type Storage = TypedStorageBuffer<$name>;
            fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage {
                create_indirect::<Self>(device, initial, $defaults)
            }
        }
    };
}
#[allow(unused_imports)]
pub(crate) use indirect_ssbo;

indirect_ssbo!(
    /// Two u32 counter slots. Original None leaves contents uninitialized;
    /// reset_indirect_args subsequently writes [0, 0x90000].
    AtomicCounterBuffer(1), "AtomicCounterBuffer", 2, || None
);
indirect_ssbo!(
    /// 16-byte VkDrawIndirectCommand. Original startup uploads all zeros;
    /// per-frame reset subsequently supplies instance_count=1.
    DrawIndirectBuffer(4), "DrawIndirectBuffer", 1, || Some(vec![0; 4])
);
indirect_ssbo!(
    /// Eight 16-byte dispatch slots. Native startup uploads [0,1,1,0] x8
    /// (141613e40 loads the original constant at142aaa1e0).
    DispatchIndirectBuffer(4), "DispatchIndirectBuffer", 8,
    || Some((0..8).flat_map(|_| [0, 1, 1, 0]).collect())
);
indirect_ssbo!(
    /// 24 x20-byte indexed commands. Native startup zeros this array; the
    /// per-frame reset copies original index counts from142aaaa78 later.
    DrawIndexedIndirectBuffer(5), "DrawIndexedIndirectBuffer", 24,
    || Some(vec![0; 120])
);

/// `get_rg_buffer<T>` 里的 SsboId 立即数（O；= `NameHash::compute(T::NAME)`，xxh3_64 已离线复算吻合）。
pub mod ssbo_ids {
    pub const ATOMIC_COUNTER: u64 = 0x5647_0308_3181_a121;
    pub const DRAW_INDIRECT: u64 = 0x3247_47c4_b06c_07f2;
    pub const DISPATCH_INDIRECT: u64 = 0xfb06_797f_e67b_2c94;
    pub const DRAW_INDEXED_INDIRECT: u64 = 0xf4a5_62f8_a5a7_ece7;
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::size_of;

    fn total<T: StorageElement>() -> usize {
        size_of::<T>() * T::BUFFER_LEN
    }

    /// RNE producer evidence distinguishes startup bytes from the later reset.
    #[test]
    fn native_indirect_startup_uses_indirect_usage_and_exact_optional_uploads() {
        use crate::render::ssbo::rhapsody_boundary::{BackendError, Buffer};
        use std::{ops::Range, sync::Mutex};
        #[derive(Default)]
        struct Device(Mutex<Vec<(BufferDesc, Option<Vec<u8>>)>>);
        impl GraphicsDevice for Device {
            fn create_buffer(
                &self,
                desc: &BufferDesc,
                _: &str,
                data: Option<&[u8]>,
            ) -> Result<Buffer, BackendError> {
                self.0
                    .lock()
                    .unwrap()
                    .push((*desc, data.map(<[u8]>::to_vec)));
                Ok(Buffer {
                    byte_size: desc.size as u64,
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
                unreachable!("startup passes initial bytes to create_buffer")
            }
        }
        let device = Device::default();
        let _ = <AtomicCounterBuffer as LibrarySsbo>::new_storage_buffer(&device, None);
        let _ = <DrawIndirectBuffer as LibrarySsbo>::new_storage_buffer(&device, None);
        let _ = <DispatchIndirectBuffer as LibrarySsbo>::new_storage_buffer(&device, None);
        let _ = <DrawIndexedIndirectBuffer as LibrarySsbo>::new_storage_buffer(&device, None);
        let calls = device.0.lock().unwrap();
        for (desc, _) in &*calls {
            assert_eq!((desc.usage, desc.memory, desc.alignment), (0x122, 1, 4));
        }
        assert_eq!(calls[0].1, None, "atomic has no startup upload");
        assert_eq!(
            calls[1].1.as_deref(),
            Some(&[0; 16][..]),
            "startup draw instance_count is zero"
        );
        assert_eq!(
            calls[2].1.as_deref(),
            Some(
                crate::systems::render_loop::compute_passes::reset_indirect_args::dispatch_init()
                    .as_slice()
            )
        );
        assert_eq!(
            calls[3].1.as_deref(),
            Some(&[0; 0x1e0][..]),
            "startup indexed counts are zero before reset"
        );
        assert_eq!(
            (
                core::mem::size_of::<AtomicCounterBuffer>(),
                AtomicCounterBuffer::BUFFER_LEN
            ),
            (4, 2)
        );
    }

    /// 缓冲总字节 = `BufferDesc::new(size, 0x122, 1)` 的 size 实参（O）。
    #[test]
    fn buffer_sizes_match_new_storage_buffer_immediates() {
        assert_eq!(total::<AtomicCounterBuffer>(), 8);
        assert_eq!(total::<DrawIndirectBuffer>(), 0x10);
        assert_eq!(total::<DispatchIndirectBuffer>(), 0x80);
        assert_eq!(total::<DrawIndexedIndirectBuffer>(), 0x1e0);
    }

    #[test]
    fn names_are_the_type_names() {
        assert_eq!(AtomicCounterBuffer::NAME, "AtomicCounterBuffer");
        assert_eq!(DrawIndirectBuffer::NAME, "DrawIndirectBuffer");
        assert_eq!(DispatchIndirectBuffer::NAME, "DispatchIndirectBuffer");
        assert_eq!(DrawIndexedIndirectBuffer::NAME, "DrawIndexedIndirectBuffer");
    }

    #[test]
    fn ssbo_ids_are_distinct() {
        let ids = [
            ssbo_ids::ATOMIC_COUNTER,
            ssbo_ids::DRAW_INDIRECT,
            ssbo_ids::DISPATCH_INDIRECT,
            ssbo_ids::DRAW_INDEXED_INDIRECT,
        ];
        for (i, a) in ids.iter().enumerate() {
            assert!(ids[i + 1..].iter().all(|b| a != b));
        }
    }
}
