//! `resources/render/spatial_resolve_offsets.rs` -- 空间解析偏移表 SSBO（A3 res_render 按机器码重写）。
//!
//! 证据（O = 机器码直接观察，I = 推断；详见 `evidence/country-core-audit/A3_res_render.md`）：
//!  * O `impl$0::default_value` 0x140b31d10（7179 B）：`rcx` 是返回槽，写 `Option<Self>`：
//!    `[+0]: u32 = 1`（`Some` 判别式，**最后**写入）、`[+4 .. +0x2004)`：512 个 16 B 元素，
//!    元素 = `(x:i32, y:i32, 0, 0)`。所以 `Self` 是 **0x2000 B、对齐 4** 的 POD（`Option` 判别式后
//!    载荷从 +4 开始），不是旧版的 `[u8; 2048]`。
//!  * O `TypedStorageBuffer::new_storage_buffer<Self>` 0x141611380：缓冲大小 0x2000、用途 0x12
//!    （UNIFORM|TRANSFER_DST）、对齐 4、调试名 `"SpatialResolveOffsets"`（长度 0x15）；
//!    `BUFFER_LEN == 1`，无初始数据时用 `default_value()`。
//!  * O 着色器侧（`dof/fullres.cs.hlsl`）声明 `cbuffer spatial_resolve_offsets : register(b21)`，
//!    `int4 spatial_resolve_offsets_array[512]`，读 `[subpixel_index * 16 + i].xy`：即 32 个
//!    子像素抖动位置 x 每位置 16 个采样偏移，与这里 32 x 16 的分组一致（每组首项 `(0,0)`，
//!    组内 16 个偏移互不相同）。
//!  * I 表项内容来自 16 路泊松式采样；这里逐项转录自机器码里的 512 个 `(x,y)`，
//!    由 `emulate` 脚本（见证据文件）把全部 `movups/movsd/movaps/mov` 存储按字节回放得到，
//!    `default_value` 写入覆盖 0x2004 字节且无遗漏。
//!
//! 旧版错误：把表当成 `[u8; 2048]`（128 项）、把 `+0` 的 `1` 当成“计数”、偏移从 `0x14` 起
//! 摆放；真实表是 512 项、起点 `+4`（`Option` 载荷）、元素为 `int4`。

use crate::render::ssbo::StorageElement;

/// 偏移组数（着色器 `subpixel_index` 取值数）。
pub const SUBPIXEL_COUNT: usize = 32;
/// 每组采样偏移数。
pub const TAPS_PER_SUBPIXEL: usize = 16;
/// 元素总数（`int4 spatial_resolve_offsets_array[512]`）。
pub const ENTRY_COUNT: usize = SUBPIXEL_COUNT * TAPS_PER_SUBPIXEL;

/// `SpatialResolveOffsetsSsbo`（O：0x2000 B，对齐 4；GPU 侧 `int4[512]`）。
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpatialResolveOffsetsSsbo {
    /// `int4` 数组：`[x, y, 0, 0]`（O：后两个分量恒为 0）。
    pub values: [[i32; 4]; ENTRY_COUNT],
}

