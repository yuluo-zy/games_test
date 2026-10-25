//! `taa.rs` — 时间抗锯齿 / TGSR（W52，二选一）。
//!
//! # 来源（O）
//! * `calculate_taa` 0x140a3b2f0（1891 B）、`calculate_tgsr` 0x140a3ba60（6211 B）、
//!   `TgsrMain::as_compute_pipeline` 0x140a3e100（1682 B）、`ComputePass::new<TgsrMain>` 0x141600e60、
//!   `calculate_hash<(&str, &TgsrMain)>` 0x14145ac20。
//! * HLSL：`reconstruction/shader-source/shaders/{taa,tgsr,tgsr_reproject,tgsr_blur_lum}.cs.hlsl`；
//!   绑定名由 FNV-1a 反解（`pass_kit::binding_hash`）。
//!
//! # `calculate_taa`（O）
//! pass `"TAA"`（`/taa.cs.hlsl`）：uniform `history_valid`(i32)；`color_tex`/`history_tex`（temporal 只读）/
//! `reprojection_tex` 读 0x19；`history_output_tex`/`output_tex` 写 0x32；uniform `bypass_neighborhood_clamping`(i32，
//! 取自某资源 `+0x20` 的 u8)、`sharpen_amount`(f32)；`dispatch_2d(history_output.w, history_output.h)`。
//! 返回 `output`。
//!
//! # `calculate_tgsr`（O）
//! 1. 取 `bypass`（同上资源 u8）。
//! 2. `get_or_create_temporal_pair("tgsr_aux", new_2d(output_extent, 0x10).sampled().storage())`
//!    （`Err` → `Result::unwrap` panic，43 字节的 `called \`Result::unwrap()\` on an \`Err\` value`）。
//! 3. 三个 RG 纹理：`"reprojected history lum"`（输出 extent，格式 0x4c）、`"reprojected history chroma"`
//!    （输出 extent，格式 0x53）、`"blurred luminance"`（颜色 extent，格式 9）。
//! 4. `"TGSR blur lum"`（`/tgsr_blur_lum.cs.hlsl`）：`color_tex`(0x19) → `output_tex`(0x32 = blurred lum)，
//!    `dispatch_2d(color extent)`。
//! 5. `"TGSR reproject"`（`/tgsr_reproject.cs.hlsl`）：`history_valid` + `history_tex`/`reprojection_tex`/`depth_tex`
//!    读，`output_lum_tex`/`output_chrom_tex` 写，`dispatch_2d(output extent)`。
//! 6. `"TGSR"`（`TgsrMain {wave32, sharpen}`）：`history_valid`(= 历史有效 && aux 有效 → `!全局`)、
//!    `color_tex`、`blurred_luminance_tex`、`history_tex`、`reprojected_history_tex_lum/chroma`、`reprojection_tex`、
//!    `aux_tex`(temporal 只读)读；`history_output_tex`、`output_tex`、`aux_output_tex` 写；uniform
//!    `bypass_neighborhood_clamping`、`sharpen_amount`；`dispatch_2d(output extent)`。返回 `output`。
//!
//! # 偏离 / 缺口
//! * aux 对描述 = `new_2d(extent, 0x10).sampled().storage()`（O 0x140bd5370 / 0x140bd5390；真实 `TextureDesc`）。
//! * `TgsrMain` 的 `PERM_wave32` / `PERM_sharpen` define 经 `AsComputeShaderSource::pipeline_defines` 进入
//!   `ComputePipelineDesc.defines`（B 阶段 i_api 接线；原先未接入）。
//! * temporal 对 = A4 `RgResources::get_or_create_temporal_texture_pair`（经 `PassEnv::temporal_pair`，`Err` → `Result::unwrap`
//!   panic）；`HistoryTexture { tex: previous, valid: fresh }`。

use crate::render::pass_builder::compute_pass_builder::RecordedComputePass;
use crate::render::pass_builder::pass_resource_access::RgTexture;
use crate::render::pass_builder::shader_uri::{
    AsComputeShaderSource, ComputeShaderSource, ShaderDefValue, ShaderDefine, ShaderUri,
    perm_define,
};
use crate::rhapsody_types::TextureDesc;

use super::pass_kit::{PassEnv, binding_hash, extent_xy};

// ---------------------------------------------------------------- 绑定名

