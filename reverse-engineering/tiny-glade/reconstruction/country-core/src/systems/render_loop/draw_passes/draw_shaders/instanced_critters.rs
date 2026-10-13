//! instanced_critters.rs — 蝴蝶 / 萤火虫 / 萤火虫光晕 / 鸭 / 鹪鹩 的实例绘制。
//!
//! PDB `systems/render_loop/draw_passes/draw_shaders/instanced_critters.rs`。机器码（O）：
//! `draw_instanced_butterfly_shader` 0x1415547e0（456 B）、`draw_instanced_firefly_shader` 0x1415549b0（828 B）、
//! `draw_instanced_firefly_glow_shader` 0x141554cf0（908 B）、`draw_duck_shader` 0x141555080（3637 B）、
//! `draw_wren_shader` 0x141555ec0（437 B）。
//!
//! 公共：预设 9 / 10（名 0x61b08b5e… / 0x297d1e4c…）。SSBO 的槽名 / `SsboId` 在 `resources::critters`。
//!
//! - **蝴蝶**：预设 9/10 → `read_storage_buffer<ButterflySsbo>`（名 0x286b288f058224b7）→ `TextureFromDisk(0x56463770c9cae8e1)`
//!   （名 0x9014886b413368cd）→ `draw_instanced_w_gl_draw_flag(0x18)`（= `ButterflySsbo::BUFFER_LEN`）。
//! - **萤火虫**：世界资源（TypeId 0xa2127765959c8b85/0xf078ccfbcec1f347）的 `f32 @+0` `intensity`；`intensity == 0.0` →
//!   `discard`；否则 预设 9/10 → 常量块存在 → `write_uniform<f32>(0x075a93062ce954b7, intensity)` →
//!   `read_storage_buffer<FireflySsbo>`（名 0x7d0b7ff9bdcd8389）→ `draw_instanced_w_gl_draw_flag(0x60)`。
//! - **萤火虫光晕**：同上，再 `read_storage_buffer<FireflyVisibilitySsbo>`（名 0x66db876b0140f6f2）、
//!   `read_storage_buffer<DofAutofocusSsbo>`（名 0xc26554c6de4b08b0）后同样绘制 0x60 个。
//! - **鸭**：实体上三个组件（TypeId 0xa0da022fc84ca3d9 / 0x272a1f294225ce7f / 0x3a8f7d4d3ac26bd0）：`Transform`、
//!   鸭参数（`f32 @+0xf0/+0xf4/+0xf8`、`u8 @+0xfc`）、状态字节。预设 9/10 → 七个 uniform → `draw()`（见 [`DuckInputs`]）。
//! - **鹪鹩**：预设 9/10 → `TextureFromDisk(0xe69294dae8320741)`（名 0x79240fc945c8cdb6）→ `nani_draw(Wren = 0x11)`。

use glam::{Mat4, Quat, Vec3};

use crate::render::nani::gpu_world::GpuWorldSubsetKind;
use crate::render::pass_builder::draw_builder::DrawBuilder;
use crate::render::ssbo::StorageElement;
use crate::systems::render_loop::draw_passes::draw_shaders::common_includes::{
    bind_disk, bind_terrain_heightmap, bind_terrain_lighting,
};
use crate::systems::render_loop::draw_passes::draw_shaders::ssbo_tags::{
    DofAutofocusTag, ReadSsboExt,
};
use crate::systems::render_loop::resources::critters::{
    ButterflySsbo, FireflySsbo, FireflyVisibilitySsbo,
};

pub const BUTTERFLY_BINDING: u64 = 0x286b_288f_0582_24b7;
pub const BUTTERFLY_TEXTURE: (u64, u64) = (0x5646_3770_c9ca_e8e1, 0x9014_886b_4133_68cd);
pub const FIREFLY_INTENSITY_UNIFORM: u64 = 0x075a_9306_2ce9_54b7;
pub const FIREFLY_BINDING: u64 = 0x7d0b_7ff9_bdcd_8389;
pub const FIREFLY_VISIBILITY_BINDING: u64 = 0x66db_876b_0140_f6f2;
pub const FIREFLY_DOF_BINDING: u64 = 0xc265_54c6_de4b_08b0;
pub const WREN_TEXTURE: (u64, u64) = (0xe692_94da_e832_0741, 0x7924_0fc9_45c8_cdb6);

