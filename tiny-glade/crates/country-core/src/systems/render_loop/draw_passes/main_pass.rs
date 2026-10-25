//! `main_pass.rs` — W25 `main_pass`（0x140833bc0，7794 B）、W30 `main_pass_oc2`（0x140835a40，6498 B）、
//! W34 `late_forward_and_deferred_pass`（0x1408373b0，2177 B）。
//!
//! 三个函数都是若干个同形的“子 pass”：`RasterPass::new_impl(名字, 颜色…, 深度, 视口 = 深度纹理 extent)` →
//! 遍历 `DrawLists` 的一个列表（元素 0x80 B）→ `render`。共用部分见 [`d_main_support::run_list_pass`]；
//! 这里只落实各子 pass 的 **名字 / 颜色 tag / 深度 tag / 列表下标 / 标志 / 状态改写**，全部来自机器码。
//!
//! 参数形态（O，`main_pass` 序言保存的寄存器）：`rcx` = 宿主 ctx、`rdx` = `&World`、`r8` = 纹理组结构（连续的
//! `Texture`，每个 0x24 B：`+0` G-buffer 颜色、`+0x24` 深度、`+0x48` 速度）、`r9` = 另一张纹理（emissive / forward 颜色），
//! 第 5 个（栈）参数 = `&DrawLists`。
//!
//! `with_stencil_write`（main_pass.rs:22）：`set_stencil_test_enable` + `set_stencil_pass_op`，机器码
//! `byte1 = (byte1 & 0x8e) | 0x21`；元素的 `stencil_ref`（`+0x64` 的 `Option<u8>`）在 `DrawBuilder::new` 之后
//! 经 `Option::unwrap` 写入 `packet.+0xb1`（None → panic）。

use crate::render::pass_builder::framebuffer_binding::AttachmentTexture;
use crate::systems::render_loop::draw_passes::d_main_support::{
    ColorSpec, DepthSpec, DrawPassCtx, ItemMode, ListPass, run_list_layer, with_stencil_write,
};
use crate::systems::render_loop::draw_passes::draw_lists::{DrawLists, list};

/// 纹理组（`main_pass` 的 `r8`）。O：`+0` / `+0x24` / `+0x48`；名称 I（由 pass 名与访问推断）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MainPassTargets {
    /// `+0`：G-buffer 颜色。
    pub gbuffer: AttachmentTexture,
    /// `+0x24`：深度（所有子 pass 的视口都取它的 `extent_2d`，`rcx = struct + 0x28`）。
    pub depth: AttachmentTexture,
    /// `+0x48`：速度。
    pub velocity: AttachmentTexture,
}

/// `with_stencil_write`（O：机器码 `and al,-0x72; or al,0x21` 作用在 `RasterState` 的第 1 字节）。
pub fn with_stencil_write_state(state: u32) -> u32 {
    with_stencil_write(state)
}

fn depth_write_load(t: &MainPassTargets) -> DepthSpec {
    DepthSpec::Write {
        tex: t.depth,
        layer: 0,
        clear: None,
    }
}

fn stencil_pass<'s>(
    name: &'static str,
    colors: &'s [ColorSpec],
    depth: DepthSpec,
    viewport: AttachmentTexture,
) -> ListPass<'s> {
    ListPass {
        name,
        colors,
        depth,
        viewport,
        flag: false,
        mode: ItemMode::Stencil,
        variant: 0,
        extra: None,
    }
}

/// 子 pass 名（O：字符串池 0x142a929fc 起，逐个长度由 `mov r8d` 给出）。
pub mod names {
    pub const GBUFFER: &str = "G-buffer";
    pub const GBUFFER_EMISSIVE: &str = "G-buffer emissive";
    pub const GBUFFER_VELOCITY: &str = "G-buffer velocity";
    pub const FORWARD: &str = "Forward";
    pub const FORWARD_VELOCITY: &str = "ForwardVelocity";
    pub const VELOCITY: &str = "Velocity";
    pub const GBUFFER_OC2: &str = "G-buffer OC/2";
    pub const GBUFFER_EMISSIVE_OC2: &str = "G-buffer emissive OC/2";
    pub const GBUFFER_VELOCITY_OC2: &str = "G-buffer velocity OC/2";
    pub const FORWARD_OC2: &str = "Forward OC/2";
    pub const VELOCITY_OC2: &str = "Velocity OC/2";
    pub const LATE_DEFERRED: &str = "Late Deferred";
    pub const LATE_FORWARD: &str = "Late Forward";
}

