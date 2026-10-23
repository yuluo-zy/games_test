//! `shadow_pass.rs` — W23 阴影级联光栅（0x1409cc270，2210 B）。
//!
//! O：puffin scope；取 `AssetTextureLibrary`（TypeId -0x597a9e4c…）→ 对 **cascade i = 0..3**（`cmp r14, 3`）各建一个 pass：
//! `get_rg_texture(预设 0x11, …)` 取阴影图集 → `extent_2d` → `RasterPass::new_impl("Shadows"(7), 0 个颜色,
//! 深度 = { tag0 可写, layer = i, 纹理 = 图集, clear = 1.0 }, 视口 = 图集 extent)`（深度字 `0x3f80000000000001` = `+0x10 = 1`、`+0x14 = 1.0f`）。
//! 每个 pass 遍历 **`DrawLists` 第 0 个列表**（`[r8+8]/[r8+0x10]`，即 `&DrawLists` 起始）；
//! `Res<Time>` 在循环内每次重新读取；`CommonDrawParams` 的字节为 `cascade + 2`（`add r14b, 2`），
//! `DrawBuilder::new(variant = 1)`（`mov r9b, 1`）；`bind_common` 之后 `state.byte0 &= 0xcf`；
//! 然后回调 `item+0x70`，`RasterPass::render`。

use crate::systems::render_loop::draw_passes::d_main_support::{
    DepthSpec, DrawPassCtx, ItemMode, ListPass, run_independent_list_passes,
};
use crate::systems::render_loop::draw_passes::draw_lists::DrawLists;

/// pass 名（O：`mov r8d, 7`）。
pub const SHADOW_PASS_NAME: &str = "Shadows";
/// 阴影图集的 RG 纹理预设（O：`mov r8b, 0x11`）。
pub const SHADOW_ATLAS_PRESET: u8 = 0x11;
/// 级联数（O：`cmp r14, 3`）。
pub const SHADOW_CASCADES: u32 = 3;
/// 阴影列表下标（O：`DrawLists +0` 起始）。
pub const SHADOW_LIST: usize = 0;

/// `shadow_pass(ctx, world, lists)`。
pub fn shadow_pass(ctx: &mut DrawPassCtx<'_>, lists: &DrawLists) {
    let atlas = ctx.host().rg_texture(SHADOW_ATLAS_PRESET);
    let specs: Vec<_> = (0..SHADOW_CASCADES).map(|cascade| ListPass {
            name: SHADOW_PASS_NAME,
            colors: &[],
            depth: DepthSpec::Write {
                tex: atlas,
                layer: cascade,
                clear: Some(1.0),
            },
            viewport: atlas,
            flag: true,
            mode: ItemMode::Shadow {
                flag: (cascade + 2) as u8,
            },
            variant: 1,
            extra: None,
        }).collect();
    run_independent_list_passes(ctx, &specs, lists.list(SHADOW_LIST));
}
