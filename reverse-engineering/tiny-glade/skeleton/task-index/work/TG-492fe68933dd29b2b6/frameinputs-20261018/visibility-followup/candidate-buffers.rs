//! buffers.rs — nani 绘制参数与 SSBO（B 阶段按机器码重写，n_world）。
//!
//! 机器码锚点（B patched EXE cf048e27…）：
//! * `nani_calculate_draw_list_params` 0x1408baed0（1918B，O）：`enum_iterator::All<GpuWorldSubsetKind>`
//!   展开成跳表（0x142aa15a8，27 项，逐项 `next = k + 1`，O）；每个 kind 读 `DAT_142aa14c0[kind]`（u32×27）。
//! * `impl$9::default_value` 0x1408bb650（1392B，O）：`StorageElement for NaniDrawListOffsetDefaults`
//!   （PDB：`read_access_buffer<NaniDrawListOffsetDefaults>`、ssbo.rs 注释同名）。跳表 0x142aa1618，
//!   同样逐项 `next = k + 1`；每个 kind 写 4 个 u32：`base, base+s, base+2s, base+3s`，`base += 4s`，
//!   其中 `s` 恰为 `DAT_142aa14c0[kind]`（27 项逐项核对相等，O）。结果 `{tag = 1 (Some), [u32; 108]}`。
//! * `drop_in_place<NaniDrawListParams>` 0x14088fed0：4 个 `Vec<u32>` 的释放。
//!
//! 旧版把 `nani_calculate_draw_list_params` 与 `default_value` 说成“同一代码体的两个入口”，是 Ghidra 跳表
//! 误读：二者是两个独立函数（地址范围 0x1408baed0+0x77e 与 0x1408bb650+0x570 不重叠）。

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::doc_lazy_continuation)]
#![allow(dead_code, unused_imports)]

use alloc::vec::Vec;

use crate::render::ssbo::StorageElement;

/// 每 kind 每 draw-list 的最大 draw 数（O：`DAT_142aa14c0`，u32×27，逐值从 EXE 读出）。
/// 同一张表也是 `GpuWorldSubsetKind::max_instances`（`DAT_142aba790` / `DAT_142c77010`，三份内容逐值相同）。
pub const MAX_DRAWS_PER_LIST: [u32; 27] = [
    8192, 8192, 4096, 32768, 8192, 8192, 8192, 8192, 262144, 40960, 1024, 53120, 9600, 131072, 512,
    8192, 4096, 1024, 32768, 65536, 2048, 1024, 2048, 131072, 81920, 256, 32768,
];

/// 每个 subset kind 的 draw list 数（O：循环体展开 4 份；`list_index = kind*4 | 变体`）。
pub const LISTS_PER_KIND: usize = 4;
/// 细粒度排序每批的 draw 数 = `1 << 5`（O：`shr edi, 5`）。
pub const FINE_GRAINED_SHIFT: u32 = 5;
/// 未启用细粒度时，kind ≥ 2 的 draw 数钳制值（O：`mov edi, 0x40`）。
pub const NON_FINE_GRAINED_DRAWS: u32 = 0x40;

/// `NaniDrawListParams`（0x70B，O：`drop_in_place` 0x14088fed0 释放 4 个 Vec）。
/// 字段顺序即返回缓冲偏移：+0x00/0x18/0x30/0x48 四个 `Vec<u32>`，+0x60/0x64/0x68/0x6c 四个 u32 汇总。
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct NaniDrawListParams {
    /// +0x00：每 kind 的 draw-list 起始偏移（前缀和，`MAX_DRAWS_PER_LIST` 累加）。
    pub cmd_offsets: Vec<u32>,
    /// +0x18：每 kind 的实例分配起始偏移（前缀和；累加的是钳制后的 draw 数）。
    /// `GpuWorld::new` 把这个 Vec 克隆进 `TypedStorageBuffer<NaniDrawListInfo>`（O）。
    pub max_instance_offsets: Vec<u32>,
    /// +0x30：每 kind 的细粒度子列表起始偏移（前缀和）。
    pub fine_grained_offsets: Vec<u32>,
    /// +0x48：每 kind 的批次起始偏移（前缀和）。
    pub batch_offsets: Vec<u32>,
    /// +0x60：`cmd_offsets` 总和。
    pub total_cmd_offset: u32,
    /// +0x64：`max_instance_offsets` 总和。
    pub total_max_instances: u32,
    /// +0x68：`fine_grained_offsets` 总和。
    pub total_fine_grained: u32,
    /// +0x6c：`batch_offsets` 总和。
    pub total_batches: u32,
}