// 鸭的 uniform 名（O，按写入顺序）。
pub const DUCK_MODEL_MATRIX: u64 = 0x702c_4e77_e30f_d575;
pub const DUCK_F32_F0: u64 = 0x3cc1_85d8_0517_a5cb;
pub const DUCK_F32_F8: u64 = 0x3999_9284_66b1_1a05;
pub const DUCK_BOOL_FC: u64 = 0x685c_1514_1404_a981;
pub const DUCK_F32_F4: u64 = 0x75f3_74ac_3594_12b7;
pub const DUCK_STATE_LT_2: u64 = 0xf4e0_6bc0_56d9_47d9; // -0xb1f943fa926b827 取补
pub const DUCK_STATE_EQ_3: u64 = 0xf63b_8b81_ea72_ad20; // -0x9c4747e158d52e0 取补

/// 蝴蝶：`draw_instanced_butterfly_shader`（O 0x1415547e0）。
pub fn draw_instanced_butterfly_shader(b: DrawBuilder<'_>) {
    let b = bind_terrain_heightmap(b);
    let b = bind_terrain_lighting(b);
    let b = b.read_ssbo::<ButterflySsbo>(BUTTERFLY_BINDING);
    let b = bind_disk(b, BUTTERFLY_TEXTURE.0, BUTTERFLY_TEXTURE.1);
    b.draw_instanced_w_gl_draw_flag(ButterflySsbo::BUFFER_LEN as u32);
}

/// `ucomiss intensity, 0.0; jne/jp` 的取反：恰为 0（含 -0.0）→ 丢弃；NaN 走绘制分支。
pub fn firefly_is_hidden(intensity: f32) -> bool {
    intensity == 0.0
}

/// 萤火虫：`draw_instanced_firefly_shader`（O 0x1415549b0）。
pub fn draw_instanced_firefly_shader(b: DrawBuilder<'_>, intensity: f32) {
    if firefly_is_hidden(intensity) {
        b.discard();
        return;
    }
    let b = bind_terrain_heightmap(b);
    let b = bind_terrain_lighting(b);
    let b = b.uniform(FIREFLY_INTENSITY_UNIFORM, intensity);
    let b = b.read_ssbo::<FireflySsbo>(FIREFLY_BINDING);
    b.draw_instanced_w_gl_draw_flag(FireflySsbo::BUFFER_LEN as u32);
}

/// 萤火虫光晕：`draw_instanced_firefly_glow_shader`（O 0x141554cf0）。
pub fn draw_instanced_firefly_glow_shader(b: DrawBuilder<'_>, intensity: f32) {
    if firefly_is_hidden(intensity) {
        b.discard();
        return;
    }
    let b = bind_terrain_heightmap(b);
    let b = bind_terrain_lighting(b);
    let b = b.uniform(FIREFLY_INTENSITY_UNIFORM, intensity);
    let b = b.read_ssbo::<FireflySsbo>(FIREFLY_BINDING);
    let b = b.read_ssbo::<FireflyVisibilitySsbo>(FIREFLY_VISIBILITY_BINDING);
    let b = b.read_ssbo::<DofAutofocusTag>(FIREFLY_DOF_BINDING);
    b.draw_instanced_w_gl_draw_flag(FireflySsbo::BUFFER_LEN as u32);
}

/// 鸭实体上取到的数据。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DuckInputs {
    /// `Transform`（旋转 @0、平移 @0x10、缩放 @0x1c）。
    pub rotation: Quat,
    pub translation: Vec3,
    pub scale: Vec3,
    /// 鸭参数组件：`f32 @+0xf0`、`+0xf4`、`+0xf8` 与 `u8 @+0xfc`。
    pub f_f0: f32,
    pub f_f4: f32,
    pub f_f8: f32,
    pub b_fc: bool,
    /// 状态字节（第三个组件的首字节）。
    pub state: u8,
}

impl DuckInputs {
    /// `Transform::compute_matrix` 内联（O 0x14155531a 起）：`Mat4::from_scale_rotation_translation`。
    pub fn model_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation)
    }
}

/// 鸭：`draw_duck_shader`（O 0x141555080）。
pub fn draw_duck_shader(b: DrawBuilder<'_>, duck: &DuckInputs) {
    let b = bind_terrain_heightmap(b);
    let b = bind_terrain_lighting(b);
    let b = b.uniform(DUCK_MODEL_MATRIX, duck.model_matrix());
    let b = b.uniform(DUCK_F32_F0, duck.f_f0);
    let b = b.uniform(DUCK_F32_F8, duck.f_f8);
    let b = b.uniform(DUCK_BOOL_FC, duck.b_fc);
    let b = b.uniform(DUCK_F32_F4, duck.f_f4);
    let b = b.uniform(DUCK_STATE_LT_2, (duck.state < 2) as i32);
    let b = b.uniform(DUCK_STATE_EQ_3, (duck.state == 3) as i32);
    b.draw();
}

