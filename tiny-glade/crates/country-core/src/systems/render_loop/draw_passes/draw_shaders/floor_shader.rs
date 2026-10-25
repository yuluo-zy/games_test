//! floor_shader.rs — 地形地面（`_terrain_editing_floor.raster`）。
//!
//! PDB `systems/render_loop/draw_passes/draw_shaders/floor_shader.rs`（编译单元 530）。机器码（O）：
//! `draw_floor_shader` 0x1409cacb0（2090 B）。ABI：`rcx = &mut DrawBuilder`（0xf8 B 值按指针传入）、`r9b = bool`；
//! `rdx` / `r8` 未被读取（O），所以实际签名是 `(DrawBuilder, <未用>, <未用>, bool)`；那个 bool 只流向
//! `write_uniform<bool>(0x0be4740b2c6814ba)`，语义 U（`rocky_terrain_shader` 用同名 uniform，I：同一地形开关）。
//!
//! 顺序（O）：
//! 0. `packet.state` 第 0 字节：`(b & 0xf3) | 0x08`（清 bit2..3 再置 bit3；与 grass / roof 的 `| 4` 同属“剔除模式”两位，I）
//! 1. 预设 9 / 10 / 6 / 0x13（名 0x61b08b5e…、0x297d1e4c…、0xd7f48130…、0x6ad63ccd…）
//! 2. `TextureFromDisk(0x4a2b687d0c1ecf58)`（名 0xd0cc743d37cfaefb）
//! 3. `bind_stone_floor_textures`（0x14090c300）
//! 4. `bind_wall_color_customization`（0x14090bb70；末尾读 `WallColorIdSsbo`）
//! 5. 磁盘 0x84b51135b59095ba（名 0x8bb8ef45…）、0xb484366d00600ae5（名 0x8bb8f245…）、0x59b98d76c4e95bd1（名 0x32e29129…）
//! 6. 预设 0x15 / 7 / 0xc / 0xe / 0x14（名 0x4a9aecb3…、0x3ec9710e…、0x8f870a5c…、0x4ee098b3…、0x49f43a48…）
//! 7. 磁盘 0x954b7e4974bbcd6b（名 0x07e65d19174a2f49）
//! 8. 常量块存在 → `write_uniform<bool>(0x0be4740b2c6814ba, flag)`
//! 9. `draw()`（不改 draw 种类，缺省 `InstancedGlDrawFlag{flag:1}`）
//!
//! shader URI（O）：`/terrain_editing/floor.raster.hlsl`。纹理文件名与预设的真实名字 U（只知其哈希）。

use crate::render::pass_builder::bind_resources::WallColorIdSsbo;
use crate::render::pass_builder::draw_builder::DrawBuilder;
use crate::systems::render_loop::draw_passes::draw_shaders::common_includes::{
    bind_disk, bind_preset,
};

/// 地面开关 uniform 名（O）。
pub const FLOOR_FLAG_UNIFORM: u64 = 0x0be4_740b_2c68_14ba;

/// 第一段预设（O：id, 名字哈希）。
pub const FLOOR_PRESETS_HEAD: [(u8, u64); 4] = [
    (0x09, 0x61b0_8b5e_0470_c5f2),
    (0x0a, 0x297d_1e4c_dae8_f657),
    (0x06, 0xd7f4_8130_3b10_4abc),
    (0x13, 0x6ad6_3ccd_18c8_f041),
];
/// 第一张磁盘纹理（O：文件键, 名字哈希）。
pub const FLOOR_DISK_HEAD: (u64, u64) = (0x4a2b_687d_0c1e_cf58, 0xd0cc_743d_37cf_aefb);
/// 墙色之后的三张磁盘纹理。
pub const FLOOR_DISK_MID: [(u64, u64); 3] = [
    (0x84b5_1135_b590_95ba, 0x8bb8_ef45_1d9c_51f4),
    (0xb484_366d_0060_0ae5, 0x8bb8_f245_1d9c_570d),
    (0x59b9_8d76_c4e9_5bd1, 0x32e2_9129_77f5_f21d),
];
/// 末段预设。
pub const FLOOR_PRESETS_TAIL: [(u8, u64); 5] = [
    (0x15, 0x4a9a_ecb3_eff0_f013),
    (0x07, 0x3ec9_710e_6411_d8d1),
    (0x0c, 0x8f87_0a5c_7060_b0f4),
    (0x0e, 0x4ee0_98b3_8612_d2b9),
    (0x14, 0x49f4_3a48_7883_738d),
];
/// 最后一张磁盘纹理。
pub const FLOOR_DISK_TAIL: (u64, u64) = (0x954b_7e49_74bb_cd6b, 0x07e6_5d19_174a_2f49);

