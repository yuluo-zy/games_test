//! clearing_shaders.rs — 空地里的树：树干 / 树冠（nani GPU-driven 路径）。
//!
//! PDB `systems/render_loop/draw_passes/draw_shaders/clearing_shaders.rs`。机器码（O）：`tree_trunk` 0x1409c9830
//! （2841 B）、`tree_canopy` 0x1409ca350（2400 B）。ABI：`rcx = &mut DrawBuilder`、`r8 = &World`。
//!
//! 公共：
//! - `DrawBuilder +0xf0 != 0`（`bind_common` 写入的模式字节）→ `packet.state` 第 0 字节 `(b & 0xf3) | 4`
//!   （trunk 里是条件执行，canopy 同形：`cmp byte ptr [rcx+0xf0], 0; je`）。
//! - 世界资源 A（TypeId 0xd2f519263066413b/0x5192b15f962559e4）`f32 @+4` = `wind`（名 I：只知它是第一个 uniform 的来源）。
//! - 世界资源 B（TypeId 0xbba451ee6a08a3ff/0x8d008d15febcf29d，与 ivy 叶同一个，即相机资源）：`byte @+0x4d0 == 3` →
//!   `season_is_3`（bool uniform 0x515e89659697cbe9）；`CameraMatrices::eye_position()`（仅 trunk）。
//! - 世界资源 C（TypeId 0x1b4e385d376c1cc0/0x8c38d6ede2a437ec）`f32 @+4` = `v`；uniform `(1 - v) * 15 + 30 * v`
//!   （名 0x7f404450f366d335）。
//!
//! ## tree_trunk（O）
//! f32 uniform 0xcb2a3feabd114d69（= A）→ 六张磁盘纹理 → Vec3 uniform 0x4bbdb30b873d3990（= eye）→ bool uniform → f32 uniform（C）→
//! 预设 0xe（名 0x4ee098b38612d2b9）→ `nani_draw(TreeTrunk = 1)`。
//! （O 序：先读 A、B；纹理在两个 uniform 之间；eye 在纹理之后读取。）
//!
//! ## tree_canopy（O）
//! 先读 B（`season = byte @+0x4d0`，`sun = f32 @ B + SEASON_OFFSETS[season] + 0x104`）、A；五张磁盘纹理 → f32 uniform 0xcb2a3f…（= A）→
//! f32 uniform 0x090fac5af236f89d（= [`canopy_factor`]）→ bool → f32（C）→ `nani_draw(Canopy = 0)`。

use glam::Vec3;

use crate::render::nani::gpu_world::GpuWorldSubsetKind;
use crate::render::pass_builder::draw_builder::DrawBuilder;
use crate::systems::render_loop::draw_passes::draw_shaders::common_includes::{
    DrawModeExt, bind_disk, bind_preset,
};

pub const TREE_WIND_UNIFORM: u64 = 0xcb2a_3fea_bd11_4d69;
pub const TREE_EYE_UNIFORM: u64 = 0x4bbd_b30b_873d_3990;
pub const TREE_SEASON3_UNIFORM: u64 = 0x515e_8965_9697_cbe9;
pub const TREE_BLEND_UNIFORM: u64 = 0x7f40_4450_f366_d335;
pub const CANOPY_FACTOR_UNIFORM: u64 = 0x090f_ac5a_f236_f89d;
pub const TRUNK_PRESET: (u8, u64) = (0xe, 0x4ee0_98b3_8612_d2b9);