/// 鹪鹩：`draw_wren_shader`（O 0x141555ec0）。`nani` = `NaniDrawExt::nani_draw`（带 `GpuWorldSubsetKind::Wren`）。
pub fn draw_wren_shader<'p>(
    b: DrawBuilder<'p>,
    nani: impl FnOnce(DrawBuilder<'p>, GpuWorldSubsetKind),
) {
    let b = bind_terrain_heightmap(b);
    let b = bind_terrain_lighting(b);
    let b = bind_disk(b, WREN_TEXTURE.0, WREN_TEXTURE.1);
    nani(b, GpuWorldSubsetKind::Wren);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::pass_builder::draw_stream::DrawKind;
    use crate::systems::render_loop::draw_passes::draw_shaders::common_includes::{
        TERRAIN_HEIGHTMAP_NAME, TERRAIN_LIGHTING_NAME,
    };
    use crate::systems::render_loop::draw_passes::draw_shaders::test_support::{
        Call, Rig, as_f32, as_i32, rg, uniform_at,
    };

    fn base() -> Vec<u64> {
        vec![TERRAIN_HEIGHTMAP_NAME, TERRAIN_LIGHTING_NAME]
    }

    #[test]
    fn butterfly_draws_all_24_instances_in_order() {
        let mut names = base();
        names.extend([BUTTERFLY_BINDING, BUTTERFLY_TEXTURE.1]);
        let mut rig = Rig::new(&names, &[]);
        draw_instanced_butterfly_shader(rig.builder());
        assert_eq!(
            rig.calls(),
            vec![
                Call::Preset(9),
                Call::Preset(10),
                rg::<ButterflySsbo>(),
                Call::Disk(0x56463770c9cae8e1)
            ]
        );
        assert_eq!(
            rig.finish()[0].kind,
            DrawKind::InstancedGlDrawFlag { flag: 0x18 }
        );
    }

    #[test]
    fn firefly_hidden_at_zero_intensity_is_discarded() {
        assert!(firefly_is_hidden(0.0) && firefly_is_hidden(-0.0));
        assert!(!firefly_is_hidden(0.001) && !firefly_is_hidden(f32::NAN));
        let mut rig = Rig::new(&base(), &[FIREFLY_INTENSITY_UNIFORM]);
        draw_instanced_firefly_shader(rig.builder(), 0.0);
        assert!(rig.finish().is_empty());
        assert!(rig.calls().is_empty(), "discard 发生在任何绑定之前");
    }

    #[test]
    fn firefly_writes_intensity_then_reads_the_buffer_and_draws_96() {
        let mut names = base();
        names.push(FIREFLY_BINDING);
        let mut rig = Rig::new(&names, &[FIREFLY_INTENSITY_UNIFORM]);
        draw_instanced_firefly_shader(rig.builder(), 0.75);
        assert_eq!(
            rig.calls(),
            vec![Call::Preset(9), Call::Preset(10), rg::<FireflySsbo>()]
        );
        assert_eq!(
            rig.finish()[0].kind,
            DrawKind::InstancedGlDrawFlag { flag: 0x60 }
        );
        let mut rig = Rig::new(&names, &[FIREFLY_INTENSITY_UNIFORM]);
        let b = rig.builder().uniform(FIREFLY_INTENSITY_UNIFORM, 0.75f32);
        assert_eq!(as_f32(&uniform_at(b.constants_data().unwrap(), 0, 4)), 0.75);
        b.draw();
        rig.finish();
    }

    #[test]
    fn glow_additionally_reads_visibility_and_dof() {
        let mut names = base();
        names.extend([
            FIREFLY_BINDING,
            FIREFLY_VISIBILITY_BINDING,
            FIREFLY_DOF_BINDING,
        ]);
        let mut rig = Rig::new(&names, &[FIREFLY_INTENSITY_UNIFORM]);
        draw_instanced_firefly_glow_shader(rig.builder(), 1.0);
        assert_eq!(
            rig.calls(),
            vec![
                Call::Preset(9),
                Call::Preset(10),
                rg::<FireflySsbo>(),
                rg::<FireflyVisibilitySsbo>(),
                rg::<DofAutofocusTag>()
            ]
        );
        assert_eq!(
            rig.finish()[0].kind,
            DrawKind::InstancedGlDrawFlag { flag: 0x60 }
        );
        let mut rig = Rig::new(&names, &[FIREFLY_INTENSITY_UNIFORM]);
        draw_instanced_firefly_glow_shader(rig.builder(), 0.0);
        assert!(rig.finish().is_empty());
    }

    fn duck(state: u8) -> DuckInputs {
        DuckInputs {
            rotation: Quat::from_rotation_y(0.5),
            translation: Vec3::new(1.0, 2.0, 3.0),
            scale: Vec3::new(1.0, 2.0, 0.5),
            f_f0: 0.1,
            f_f4: 0.2,
            f_f8: 0.3,
            b_fc: true,
            state,
        }
    }

    #[test]
    fn duck_uniform_values_and_order() {
        let uniforms = [
            DUCK_MODEL_MATRIX,
            DUCK_F32_F0,
            DUCK_F32_F8,
            DUCK_BOOL_FC,
            DUCK_F32_F4,
            DUCK_STATE_LT_2,
            DUCK_STATE_EQ_3,
        ];
        for state in 0..5u8 {
            let mut rig = Rig::new(&base(), &uniforms);
            let d = duck(state);
            let b = bind_terrain_lighting(bind_terrain_heightmap(rig.builder()));
            let b = b.uniform(DUCK_MODEL_MATRIX, d.model_matrix());
            let b = b
                .uniform(DUCK_F32_F0, d.f_f0)
                .uniform(DUCK_F32_F8, d.f_f8)
                .uniform(DUCK_BOOL_FC, d.b_fc);
            let b = b.uniform(DUCK_F32_F4, d.f_f4);
            let b = b
                .uniform(DUCK_STATE_LT_2, (d.state < 2) as i32)
                .uniform(DUCK_STATE_EQ_3, (d.state == 3) as i32);
            let data = b.constants_data().unwrap().to_vec();
            assert_eq!(
                as_i32(&uniform_at(&data, 5, 4)),
                (state < 2) as i32,
                "state {state}"
            );
            assert_eq!(
                as_i32(&uniform_at(&data, 6, 4)),
                (state == 3) as i32,
                "state {state}"
            );
            assert_eq!(as_f32(&uniform_at(&data, 1, 4)), 0.1);
            assert_eq!(
                as_f32(&uniform_at(&data, 2, 4)),
                0.3,
                "第二个 f32 uniform 取 +0xf8"
            );
            assert_eq!(
                as_f32(&uniform_at(&data, 4, 4)),
                0.2,
                "最后一个 f32 uniform 取 +0xf4"
            );
            assert_eq!(as_i32(&uniform_at(&data, 3, 4)), 1);
            b.draw();
            rig.finish();
        }
    }

    #[test]
    fn duck_draw_uses_the_default_draw_kind_and_matrix_is_srt() {
        let uniforms = [
            DUCK_MODEL_MATRIX,
            DUCK_F32_F0,
            DUCK_F32_F8,
            DUCK_BOOL_FC,
            DUCK_F32_F4,
            DUCK_STATE_LT_2,
            DUCK_STATE_EQ_3,
        ];
        let mut rig = Rig::new(&base(), &uniforms);
        draw_duck_shader(rig.builder(), &duck(3));
        assert_eq!(rig.calls(), vec![Call::Preset(9), Call::Preset(10)]);
        assert_eq!(
            rig.finish()[0].kind,
            DrawKind::InstancedGlDrawFlag { flag: 1 }
        );
        let m = duck(0).model_matrix();
        assert!(
            (m.w_axis.truncate() - Vec3::new(1.0, 2.0, 3.0)).length() < 1e-6,
            "平移在第 4 列"
        );
        assert!(
            (m.y_axis.truncate().length() - 2.0).abs() < 1e-6,
            "y 缩放 2"
        );
    }

    #[test]
    fn wren_hands_the_wren_subset_to_nani_draw() {
        let mut names = base();
        names.push(WREN_TEXTURE.1);
        let mut rig = Rig::new(&names, &[]);
        let mut seen = None;
        draw_wren_shader(rig.builder(), |b, kind| {
            seen = Some(kind);
            b.draw();
        });
        assert_eq!(seen, Some(GpuWorldSubsetKind::Wren));
        assert_eq!(GpuWorldSubsetKind::Wren as u8, 0x11, "O：`mov dl, 0x11`");
        assert_eq!(
            rig.calls(),
            vec![
                Call::Preset(9),
                Call::Preset(10),
                Call::Disk(0xe69294dae8320741)
            ]
        );
        rig.finish();
    }
}