pub const H_HISTORY_VALID: u64 = binding_hash("history_valid");
pub const H_COLOR_TEX: u64 = binding_hash("color_tex");
pub const H_HISTORY_TEX: u64 = binding_hash("history_tex");
pub const H_REPROJECTION_TEX: u64 = binding_hash("reprojection_tex");
pub const H_HISTORY_OUTPUT_TEX: u64 = binding_hash("history_output_tex");
pub const H_OUTPUT_TEX: u64 = binding_hash("output_tex");
pub const H_BYPASS_CLAMP: u64 = binding_hash("bypass_neighborhood_clamping");
pub const H_SHARPEN: u64 = binding_hash("sharpen_amount");
pub const H_DEPTH_TEX: u64 = binding_hash("depth_tex");
pub const H_OUTPUT_LUM_TEX: u64 = binding_hash("output_lum_tex");
pub const H_OUTPUT_CHROM_TEX: u64 = binding_hash("output_chrom_tex");
pub const H_BLURRED_LUM_TEX: u64 = binding_hash("blurred_luminance_tex");
pub const H_REPROJ_HIST_LUM: u64 = binding_hash("reprojected_history_tex_lum");
pub const H_REPROJ_HIST_CHROMA: u64 = binding_hash("reprojected_history_tex_chroma");
pub const H_AUX_TEX: u64 = binding_hash("aux_tex");
pub const H_AUX_OUTPUT_TEX: u64 = binding_hash("aux_output_tex");

pub const ACCESS_READ: u8 = 0x19;
pub const ACCESS_WRITE: u8 = 0x32;
/// 全局字节 `0x143634d50`（O：`history_valid` 的 `xor 1` 对象；语义 U）。
pub const HISTORY_GLOBAL_ADDR: u64 = 0x1_4363_4d50;

// ---------------------------------------------------------------- TgsrMain

/// `/tgsr.cs.hlsl:cs_main` 源（O）。字段是两个 u32 `PERM_*` 排列值（O：`impl$1::as_compute_pipeline`
/// 对 `[self+0]`、`[self+4]` 各做一次 `u32` Display）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TgsrMain {
    pub wave32: u32,
    pub sharpen: u32,
}

impl TgsrMain {
    pub const URI_WITH_ENTRY: &'static str = "/tgsr.cs.hlsl:cs_main";

    /// `calculate_tgsr` 里的构造：`wave32 = min_subgroup_size.is_some_and(|s| s < 0x21)`，
    /// `sharpen = sharpen_amount > 0.0`（O：`cmp edx,0x21; setb; and al`；`ucomiss xmm6, 0; seta`）。
    pub fn new(min_subgroup_size: Option<u32>, sharpen_amount: f32) -> Self {
        TgsrMain {
            wave32: u32::from(min_subgroup_size.is_some_and(|s| s < 0x21)),
            sharpen: u32::from(sharpen_amount > 0.0),
        }
    }
}

impl AsComputeShaderSource for TgsrMain {
    /// O（0x14145ac20）：`calculate_hash(&("/tgsr.cs.hlsl:cs_main", &self))`，含两个 u32 字段。
    fn source_hash(&self) -> u64 {
        crate::render::utils::calculate_hash(&(Self::URI_WITH_ENTRY, self))
    }
    fn as_compute_pipeline(&self) -> ComputeShaderSource<'_> {
        ComputeShaderSource {
            uri: ShaderUri::parse(Self::URI_WITH_ENTRY),
            defs: ShaderDefValue::default(),
        }
    }
    /// `as_compute_pipeline` 的 `defines`（O 顺序：`PERM_wave32`，`PERM_sharpen`；`u32` Display）。
    fn pipeline_defines(&self) -> Vec<ShaderDefine> {
        vec![
            perm_define("PERM_wave32", self.wave32),
            perm_define("PERM_sharpen", self.sharpen),
        ]
    }
}

// ---------------------------------------------------------------- 历史 / temporal

/// temporal 只读句柄 + 有效字节（O：`ReadOnlyHandle<Texture>`，`+0x24` 的 u8 = 该历史是否有效）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HistoryTexture {
    pub tex: RgTexture,
    pub valid: bool,
}

/// `history_valid` uniform 值（O：`valid ? (G ^ 1) : 0`，写成 i32）。
pub fn history_valid_uniform(valid: bool, global: bool) -> i32 {
    if valid { i32::from(!global) } else { 0 }
}

// ---------------------------------------------------------------- calculate_taa