/// 旧名（lib.rs 重导出沿用）。
pub type DrawListParams = NaniDrawListParams;

/// 计算 27 种 subset 的绘制参数（0x1408baed0，O）。对 `k in 0..27`（`T = MAX_DRAWS_PER_LIST[k]`），每个 kind 重复 4 次（4 个 draw list）：
/// 1. `cmd_offsets.push(acc_cmd); acc_cmd += T`；
/// 2. `draws = T`，若 `!FORCE_FINE && (k >= 2 || FORCE_NO_FINE)` 则 `draws = 0x40`；
///    `max_instance_offsets.push(acc_inst); acc_inst += draws`；
/// 3. `fine = draws >> 5`，若 `!FORCE_FINE && (FORCE_NO_FINE || k >= 2)` 则 `fine = 0`；
///    `fine_grained_offsets.push(acc_fine); acc_fine += fine`；
/// 4. `batch_offsets.push(acc_batch); acc_batch += (fine >> 10) + ((fine & 0x3fe) != 0)`。
///    （机器码掩码是 `0x3fe` 而不是 `0x3ff`：`test ebx, 0x3fe; setne`，照抄，O。）
/// 开关：0x143634d9c = `NANI_FORCE_FINE_GRAINED_SORTING`，0x143634d9d = `NANI_FORCE_NO_FINE_GRAINED_SORTING`
/// （按 `use_fine_grained_sorting` 的既有命名核对：前者置位时全部取 T 与 `T>>5`，O）。
pub fn nani_calculate_draw_list_params() -> NaniDrawListParams {
    calculate_with_flags(
        crate::render::nani::NANI_FORCE_FINE_GRAINED_SORTING,
        crate::render::nani::NANI_FORCE_NO_FINE_GRAINED_SORTING,
    )
}

/// 与 `nani_calculate_draw_list_params` 同逻辑，开关显式传入（测试用；原版读两个静态字节）。
pub fn calculate_with_flags(force_fine: bool, force_no_fine: bool) -> NaniDrawListParams {
    let mut p = NaniDrawListParams::default();
    let (mut acc_cmd, mut acc_inst, mut acc_fine, mut acc_batch) = (0u32, 0u32, 0u32, 0u32);
    for (k, &t) in MAX_DRAWS_PER_LIST.iter().enumerate() {
        // O（B_i_nani 复核）：循环体在机器码里展开 4 份（0x1408bb0bc / 0x1408bb1f6 / 0x1408bb336 / 0x1408bb475
        // 各起一份，只有第 4 份末尾经跳表 `jmp rax` 进入下一个 kind）：每个 kind 往四个 Vec 各 push 4 次，
        // 即每个 kind 有 4 个 draw list（`nani_draw` 的 `list_index = kind*4 | pass 变体`）。
        // Vec 长度因此是 27*4 = 108（= `NaniDrawListInfo` 缓冲 0x1b0 B = `NaniDrawListOffsetDefaults` 的大小）。
        for _list in 0..LISTS_PER_KIND {
            p.cmd_offsets.push(acc_cmd);
            acc_cmd += t;

            // `cl = (k >= 2) | FORCE_NO_FINE`；`FORCE_FINE` 优先。
            let clamp = !force_fine && (k >= 2 || force_no_fine);
            let draws = if clamp { NON_FINE_GRAINED_DRAWS } else { t };
            p.max_instance_offsets.push(acc_inst);
            acc_inst += draws;

            let fine = if !force_fine && (force_no_fine || k >= 2) {
                0
            } else {
                draws >> FINE_GRAINED_SHIFT
            };
            p.fine_grained_offsets.push(acc_fine);
            acc_fine += fine;

            p.batch_offsets.push(acc_batch);
            acc_batch += (fine >> 10) + u32::from(fine & 0x3fe != 0);
        }
    }
    p.total_cmd_offset = acc_cmd;
    p.total_max_instances = acc_inst;
    p.total_fine_grained = acc_fine;
    p.total_batches = acc_batch;
    p
}

