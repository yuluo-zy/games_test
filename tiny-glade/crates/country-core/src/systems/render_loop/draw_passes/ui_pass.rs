//! `ui_pass.rs` — UI pass（0x140837c40）与 loading-screen pass（0x140837ff0），各 929 B，机器码仅列表偏移不同。
//!
//! O（两者共有）：`extent_2d(target)` → `RasterPass::new_impl("UI"(2), [tag0 = target], 无深度, 视口 = target)` →
//! `CommonDrawParams { time, flag = true }` → 遍历一个 `DrawLists` 列表（`ui_pass` 读 `+0x1d0/+0x1d8` = 第 19 个，
//! `loading_screen_pass` 读 `+0x1b8/+0x1c0` = 第 18 个）→ `DrawPacket::new` / `DrawBuilder::new(variant 0)` /
//! `bind_common` / 回调 `item+0x70`，**无状态位改写** → `RasterPass::render`。
//! 两个 pass 的名字都是 "UI"（同一字符串地址 0x142a92be0，长度 2）。

use crate::render::pass_builder::framebuffer_binding::AttachmentTexture;
use crate::systems::render_loop::draw_passes::d_main_support::{
    ColorSpec, DepthSpec, DrawPassCtx, ItemMode, ListPass, run_list_pass,
};
use crate::systems::render_loop::draw_passes::draw_lists::{DrawLists, list};

/// pass 名（O：`mov r8d, 2`）。
pub const UI_PASS_NAME: &str = "UI";

fn run(ctx: &mut DrawPassCtx<'_>, target: &AttachmentTexture, lists: &DrawLists, idx: usize) {
    let colors = [ColorSpec::tag0(*target)];
    let spec = ListPass {
        name: UI_PASS_NAME,
        colors: &colors,
        depth: DepthSpec::None,
        viewport: *target,
        flag: true,
        mode: ItemMode::Plain,
        variant: 0,
        extra: None,
    };
    run_list_pass(ctx, &spec, lists.list(idx));
}

/// `ui_pass`（0x140837c40）。
pub fn ui_pass(ctx: &mut DrawPassCtx<'_>, target: &AttachmentTexture, lists: &DrawLists) {
    run(ctx, target, lists, list::UI);
}

/// `loading_screen_pass`（0x140837ff0）。
pub fn loading_screen_pass(
    ctx: &mut DrawPassCtx<'_>,
    target: &AttachmentTexture,
    lists: &DrawLists,
) {
    run(ctx, target, lists, list::LOADING_SCREEN);
}
