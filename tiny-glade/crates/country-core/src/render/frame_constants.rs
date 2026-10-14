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
//! 未恢复（U）：其余 0xf0 − 4 字节的头部字段（含 UBO 的 light/time 等）；`views` 起始偏移
//! 取 0xf0（I：+0x80 落在头部，视图数组只能在 [0xf0,0x1090)）。该结构由更高层系统
//! `prepare_frame_constants` 填充，其余字段待该系统恢复时补全。

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::field_reassign_with_default)]
use crate::render::view_constants::ViewConstants;

/// 视图数量（O：`[ViewConstants; 5]`）。
pub const VIEW_COUNT: usize = 5;

/// 视图槽位索引。`MAIN_VIEW = 0` 为 I（`prepare_frame_constants` 里唯一一次 `set_pixel_offset`
/// 对应的视图，其余 4 个视图不抖动）；`is_shadow_view`/`sun_cascade` 的 PDB 名称为 O，语义 I。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RenderViewIndex(pub u32);

impl RenderViewIndex {
    pub const MAIN_VIEW: RenderViewIndex = RenderViewIndex(0);
    pub const SUN_CASCADE_0: RenderViewIndex = RenderViewIndex(1);

    /// I：非主视图即阴影视图。
    pub fn is_shadow_view(self) -> bool {
        self.0 != Self::MAIN_VIEW.0
    }

    /// I：阴影视图的级联序号；主视图为 `None`（旧实现返回 -1，已改为 `Option`）。
    pub fn sun_cascade(self) -> Option<u32> {
        self.is_shadow_view().then(|| self.0 - 1)
    }
}

/// 逐帧常量块（0x1090 字节）。
#[repr(C, align(16))]
#[derive(Clone, Debug)]
pub struct FrameConstants {
    /// +0x00..+0x80，U。
    head0: [u32; 0x20],
    /// +0x80，O：`should_reset_temporal() == (value == 0)`。
    /// 取名 I：`frame_counter`——首帧（计数 0）重置时域累积。
    pub frame_counter: u32,
    /// +0x84..+0xf0，U。
    head1: [u32; 0x1b],
    /// +0xf0，I（偏移）；O（数量与元素类型）。
    pub views: [ViewConstants; VIEW_COUNT],
}

const _: () = assert!(core::mem::size_of::<FrameConstants>() == 0x1090);

impl Default for FrameConstants {
    /// `views` 为 O（`array::default<ViewConstants>`：单位矩阵、偏移 0）；头部为 U，暂置 0。
    fn default() -> Self {
        Self {
            head0: [0; 0x20],
            frame_counter: 0,
            head1: [0; 0x1b],
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

    /// `main_view`（O 名称）。
    pub fn main_view(&self) -> &ViewConstants {
        self.view(RenderViewIndex::MAIN_VIEW)
    }

    /// `sun_view`（O 名称；I：返回指定级联）。
    pub fn sun_view(&self, cascade: u32) -> &ViewConstants {
        self.view(RenderViewIndex(cascade + 1))
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
        assert_eq!(RenderViewIndex(3).sun_cascade(), Some(2));
        let mut f = FrameConstants::default();
        f.views[2].pixel_offset.x = 1.5;
        assert_eq!(f.sun_view(1).pixel_offset.x, 1.5);
    }
}
