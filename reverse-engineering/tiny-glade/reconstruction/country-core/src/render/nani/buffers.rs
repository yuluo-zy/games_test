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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

/// SSBO 载体宏：把仅有名字的载体接入两套 SSBO trait。
///  * `StorageElement`：`NAME` = 类型名（O：`read_access_buffer<NaniInstanceIndirectionSsbo>` 0x140972320 的槽名
///    字符串即 `"NaniInstanceIndirectionSsbo"`，0x142c8151d）；`BUFFER_LEN = 1`、`default_value = Some(Self)`
///    （I：真实元素布局 U，沿用 `bind_resources::carrier!` 的做法）。
///  * `asset_libraries::ssbo_library::SsboType`（`AssetSsboLibrary::get_rg_buffer<T>` 的约束，O：nani_draw 调用
///    `get_rg_buffer<NaniDrawListLengthSsbo>` / `<NaniDrawListSsbo>`）。
///  * `pass_resource_access::SsboType`（`DrawBuilder::read_storage_buffer<T>` 的约束）：`rg_buffer_hash()` =
///    库里的 `SsboId::of::<T>()`（I：RG 缓冲按库键解析，`PassHost::rg_buffer_resource(hash)`）。
macro_rules! nani_ssbo_carrier {
    ($($(#[$doc:meta])* $name:ident),+ $(,)?) => {$(
        $(#[$doc])*
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
        pub struct $name;

        impl StorageElement for $name {
            const NAME: &'static str = stringify!($name);
            const BUFFER_LEN: usize = 1;
            fn default_value() -> Option<Self> {
                Some($name)
            }
        }

        impl crate::asset_libraries::ssbo_library::SsboType for $name {
            type Storage = crate::render::ssbo::TypedStorageBuffer<$name>;
            fn new_storage_buffer(
                device: &dyn crate::render::ssbo::rhapsody_boundary::GraphicsDevice,
                initial: Option<&[u8]>,
            ) -> Self::Storage {
                match initial {
                    None => crate::render::ssbo::TypedStorageBuffer::<$name>::new(device, None),
                    Some(bytes) => {
                        crate::render::ssbo::TypedStorageBuffer::<$name>::new_storage_buffer(device, Some(bytes))
                    }
                }
            }
        }

        impl crate::render::pass_builder::pass_resource_access::SsboType for $name {
            fn rg_buffer_hash() -> u64 {
                crate::asset_libraries::ssbo_library::SsboId::of::<$name>().0.0
            }
        }
    )+};
}

nani_ssbo_carrier! {
    /// SSBO 载体：draw-list 长度（PDB：`write_access_buffer<NaniDrawListLengthSsbo>` 0x140975920）。
    NaniDrawListLengthSsbo,
    /// SSBO 载体：间接 draw 列表（PDB：`write_access_buffer<NaniDrawListSsbo>` 0x140973e20）。
    NaniDrawListSsbo,
    /// SSBO 载体：实例间接寻址表（PDB：`read_access_buffer<NaniInstanceIndirectionSsbo>` 0x140972320）。
    NaniInstanceIndirectionSsbo,
    /// 以下四个 SSBO 载体只在 PDB 的 `pass_resource_access::*_access_buffer<T>` / `AssetSsboLibrary` 单态化名里出现
    /// （O 名称，字段 U）。
    NaniInstanceIndirectionAllocatorSsbo,
    NaniFilteredInstanceAllocatorSsbo,
    NaniNextVisibilityBitmaskSsbo,
    NaniPrevVisibilityBitmaskSsbo,
}

#[cfg(test)]
mod tests {
    use super::*;

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