/// `calculate_taa` 的输入（O 参数顺序：`param_4..param_9`）。
pub struct TaaInputs<'a> {
    pub color: &'a RgTexture,
    pub output: &'a RgTexture,
    pub reprojection: &'a RgTexture,
    pub history_output: &'a RgTexture,
    pub history: &'a HistoryTexture,
    pub sharpen_amount: f32,
    /// 资源 `+0x20` 的 u8（`bypass_neighborhood_clamping`）。
    pub bypass_clamp: u8,
    /// `*0x143634d50`。
    pub history_global: bool,
}

/// W52 TAA（O 0x140a3b2f0）。返回 `output` 与声明好的 pass。
pub fn calculate_taa(env: &mut PassEnv<'_>, i: &TaaInputs<'_>) -> (RgTexture, RecordedComputePass) {
    let (w, h) = extent_xy(&i.history_output.desc);
    let pass = env
        .new_pass("TAA", &"/taa.cs.hlsl")
        .uniform(
            H_HISTORY_VALID,
            history_valid_uniform(i.history.valid, i.history_global),
        )
        .bind_texture(H_COLOR_TEX, &i.color, ACCESS_READ)
        .bind_texture(H_HISTORY_TEX, &&i.history.tex, ACCESS_READ)
        .bind_texture(H_REPROJECTION_TEX, &i.reprojection, ACCESS_READ)
        .bind_texture(H_HISTORY_OUTPUT_TEX, &i.history_output, ACCESS_WRITE)
        .bind_texture(H_OUTPUT_TEX, &i.output, ACCESS_WRITE)
        .uniform(H_BYPASS_CLAMP, i.bypass_clamp as i32)
        .uniform(H_SHARPEN, i.sharpen_amount);
    // Original shader samples history before its validity branch. Native
    // temporal creation passes TextureInitialData::None; no clear is invented.
    let pass = if !i.history.valid { pass.allow_undefined_texture_contents(H_HISTORY_TEX) } else { pass };
    let rec = pass.dispatch_2d(w, h);
    (*i.output, rec)
}

// ---------------------------------------------------------------- calculate_tgsr

/// `calculate_tgsr` 的输入（O 参数顺序：`param_4..param_10`；`param_8` = `history_output`）。
pub struct TgsrInputs<'a> {
    pub color: &'a RgTexture,
    pub depth: &'a RgTexture,
    pub output: &'a RgTexture,
    pub reprojection: &'a RgTexture,
    pub history_output: &'a RgTexture,
    pub history: &'a HistoryTexture,
    pub sharpen_amount: f32,
    pub bypass_clamp: u8,
    pub history_global: bool,
    /// `GraphicsDevice::min_subgroup_size()`（O：`[dev+0x40]` 标志、`[dev+0x44]` 值 → `Option<u32>`）。
    pub min_subgroup_size: Option<u32>,
}

