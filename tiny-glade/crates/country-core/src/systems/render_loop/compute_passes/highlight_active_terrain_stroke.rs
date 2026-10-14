//! `highlight_active_terrain_stroke.rs` — 高亮当前编辑的地形笔画（W8，0x140a58450，3531 B）。
//!
//! 机器码（O）：三个 world 资源（缺失 → `Option::unwrap`）：
//! * `0xc2b8e941c91a6428 / 0x59f3c6d1d3da43a0`：读 `u32` 判别式 `m`（应用/编辑模式，名 U）。只有 `m == 5` 或 `m == 10` 才取下面的事件
//!   （机器码：`m-6 < 0xf ? m-5 : 0`，`== 5 || (== 0 && m == 5)`）。
//! * `0xc26ebbeefbfc3261 / 0x7a483ee28328d5ec`：`Events<AddTerrainHighlightCmd>`（双缓冲，每项 16 B：`EventId` 8 B + 载荷 `i32 @+8`）。
//!   `EventCursor::read` + `last`：**新缓冲非空取新缓冲最后一项，否则取旧缓冲最后一项**；两者皆空则下标保持 -1。
//! * `0x48b321ef779dc301 / 0xbd65767194d9ddc8`：读 `u8 @+0xf9`；非 0 → 下标强制 -1（该资源在任何模式下都读）。
//!
//! `idx = -1` 为默认。**Pass 1** `ComputePass::new("Create a highlighted terrain stroke mask", "/terrain_editing/calc_highlight_hill_mask.cs.hlsl")`
//! （名长 0x28、URI 长 0x31）：uniform i32 `highlight_stroke_index = idx` → `stroke_indices`（预设 0xb，读 0x19）→ `img_out`（预设 0xc，写 0x32）→
//! `dispatch_2d(0x100, 0x100)`（常量 `{1, 0x100, 0x100, 1}`，0x142ad0480）。
//! `idx != -1` 时再做两遍高斯模糊：`create_raw_resource("blur temp")`（名长 9）`new_2d((0x100,0x100), 0x4c)`；
//! `ComputePass::new("Blur Terrain Highlight Mask", "/gauss_blur_r16f.cs.hlsl")`（名长 0x1b、URI 长 0x18）两次：
//! ① `is_horizontal = true`、`sigma = 1.0`、`img_in`（预设 0xc，读）→ `img_out`（temp，写 0x32）；
//! ② `is_horizontal = false`、`sigma = 1.0`、`img_in`（temp，读）→ `img_out`（预设 0xc，写 0x32）；均 `dispatch_2d(0x100, 0x100)`。
//!
//! `AddTerrainHighlightCmd` 的载荷为单个 `i32`（O：`+8` 处读 i32）；其余字节 U。

// Rust 1.88 enables newer style suggestions; retain original nested branches and modulo expressions.
#![allow(clippy::collapsible_if)]
use crate::render::pass_builder::compute_pass_builder::RecordedComputePass;
use crate::render::pass_builder::pass_resource_access::WriteTextureUnchecked;
use crate::render::pass_builder::pass_resources::TypeId128;
use crate::resources::texture_presets::TexturePreset;
use crate::rhapsody_types::TextureDesc;
use crate::systems::render_loop::compute_passes::pass_kit::{PassEnv, binding, binding_hash};

