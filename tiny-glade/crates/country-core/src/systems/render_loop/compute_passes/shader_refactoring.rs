//! `shader_refactoring.rs` — 着色器重构对拍（开发用；W54，`render_shader_refactoring` 0x14098f9f0，4231 B）。
//!
//! 入参（O，`rdx`/`r8`）：`(recorder, world, frame: &mut Handle<Texture>)`；`frame` 是被对拍的画面（读 `desc` 的 extent 与格式）。
//! world 资源 `ResMut<ShaderRefactoringState>`（TypeId `0xc95ed87f413070ab / 0xe67d7e3df983f8ce`，哈希表内联查找，缺失 → `Option::unwrap`），
//! 布局 `{u32 disc @0, u32 payload @4}`；`world+0x5b0` = 当前 change tick（写回资源条目 `+0x5c`，即 `ResMut` 的 changed tick）。
//!
//! **第一层 `match disc`**（跳转表 0x142ab9ff0，O）：
//! `0` → 直接返回（不碰 tick）；`5` → **`Result::unwrap()` 失败的 panic**（0x140990721 跳进 ref-temporal 创建结果的 `unwrap_failed`
//! thunk 中部：`mov edx, 0x2b; call unwrap_failed`，0x2b = 43 = 文案 ``called `Result::unwrap()` on an `Err` value`` 的长度；
//! 原版文案后还跟 `: {err:?}`，该错误的 Debug 内容 U）。**纠错**：此前写成 `unreachable!()`（`render-gap/DIGEST.md §3.1`）；
//! `1` → `kind = 2`；`2` → `al = 0`，`3|4` → `al = 1`，后三者 `kind = ((payload & 3) == 3) ? al : 2`；
//! `6..=9`：编译器把这几个跳转表项合并进共享收尾代码的**退化分支**（帧槽未初始化，写垃圾地址，UB）。但第二张表的更新规则
//! 只产出 disc ∈ {0, 2, 3, 4}（1→{2,0}；2→{2,payload+1}|{3,0}；3/4→{3|4,payload+1}），所以 disc 5..=9 实际**不可达**；
//! 按 no-op 处理是安全且正确的近似。
//!
//! **公共前置**（`disc ∈ {1,2,3,4}`）：`ext = extent_2d(frame.desc)`，`fmt = frame.desc 格式字`；
//! `get_or_create_temporal("shader refactoring feedback")`（名长 0x1b）：`new_2d(ext, fmt).storage().sampled()`（`Err` → `Result::unwrap` panic）。
//!
//! **`kind == 2`**：不声明 pass。**`kind ∈ {0,1}`**：再 `get_or_create_temporal("shader refactoring ref")`（名长 0x16）：
//! `new_2d(ext, 0x7a).storage().sampled()`（`Err` → panic），然后：
//! * `kind == 1`：`create_raw_resource("shader refactoring capture")`（名长 0x1a）`new_2d(ext, 0x7a)`；
//!   **"Shader refactoring capture"** `/shader_refactoring.cs.hlsl:capture`（名长 0x1a、URI 长 0x23）：`rendered_tex ← frame`（读 0x19）、
//!   `reference_tex ← capture 资源`（写 0x32）、`dispatch_2d(ext)`；
//!   **"Shader refactoring compare"** `/shader_refactoring.cs.hlsl:compare`：`rendered_tex ← capture 资源`（读）、`reference_tex ← ref 历史纹理`（写）、
//!   `feedback_tex ← feedback 纹理`（写）、`dispatch_2d(ext)`。
//! * `kind == 0`：只有 capture pass：`rendered_tex ← frame`（读）、`reference_tex ← ref 纹理`（写）、`dispatch_2d(ext)`。
//!
//! **状态更新**（`ResMut` 写回；第二张跳转表 0x142aba004，`disc ∈ {1,2,3,4}`；`tick` 写入条目）：
//! `1` → `{2, 0}`；`2` → `payload < 4 ? {2, payload+1} : {3, 0}`；`3|4` → `{disc, payload+1}`，并且
//! `frame.desc[+0x20 处的 u32] = feedback 纹理句柄的同一偏移`（U：该字段语义未知，返回标志 `desc_tail_copied`）。
//!
//! 两个 temporal 在原版是**单个** `Temporal<Texture>`（`get_or_create_temporal`，非 `_pair`）；这里用 `RgResources::get_or_create_temporal_texture`（单张）。

