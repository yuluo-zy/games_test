//! `resources::texture_presets` — 纹理预设绑定类型（O 名，字段 typed gap）。
//!
//! 原符号：`country_core::resources::texture_presets::{TexturePreset,
//! TextureFromDisk}`，用于 `bind_texture_to_draw_builder<T>` /
//! `bind_texture_to_compute_pass<T>` 泛型实现（wp1 draw_builder::impl$3 /
//! compute_pass_builder::impl$4，反编译 O）。
//!
//! - `TextureFromDisk(name_hash)`：按 64 位名字哈希取磁盘纹理资源。
//!   已知 O 值（见各模块 contract）：`0x579f39ab91a01865` =
//!   water_noise_texture；其余（clearing/floor/rocky/pebble/brick/leaf/
//!   critter/ivy 族）名字待资源库恢复（U）。
//! - `TexturePreset(preset_byte)`：数值预设槽。观测值 8/9/10/0xc/0xe/
//!   0x13/0x14/0x15/0x18/5/6/7/0xf 等（语义 U；water_planes_draw 记录
//!   preset 9/10/0x13/0x18/8 为水面纹理槽）。
//!
//! WP18 draw_shaders 以 tuple 形式构造（`TexturePreset(0xc)` / `TextureFromDisk(0x…)`）。
//! `TextureFromDisk` 的 Event 形态与 draw 路径冲突 → draw 形态胜。
//!
//! **0xa59890（`impl$1::from`）是 `From<TexturePreset> for` 预设纹理描述**（`render-gap/DIGEST.md §4.1`、反编译
//! `decomp-0x140A59890.txt`、`0x142ad35b0` 跳转表逐项核对），不是 `From<u8> for TexturePreset`（后者没有机器码证据，
//! 此前 WP21 的归因错位）。预设共 **25 个**（0..=0x18）：`enum_iterator::All<TexturePreset>` 的折叠函数 0x141474b90 里
//! 状态 0..=24 依次产出预设、状态 25 是结束标记——此前把结束标记也算成变体（26 个）是多算了一个。

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::manual_range_patterns)]
use crate::rhapsody_types::TextureDesc;

/// 磁盘纹理绑定（64 位名字哈希，O：`TextureFromDisk`）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextureFromDisk(pub u64);

/// 数值纹理预设槽（O：`TexturePreset`，共 [`TexturePreset::COUNT`] 个，值 0..=0x18）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TexturePreset(pub u8);

impl From<u8> for TexturePreset {
    /// 平凡的新类型转换（**没有机器码证据**；0xa59890 不是它，见模块文档）。
    fn from(v: u8) -> Self {
        TexturePreset(v)
    }
}

/// 一个预设的纹理描述 + 嵌入的 64 位名字哈希（O：0x140a59890 的返回值，0x30 B：`{TextureDesc 0x20 B @0,
/// NameHash @0x20, 0 填充 @0x28..}`；哈希是反编译里的 64 位立即数，应为预设名的 `NameHash`（xxh3，I，名字未还原））。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PresetTexture {
    pub desc: TextureDesc,
    pub name_hash: u64,
}

/// 预设表的一行：`(extent, 格式, 数组层数 | 0, 附加用途, 嵌入哈希)`（O，逐 case 抄自反编译）。
const fn row(extent: [u32; 2], format: u32, hash: u64) -> (TextureDesc, u64) {
    (
        TextureDesc {
            flags: 0,
            extent: [extent[0], extent[1], 1],
            kind: 2,
            sampled: true,
            mip_levels: 1,
            array_elements: 1,
            storage: true,
            color_attachment: false,
            depth_stencil_attachment: false,
            format,
        },
        hash,
    )
}

impl TexturePreset {
    /// 预设个数（O：`enum_iterator` 状态 0..=24；状态 25 是结束标记）。
    pub const COUNT: u8 = 25;