pub const MASK: (&str, &str) = (
    "Create a highlighted terrain stroke mask",
    "/terrain_editing/calc_highlight_hill_mask.cs.hlsl",
);
pub const BLUR: (&str, &str) = ("Blur Terrain Highlight Mask", "/gauss_blur_r16f.cs.hlsl");
pub const BLUR_TEMP_NAME: &str = "blur temp";
pub const BLUR_TEMP_FORMAT: u32 = 0x4c;
pub const MASK_SIZE: u32 = 0x100;
pub const HIGHLIGHT_STROKE_INDEX: u64 = binding_hash("highlight_stroke_index");
/// 模式资源 TypeId（O）。
pub const MODE_TYPE_ID: TypeId128 = TypeId128 {
    lo: 0xc2b8_e941_c91a_6428,
    hi: 0x59f3_c6d1_d3da_43a0,
};
/// `Events<AddTerrainHighlightCmd>` TypeId（O）。
pub const EVENTS_TYPE_ID: TypeId128 = TypeId128 {
    lo: 0xc26e_bbee_fbfc_3261,
    hi: 0x7a48_3ee2_8328_d5ec,
};
/// 抑制资源 TypeId（O；字节 @+0xf9）。
pub const SUPPRESS_TYPE_ID: TypeId128 = TypeId128 {
    lo: 0x48b3_21ef_779d_c301,
    hi: 0xbd65_7671_94d9_ddc8,
};
pub const SUPPRESS_BYTE_OFFSET: usize = 0xf9;

/// 高亮笔画事件（O 名；载荷 `i32 @+8`）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AddTerrainHighlightCmd(pub i32);

/// 选择高亮的笔画下标（O：见模块文档；默认 -1）。
pub fn highlight_stroke_index(
    mode: u32,
    events_old: &[AddTerrainHighlightCmd],
    events_new: &[AddTerrainHighlightCmd],
    suppress: u8,
) -> i32 {
    let mut idx = -1;
    if mode == 5 || mode == 10 {
        if let Some(last) = events_new.last().or_else(|| events_old.last()) {
            idx = last.0;
        }
    }
    if suppress != 0 {
        idx = -1;
    }
    idx
}