use crate::render::pass_builder::compute_pass_builder::RecordedComputePass;
use crate::render::pass_builder::pass_resource_access::{RgTexture, WriteTextureUnchecked};
use crate::render::pass_builder::pass_resources::TypeId128;
use crate::rhapsody_types::TextureDesc;
use crate::systems::render_loop::compute_passes::pass_kit::{
    PassEnv, binding_hash, dispatch_extent, extent_of,
};

pub const FEEDBACK_NAME: &str = "shader refactoring feedback";
pub const REF_NAME: &str = "shader refactoring ref";
pub const CAPTURE_NAME: &str = "shader refactoring capture";
pub const CAPTURE: (&str, &str) = (
    "Shader refactoring capture",
    "/shader_refactoring.cs.hlsl:capture",
);
pub const COMPARE: (&str, &str) = (
    "Shader refactoring compare",
    "/shader_refactoring.cs.hlsl:compare",
);
pub const REF_FORMAT: u32 = 0x7a;
pub const RENDERED_TEX: u64 = binding_hash("rendered_tex");
pub const REFERENCE_TEX: u64 = binding_hash("reference_tex");
pub const FEEDBACK_TEX: u64 = binding_hash("feedback_tex");
pub const UNREACHABLE_PANIC: &str = "internal error: entered unreachable code";
/// disc 5 的 panic 文案前缀（O：`unwrap_failed` 的 43 B 消息；其后原版还有 `: {err:?}`，内容 U）。
pub const DISC5_UNWRAP_PANIC: &str = "called `Result::unwrap()` on an `Err` value";
/// `ShaderRefactoringState` 的 TypeId（O）。
pub const STATE_TYPE_ID: TypeId128 = TypeId128 {
    lo: 0xc95e_d87f_4130_70ab,
    hi: 0xe67d_7e3d_f983_f8ce,
};

/// `ShaderRefactoringState`（O：`{disc, payload}`，各 u32）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RefactoringState {
    pub disc: u32,
    pub payload: u32,
}

/// `ResMut<ShaderRefactoringState>`：值 + changed tick。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StateMut {
    pub state: RefactoringState,
    pub changed_tick: u32,
}

/// 被对拍的画面句柄 + 其 `desc` 的格式字（O：`[r15+0xc]`）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrameHandle {
    pub texture: RgTexture,
    pub format: u32,
}

pub struct RefactoringRun {
    pub passes: Vec<RecordedComputePass>,
    /// 状态 3/4：把 feedback 纹理 desc 尾字拷进 `frame`（U：语义未知）。
    pub desc_tail_copied: bool,
}

/// 第一层 `match`：返回 `Some(kind)`（`kind ∈ {0,1,2}`），`None` = 直接返回（disc 0 / 6..=9）。`5` → panic。
/// `get_or_create_temporal(name, TextureDesc)` + `Result::unwrap`（O：单个 `Temporal<Texture>`，非 `_pair`）。
fn temporal_texture(env: &mut PassEnv<'_>, name: &str, desc: TextureDesc) -> RgTexture {
    env.frame
        .get_or_create_temporal_texture(name, &desc)
        .unwrap_or_else(|e| panic!("called `Result::unwrap()` on an `Err` value: {e:?}"))
        .texture
}

pub fn classify(state: RefactoringState) -> Option<u32> {
    let phase = |al: u32| if state.payload & 3 == 3 { al } else { 2 };
    match state.disc {
        0 => None,
        1 => Some(2),
        2 => Some(phase(0)),
        3 | 4 => Some(phase(1)),
        5 => panic!("{DISC5_UNWRAP_PANIC}"),
        _ => None, // 6..=9：退化分支，实际不可达（第二表只产出 0/2/3/4）
    }
}

/// 第二张跳转表：通过后的状态更新（O）。
pub fn next_state(state: RefactoringState) -> RefactoringState {
    match state.disc {
        1 => RefactoringState {
            disc: 2,
            payload: 0,
        },
        2 if state.payload < 4 => RefactoringState {
            disc: 2,
            payload: state.payload + 1,
        },
        2 => RefactoringState {
            disc: 3,
            payload: 0,
        },
        3 | 4 => RefactoringState {
            disc: state.disc,
            payload: state.payload + 1,
        },
        _ => panic!("{UNREACHABLE_PANIC}"),
    }
}

