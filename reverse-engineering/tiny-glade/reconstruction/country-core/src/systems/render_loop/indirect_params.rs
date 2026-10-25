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
//! | `AtomicCounterBuffer` (0x1412cb1e0) | 0x564703083181a121 | "AtomicCounterBuffer" | 8（0x1416148e0） | 8 B × 1 |
//! | `DrawIndirectBuffer` (0x1412d6130) | 0x324747c4b06c07f2 | "DrawIndirectBuffer" | 0x10（0x141610820） | 16 B × 1 |
//! | `DispatchIndirectBuffer` (0x1412d2170) | 0xfb06797fe67b2c94 | "DispatchIndirectBuffer" | 0x80（0x141613db0） | 16 B × 8 (I) |
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
use crate::render::ssbo::rhapsody_boundary::GraphicsDevice;
use crate::render::ssbo::{StorageElement, TypedStorageBuffer};
use crate::typed_gap::Gap;

/// 声明一个固定个数的 SSBO 元素类型（`TypedStorageBuffer<T>` 存储；`default_value` 为全零首元素以外无逻辑）。
macro_rules! indirect_ssbo {
    ($(#[$m:meta])* $name:ident($words:literal), $short:literal, $len:literal) => {
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
                match initial {
                    None => TypedStorageBuffer::<$name>::new(device, None),
                    Some(bytes) => TypedStorageBuffer::<$name>::new_storage_buffer(device, Some(bytes)),
                }
            }
        }
    };
}
#[allow(unused_imports)]
pub(crate) use indirect_ssbo;

indirect_ssbo!(
    /// 原子计数缓冲：8 B（两个 u32；`reset_indirect_args` 写入 `[0, 0x90000]`）。
    AtomicCounterBuffer(2), "AtomicCounterBuffer", 1
);
indirect_ssbo!(
    /// `VkDrawIndirectCommand` 缓冲：16 B（`reset_indirect_args` 写入 `[0,1,0,0]`）。
    DrawIndirectBuffer(4), "DrawIndirectBuffer", 1
);
indirect_ssbo!(
    /// `VkDispatchIndirectCommand` 缓冲：8 个 16 B 槽（`x,y,z` + 填充；`reset_indirect_args` 写入 8×`[0,1,1,0]`）。
    DispatchIndirectBuffer(4), "DispatchIndirectBuffer", 8
);
indirect_ssbo!(
    /// `VkDrawIndexedIndirectCommand` 缓冲：24 条 × 20 B。
    DrawIndexedIndirectBuffer(5), "DrawIndexedIndirectBuffer", 24
);

/// `get_rg_buffer<T>` 里的 SsboId 立即数（O；= `NameHash::compute(T::NAME)`，xxh3_64 已离线复算吻合）。
pub mod ssbo_ids {
    pub const ATOMIC_COUNTER: u64 = 0x5647_0308_3181_a121;
    pub const DRAW_INDIRECT: u64 = 0x3247_47c4_b06c_07f2;
    pub const DISPATCH_INDIRECT: u64 = 0xfb06_797f_e67b_2c94;
    pub const DRAW_INDEXED_INDIRECT: u64 = 0xf4a5_62f8_a5a7_ece7;
}

/// 分桶 DrawIndirect 子数组（nani GPU-driven 分桶）。PDB 无库实例；布局 U。
#[derive(Clone, Copy, Debug, Default)]
pub struct DrawIndirectBucket(pub Gap);

/// 分桶 DrawIndexedIndirect 子数组。PDB 无库实例；布局 U。
#[derive(Clone, Copy, Debug, Default)]
pub struct DrawIndexedIndirectBucket(pub Gap);

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::size_of;

    fn total<T: StorageElement>() -> usize {
        size_of::<T>() * T::BUFFER_LEN
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