/// W8：返回已声明的 pass（`idx == -1` 时 1 个，否则 3 个）。
pub fn highlight_active_terrain_stroke(
    env: &mut PassEnv<'_>,
    mode: u32,
    events_old: &[AddTerrainHighlightCmd],
    events_new: &[AddTerrainHighlightCmd],
    suppress: u8,
) -> Vec<RecordedComputePass> {
    let idx = highlight_stroke_index(mode, events_old, events_new, suppress);
    let mask = env
        .pass(MASK.0, MASK.1)
        .uniform(HIGHLIGHT_STROKE_INDEX, idx)
        .bind_texture(binding::STROKE_INDICES, &TexturePreset(0xb), 0x19)
        .bind_texture(binding::IMG_OUT, &TexturePreset(0xc), 0x32)
        .dispatch_2d(MASK_SIZE, MASK_SIZE);
    let mut passes = vec![mask];
    if idx == -1 {
        return passes;
    }

    let temp = env.create_texture(
        BLUR_TEMP_NAME,
        TextureDesc::new_2d([MASK_SIZE, MASK_SIZE], BLUR_TEMP_FORMAT),
    );
    passes.push(
        env.pass(BLUR.0, BLUR.1)
            .uniform(binding::IS_HORIZONTAL, true)
            .uniform(binding::SIGMA, 1.0f32)
            .bind_texture(binding::IMG_IN, &TexturePreset(0xc), 0x19)
            .bind_texture(binding::IMG_OUT, &WriteTextureUnchecked(temp), 0x32)
            .dispatch_2d(MASK_SIZE, MASK_SIZE),
    );
    passes.push(
        env.pass(BLUR.0, BLUR.1)
            .uniform(binding::IS_HORIZONTAL, false)
            .uniform(binding::SIGMA, 1.0f32)
            .bind_texture(binding::IMG_IN, &&temp, 0x19)
            .bind_texture(binding::IMG_OUT, &TexturePreset(0xc), 0x32)
            .dispatch_2d(MASK_SIZE, MASK_SIZE),
    );
    passes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::render_loop::compute_passes::pass_kit::test_env::{accesses, env, threads};

    const fn e(i: i32) -> AddTerrainHighlightCmd {
        AddTerrainHighlightCmd(i)
    }

    #[test]
    fn names_match_exe_immediates() {
        assert_eq!(HIGHLIGHT_STROKE_INDEX, 0xb02f_eb01_7e06_f117);
        assert_eq!(binding::IMG_OUT, 0x1ca5_1a73_3732_08db);
        assert_eq!(binding::IMG_IN, 0xd82d_7a0a_db91_64c2);
        assert_eq!(binding::IS_HORIZONTAL, 0xd91f_f837_2094_bc22);
        assert_eq!(binding::SIGMA, 0xdc02_2c4f_3092_a794);
    }

    #[test]
    fn index_selection_follows_mode_events_and_suppress() {
        let old = [e(1), e(2)];
        let new = [e(7), e(8)];
        // 模式 5 / 10：新缓冲最后一项优先
        assert_eq!(highlight_stroke_index(5, &old, &new, 0), 8);
        assert_eq!(highlight_stroke_index(10, &old, &new, 0), 8);
        // 新缓冲为空 → 旧缓冲最后一项
        assert_eq!(highlight_stroke_index(5, &old, &[], 0), 2);
        // 双空 → -1
        assert_eq!(highlight_stroke_index(5, &[], &[], 0), -1);
        // 其它模式（含边界 4/6/9/11/20）→ -1
        for m in [0u32, 4, 6, 9, 11, 20, 21, u32::MAX] {
            assert_eq!(highlight_stroke_index(m, &old, &new, 0), -1, "mode {m}");
        }
        // 抑制字节非 0 → 强制 -1
        assert_eq!(highlight_stroke_index(5, &old, &new, 1), -1);
        assert_eq!(highlight_stroke_index(10, &old, &new, 0xff), -1);
    }

    #[test]
    fn without_a_highlight_only_the_mask_pass_runs() {
        let mut env = env(
            &[binding::STROKE_INDICES, binding::IMG_OUT],
            &[HIGHLIGHT_STROKE_INDEX],
        );
        let passes = env.run(|p| highlight_active_terrain_stroke(p, 0, &[], &[], 0));
        assert_eq!(passes.len(), 1);
        assert_eq!(passes[0].name, MASK.0);
        assert_eq!(threads(&passes[0]), (256, 256, 1));
        assert_eq!(
            env.uniform_u32(&passes[0], HIGHLIGHT_STROKE_INDEX) as i32,
            -1
        );
        assert!(env.frame.textures.is_empty());
    }

    #[test]
    fn with_a_highlight_it_blurs_horizontally_then_vertically() {
        let mut env = env(
            &[binding::STROKE_INDICES, binding::IMG_OUT, binding::IMG_IN],
            &[
                HIGHLIGHT_STROKE_INDEX,
                binding::IS_HORIZONTAL,
                binding::SIGMA,
            ],
        );
        let passes = env.run(|p| highlight_active_terrain_stroke(p, 10, &[], &[e(42)], 0));
        assert_eq!(passes.len(), 3);
        assert_eq!(env.uniform_u32(&passes[0], HIGHLIGHT_STROKE_INDEX), 42);
        let (n, s, t) = &env.frame.textures[0];
        assert_eq!(
            (n.as_str(), s.extent_2d(), s.format),
            ("blur temp", [256, 256], 0x4c)
        );
        assert_eq!((passes[1].name, passes[2].name), (BLUR.0, BLUR.0));
        assert_eq!(env.uniform_u32(&passes[1], binding::IS_HORIZONTAL), 1);
        assert_eq!(env.uniform_u32(&passes[2], binding::IS_HORIZONTAL), 0);
        assert_eq!(env.uniform_f32(&passes[1], binding::SIGMA), 1.0);
        assert_eq!(env.uniform_f32(&passes[2], binding::SIGMA), 1.0);
        // ① img_out = temp（写）；② img_in = temp（读）
        let a1 = accesses(&passes[1]);
        let a2 = accesses(&passes[2]);
        assert_eq!((a1[1].0, a1[1].1), (t.resource, 0x32));
        assert_eq!((a2[0].0, a2[0].1), (t.resource, 0x19));
        for p in &passes {
            assert_eq!(threads(p), (256, 256, 1));
        }
    }
}