/// `NaniDrawListOffsetDefaults`（0x1b0B = `[u32; 108]`，O）：每 kind 4 个偏移（27 × 4）。
/// `BUFFER_LEN == 1`（uniform/单结构体）；`default_value` 即 0x1408bb650。
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, bytemuck::NoUninit)]
pub struct NaniDrawListOffsetDefaults {
    pub offsets: [u32; 108],
}

impl NaniDrawListOffsetDefaults {
    /// 0x1408bb650 的计算体：`offsets[4k + j] = base_k + j * T[k]`，`base_{k+1} = base_k + 4 * T[k]`（O）。
    pub fn compute() -> Self {
        let mut offsets = [0u32; 108];
        let mut base = 0u32;
        for (k, &s) in MAX_DRAWS_PER_LIST.iter().enumerate() {
            for j in 0..4u32 {
                offsets[4 * k + j as usize] = base + j * s;
            }
            base += 4 * s;
        }
        Self { offsets }
    }
}

impl StorageElement for NaniDrawListOffsetDefaults {
    const NAME: &'static str = "NaniDrawListOffsetDefaults";
    const BUFFER_LEN: usize = 1;
    /// `impl$9::default_value`：返回 `{tag = 1, compute()}`（`Option::Some`，O）。
    fn default_value() -> Option<Self> {
        Some(Self::compute())
    }
}

/// Original add_new<NaniDrawListOffsetDefaults>@1412bdff0 allocates 48B,
/// unlike the three fixed counters' 40B TypedStorageBuffer. Default producer
/// @1408bb650 supplies one actual 432B prefix-offset record, count=1.
impl crate::asset_libraries::ssbo_library::SsboType for NaniDrawListOffsetDefaults {
    type Storage = crate::render::ssbo::InstanceBuffer<Self>;
    fn new_storage_buffer(
        device: &dyn crate::render::ssbo::rhapsody_boundary::GraphicsDevice,
        initial: Option<&[u8]>,
    ) -> Self::Storage {
        crate::render::ssbo::InstanceBuffer::new_storage_buffer(device, initial)
    }
}
impl crate::render::pass_builder::pass_resource_access::SsboType for NaniDrawListOffsetDefaults {
    fn rg_buffer_hash() -> u64 {
        crate::asset_libraries::ssbo_library::SsboId::of::<Self>()
            .0
             .0
    }
}

// Original B byte constructors are retained in this file family's evidence
// pack. This source adapter uses the current device API; it does not reproduce
// rhapsody's internal representation. None on a fixed counter uploads nothing.
fn create_nani_storage<T: StorageElement>(
    device: &dyn crate::render::ssbo::rhapsody_boundary::GraphicsDevice,
    initial: Option<&[u8]>,
    usage: u32,
) -> crate::render::ssbo::TypedStorageBuffer<T> {
    use crate::render::ssbo::{rhapsody_boundary::BufferDesc, TypedStorageBuffer};
    let element_size = core::mem::size_of::<T>();
    if let Some(bytes) = initial {
        assert_eq!(bytes.len() % element_size, 0);
        assert_eq!(bytes.as_ptr() as usize % 4, 0);
        if T::BUFFER_LEN != 0 && bytes.len() / element_size > T::BUFFER_LEN {
            panic!(
                "Excessive data size supplied for buffer {}",
                core::any::type_name::<T>()
            );
        }
    } else if T::BUFFER_LEN == 0 {
        panic!("Storage buffer {} (type {}) has `BUFFER_LEN` of zero. Such buffers must be created with initial data.", T::NAME, core::any::type_name::<T>());
    }
    let size = if T::BUFFER_LEN == 0 {
        initial
            .expect("dynamic buffer requires original initial data")
            .len()
    } else {
        T::BUFFER_LEN * element_size
    };
    let desc = BufferDesc::new(size, usage, 1).alignment(4);
    let buffer = device
        .create_buffer(&desc, T::NAME, initial)
        .expect("called `Result::unwrap()` on an `Err` value: create_buffer");
    TypedStorageBuffer::from_created_buffer(buffer)
}