    /// `From<TexturePreset>` 的 switch（O：0x140a59890，跳转表 0x142ad35b0，25 项）。
    ///
    /// 每个 case 都是 `new_2d(extent, format)` → `.sampled()` → 再加用途位，最后嵌入哈希：
    /// * 绝大多数：`.sampled().storage()`（usage = 0x04|0x08）；
    /// * **8**：`.sampled().storage().mip_levels(5)`；
    /// * **10**、**0x11**、**0x12**：`.sampled().depth_stencil_attachment()`（usage = 0x04|0x20，**无** storage）；
    /// * **0xf**、**0x10**：`.sampled().color_attachment()`（usage = 0x04|0x10，**无** storage）；
    /// * **0x11/0x12**：`new_2d_array(extent, 3, 0x7e)`；**0x18**：先 `.storage()` 再 `.sampled()`（位或，结果相同）。
    ///
    /// 越界（≥ [`Self::COUNT`]）：原版没有边界检查（枚举只有 25 个变体，跳转表之外是数据，UB），这里 panic。
    pub fn texture(self) -> PresetTexture {
        const SQ512: [u32; 2] = [512, 512];
        let (mut desc, name_hash) = match self.0 {
            0 => row([64, 64], 0x53, 0xe362_4a5f_a036_3c58),
            1 => row(SQ512, 0x4c, 0xc2fc_59b8_a62a_7bb9),
            2 => row(SQ512, 0x4c, 0x50f5_0a6f_d3a4_462f),
            3 => row(SQ512, 0x4c, 0xced8_911e_ca07_05eb),
            4 => row(SQ512, 0x4c, 0xa25c_046b_a6c0_5bb4),
            5 => row(SQ512, 0x4c, 0x240e_a7ee_12ea_47f7),
            6 => row(SQ512, 0x61, 0x6749_a66b_cf03_8be9),
            7 => row(SQ512, 0x4c, 0xa5be_6bd0_3897_6cb3),
            8 => {
                let (mut d, h) = row(SQ512, 0x7a, 0x09fa_1678_d41f_0c18);
                d.mip_levels = 5;
                (d, h)
            }
            9 => row(SQ512, 0x46, 0x4ed8_767e_bd4c_321c),
            10 => {
                let (mut d, h) = row(SQ512, 0x7c, 0xe673_b662_ce97_671c);
                d.storage = false;
                d.depth_stencil_attachment = true;
                (d, h)
            }
            0xb => row(SQ512, 0x63, 0x7efe_10a2_cce1_1c9f),
            0xc => row([256, 256], 0x4c, 0x1cac_83c5_80b2_d77b),
            0xd => row(SQ512, 0x53, 0x3887_a26d_3181_f0e2),
            0xe => row(SQ512, 0x61, 0x84a0_37e6_97de_0324),
            0xf => {
                let (mut d, h) = row(SQ512, 0x61, 0x35da_f427_88d4_cb5b);
                d.storage = false;
                d.color_attachment = true;
                (d, h)
            }
            0x10 => {
                let (mut d, h) = row(SQ512, 9, 0x7abd_76ee_26e8_d462);
                d.storage = false;
                d.color_attachment = true;
                (d, h)
            }
            0x11 => {
                let mut d = TextureDesc::new_2d_array([1200, 1200], 3, 0x7e).sampled();
                d.depth_stencil_attachment = true;
                (d, 0x98c1_1fe9_5d7d_3b89)
            }
            0x12 => {
                let mut d = TextureDesc::new_2d_array([1, 1], 3, 0x7e).sampled();
                d.depth_stencil_attachment = true;
                (d, 0xe6de_e572_9986_6376)
            }
            0x13 => row(SQ512, 0x61, 0xa738_0497_1c10_b016),
            0x14 => row(SQ512, 0x61, 0xc3f7_d6e6_c37c_ec9d),
            0x15 => row(SQ512, 0x53, 0x6759_183d_0a9c_8971),
            0x16 => row(SQ512, 9, 0xffb5_ef44_962b_ea4d),
            0x17 => row([225, 225], 0x4c, 0x3545_71e6_bba6_9ded),
            0x18 => row([256, 256], 0x25, 0x70b0_1834_b4b4_0879),
            n => panic!(
                "TexturePreset {n:#x} is out of range (the original has no bounds check: undefined behaviour)"
            ),
        };
        // `row()` 已按"sampled + storage"建；2D 数组两项（0x11/0x12）与用途位互斥的项在上面单独处理。
        desc.sampled = true;
        desc.mip_levels = desc.mip_levels.max(1);
        desc.array_elements = desc.array_elements.max(1);
        PresetTexture { desc, name_hash }
    }