/// 树干六张磁盘纹理（O：文件键, 名字哈希，按绑定顺序）。
pub const TRUNK_TEXTURES: [(u64, u64); 6] = [
    (0x0ce1_615b_9bd3_e034, 0x7469_4d8f_8265_789b),
    (0x32dc_e693_9b56_9f34, 0xf820_68db_6dad_b67b),
    (0x8215_a488_d995_62a6, 0x678c_077f_072e_268f),
    (0x9320_1d69_003f_98d4, 0x5610_60fc_8f81_7b1e),
    (0xaa12_b013_11b7_7c4d, 0x62e4_85c5_99e5_e602),
    (0x004d_bb39_acf1_c7b7, 0x30a5_0108_2a42_b18c),
];
/// 树冠五张磁盘纹理。
pub const CANOPY_TEXTURES: [(u64, u64); 5] = [
    (0x579f_39ab_91a0_1865, 0x815b_f503_6966_978c),
    (0x521d_5b30_024d_9662, 0xbb7e_a833_cfa3_de15),
    (0x9320_1d69_003f_98d4, 0x5610_60fc_8f81_7b1e),
    (0xaa12_b013_11b7_7c4d, 0x62e4_85c5_99e5_e602),
    (0x004d_bb39_acf1_c7b7, 0x30a5_0108_2a42_b18c),
];
/// 相机资源里“按季节”的子结构偏移表（O：0x142ac0cf0，六项）；每项里 `f32 @+0x104` 是树冠参数。
pub const SEASON_OFFSETS: [usize; 6] = [0x8c, 0x144, 0x1c4, 0x264, 0x31c, 0x3a4];
pub const SEASON_FIELD_OFFSET: usize = 0x104;
/// 季节字节在相机资源里的偏移与“第 3 季”的取值（O：`cmp byte ptr [r15+0x4d0], 3`）。
pub const SEASON_BYTE_OFFSET: usize = 0x4d0;
pub const SEASON_SPECIAL: u8 = 3;

/// 季节子结构里取树冠参数的偏移（O：`[r15 + table[season] + 0x104]`；越界 → 与原版一样越界读，这里 panic）。
pub fn canopy_param_offset(season: u8) -> usize {
    SEASON_OFFSETS[season as usize] + SEASON_FIELD_OFFSET
}

/// 树冠 uniform：`max(param * (π/180) * 0.5, 0.07) * 0.8`（O：f32 常量 0x3c8efa35 / 0.5 / 0x3d8f5c29 / 0x3f4ccccd）。
pub fn canopy_factor(param: f32) -> f32 {
    let deg2rad = f32::from_bits(0x3c8e_fa35);
    ((param * deg2rad) * 0.5).max(f32::from_bits(0x3d8f_5c29)) * f32::from_bits(0x3f4c_cccd)
}

/// 混合 uniform：`(1 - v) * 15 + 30 * v`（O：`subss/mulss/addss` 顺序）。
pub fn blend_value(v: f32) -> f32 {
    (1.0 - v) * 15.0 + 30.0 * v
}

/// `packet.state` 第 0 字节：`(b & 0xf3) | 4`。
pub fn tree_state(state: u32) -> u32 {
    (state & !0x0c) | 0x04
}

fn apply_cull(b: &mut DrawBuilder<'_>) {
    if b.draw_mode() != 0 {
        let p = b.packet_mut();
        p.state.0 = tree_state(p.state.0);
    }
}

/// 世界输入（树干）。
#[derive(Clone, Copy, Debug)]
pub struct TreeTrunkInputs {
    /// 资源 A `f32 @+4`。
    pub wind: f32,
    /// `CameraMatrices::eye_position()`。
    pub eye: Vec3,
    /// 相机资源 `byte @+0x4d0 == 3`。
    pub season_is_3: bool,
    /// 资源 C `f32 @+4`。
    pub blend: f32,
}

/// `tree_trunk`（O 0x1409c9830）。`nani`：`NaniDrawExt::nani_draw`（kind = `TreeTrunk`）。
pub fn tree_trunk<'p>(
    mut b: DrawBuilder<'p>,
    inputs: &TreeTrunkInputs,
    nani: impl FnOnce(DrawBuilder<'p>, GpuWorldSubsetKind),
) {
    apply_cull(&mut b);
    for (key, name) in TRUNK_TEXTURES {
        b = bind_disk(b, key, name);
    }
    let b = b.uniform(TREE_WIND_UNIFORM, inputs.wind);
    let b = b.uniform(TREE_EYE_UNIFORM, inputs.eye);
    let b = b.uniform(TREE_SEASON3_UNIFORM, inputs.season_is_3);
    let b = b.uniform(TREE_BLEND_UNIFORM, blend_value(inputs.blend));
    let b = bind_preset(b, TRUNK_PRESET.0, TRUNK_PRESET.1);
    nani(b, GpuWorldSubsetKind::TreeTrunk);
}

