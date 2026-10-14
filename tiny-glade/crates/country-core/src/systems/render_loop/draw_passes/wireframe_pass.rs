//! `wireframe_pass.rs` — 调试线框 pass（0x1409ccb20，929 B）。
//!
//! O 顺序：`extent_2d(target)` → `RasterPass::new_impl("Wireframe"(9), [ColorAttachment tag0 = target], 无深度(tag2), 视口 = target extent)`
//! → `Res<Time>`（TypeId 0x4bc362195b80f729）→ `CommonDrawParams { time, flag = true }`（`mov byte [rbp+0x89c], 1`）
//! → 遍历 `DrawLists +0x1e8/+0x1f0`（第 20 个列表，元素 0x80 B）：
//! `DrawPacket::new(head = [1, item+0x50], shader = item+0x58)` → `DrawBuilder::new(variant 0)` → `bind_common(params, item)`
//! → 状态改写 `byte3 |= 0x80`（`set_polygon_mode`，I）、`byte0 &= 0x7f`（`set_depth_test_enable(false)`，I，与 mei_pass 同一位）、
//! 混合 1.0 → 回调 `item+0x70`（builder, item, world）→ `RasterPass::render`。

use crate::render::pass_builder::framebuffer_binding::AttachmentTexture;
use crate::systems::render_loop::draw_passes::d_main_support::{
    ColorSpec, DepthSpec, DrawPassCtx, ItemMode, ListPass, run_list_pass,
};
use crate::systems::render_loop::draw_passes::draw_lists::{DrawLists, list};

/// pass 名（O：`mov r8d, 9`，字符串 "Wireframe"）。
pub const WIREFRAME_PASS_NAME: &str = "Wireframe";

/// `wireframe_pass(ctx, world, target, lists)`。
pub fn wireframe_pass(ctx: &mut DrawPassCtx<'_>, target: &AttachmentTexture, lists: &DrawLists) {
    let colors = [ColorSpec::tag0(*target)];
    let spec = ListPass {
        name: WIREFRAME_PASS_NAME,
        colors: &colors,
        depth: DepthSpec::None,
        viewport: *target,
        flag: true,
        mode: ItemMode::Wireframe,
        variant: 0,
        extra: None,
    };
    run_list_pass(ctx, &spec, lists.list(list::WIREFRAME));
}
