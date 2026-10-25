//! `FrameConstants` — per-frame GPU constant block (layer A1).
//!
//! 证据（O，均为指令级核对，见 `evidence/country-core-audit/L0_NOTES.md`）：
//!  * `size_of::<FrameConstants>() == 0x1090`：`App::insert_resource<FrameConstants>` 0x14012d450 与
//!    `init_state<FrameConstants>` 0x1401c42b0 的 `ComponentInfo`/存储 layout 立即数。
//!  * 其中 5 个 `ViewConstants`（5×0x320 = 0xfa0）：`array::default<ViewConstants>`（0x1400ef490）
//!    把 5 个单位矩阵模板 memcpy 到 `rcx+{0,0x320,0x640,0x960,0xc80}`；`prepare_frame_constants`
//!    （0x140af40b0）调用 4 次 `ViewportConstantBuilder::build` + 1 次 `set_pixel_offset`。
//!  * `should_reset_temporal`（0x140976880）= `cmp dword [self+0x80], 0 ; sete al`，即该 u32 **等于 0**
//!    时为真（旧实现读 bool 字段，语义相反）。
//!
//! Fresh B verification (2026-10-18): `prepare_frame_constants` 0x140af40b0
//! writes all header words and copies 4000 bytes of views at +0xf0. Field
//! names/types also match original SPIR-V's FrameConstants reflection (27
//! members, view array stride 800). Native run_game_thread 0x1405a4f30..4ff0
//! zeroes the startup header, then 0x1405a4f87 calls the five-view constructor.
//! The per-frame producer and its fourteen input resources remain a separate
//! recovery task; knowing the layout does not make a zero startup block a
//! populated gameplay frame.

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::field_reassign_with_default)]
use crate::render::view_constants::ViewConstants;

/// 视图数量（O：`[ViewConstants; 5]`）。
pub const VIEW_COUNT: usize = 5;

/// View slots: jittered main, unjittered main, then three sun cascades.
/// B draw_bricks<0> @0x1408a53f2 compares the view index against 2;
/// shadow_pass @0x1409cc270 supplies cascade + 2. The original memcpy chain
/// in prepare_frame_constants confirms both main views precede the cascades.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RenderViewIndex(pub u32);

impl RenderViewIndex {
    pub const MAIN_VIEW: RenderViewIndex = RenderViewIndex(0);
    pub const SUN_CASCADE_0: RenderViewIndex = RenderViewIndex(2);

    /// Original `is_shadow_view`: index >= 2.
    pub fn is_shadow_view(self) -> bool {
        self.0 >= 2
    }

    /// Rust projection of the confirmed cascade slots; this Option-returning
    /// helper is not an original PDB interface.
    pub fn sun_cascade(self) -> Option<u32> {
        self.is_shadow_view().then(|| self.0 - 2)
    }
}

/// 逐帧常量块（0x1090 字节）。
#[repr(C, align(16))]
#[derive(Clone, Debug, bevy_ecs::resource::Resource)]
pub struct FrameConstants {
    pub sun_color: [f32; 4],
    pub sky_low_color: [f32; 4],
    pub sky_high_color: [f32; 4],
    pub fog_tint_thickness: [f32; 4],
    pub fake_light_color: [f32; 4],
    pub fake_light_dir: [f32; 4],
    pub fake_light2_color: [f32; 4],
    pub fake_light2_dir: [f32; 4],
    /// +0x80，O：`should_reset_temporal() == (value == 0)`。
    /// 取名 I：`frame_counter`——首帧（计数 0）重置时域累积。
    pub frame_counter: u32,
    pub delta_time_seconds: f32,
    pub ev_mult: f32,
    pub glade_theme_id: u32,
    pub inner_rendering_resolution: [u32; 2],
    pub game_time_sec: u32,
    pub game_time_frac: f32,
    pub editing_highlight_color: [f32; 3],
    pub time_of_day: f32,
    pub dir_to_sun: [f32; 3],
    pub camera_idx: u32,
    pub dir_to_moon: [f32; 3],
    pub render_overrides: u32,
    pub global_mip_bias: f32,
    pub highlighted_transient_stair_edge_id: u32,
    pub pad1: u32,
    pub pad2: u32,
    pub debug_tweakers: [f32; 4],
    /// +0xf0, confirmed by the native memcpy destination and SPIR-V reflection.
    pub views: [ViewConstants; VIEW_COUNT],
}