/// 机器码里的偏移表（O，逐项转录，按 `[组][组内序号]`）。
#[rustfmt::skip]
const OFFSETS: [[i32; 2]; ENTRY_COUNT] = [
    // 子集 0
    [0, 0], [-1, -1], [2, -1], [-2, -2], [-2, 2], [2, 2], [0, -3], [-3, 0], [3, 0], [3, 1], [-1, 3], [1, 3], [2, -3], [-3, -2], [1, 4], [-3, 3],
    // 子集 1
    [0, 0], [0, 1], [-1, 1], [1, 1], [0, -2], [-2, 1], [1, 2], [2, -2], [1, -3], [-4, 0], [4, 0], [-1, -4], [-4, -1], [-4, 1], [-3, -3], [3, 3],
    // 子集 2
    [0, 0], [0, -1], [-1, 0], [1, -1], [-2, -1], [2, 1], [-1, 2], [0, 3], [3, -1], [-2, -3], [3, -2], [-3, 2], [0, -4], [4, 1], [-1, 4], [3, -3],
    // 子集 3
    [0, 0], [1, 0], [-2, 0], [2, 0], [0, 2], [-1, -2], [1, -2], [-1, -3], [-3, -1], [-3, 1], [3, 2], [-2, 3], [2, 3], [0, 4], [1, -4], [4, -1],
    // 子集 4
    [0, 0], [0, 1], [-1, -1], [1, -2], [2, -2], [3, 0], [1, 3], [-3, 2], [-4, 0], [-1, -4], [-1, 4], [-3, -3], [4, 2], [1, -5], [-4, -4], [4, -4],
    // 子集 5
    [0, 0], [0, -1], [1, 0], [-2, 1], [2, 2], [0, 3], [-3, -1], [-3, -2], [0, -4], [4, -1], [-3, 3], [3, 3], [-2, 4], [3, -4], [5, -1], [-2, -5],
    // 子集 6
    [0, 0], [-1, 1], [-1, -2], [2, 1], [0, -3], [3, -1], [-3, 1], [-1, 3], [3, -2], [1, 4], [3, -3], [-4, -2], [0, -5], [4, 3], [-5, -1], [5, 1],
    // 子集 7
    [0, 0], [-1, 0], [1, 1], [-2, 0], [2, -1], [1, 2], [-2, -2], [0, 4], [1, -4], [-4, 1], [4, 1], [-2, -4], [2, -4], [-4, 2], [-4, -3], [-3, 4],
    // 子集 8
    [0, 0], [0, -1], [1, 1], [-2, -2], [2, -2], [-3, 0], [3, 1], [-1, 3], [-1, 4], [-2, -4], [5, 0], [-4, 3], [3, 4], [-5, -1], [2, -5], [4, -4],
    // 子集 9
    [0, 0], [1, -1], [-2, 1], [1, 2], [1, -3], [-3, -2], [-3, 2], [4, 0], [4, 2], [2, 4], [3, -4], [-1, -5], [-5, -3], [-3, 5], [6, -2], [-6, 2],
    // 子集 10
    [0, 0], [0, 1], [0, -2], [-1, -2], [-2, 2], [3, -1], [-4, 0], [1, 4], [3, 3], [4, -2], [0, -5], [-3, -4], [1, -5], [-5, 1], [-1, 5], [1, 5],
    // 子集 11
    [0, 0], [-1, 0], [0, 2], [2, 1], [0, -3], [-3, -1], [2, -3], [4, -1], [-3, -3], [-4, 2], [-2, 4], [5, 2], [4, 4], [5, -3], [-5, 3], [3, 5],
    // 子集 12
    [0, 0], [0, 1], [1, -1], [0, -3], [-3, 0], [3, 1], [0, 4], [-4, -3], [-4, 3], [4, 3], [5, -1], [-3, -5], [3, -5], [5, -3], [-3, 5], [-6, 0],
    // 子集 13
    [0, 0], [-2, -1], [2, -1], [-2, -2], [-2, 2], [1, 3], [-4, 1], [4, 1], [3, -3], [-1, -5], [2, -5], [-2, 5], [2, 5], [0, -6], [-6, -1], [6, 2],
    // 子集 14
    [0, 0], [0, 2], [-2, 1], [1, -3], [3, -1], [3, 2], [-1, -4], [-4, -2], [-2, 4], [5, -2], [-4, -4], [3, 5], [0, 6], [-6, -2], [-6, 2], [4, -5],
    // 子集 15
    [0, 0], [0, -1], [1, 1], [-1, -2], [-2, 3], [1, 4], [-3, -4], [4, -3], [5, 0], [1, -5], [-5, -1], [-5, 2], [5, 3], [6, 1], [1, 6], [-4, 6],
    // 子集 16
    [0, 0], [1, -3], [1, 3], [-3, 2], [4, -1], [3, -4], [-5, 0], [-1, -5], [5, 2], [-2, 5], [-4, -4], [1, 6], [-5, 5], [3, 7], [-8, 1], [7, 4],
    // 子集 17
    [0, 0], [-1, -1], [2, -1], [-1, 2], [3, 1], [-4, -2], [2, 4], [-2, -5], [-5, 2], [6, -2], [-2, 6], [5, -4], [6, 3], [0, -7], [5, 5], [4, -6],
    // 子集 18
    [0, 0], [2, 2], [-1, -3], [-3, -1], [0, 4], [-3, 3], [4, -2], [1, -5], [-5, -3], [6, 0], [-6, 1], [3, -6], [3, 6], [-1, 7], [-4, -7], [-7, -4],
    // 子集 19
    [0, 0], [-1, 0], [0, 2], [-2, -2], [2, -2], [4, 1], [-2, 4], [4, 3], [-3, -5], [5, -3], [-6, 0], [1, -6], [-5, 4], [7, 0], [1, 7], [-3, -7],
    // 子集 20
    [0, 0], [0, -1], [-2, 2], [3, 2], [-3, -3], [-5, 2], [0, -6], [6, 1], [-1, 6], [-6, -2], [6, -3], [3, 6], [4, -6], [-5, 6], [7, 4], [-8, 2],
    // 子集 21
    [0, 0], [1, 2], [1, -3], [-4, 0], [4, -1], [-3, 4], [3, -5], [1, 6], [-2, -6], [-5, -5], [-8, 0], [8, -1], [4, 7], [7, -5], [2, -9], [-7, -6],
    // 子集 22
    [0, 0], [-1, 0], [-2, -2], [0, 3], [3, -2], [-1, -4], [4, 3], [5, -2], [1, -6], [-6, 1], [-2, 6], [5, 4], [-6, -3], [-6, 4], [8, 0], [-4, -7],
    // 子集 23
    [0, 0], [2, 0], [-2, 1], [-4, -1], [1, 4], [3, -4], [5, 1], [-4, 4], [3, 5], [-4, -5], [-7, -2], [7, -2], [2, -8], [8, 3], [-3, 8], [5, -7],
    // 子集 24
    [0, 0], [0, -1], [4, -3], [-4, 3], [0, 5], [4, 4], [-3, -5], [1, -6], [-6, -2], [7, -1], [0, 8], [-8, 3], [8, 3], [5, 7], [7, -6], [-5, 8],
    // 子集 25
    [0, 0], [2, 2], [0, -3], [3, 0], [-3, -1], [-1, 3], [-6, 1], [4, -5], [6, 3], [-6, -5], [-6, 5], [0, -8], [-9, -1], [-2, 9], [5, -8], [-3, -9],
    // 子集 26
    [0, 0], [-2, 1], [-1, -4], [5, 0], [2, 6], [-3, 6], [-4, -6], [3, -7], [-7, -3], [7, -3], [8, 1], [8, 6], [-10, 1], [-7, 8], [-10, 4], [0, 11],
    // 子集 27
    [0, 0], [1, 2], [2, -2], [-3, -3], [-3, 3], [3, -4], [-5, 0], [3, 4], [-2, 6], [-1, -7], [6, 5], [-8, 2], [2, 8], [7, -5], [-7, 5], [2, -9],
    // 子集 28
    [0, 0], [0, -2], [2, 2], [-4, -2], [-2, 5], [-7, 1], [4, -7], [7, -4], [6, 7], [9, 3], [-6, -9], [-9, 6], [0, -11], [0, 11], [-10, -6], [-7, 10],
    // 子集 29
    [0, 0], [-3, 1], [3, -2], [1, -7], [-6, 4], [2, 7], [-7, -4], [8, -1], [-2, -8], [-10, 3], [10, -4], [-4, 10], [4, 10], [-11, -2], [12, 1], [10, 7],
    // 子集 30
    [0, 0], [-1, 2], [-1, -4], [4, 1], [4, -4], [4, 5], [-5, -5], [7, 3], [-3, 7], [-8, -1], [-6, 6], [0, 9], [3, -10], [7, -8], [-9, -8], [-12, 3],
    // 子集 31
    [0, 0], [1, -5], [-5, 1], [1, 5], [-3, -5], [6, 0], [-4, -8], [-10, 0], [10, -1], [5, 9], [8, -7], [9, 6], [-11, -4], [2, 12], [-7, -10], [-4, 12],
];