/// `packet.state` 第 0 字节的改写（O：`and al, -0xd; or al, 8`）。
pub fn floor_state(state: u32) -> u32 {
    (state & !0x0c) | 0x08
}

/// `draw_floor_shader`（O 0x1409cacb0）。`flag` = `r9b`。
pub fn draw_floor_shader(mut b: DrawBuilder<'_>, flag: bool) {
    let p = b.packet_mut();
    p.state.0 = floor_state(p.state.0);
    for (preset, name) in FLOOR_PRESETS_HEAD {
        b = bind_preset(b, preset, name);
    }
    b = bind_disk(b, FLOOR_DISK_HEAD.0, FLOOR_DISK_HEAD.1);
    b = b.bind_stone_floor_textures();
    b = b.bind_wall_color_customization::<WallColorIdSsbo>();
    for (key, name) in FLOOR_DISK_MID {
        b = bind_disk(b, key, name);
    }
    for (preset, name) in FLOOR_PRESETS_TAIL {
        b = bind_preset(b, preset, name);
    }
    b = bind_disk(b, FLOOR_DISK_TAIL.0, FLOOR_DISK_TAIL.1);
    b.uniform(FLOOR_FLAG_UNIFORM, flag).draw();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::pass_builder::bind_resources::WallColorIdSsbo;
    use crate::render::pass_builder::draw_builder::{STONE_FLOOR_TEXTURES, WALL_COLOR_TEXTURES};
    use crate::render::pass_builder::draw_stream::DrawKind;
    use crate::systems::render_loop::draw_passes::draw_shaders::test_support::{
        Call, Rig, as_i32, rg, uniform_at,
    };

    fn all_names() -> Vec<u64> {
        let mut v: Vec<u64> = FLOOR_PRESETS_HEAD.iter().map(|p| p.1).collect();
        v.push(FLOOR_DISK_HEAD.1);
        v.extend(FLOOR_DISK_MID.iter().map(|d| d.1));
        v.extend(FLOOR_PRESETS_TAIL.iter().map(|p| p.1));
        v.push(FLOOR_DISK_TAIL.1);
        v
    }

    #[test]
    fn state_rewrite_clears_bits_2_3_and_sets_bit_3() {
        assert_eq!(floor_state(0x8000_0080), 0x8000_0088);
        assert_eq!(floor_state(0x0c), 0x08);
        assert_eq!(floor_state(0xff), 0xfb, "0xff & 0xf3 | 8 = 0xfb");
    }

    #[test]
    fn bind_order_matches_the_machine_code() {
        let mut rig = Rig::new(&all_names(), &[FLOOR_FLAG_UNIFORM]);
        draw_floor_shader(rig.builder(), true);
        let calls = rig.calls();
        let d = rig.finish();
        let mut want = vec![
            Call::Preset(9),
            Call::Preset(0xa),
            Call::Preset(6),
            Call::Preset(0x13),
            Call::Disk(FLOOR_DISK_HEAD.0),
        ];
        // bind_stone_floor_textures：12 张磁盘纹理；bind_wall_color_customization：4 张 + WallColorId。
        want.extend(STONE_FLOOR_TEXTURES.iter().map(|t| Call::Disk(t.0)));
        want.extend(WALL_COLOR_TEXTURES.iter().map(|t| Call::Disk(t.0)));
        want.push(rg::<WallColorIdSsbo>());
        want.extend(FLOOR_DISK_MID.iter().map(|d| Call::Disk(d.0)));
        want.extend([0x15u8, 7, 0xc, 0xe, 0x14].map(Call::Preset));
        want.push(Call::Disk(FLOOR_DISK_TAIL.0));
        assert_eq!(calls, want);
        assert_eq!(d.len(), 1);
        assert_eq!(
            d[0].kind,
            DrawKind::InstancedGlDrawFlag { flag: 1 },
            "`draw()` 不改 draw 种类"
        );
        assert_eq!(d[0].state.0 & 0x0c, 0x08, "第 0 字节 bit2..3 = 0b10");
    }

    #[test]
    fn flag_is_written_as_a_u32_bool_uniform() {
        for flag in [false, true] {
            let mut rig = Rig::new(&all_names(), &[FLOOR_FLAG_UNIFORM]);
            let b = rig.builder().uniform(FLOOR_FLAG_UNIFORM, flag);
            let data = b.constants_data().unwrap().to_vec();
            assert_eq!(as_i32(&uniform_at(&data, 0, 4)), flag as i32);
            b.draw();
            rig.finish();
        }
    }

    #[test]
    fn without_a_constant_block_the_flag_is_dropped_silently() {
        let mut rig = Rig::new(&all_names(), &[]);
        draw_floor_shader(rig.builder(), true);
        assert_eq!(rig.finish().len(), 1);
    }
}