const _: () = assert!(core::mem::size_of::<FrameConstants>() == 0x1090);

impl Default for FrameConstants {
    /// Original startup header is zero; all five views have identity matrices
    /// and zero offsets (`array::default<ViewConstants>` 0x1400ef490).
    fn default() -> Self {
        Self {
            sun_color: [0.0; 4], sky_low_color: [0.0; 4], sky_high_color: [0.0; 4],
            fog_tint_thickness: [0.0; 4], fake_light_color: [0.0; 4], fake_light_dir: [0.0; 4],
            fake_light2_color: [0.0; 4], fake_light2_dir: [0.0; 4],
            frame_counter: 0,
            delta_time_seconds: 0.0, ev_mult: 0.0, glade_theme_id: 0,
            inner_rendering_resolution: [0; 2], game_time_sec: 0, game_time_frac: 0.0,
            editing_highlight_color: [0.0; 3], time_of_day: 0.0,
            dir_to_sun: [0.0; 3], camera_idx: 0, dir_to_moon: [0.0; 3], render_overrides: 0,
            global_mip_bias: 0.0, highlighted_transient_stair_edge_id: 0,
            pad1: 0, pad2: 0, debug_tweakers: [0.0; 4],
            views: [ViewConstants::default(); VIEW_COUNT],
        }
    }
}

impl FrameConstants {
    /// `should_reset_temporal`（O: 0x140976880）：`frame_counter == 0`。
    pub fn should_reset_temporal(&self) -> bool {
        self.frame_counter == 0
    }

    /// 取视图（I：`RenderViewIndex` 直接索引，越界 panic 同 slice）。
    pub fn view(&self, index: RenderViewIndex) -> &ViewConstants {
        &self.views[index.0 as usize]
    }

    /// Rust accessor for the original jittered-main slot.
    pub fn main_view(&self) -> &ViewConstants {
        self.view(RenderViewIndex::MAIN_VIEW)
    }

    /// Rust accessor: the original shadow consumer supplies cascade + 2.
    pub fn sun_view(&self, cascade: u32) -> &ViewConstants {
        self.view(RenderViewIndex(cascade + 2))
    }

    /// `TREE_SEE_THROUGH`（O：`impl$15::fmt` 的字符串；值 U）。
    pub const TREE_SEE_THROUGH: &'static str = "TREE_SEE_THROUGH";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_and_default() {
        assert_eq!(core::mem::size_of::<FrameConstants>(), 0x1090);
        let f = FrameConstants::default();
        assert!(f.should_reset_temporal());
        assert_eq!(f.views[4], ViewConstants::default());
    }

    #[test]
    fn reset_flag_is_zero_test() {
        let mut f = FrameConstants::default();
        f.frame_counter = 7;
        assert!(!f.should_reset_temporal());
        f.frame_counter = 0;
        assert!(f.should_reset_temporal());
    }

    #[test]
    fn view_indexing() {
        assert_eq!(RenderViewIndex::MAIN_VIEW.sun_cascade(), None);
        assert_eq!(RenderViewIndex(1).sun_cascade(), None);
        assert!(!RenderViewIndex(1).is_shadow_view());
        assert!(RenderViewIndex(2).is_shadow_view());
        assert_eq!(RenderViewIndex(3).sun_cascade(), Some(1));
        let mut f = FrameConstants::default();
        f.views[3].pixel_offset.x = 1.5;
        assert_eq!(f.sun_view(1).pixel_offset.x, 1.5);
    }
}