macro_rules! nani_storage {
    ($(#[$doc:meta])* $name:ident($words:literal), $len:literal, $usage:literal) => {
        $(#[$doc])*
        #[repr(C)]
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, bytemuck::NoUninit)]
        pub struct $name(pub [u32; $words]);
        impl StorageElement for $name {
            const NAME: &'static str = stringify!($name);
            const BUFFER_LEN: usize = $len;
        }
        impl crate::asset_libraries::ssbo_library::SsboType for $name {
            type Storage = crate::render::ssbo::TypedStorageBuffer<Self>;
            fn new_storage_buffer(
                device: &dyn crate::render::ssbo::rhapsody_boundary::GraphicsDevice,
                initial: Option<&[u8]>,
            ) -> Self::Storage {
                create_nani_storage::<Self>(device, initial, $usage)
            }
        }
        impl crate::render::pass_builder::pass_resource_access::SsboType for $name {
            fn rg_buffer_hash() -> u64 {
                crate::asset_libraries::ssbo_library::SsboId::of::<Self>().0.0
            }
        }
    };
}

nani_storage!(
    /// Original constructor@141612870: 108 u32 slots, 432B, usage0x122;
    /// None creates uninitialized storage, not a fabricated zero upload.
    NaniDrawListLengthSsbo(1), 108, 0x122
);
nani_storage!(
    /// Original constructor@141613500: 108 u32 allocator slots, usage0x22.
    NaniInstanceIndirectionAllocatorSsbo(1), 108, 0x22
);
nani_storage!(
    /// Original constructor@141612650: 108 u32 allocator slots, usage0x22.
    NaniFilteredInstanceAllocatorSsbo(1), 108, 0x22
);
nani_storage!(
    /// Original add_new_with_data@1412bb2f0 scales element count by20.
    /// Preserve five words without guessing unrecovered native field names.
    /// Constructor@141613940 requires initial bytes and usage0x122.
    NaniDrawListSsbo(5), 0, 0x122
);
nani_storage!(
    /// Original@14160ffc0: dynamic u32 table, actual params' second Vec.
    NaniListDrawCmdOffsets(1), 0, 0x22
);
nani_storage!(
    /// Original@1416132d0: dynamic u32 table, initial zero records supplied
    /// by init_populate rather than a fixed-capacity constructor default.
    NaniInstanceIndirectionSsbo(1), 0, 0x22
);

/// Bounded Nani portion of init_populate_ssbo_library@140909e80. The full
/// original function also registers 65 other entries and maps CustomCursor;
/// this explicit subset is not a replacement for that full startup system.
/// Actual data: draw-list zeros[params+64], offsets=params second Vec,
/// indirection zeros[params+60]. No empty vector stands in for these lengths.
pub fn populate_nani_ssbo_library(
    library: &crate::asset_libraries::ssbo_library::AssetSsboLibrary,
    device: &dyn crate::render::ssbo::rhapsody_boundary::GraphicsDevice,
) {
    library.add_new::<NaniDrawListLengthSsbo>(device);
    library.add_new::<NaniDrawListOffsetDefaults>(device);
    library.add_new::<NaniInstanceIndirectionAllocatorSsbo>(device);
    library.add_new::<NaniFilteredInstanceAllocatorSsbo>(device);
    let params = nani_calculate_draw_list_params();
    let draw_list = vec![NaniDrawListSsbo::default(); params.total_max_instances as usize];
    library.add_new_with_data(device, &draw_list);
    let offsets: Vec<_> = params
        .max_instance_offsets
        .iter()
        .map(|&word| NaniListDrawCmdOffsets([word]))
        .collect();
    library.add_new_with_data(device, &offsets);
    let indirection =
        vec![NaniInstanceIndirectionSsbo::default(); params.total_cmd_offset as usize];
    library.add_new_with_data(device, &indirection);
}