/// W25 — 主 G-buffer / forward / velocity（6 个子 pass，O：0x140833bc0）。
///
/// `forward_enabled`：第二个资源（TypeId 0xcb…→`ShaderRefactoringState`，枚举，首个 dword）`== 0`（= `disabled()`）
/// 时才运行 `Forward` 与 `ForwardVelocity`（O：`cmp dword [rax], 0; jne → Velocity`）。
pub fn main_pass(
    ctx: &mut DrawPassCtx<'_>,
    targets: &MainPassTargets,
    emissive: &AttachmentTexture,
    lists: &DrawLists,
    shader_refactoring_disabled: bool,
) {
    let vp = targets.depth;
    let c1 = [ColorSpec::tag2(targets.gbuffer)];
    let c2 = [ColorSpec::tag0(targets.gbuffer), ColorSpec::tag1(*emissive)];
    let c3 = [ColorSpec::tag0(targets.gbuffer), ColorSpec::tag1(targets.velocity)];
    let c4 = [ColorSpec::tag0(*emissive)];
    let c5 = [ColorSpec::tag0(*emissive), ColorSpec::tag0(targets.velocity)];
    let c6 = [ColorSpec::tag0(targets.velocity)];
    let mut layer = vec![
        (stencil_pass(names::GBUFFER, &c1, DepthSpec::Write {
            tex: targets.depth, layer: 0, clear: Some(0.0),
        }, vp), lists.list(list::GBUFFER)),
        (stencil_pass(names::GBUFFER_EMISSIVE, &c2, depth_write_load(targets), vp), lists.list(list::GBUFFER_EMISSIVE)),
        (stencil_pass(names::GBUFFER_VELOCITY, &c3, depth_write_load(targets), vp), lists.list(list::GBUFFER_VELOCITY)),
    ];
    if shader_refactoring_disabled {
        layer.push((stencil_pass(names::FORWARD, &c4, depth_write_load(targets), vp), lists.list(list::FORWARD)));
        layer.push((stencil_pass(names::FORWARD_VELOCITY, &c5, depth_write_load(targets), vp), lists.list(list::FORWARD_VELOCITY)));
    }
    layer.push((ListPass {
        name: names::VELOCITY, colors: &c6,
        depth: DepthSpec::ReadOnly { tex: targets.depth }, viewport: vp,
        flag: false, mode: ItemMode::Velocity, variant: 0, extra: None,
    }, lists.list(list::VELOCITY)));
    let jobs: Vec<_> = layer.iter().map(|(spec, items)| (spec, *items)).collect();
    run_list_layer(ctx, &jobs);
}

/// W30 — 遮挡 pass 2（HZB 之后；5 个子 pass，全部 tag0 / 深度加载，无 `Forward` 门，O：0x140835a40）。
pub fn main_pass_oc2(
    ctx: &mut DrawPassCtx<'_>,
    targets: &MainPassTargets,
    emissive: &AttachmentTexture,
    lists: &DrawLists,
) {
    let vp = targets.depth;
    let c1 = [ColorSpec::tag0(targets.gbuffer)];
    let c2 = [ColorSpec::tag0(targets.gbuffer), ColorSpec::tag0(*emissive)];
    let c3 = [ColorSpec::tag0(targets.gbuffer), ColorSpec::tag0(targets.velocity)];
    let c4 = [ColorSpec::tag0(*emissive)];
    let c5 = [ColorSpec::tag0(targets.velocity)];
    let layer = [
        (stencil_pass(names::GBUFFER_OC2, &c1, depth_write_load(targets), vp), lists.list(list::GBUFFER_OC2)),
        (stencil_pass(names::GBUFFER_EMISSIVE_OC2, &c2, depth_write_load(targets), vp), lists.list(list::GBUFFER_EMISSIVE_OC2)),
        (stencil_pass(names::GBUFFER_VELOCITY_OC2, &c3, depth_write_load(targets), vp), lists.list(list::GBUFFER_VELOCITY_OC2)),
        (stencil_pass(names::FORWARD_OC2, &c4, depth_write_load(targets), vp), lists.list(list::FORWARD_OC2)),
        (ListPass {
            name: names::VELOCITY_OC2, colors: &c5,
            depth: DepthSpec::ReadOnly { tex: targets.depth }, viewport: vp,
            flag: false, mode: ItemMode::Velocity, variant: 0, extra: None,
        }, lists.list(list::VELOCITY_OC2)),
    ];
    let jobs: Vec<_> = layer.iter().map(|(spec, items)| (spec, *items)).collect();
    run_list_layer(ctx, &jobs);
}

/// W34 — `Late Deferred`（`DrawLists +0x128/+0x130`）与 `Late Forward`（`+0x140/+0x148`）两个子 pass（O：0x1408373b0）。
///
/// O：每个子 pass 前先 `test [lists + len], len; je skip`（`impl$0::is_empty` 守卫）——**列表为空则整个子 pass（含
/// `RasterPass::new`）都不创建**。Late Deferred 颜色 tag0 = `targets.gbuffer`（`r8 + 0`）；
/// Late Forward 颜色 tag0 = 第 4 参数 `scene_color`（`r9`，0x1408373d6 保存、0x14083773e 恢复、
/// 0x140837787 写入附件）。两者深度 tag0 可写/加载 = `r8 + 0x24`，视口 = 深度 extent（`r8 + 0x28`）。
pub fn late_forward_and_deferred_pass(
    ctx: &mut DrawPassCtx<'_>,
    targets: &MainPassTargets,
    scene_color: &AttachmentTexture,
    lists: &DrawLists,
) {
    let colors = [[ColorSpec::tag0(targets.gbuffer)], [ColorSpec::tag0(*scene_color)]];
    let specs = [
        stencil_pass(names::LATE_DEFERRED, &colors[0], depth_write_load(targets), targets.depth),
        stencil_pass(names::LATE_FORWARD, &colors[1], depth_write_load(targets), targets.depth),
    ];
    let items = [lists.list(list::LATE_DEFERRED), lists.list(list::LATE_FORWARD)];
    let jobs: Vec<_> = specs.iter().zip(items).filter(|(_, items)| !items.is_empty()).collect();
    run_list_layer(ctx, &jobs);
}