impl SpatialResolveOffsetsSsbo {
    /// `default_value`（O 0x140b31d10）：返回 `Some(表)`。
    pub fn default_value() -> Option<Self> {
        let mut values = [[0i32; 4]; ENTRY_COUNT];
        let mut i = 0;
        while i < ENTRY_COUNT {
            values[i] = [OFFSETS[i][0], OFFSETS[i][1], 0, 0];
            i += 1;
        }
        Some(Self { values })
    }

    /// 第 `subpixel` 组的 16 个偏移（着色器的 `[subpixel_index * 16 + i].xy`）。
    pub fn taps(&self, subpixel: usize) -> &[[i32; 4]] {
        &self.values[subpixel * TAPS_PER_SUBPIXEL..(subpixel + 1) * TAPS_PER_SUBPIXEL]
    }
}

impl StorageElement for SpatialResolveOffsetsSsbo {
    /// O：`create_buffer` 的名字实参（ptr 0x142c7fbce，长度 0x15）。
    const NAME: &'static str = "SpatialResolveOffsets";
    /// O：`new_storage_buffer` 里 `BufferDesc::new(0x2000, 0x12, ..)`：单个结构体 / uniform。
    const BUFFER_LEN: usize = 1;
    fn default_value() -> Option<Self> {
        SpatialResolveOffsetsSsbo::default_value()
    }
}