    /// 预设是否带 mip（8）/ depth-stencil 附件 / color 附件，由 [`Self::texture`] 的用途位推出。
    /// **纠错**：此前把 11/12 判为 depth，实为普通 storage（`new_2d(0x200,0x200,0x63)` / `new_2d(0x100,0x100,0x4c)`）；
    /// depth 附件是 10、0x11、0x12。
    pub fn usage(&self) -> (bool, bool, bool) {
        let d = self.texture().desc;
        (
            d.mip_levels > 1,
            d.depth_stencil_attachment,
            d.color_attachment,
        )
    }
}

impl From<TexturePreset> for PresetTexture {
    fn from(p: TexturePreset) -> Self {
        p.texture()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn usage_bits(d: &TextureDesc) -> u8 {
        (d.sampled as u8) << 2
            | (d.storage as u8) << 3
            | (d.color_attachment as u8) << 4
            | (d.depth_stencil_attachment as u8) << 5
    }

    #[test]
    fn there_are_25_presets_and_the_26th_is_out_of_range() {
        assert_eq!(TexturePreset::COUNT, 25);
        for p in 0..TexturePreset::COUNT {
            let t = TexturePreset(p).texture();
            assert!(t.desc.sampled, "preset {p:#x} is always sampled");
            assert_ne!(t.name_hash, 0);
        }
        assert!(std::panic::catch_unwind(|| TexturePreset(25).texture()).is_err());
    }

    #[test]
    fn table_matches_the_decompiled_switch() {
        let ex = |p: u8| {
            let d = TexturePreset(p).texture().desc;
            (d.extent, d.format)
        };
        assert_eq!(ex(0), ([64, 64, 1], 0x53));
        assert_eq!(ex(1), ([512, 512, 1], 0x4c));
        assert_eq!(ex(6), ([512, 512, 1], 0x61));
        assert_eq!(ex(8), ([512, 512, 1], 0x7a));
        assert_eq!(ex(10), ([512, 512, 1], 0x7c));
        assert_eq!(ex(0xb), ([512, 512, 1], 0x63));
        assert_eq!(ex(0xc), ([256, 256, 1], 0x4c));
        assert_eq!(ex(0x10), ([512, 512, 1], 9));
        assert_eq!(ex(0x11), ([1200, 1200, 1], 0x7e));
        assert_eq!(ex(0x12), ([1, 1, 1], 0x7e));
        assert_eq!(ex(0x17), ([225, 225, 1], 0x4c));
        assert_eq!(ex(0x18), ([256, 256, 1], 0x25));
        assert_eq!(TexturePreset(0).texture().name_hash, 0xe362_4a5f_a036_3c58);
        assert_eq!(
            TexturePreset(0x18).texture().name_hash,
            0x70b0_1834_b4b4_0879
        );
        // 25 个哈希互不相同（各是不同预设名的 NameHash）。
        let mut h: Vec<u64> = (0..25)
            .map(|p| TexturePreset(p).texture().name_hash)
            .collect();
        h.sort_unstable();
        h.dedup();
        assert_eq!(h.len(), 25);
    }

    #[test]
    fn usage_bits_follow_the_builder_chain() {
        const SAMPLED_STORAGE: u8 = 0x04 | 0x08;
        for p in 0..25u8 {
            let d = TexturePreset(p).texture().desc;
            let expect = match p {
                10 | 0x11 | 0x12 => 0x04 | 0x20,
                0xf | 0x10 => 0x04 | 0x10,
                _ => SAMPLED_STORAGE,
            };
            assert_eq!(usage_bits(&d), expect, "usage of preset {p:#x}");
        }
        // 纠错：11/12 是普通 storage，不是 depth。
        assert_eq!(TexturePreset(11).usage(), (false, false, false));
        assert_eq!(TexturePreset(12).usage(), (false, false, false));
        assert_eq!(TexturePreset(8).usage(), (true, false, false));
        assert_eq!(TexturePreset(10).usage(), (false, true, false));
        assert_eq!(TexturePreset(0x11).usage(), (false, true, false));
        assert_eq!(TexturePreset(15).usage(), (false, false, true));
        assert_eq!(TexturePreset(16).usage(), (false, false, true));
    }

    #[test]
    fn only_preset_8_has_multiple_mips_and_only_arrays_have_multiple_layers() {
        for p in 0..25u8 {
            let d = TexturePreset(p).texture().desc;
            assert_eq!(d.mip_levels, if p == 8 { 5 } else { 1 }, "mips of {p:#x}");
            assert_eq!(
                d.array_elements,
                if matches!(p, 0x11 | 0x12) { 3 } else { 1 },
                "layers of {p:#x}"
            );
        }
    }
}