/// W52 TGSR（O 0x140a3ba60）。返回 `output` 与按序声明的 3 个 pass。
pub fn calculate_tgsr(
    env: &mut PassEnv<'_>,
    i: &TgsrInputs<'_>,
) -> (RgTexture, Vec<RecordedComputePass>) {
    let (cw, ch) = extent_xy(&i.color.desc);
    let (ow, oh) = extent_xy(&i.output.desc);

    // aux 对：sampled + storage。
    let aux_desc = TextureDesc::new_2d([ow, oh], 0x10).sampled().storage();
    let aux = env.temporal_pair("tgsr_aux", aux_desc);

    let lum = env.create_texture(
        "reprojected history lum",
        TextureDesc::new_2d([ow, oh], 0x4c),
    );
    let chroma = env.create_texture(
        "reprojected history chroma",
        TextureDesc::new_2d([ow, oh], 0x53),
    );
    let blurred = env.create_texture("blurred luminance", TextureDesc::new_2d([cw, ch], 9));

    let mut passes = Vec::with_capacity(3);
    passes.push(
        env.new_pass("TGSR blur lum", &"/tgsr_blur_lum.cs.hlsl")
            .bind_texture(H_COLOR_TEX, &i.color, ACCESS_READ)
            .bind_texture(H_OUTPUT_TEX, &&blurred, ACCESS_WRITE)
            .dispatch_2d(cw, ch),
    );
    let reproject = env.new_pass("TGSR reproject", &"/tgsr_reproject.cs.hlsl")
            .uniform(
                H_HISTORY_VALID,
                history_valid_uniform(i.history.valid, i.history_global),
            )
            .bind_texture(H_HISTORY_TEX, &&i.history.tex, ACCESS_READ)
            .bind_texture(H_REPROJECTION_TEX, &i.reprojection, ACCESS_READ)
            .bind_texture(H_DEPTH_TEX, &i.depth, ACCESS_READ)
            .bind_texture(H_OUTPUT_LUM_TEX, &&lum, ACCESS_WRITE)
            .bind_texture(H_OUTPUT_CHROM_TEX, &&chroma, ACCESS_WRITE);
    let reproject = if !i.history.valid { reproject.allow_undefined_texture_contents(H_HISTORY_TEX) } else { reproject };
    passes.push(reproject.dispatch_2d(ow, oh));
    let main = TgsrMain::new(i.min_subgroup_size, i.sharpen_amount);
    let pass = env.new_pass("TGSR", &main)
            .uniform(
                H_HISTORY_VALID,
                history_valid_uniform(i.history.valid && aux.fresh, i.history_global),
            )
            .bind_texture(H_COLOR_TEX, &i.color, ACCESS_READ)
            .bind_texture(H_BLURRED_LUM_TEX, &&blurred, ACCESS_READ)
            .bind_texture(H_HISTORY_TEX, &&i.history.tex, ACCESS_READ)
            .bind_texture(H_REPROJ_HIST_LUM, &&lum, ACCESS_READ)
            .bind_texture(H_REPROJ_HIST_CHROMA, &&chroma, ACCESS_READ)
            .bind_texture(H_REPROJECTION_TEX, &i.reprojection, ACCESS_READ)
            .bind_texture(H_AUX_TEX, &&aux.previous, ACCESS_READ)
            .bind_texture(H_HISTORY_OUTPUT_TEX, &i.history_output, ACCESS_WRITE)
            .bind_texture(H_OUTPUT_TEX, &i.output, ACCESS_WRITE)
            .bind_texture(H_AUX_OUTPUT_TEX, &&aux.current, ACCESS_WRITE)
            .uniform(H_BYPASS_CLAMP, i.bypass_clamp as i32)
            .uniform(H_SHARPEN, i.sharpen_amount);
    // All four packaged TGSR main permutations gate actual aux sampling by
    // this uniform; unlike the preceding reproject history reads it is inactive.
    let pass = if history_valid_uniform(i.history.valid && aux.fresh, i.history_global) == 0 {
        pass.inactive_texture_contents(H_AUX_TEX)
    } else { pass };
    passes.push(pass.dispatch_2d(ow, oh));
    (*i.output, passes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::pass_builder::compute_pass_builder::DispatchMode;
    use crate::systems::render_loop::compute_passes::pass_kit::imm;
    use crate::systems::render_loop::compute_passes::pass_kit::test_env::*;

    #[test]
    fn hashes_are_exe_immediates() {
        assert_eq!(H_HISTORY_VALID, imm(-0x565ef6489f1126fe));
        assert_eq!(H_COLOR_TEX, imm(-0x266871abe31656a2));
        assert_eq!(H_HISTORY_TEX, 0x185a603c077dad19);
        assert_eq!(H_REPROJECTION_TEX, imm(-0x4758b3327ca5941d));
        assert_eq!(H_HISTORY_OUTPUT_TEX, imm(-0x558afea59529e6ed));
        assert_eq!(H_OUTPUT_TEX, imm(-0x77de65c519676d4e));
        assert_eq!(H_BYPASS_CLAMP, imm(-0x3b307e0d0b5aa06e));
        assert_eq!(H_SHARPEN, imm(-0x56fd2af498ade2b7));
        assert_eq!(H_OUTPUT_LUM_TEX, imm(-0x1edde334d35f2cf1));
        assert_eq!(H_OUTPUT_CHROM_TEX, imm(-0x5777883842b8a882));
        assert_eq!(H_BLURRED_LUM_TEX, imm(-0x3487064d6bdcffb8));
        assert_eq!(H_REPROJ_HIST_LUM, imm(-0x305844e28da88a1c));
        assert_eq!(H_REPROJ_HIST_CHROMA, imm(-0x27ed945d8a99cd38));
        assert_eq!(H_AUX_TEX, 0x7d070ced7e2b6d57);
        assert_eq!(H_AUX_OUTPUT_TEX, imm(-0x467f3ad6c479854b));
    }

    #[test]
    fn tgsr_main_permutations() {
        assert_eq!(
            TgsrMain::new(Some(32), 0.5),
            TgsrMain {
                wave32: 1,
                sharpen: 1
            }
        );
        assert_eq!(
            TgsrMain::new(Some(33), 0.0),
            TgsrMain {
                wave32: 0,
                sharpen: 0
            }
        );
        assert_eq!(
            TgsrMain::new(None, -1.0),
            TgsrMain {
                wave32: 0,
                sharpen: 0
            }
        );
        let d = TgsrMain {
            wave32: 1,
            sharpen: 0,
        }
        .pipeline_defines();
        assert_eq!(d[0], ("PERM_wave32".to_owned(), "1".to_owned()));
        assert_eq!(d[1], ("PERM_sharpen".to_owned(), "0".to_owned()));
        // 两个字段都进 hash。
        let a = TgsrMain {
            wave32: 0,
            sharpen: 0,
        }
        .source_hash();
        assert_ne!(
            a,
            TgsrMain {
                wave32: 1,
                sharpen: 0
            }
            .source_hash()
        );
        assert_ne!(
            a,
            TgsrMain {
                wave32: 0,
                sharpen: 1
            }
            .source_hash()
        );
    }

    #[test]
    fn history_valid_uniform_follows_the_global() {
        assert_eq!(history_valid_uniform(true, false), 1);
        assert_eq!(history_valid_uniform(true, true), 0);
        assert_eq!(history_valid_uniform(false, false), 0);
    }

    fn tex(id: u32, w: u32, h: u32) -> RgTexture {
        RgTexture {
            resource: id,
            desc: TextureDesc::new([w, h, 1], 0x25, 0),
        }
    }

    #[test]
    fn taa_pass_declaration_and_uniforms() {
        let mut e = env_with(
            &[
                H_COLOR_TEX,
                H_HISTORY_TEX,
                H_REPROJECTION_TEX,
                H_HISTORY_OUTPUT_TEX,
                H_OUTPUT_TEX,
            ],
            &[H_HISTORY_VALID, H_BYPASS_CLAMP, H_SHARPEN],
            |_, _| {},
        );
        let (c, o, r, ho) = (tex(1, 8, 8), tex(2, 8, 8), tex(3, 8, 8), tex(4, 16, 12));
        let hist = HistoryTexture {
            tex: tex(5, 8, 8),
            valid: true,
        };
        let (out, rec) = e.run(|penv| {
            calculate_taa(
                penv,
                &TaaInputs {
                    color: &c,
                    output: &o,
                    reprojection: &r,
                    history_output: &ho,
                    history: &hist,
                    sharpen_amount: 0.25,
                    bypass_clamp: 1,
                    history_global: false,
                },
            )
        });
        assert_eq!(out, o);
        assert_eq!(rec.name, "TAA");
        assert_eq!(e.world.backend.paths.borrow()[0], "/taa.cs.hlsl:cs_main");
        assert!(matches!(
            rec.dispatch,
            DispatchMode::Threads { x: 16, y: 12, z: 1 }
        ));
        assert_eq!(e.uniform_u32(&rec, H_HISTORY_VALID), 1);
        assert_eq!(e.uniform_u32(&rec, H_BYPASS_CLAMP), 1);
        assert_eq!(e.uniform_f32(&rec, H_SHARPEN), 0.25);
        // 访问顺序：color library(ctor) + color/history/reproj 读 + 两个写。
        let acc: Vec<_> = rec
            .builder
            .accesses
            .iter()
            .map(|a| (a.resource, a.access))
            .collect();
        assert_eq!(
            acc[1..],
            [(1, 0x19), (5, 0x19), (3, 0x19), (4, 0x32), (2, 0x32)]
        );
    }

    fn tgsr_env() -> Env {
        env_with(
            &[
                H_COLOR_TEX,
                H_HISTORY_TEX,
                H_REPROJECTION_TEX,
                H_HISTORY_OUTPUT_TEX,
                H_OUTPUT_TEX,
                H_DEPTH_TEX,
                H_OUTPUT_LUM_TEX,
                H_OUTPUT_CHROM_TEX,
                H_BLURRED_LUM_TEX,
                H_REPROJ_HIST_LUM,
                H_REPROJ_HIST_CHROMA,
                H_AUX_TEX,
                H_AUX_OUTPUT_TEX,
            ],
            &[H_HISTORY_VALID, H_BYPASS_CLAMP, H_SHARPEN],
            |_, _| {},
        )
    }

    fn tgsr_inputs<'a>(t: &'a [RgTexture; 5], hist: &'a HistoryTexture) -> TgsrInputs<'a> {
        TgsrInputs {
            color: &t[0],
            depth: &t[1],
            output: &t[2],
            reprojection: &t[3],
            history_output: &t[4],
            history: hist,
            sharpen_amount: 0.0,
            bypass_clamp: 0,
            history_global: false,
            min_subgroup_size: Some(32),
        }
    }

    #[test]
    fn tgsr_creates_resources_then_three_passes_in_order() {
        let mut e = tgsr_env();
        let t = [
            tex(1, 40, 20),
            tex(2, 40, 20),
            tex(3, 80, 40),
            tex(4, 80, 40),
            tex(5, 80, 40),
        ];
        let hist = HistoryTexture {
            tex: tex(6, 80, 40),
            valid: true,
        };
        let (out, passes) = e.run(|penv| calculate_tgsr(penv, &tgsr_inputs(&t, &hist)));
        assert_eq!(out, t[2]);
        // aux 对先于三张 RG 纹理创建（O：get_or_create_temporal_pair 在前）。
        assert_eq!(e.frame.temporal_pairs.len(), 1);
        assert_eq!(e.frame.temporal_pairs[0].0, "tgsr_aux");
        let created: Vec<_> = e.frame.textures.iter().map(|c| c.0.as_str()).collect();
        assert_eq!(
            created,
            [
                "reprojected history lum",
                "reprojected history chroma",
                "blurred luminance"
            ]
        );
        let aux = e.frame.temporal_pairs[0].1;
        assert_eq!(
            (aux.extent, aux.format, aux.sampled, aux.storage),
            ([80, 40, 1], 0x10, true, true)
        );
        assert_eq!(e.frame.textures[0].1.format, 0x4c);
        assert_eq!(e.frame.textures[1].1.format, 0x53);
        assert_eq!(
            (e.frame.textures[2].1.format, e.frame.textures[2].1.extent),
            (9, [40, 20, 1])
        );
        let names: Vec<_> = passes.iter().map(|p| p.name).collect();
        assert_eq!(names, ["TGSR blur lum", "TGSR reproject", "TGSR"]);
        assert!(matches!(
            passes[0].dispatch,
            DispatchMode::Threads { x: 40, y: 20, z: 1 }
        ));
        assert!(matches!(
            passes[1].dispatch,
            DispatchMode::Threads { x: 80, y: 40, z: 1 }
        ));
        assert!(matches!(
            passes[2].dispatch,
            DispatchMode::Threads { x: 80, y: 40, z: 1 }
        ));
        let uris = e.world.backend.paths.borrow().clone();
        assert_eq!(
            uris,
            [
                "/tgsr_blur_lum.cs.hlsl:cs_main",
                "/tgsr_reproject.cs.hlsl:cs_main",
                "/tgsr.cs.hlsl:cs_main"
            ]
        );
        // history_valid: reproject 只看 history；主 pass 还要 aux 历史有效（此处 aux 无效 → 0）。
        assert_eq!(e.uniform_u32(&passes[1], H_HISTORY_VALID), 1);
        assert_eq!(e.uniform_u32(&passes[2], H_HISTORY_VALID), 0);
        // 主 pass 的 10 个纹理访问 + ctor 的 color library。
        assert_eq!(passes[2].builder.accesses.len(), 11);
        // 写访问：history_output(5)、output(3)、aux 当前帧（temporal 对的 current = 第一个创建的 RG 资源 101）。
        let writes: Vec<_> = passes[2]
            .builder
            .accesses
            .iter()
            .filter(|a| a.access == 0x32)
            .map(|a| a.resource)
            .collect();
        assert_eq!(writes, [5, 3, 101]);
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value")]
    fn temporal_pair_error_panics_like_result_unwrap() {
        let mut e = tgsr_env();
        e.frame.fail_temporal = true;
        let t = [
            tex(1, 4, 4),
            tex(2, 4, 4),
            tex(3, 4, 4),
            tex(4, 4, 4),
            tex(5, 4, 4),
        ];
        let hist = HistoryTexture {
            tex: tex(6, 4, 4),
            valid: true,
        };
        e.run(|penv| calculate_tgsr(penv, &tgsr_inputs(&t, &hist)));
    }
}