/// W54：按状态机声明对拍 pass，并写回状态。
pub fn render_shader_refactoring(
    env: &mut PassEnv<'_>,
    state: &mut StateMut,
    world_tick: u32,
    frame: &FrameHandle,
) -> RefactoringRun {
    let Some(kind) = classify(state.state) else {
        return RefactoringRun {
            passes: Vec::new(),
            desc_tail_copied: false,
        };
    };
    let ext = extent_of(&frame.texture);
    let ext = [ext.width, ext.height];
    let spec = |format| TextureDesc::new_2d(ext, format).storage().sampled();
    let feedback = temporal_texture(env, FEEDBACK_NAME, spec(frame.format));

    let mut passes = Vec::new();
    if kind != 2 {
        let reference = temporal_texture(env, REF_NAME, spec(REF_FORMAT));
        if kind == 1 {
            let captured = env.create_texture(CAPTURE_NAME, TextureDesc::new_2d(ext, REF_FORMAT));
            let pass = env
                .pass(CAPTURE.0, CAPTURE.1)
                .bind_texture(RENDERED_TEX, &&frame.texture, 0x19)
                .bind_texture(REFERENCE_TEX, &WriteTextureUnchecked(captured), 0x32);
            passes.push(dispatch_extent(pass, &frame.texture));
            let pass = env
                .pass(COMPARE.0, COMPARE.1)
                .bind_texture(RENDERED_TEX, &&captured, 0x19)
                .bind_texture(REFERENCE_TEX, &WriteTextureUnchecked(reference), 0x32)
                .bind_texture(FEEDBACK_TEX, &WriteTextureUnchecked(feedback), 0x32);
            passes.push(dispatch_extent(pass, &frame.texture));
        } else {
            let pass = env
                .pass(CAPTURE.0, CAPTURE.1)
                .bind_texture(RENDERED_TEX, &&frame.texture, 0x19)
                .bind_texture(REFERENCE_TEX, &WriteTextureUnchecked(reference), 0x32);
            passes.push(dispatch_extent(pass, &frame.texture));
        }
    }

    state.changed_tick = world_tick;
    let desc_tail_copied = matches!(state.state.disc, 3 | 4);
    state.state = next_state(state.state);
    RefactoringRun {
        passes,
        desc_tail_copied,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::render_loop::compute_passes::pass_kit::test_env::{
        Env, accesses, env, rgtex, threads,
    };

    fn st(disc: u32, payload: u32) -> StateMut {
        StateMut {
            state: RefactoringState { disc, payload },
            changed_tick: 0,
        }
    }

    fn run(mut s: StateMut) -> (Env, StateMut, RefactoringRun) {
        let mut e = env(&[RENDERED_TEX, REFERENCE_TEX, FEEDBACK_TEX], &[]);
        let f = FrameHandle {
            texture: rgtex(5, 320, 200),
            format: 0x30,
        };
        let out = e.run(|env| render_shader_refactoring(env, &mut s, 777, &f));
        (e, s, out)
    }

    #[test]
    fn names_match_exe_immediates() {
        assert_eq!(RENDERED_TEX, 0x0ce3_1214_5f3c_6962);
        assert_eq!(REFERENCE_TEX, 0xef80_5d66_075b_336c);
        assert_eq!(FEEDBACK_TEX, 0x2f39_695d_e7a8_5dd4);
    }

    #[test]
    fn classify_matches_the_first_jump_table() {
        let c = |d, p| {
            classify(RefactoringState {
                disc: d,
                payload: p,
            })
        };
        assert_eq!(c(0, 3), None);
        assert_eq!(c(1, 3), Some(2), "state 1 never runs passes");
        // 状态 2：payload & 3 == 3 → kind 0，否则 2
        assert_eq!(
            (c(2, 3), c(2, 7), c(2, 0), c(2, 2)),
            (Some(0), Some(0), Some(2), Some(2))
        );
        // 状态 3/4：payload & 3 == 3 → kind 1
        assert_eq!(
            (c(3, 3), c(4, 3), c(3, 4), c(4, 5)),
            (Some(1), Some(1), Some(2), Some(2))
        );
        assert_eq!(
            c(6, 3),
            None,
            "6..=9: degenerate arms, unreachable in practice"
        );
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value")]
    fn state_five_panics_with_result_unwrap_failed() {
        classify(RefactoringState {
            disc: 5,
            payload: 0,
        });
    }

    #[test]
    fn next_state_matches_the_second_jump_table() {
        let n = |d, p| {
            next_state(RefactoringState {
                disc: d,
                payload: p,
            })
        };
        assert_eq!(
            n(1, 99),
            RefactoringState {
                disc: 2,
                payload: 0
            }
        );
        assert_eq!(
            n(2, 0),
            RefactoringState {
                disc: 2,
                payload: 1
            }
        );
        assert_eq!(
            n(2, 3),
            RefactoringState {
                disc: 2,
                payload: 4
            }
        );
        assert_eq!(
            n(2, 4),
            RefactoringState {
                disc: 3,
                payload: 0
            }
        );
        assert_eq!(
            n(2, 100),
            RefactoringState {
                disc: 3,
                payload: 0
            }
        );
        assert_eq!(
            n(3, 7),
            RefactoringState {
                disc: 3,
                payload: 8
            }
        );
        assert_eq!(
            n(4, 0),
            RefactoringState {
                disc: 4,
                payload: 1
            }
        );
    }

    #[test]
    fn disabled_state_returns_untouched() {
        let (e, s, out) = run(st(0, 3));
        assert!(out.passes.is_empty() && e.frame.temporal_textures.is_empty());
        assert_eq!(
            (s.state, s.changed_tick),
            (
                RefactoringState {
                    disc: 0,
                    payload: 3
                },
                0
            )
        );
    }

    #[test]
    fn state_one_only_creates_the_feedback_texture_and_advances() {
        let (e, s, out) = run(st(1, 0));
        assert!(out.passes.is_empty());
        assert_eq!(e.frame.temporal_textures.len(), 1);
        let (n, spec) = &e.frame.temporal_textures[0];
        assert_eq!(
            (
                n.as_str(),
                spec.extent_2d(),
                spec.format,
                spec.storage,
                spec.sampled
            ),
            (FEEDBACK_NAME, [320, 200], 0x30, true, true)
        );
        assert_eq!(
            s.state,
            RefactoringState {
                disc: 2,
                payload: 0
            }
        );
        assert_eq!(s.changed_tick, 777);
        assert!(!out.desc_tail_copied);
    }

    #[test]
    fn state_two_every_fourth_frame_captures_the_reference() {
        let (e, s, out) = run(st(2, 3));
        assert_eq!(out.passes.len(), 1);
        let p = &out.passes[0];
        assert_eq!(p.name, "Shader refactoring capture");
        assert_eq!(threads(p), (320, 200, 1));
        assert_eq!(
            *e.world.backend.paths.borrow(),
            vec!["/shader_refactoring.cs.hlsl:capture".to_string()]
        );
        let names: Vec<&str> = e
            .frame
            .temporal_textures
            .iter()
            .map(|t| t.0.as_str())
            .collect();
        assert_eq!(names, [FEEDBACK_NAME, REF_NAME]);
        assert_eq!(e.frame.temporal_textures[1].1.format, 0x7a);
        let a = accesses(p);
        assert_eq!((a[0].0, a[0].1, a[1].1), (5, 0x19, 0x32));
        assert_eq!(
            s.state,
            RefactoringState {
                disc: 2,
                payload: 4
            }
        );
    }

    #[test]
    fn states_three_and_four_capture_then_compare() {
        let (e, s, out) = run(st(3, 3));
        let n: Vec<&str> = out.passes.iter().map(|p| p.name).collect();
        assert_eq!(
            n,
            ["Shader refactoring capture", "Shader refactoring compare"]
        );
        assert_eq!(e.frame.textures[0].0.as_str(), CAPTURE_NAME);
        let cap = e.frame.textures[0].2.resource;
        assert_eq!(accesses(&out.passes[0])[1], (cap, 0x32, 0));
        let cmp = accesses(&out.passes[1]);
        assert_eq!(cmp[0], (cap, 0x19, 1));
        assert_eq!((cmp[1].1, cmp[2].1), (0x32, 0x32));
        assert_ne!(
            cmp[1].0, cmp[2].0,
            "reference and feedback are different textures"
        );
        assert!(out.desc_tail_copied);
        assert_eq!(
            s.state,
            RefactoringState {
                disc: 3,
                payload: 4
            }
        );
        let (_, s4, out4) = run(st(4, 3));
        assert!(out4.desc_tail_copied);
        assert_eq!(
            s4.state,
            RefactoringState {
                disc: 4,
                payload: 4
            }
        );
    }

    #[test]
    fn off_beat_frames_create_feedback_but_no_passes() {
        for (d, p) in [(2u32, 0u32), (2, 1), (3, 0), (4, 2)] {
            let (e, s, out) = run(st(d, p));
            assert!(out.passes.is_empty(), "disc {d} payload {p}");
            assert_eq!(e.frame.temporal_textures.len(), 1);
            assert_eq!(s.changed_tick, 777);
        }
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value")]
    fn failed_temporal_creation_unwraps() {
        let mut e = env(&[], &[]);
        e.frame.fail_temporal = true;
        let mut s = st(1, 0);
        let f = FrameHandle {
            texture: rgtex(5, 8, 8),
            format: 1,
        };
        e.run(|env| render_shader_refactoring(env, &mut s, 1, &f));
    }
}