// Original startup_nani@140a395ab/5c0 registers these independently of the
// 72-entry SSBO population. Both use a dynamic u32 vector; None is rejected,
// including on an otherwise empty World. The producer supplies zero words
// [GpuWorld.params.total_cmd_offset >> 5], in Prev then Next order.
nani_storage!(
    /// Native byte constructor14160fd90: actual byte length, usage22/mem1/align4.
    NaniPrevVisibilityBitmaskSsbo(1), 0, 0x22
);
nani_storage!(
    /// Native byte constructor141614b00: same policy, independent library key.
    NaniNextVisibilityBitmaskSsbo(1), 0, 0x22
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_nani_storage_layouts_replace_zero_sized_carriers() {
        use crate::render::ssbo::{InstanceBuffer, TypedStorageBuffer};
        use core::mem::{align_of, size_of};
        assert_eq!(size_of::<NaniDrawListLengthSsbo>(), 4);
        assert_eq!(size_of::<NaniInstanceIndirectionAllocatorSsbo>(), 4);
        assert_eq!(size_of::<NaniFilteredInstanceAllocatorSsbo>(), 4);
        assert_eq!(size_of::<NaniDrawListSsbo>(), 20);
        assert_eq!(size_of::<NaniListDrawCmdOffsets>(), 4);
        assert_eq!(size_of::<NaniInstanceIndirectionSsbo>(), 4);
        assert_eq!(align_of::<NaniDrawListSsbo>(), 4);
        assert_eq!(size_of::<TypedStorageBuffer<NaniDrawListLengthSsbo>>(), 40);
        assert_eq!(size_of::<InstanceBuffer<NaniDrawListOffsetDefaults>>(), 48);
        assert_eq!(NaniDrawListLengthSsbo::BUFFER_LEN, 108);
        assert_eq!(NaniDrawListSsbo::BUFFER_LEN, 0);
    }

    #[test]
    fn native_nani_startup_publishes_real_lengths_offsets_and_constructor_policies() {
        use crate::{
            asset_libraries::ssbo_library::AssetSsboLibrary,
            render::ssbo::rhapsody_boundary::{BackendError, Buffer, BufferDesc, GraphicsDevice},
        };
        use core::ops::Range;
        use std::cell::RefCell;
        #[derive(Debug)]
        struct Creation {
            name: String,
            desc: BufferDesc,
            data_len: Option<usize>,
            first_words: Vec<u32>,
            all_zero: bool,
        }
        // CPU boundary recorder: it confirms calls/data and owns no GPU memory.
        #[derive(Default)]
        struct Device(RefCell<Vec<Creation>>);
        impl GraphicsDevice for Device {
            fn create_buffer(
                &self,
                desc: &BufferDesc,
                name: &str,
                data: Option<&[u8]>,
            ) -> Result<Buffer, BackendError> {
                let mut calls = self.0.borrow_mut();
                calls.push(Creation {
                    name: name.into(),
                    desc: *desc,
                    data_len: data.map(<[u8]>::len),
                    first_words: data.map_or_else(Vec::new, |bytes| {
                        bytes
                            .chunks_exact(4)
                            .take(108)
                            .map(|word| u32::from_le_bytes(word.try_into().unwrap()))
                            .collect()
                    }),
                    all_zero: data.is_some_and(|bytes| bytes.iter().all(|byte| *byte == 0)),
                });
                Ok(Buffer {
                    id: calls.len() as u64,
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
                unreachable!("original Nani startup supplies bytes at creation")
            }
        }
        let device = Device::default();
        let library = AssetSsboLibrary::new();
        populate_nani_ssbo_library(&library, &device);
        let calls = device.0.borrow();
        assert_eq!(calls.len(), 7);
        assert_eq!(
            calls
                .iter()
                .map(|call| call.name.as_str())
                .collect::<Vec<_>>(),
            [
                "NaniDrawListLengthSsbo",
                "NaniDrawListOffsetDefaults",
                "NaniInstanceIndirectionAllocatorSsbo",
                "NaniFilteredInstanceAllocatorSsbo",
                "NaniDrawListSsbo",
                "NaniListDrawCmdOffsets",
                "NaniInstanceIndirectionSsbo",
            ]
        );
        for (index, size, usage, initial) in [
            (0, 432, 0x122, None),
            (1, 432, 0x12, Some(432)),
            (2, 432, 0x22, None),
            (3, 432, 0x22, None),
            (4, 71_936 * 20, 0x122, Some(71_936 * 20)),
            (5, 108 * 4, 0x22, Some(108 * 4)),
            (6, 3_788_800 * 4, 0x22, Some(3_788_800 * 4)),
        ] {
            let call = &calls[index];
            assert_eq!(
                (
                    call.desc.size,
                    call.desc.usage,
                    call.desc.memory,
                    call.desc.alignment
                ),
                (size, usage, 1, 4)
            );
            assert_eq!(call.data_len, initial);
        }
        assert_eq!(
            calls[1].first_words,
            NaniDrawListOffsetDefaults::compute().offsets
        );
        assert_eq!(
            calls[5].first_words,
            nani_calculate_draw_list_params().max_instance_offsets
        );
        assert_ne!(calls[5].first_words[9], calls[1].first_words[9]);
        assert!(calls[4].all_zero && calls[6].all_zero);
        assert_eq!(
            library
                .get::<NaniDrawListOffsetDefaults>()
                .unwrap()
                .instance_count,
            1
        );
        assert_eq!(
            library.get_buffer::<NaniDrawListSsbo>().unwrap().byte_size,
            71_936 * 20
        );
        assert_eq!(
            library
                .get_buffer::<NaniInstanceIndirectionSsbo>()
                .unwrap()
                .byte_size,
            3_788_800 * 4
        );
    }

    #[test]
    fn table_matches_exe_constants() {
        assert_eq!(MAX_DRAWS_PER_LIST.len(), 27);
        assert_eq!(MAX_DRAWS_PER_LIST[0], 8192);
        assert_eq!(MAX_DRAWS_PER_LIST[8], 262_144);
        assert_eq!(MAX_DRAWS_PER_LIST[11], 53_120);
        assert_eq!(MAX_DRAWS_PER_LIST[25], 256);
        assert_eq!(
            MAX_DRAWS_PER_LIST
                .iter()
                .map(|&v| u64::from(v))
                .sum::<u64>(),
            947_200
        );
    }

    #[test]
    fn draw_list_params_default_flags() {
        let p = calculate_with_flags(false, false);
        // 每 kind 4 个 draw list：4 个 Vec 长度都是 27*4 = 108。
        assert_eq!(p.cmd_offsets.len(), 108);
        assert_eq!(p.max_instance_offsets.len(), 108);
        assert_eq!(p.fine_grained_offsets.len(), 108);
        assert_eq!(p.batch_offsets.len(), 108);
        assert_eq!(&p.cmd_offsets[..5], &[0, 8192, 16384, 24576, 32768]);
        // kind 0/1 保留 T，kind>=2 钳到 0x40：kind 1 的 4 个 list 占 8192*4，kind 2 起每个 list 只占 0x40。
        assert_eq!(
            &p.max_instance_offsets[..9],
            &[0, 8192, 16384, 24576, 32768, 40960, 49152, 57344, 65536]
        );
        assert_eq!(p.max_instance_offsets[9], 65536 + 64);
        assert_eq!(*p.max_instance_offsets.last().unwrap(), 71_872);
        // 细粒度只对 kind 0/1：8192>>5 = 256。
        assert_eq!(
            &p.fine_grained_offsets[..9],
            &[0, 256, 512, 768, 1024, 1280, 1536, 1792, 2048]
        );
        // 批次：256 → (256>>10) + ((256 & 0x3fe) != 0) = 1。
        assert_eq!(&p.batch_offsets[..9], &[0, 1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(
            (
                p.total_cmd_offset,
                p.total_max_instances,
                p.total_fine_grained,
                p.total_batches
            ),
            (3_788_800, 71_936, 2048, 8)
        );
    }

    /// `cmd_offsets` 与 `default_value`（0x1408bb650）的 108 项逐项相等：两处互相印证 4-list 展开。
    #[test]
    fn cmd_offsets_equal_offset_defaults() {
        let p = calculate_with_flags(false, false);
        let d = NaniDrawListOffsetDefaults::compute();
        assert_eq!(p.cmd_offsets.as_slice(), &d.offsets[..]);
        // `GpuWorld::new` 的 `NaniDrawListInfo` 缓冲 = 108 * 4 B = 0x1b0。
        assert_eq!(p.max_instance_offsets.len() * 4, 0x1b0);
    }

    #[test]
    fn draw_list_params_force_flags() {
        // FORCE_FINE：全部取 T 与 T>>5。
        let f = calculate_with_flags(true, false);
        assert_eq!(f.total_cmd_offset, 3_788_800);
        assert_eq!(f.total_max_instances, 3_788_800);
        assert_eq!(f.total_fine_grained, 3_788_800 / 32);
        assert_eq!(f.total_batches, 180);
        // FORCE_NO_FINE：kind 0/1 也钳到 0x40，fine 全 0。
        let n = calculate_with_flags(false, true);
        assert_eq!(n.total_max_instances, 27 * 4 * 64);
        assert_eq!(n.total_fine_grained, 0);
        assert_eq!(n.total_batches, 0);
        // 同时置位：FORCE_FINE 优先（`test al,1; cmovne` 最后覆盖）。
        assert_eq!(calculate_with_flags(true, true), f);
    }

    #[test]
    fn batch_mask_is_3fe_not_3ff() {
        // 单个 fine = 1：`1 & 0x3fe == 0` → 不进位（若误用 0x3ff 会得 1）。
        let fine = 1u32;
        assert_eq!((fine >> 10) + u32::from(fine & 0x3fe != 0), 0);
        // fine = 1024 → 1 + 0；fine = 1026 → 1 + 1。
        assert_eq!((1024u32 >> 10) + u32::from(1024u32 & 0x3fe != 0), 1);
        assert_eq!((1026u32 >> 10) + u32::from(1026u32 & 0x3fe != 0), 2);
    }

    #[test]
    fn offset_defaults_layout() {
        let d = NaniDrawListOffsetDefaults::compute();
        // kind 0：s = 0x2000。
        assert_eq!(
            &d.offsets[..8],
            &[0, 0x2000, 0x4000, 0x6000, 0x8000, 0xa000, 0xc000, 0xe000]
        );
        // kind 2 起点 = 4*(0x2000+0x2000) = 0x10000（与反汇编 `lea [rcx+0x1000]` 序列的 base 一致）。
        assert_eq!(d.offsets[8], 0x1_0000);
        assert_eq!(d.offsets[9], 0x1_1000);
        // 最后一个 kind（26，s = 0x8000）：base = 0x37d000。
        assert_eq!(
            &d.offsets[104..],
            &[0x37_d000, 0x38_5000, 0x38_d000, 0x39_5000]
        );
        // 每 kind 的 base = 前缀和 * 4；总和 = 3_788_800。
        assert_eq!(d.offsets[104] + 4 * 0x8000, 3_788_800);
    }

    #[test]
    fn offset_defaults_is_uniform_with_default_value() {
        assert_eq!(
            <NaniDrawListOffsetDefaults as StorageElement>::BUFFER_LEN,
            1
        );
        assert_eq!(core::mem::size_of::<NaniDrawListOffsetDefaults>(), 0x1b0);
        assert_eq!(
            NaniDrawListOffsetDefaults::default_value(),
            Some(NaniDrawListOffsetDefaults::compute())
        );
    }
}