/// 世界输入（树冠）。
#[derive(Clone, Copy, Debug)]
pub struct TreeCanopyInputs {
    pub wind: f32,
    /// 相机资源季节子结构里的 `f32 @+0x104`（偏移见 [`canopy_param_offset`]）。
    pub season_param: f32,
    pub season_is_3: bool,
    pub blend: f32,
}

/// `tree_canopy`（O 0x1409ca350）。
pub fn tree_canopy<'p>(
    mut b: DrawBuilder<'p>,
    inputs: &TreeCanopyInputs,
    nani: impl FnOnce(DrawBuilder<'p>, GpuWorldSubsetKind),
) {
    apply_cull(&mut b);
    for (key, name) in CANOPY_TEXTURES {
        b = bind_disk(b, key, name);
    }
    let b = b.uniform(TREE_WIND_UNIFORM, inputs.wind);
    let b = b.uniform(CANOPY_FACTOR_UNIFORM, canopy_factor(inputs.season_param));
    let b = b.uniform(TREE_SEASON3_UNIFORM, inputs.season_is_3);
    let b = b.uniform(TREE_BLEND_UNIFORM, blend_value(inputs.blend));
    nani(b, GpuWorldSubsetKind::Canopy);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::render_loop::draw_passes::draw_shaders::test_support::{
        Call, Rig, as_f32, as_i32, uniform_at,
    };

    #[test]
    fn season_table_and_param_offset() {
        assert_eq!(SEASON_OFFSETS.len(), 6);
        assert_eq!(canopy_param_offset(0), 0x8c + 0x104);
        assert_eq!(canopy_param_offset(3), 0x264 + 0x104);
        assert_eq!(canopy_param_offset(5), 0x3a4 + 0x104);
    }

    #[test]
    fn canopy_factor_clamps_at_0_07_before_scaling() {
        // 0 度 → max(0, 0.07) * 0.8 = 0.056
        assert!((canopy_factor(0.0) - 0.056).abs() < 1e-6);
        // 80 度 → 80 * π/180 * 0.5 ≈ 0.698 → * 0.8 ≈ 0.558
        assert!((canopy_factor(80.0) - 0.5585).abs() < 1e-3);
        // 刚好跨过阈值：param * π/360 > 0.07 ⇔ param > 8.03
        assert!((canopy_factor(8.0) - 0.056).abs() < 1e-6);
        assert!(canopy_factor(9.0) > 0.056);
    }

    #[test]
    fn blend_is_linear_between_15_and_30() {
        assert_eq!(blend_value(0.0), 15.0);
        assert_eq!(blend_value(1.0), 30.0);
        assert_eq!(blend_value(0.5), 22.5);
    }

    #[test]
    fn cull_only_applies_when_the_draw_mode_byte_is_non_zero() {
        // 缺省 DrawBuilder::new 的 +0xf0 = 0：状态不变。
        let mut rig = Rig::new(&[], &[]);
        let mut b = rig.builder();
        let before = b.packet().state;
        apply_cull(&mut b);
        assert_eq!(b.packet().state, before);
        b.draw();
        rig.finish();
        assert_eq!(tree_state(0x8000_008c), 0x8000_0084);
    }

    fn trunk_names() -> Vec<u64> {
        let mut v: Vec<u64> = TRUNK_TEXTURES.iter().map(|t| t.1).collect();
        v.push(TRUNK_PRESET.1);
        v
    }

    #[test]
    fn trunk_binds_six_textures_then_preset_e_and_calls_nani_with_trunk() {
        let uniforms = [
            TREE_WIND_UNIFORM,
            TREE_EYE_UNIFORM,
            TREE_SEASON3_UNIFORM,
            TREE_BLEND_UNIFORM,
        ];
        let mut rig = Rig::new(&trunk_names(), &uniforms);
        let mut kind = None;
        tree_trunk(
            rig.builder(),
            &TreeTrunkInputs {
                wind: 0.3,
                eye: Vec3::new(1.0, 2.0, 3.0),
                season_is_3: true,
                blend: 0.25,
            },
            |b, k| {
                kind = Some(k);
                b.draw();
            },
        );
        assert_eq!(kind, Some(GpuWorldSubsetKind::TreeTrunk));
        assert_eq!(GpuWorldSubsetKind::TreeTrunk as u8, 1);
        let mut want: Vec<Call> = TRUNK_TEXTURES.iter().map(|t| Call::Disk(t.0)).collect();
        want.push(Call::Preset(0xe));
        assert_eq!(rig.calls(), want);
        rig.finish();
    }

    #[test]
    fn trunk_and_canopy_uniform_layout() {
        let uniforms = [
            TREE_WIND_UNIFORM,
            TREE_EYE_UNIFORM,
            TREE_SEASON3_UNIFORM,
            TREE_BLEND_UNIFORM,
        ];
        let mut rig = Rig::new(&trunk_names(), &uniforms);
        let mut captured = Vec::new();
        tree_trunk(
            rig.builder(),
            &TreeTrunkInputs {
                wind: 0.3,
                eye: Vec3::new(1.0, 2.0, 3.0),
                season_is_3: true,
                blend: 0.25,
            },
            |b, _| {
                captured = b.constants_data().unwrap().to_vec();
                b.draw();
            },
        );
        assert_eq!(as_f32(&uniform_at(&captured, 0, 4)), 0.3);
        let eye = uniform_at(&captured, 1, 12);
        assert_eq!(
            [as_f32(&eye[0..4]), as_f32(&eye[4..8]), as_f32(&eye[8..12])],
            [1.0, 2.0, 3.0]
        );
        assert_eq!(as_i32(&uniform_at(&captured, 2, 4)), 1);
        assert_eq!(as_f32(&uniform_at(&captured, 3, 4)), blend_value(0.25));
        rig.finish();
    }

    #[test]
    fn canopy_has_five_textures_no_preset_and_nani_canopy() {
        let names: Vec<u64> = CANOPY_TEXTURES.iter().map(|t| t.1).collect();
        let uniforms = [
            TREE_WIND_UNIFORM,
            CANOPY_FACTOR_UNIFORM,
            TREE_SEASON3_UNIFORM,
            TREE_BLEND_UNIFORM,
        ];
        let mut rig = Rig::new(&names, &uniforms);
        let mut kind = None;
        let mut data = Vec::new();
        tree_canopy(
            rig.builder(),
            &TreeCanopyInputs {
                wind: 0.5,
                season_param: 80.0,
                season_is_3: false,
                blend: 1.0,
            },
            |b, k| {
                kind = Some(k);
                data = b.constants_data().unwrap().to_vec();
                b.draw();
            },
        );
        assert_eq!(kind, Some(GpuWorldSubsetKind::Canopy));
        assert_eq!(GpuWorldSubsetKind::Canopy as u8, 0);
        assert_eq!(
            rig.calls(),
            CANOPY_TEXTURES
                .iter()
                .map(|t| Call::Disk(t.0))
                .collect::<Vec<_>>()
        );
        assert_eq!(as_f32(&uniform_at(&data, 1, 4)), canopy_factor(80.0));
        assert_eq!(as_i32(&uniform_at(&data, 2, 4)), 0);
        assert_eq!(as_f32(&uniform_at(&data, 3, 4)), 30.0);
        rig.finish();
    }

    #[test]
    fn trunk_and_canopy_share_textures_with_the_expected_overlap() {
        // 机器码里树干/树冠共用三张纹理（0x93201d69…、0xaa12b013…、0x004dbb39…）。
        let shared: Vec<_> = CANOPY_TEXTURES
            .iter()
            .filter(|t| TRUNK_TEXTURES.contains(t))
            .collect();
        assert_eq!(shared.len(), 3);
    }
}