impl crate::asset_libraries::ssbo_library::SsboType for SpatialResolveOffsetsSsbo {
    type Storage = crate::render::ssbo::TypedStorageBuffer<Self>;
    fn new_storage_buffer(
        device: &dyn crate::render::ssbo::rhapsody_boundary::GraphicsDevice,
        initial: Option<&[u8]>,
    ) -> Self::Storage {
        // Original 141611380 uses the existing single-struct uniform/storage
        // constructor: 0x2000 bytes, usage0x12, alignment4, original default table.
        crate::render::ssbo::TypedStorageBuffer::new_storage_buffer(device, initial)
    }
}
impl crate::render::pass_builder::pass_resource_access::SsboType for SpatialResolveOffsetsSsbo {
    fn rg_buffer_hash() -> u64 {
        crate::name_hash::NameHash::compute(Self::NAME).0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fnv1a(bytes: &[u8]) -> u64 {
        use std::hash::Hasher;
        let mut h = fnv::FnvHasher::default();
        h.write(bytes);
        h.finish()
    }

    #[test]
    fn size_and_alignment_match_option_layout() {
        // O：Option<Self> 的判别式在 +0，载荷在 +4 => 对齐 4；缓冲 0x2000 B。
        assert_eq!(core::mem::size_of::<SpatialResolveOffsetsSsbo>(), 0x2000);
        assert_eq!(core::mem::align_of::<SpatialResolveOffsetsSsbo>(), 4);
    }

    #[test]
    fn payload_bytes_hash_matches_machine_code_replay() {
        // 校验值 = 对 0x140b31d10 全部存储按字节回放后，+4..+0x2004 的 FNV-1a。
        let v = SpatialResolveOffsetsSsbo::default_value().unwrap();
        let bytes: &[u8] =
            unsafe { core::slice::from_raw_parts(v.values.as_ptr() as *const u8, 0x2000) };
        assert_eq!(fnv1a(bytes), 0x4287_acbb_8f83_b1e5);
    }

    #[test]
    fn first_entries_and_group_structure() {
        let v = SpatialResolveOffsetsSsbo::default_value().unwrap();
        let first: Vec<[i32; 2]> = v.values[..16].iter().map(|e| [e[0], e[1]]).collect();
        assert_eq!(
            first,
            vec![
                [0, 0],
                [-1, -1],
                [2, -1],
                [-2, -2],
                [-2, 2],
                [2, 2],
                [0, -3],
                [-3, 0],
                [3, 0],
                [3, 1],
                [-1, 3],
                [1, 3],
                [2, -3],
                [-3, -2],
                [1, 4],
                [-3, 3]
            ]
        );
        for g in 0..SUBPIXEL_COUNT {
            let taps = v.taps(g);
            assert_eq!(taps.len(), 16);
            assert_eq!([taps[0][0], taps[0][1]], [0, 0], "组 {g} 首项恒为 (0,0)");
            let mut s: Vec<[i32; 2]> = taps.iter().map(|e| [e[0], e[1]]).collect();
            s.sort();
            s.dedup();
            assert_eq!(s.len(), 16, "组 {g} 内偏移互不相同");
            assert!(taps.iter().all(|e| e[2] == 0 && e[3] == 0));
        }
    }

    #[test]
    fn last_entry_and_extent() {
        let v = SpatialResolveOffsetsSsbo::default_value().unwrap();
        // O：最后一次存储 `[rax+0x1ff4]` 的 8 字节 = 末项 (x,y)。
        let last = v.values[ENTRY_COUNT - 1];
        assert_eq!([last[0], last[1]], [-4, 12]);
        let max = v
            .values
            .iter()
            .flat_map(|e| [e[0].abs(), e[1].abs()])
            .max()
            .unwrap();
        assert_eq!(max, 12);
    }

    #[test]
    fn storage_element_contract() {
        assert_eq!(
            <SpatialResolveOffsetsSsbo as StorageElement>::NAME.len(),
            0x15
        );
        assert_eq!(<SpatialResolveOffsetsSsbo as StorageElement>::BUFFER_LEN, 1);
        assert!(<SpatialResolveOffsetsSsbo as StorageElement>::default_value().is_some());
    }
}
