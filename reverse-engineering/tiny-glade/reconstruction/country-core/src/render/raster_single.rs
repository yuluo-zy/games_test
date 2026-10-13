//! raster_single.rs — `raster_single_object`（单颜色目标 / 全屏三角形的光栅 pass 助手）。
//!
//! O = 机器码观测，I = 推断，U = 未知。三个泛型实例按函数体归并为一个泛型：
//! - 0x1409cdec0（2225 B）`…::ui_2d_pass::mei_pass::closure_env$0`
//! - 0x1409ce780（6417 B）`…::final_post::final_post::closure_env$1`
//! - 0x1409d00a0（2081 B）`…::final_post::resolve_to_swapchain::closure_env$0`
//!
//! 三者共有的骨架（O，按调用顺序）：
//! 1. `puffin::are_scopes_on` → 开 profile scope（函数名 "raster_single_object"，文件 raster_single.rs）；
//!    puffin 在离线缓存缺失，这里不复刻 scope（只影响 profiler，无游戏可见行为）。
//! 2. `TextureDesc::extent_2d(第一个颜色附件)` → 作为视口尺寸（存进 `RasterPass +0x1f8`）；
//! 3. `RasterPass::new_impl(name, 附件 ×N(=1), 深度 = 无, …)`；
//! 4. `DrawPacket::new`（着色器 = 调用方传入的 `Handle<RasterShaderProgram>`）→ `DrawBuilder::new(.., variant = 0)`；
//! 5. 闭包主体（各实例不同：纹理绑定 / uniform / 光栅状态位改写）——这里表现为 `closure(builder) -> builder`；
//! 6. `DrawBuilder::draw_no_vao(3)`（全屏三角形，三个实例都是 `mov edx, 3`）；
//! 7. `RasterPass::render`。
//!
//! 闭包本身属于 `ui_2d_pass` / `final_post`（不在本文件范围）。mei_pass 的闭包额外改写光栅状态
//! （`byte0 &= 0x7f`、`byte3 = (byte3 & 0x9f) | 0x40`，见 [`RasterState`](crate::render::pass_builder::draw_stream::RasterState)）。

use crate::render::pass_builder::draw_builder::DrawBuilder;
use crate::render::pass_builder::draw_stream::{DrawPacket, PassHost, RasterPass};
use crate::render::pass_builder::framebuffer_binding::ColorAttachment;
use crate::render::shader::RasterShaderProgram;
use crate::rhapsody_contract::Handle;

/// 全屏三角形的顶点数（O：三个实例都以 `edx = 3` 调 `draw_no_vao`）。
pub const FULLSCREEN_TRIANGLE_VERTICES: u32 = 3;

/// `raster_single_object<N, [ColorAttachment; N], CLOSURE>`。
/// 原版的 `N` 恒为 1（O：模板实参 `1` / `array<ColorAttachment,1>`）。
pub fn raster_single_object<const N: usize, F>(
    name: &'static str,
    attachments: &[ColorAttachment; N],
    shader: Handle<RasterShaderProgram>,
    host: alloc::boxed::Box<dyn PassHost>,
    closure: F,
) where
    F: for<'p> FnOnce(DrawBuilder<'p>) -> DrawBuilder<'p>,
{
    // 2. 视口 = 第一个附件的 extent_2d（N == 0 在原版里不存在）。
    let extent = attachments[0].desc().extent;
    // 3. RasterPass（无深度）。
    let mut pass =
        RasterPass::new(name, attachments, None, host).with_viewport([extent.0, extent.1]);
    // 4. DrawPacket → DrawBuilder（variant 0）。
    let state = pass.host.default_raster_state();
    let packet = DrawPacket::new(shader, [0, 0], state);
    let builder = DrawBuilder::new(&mut pass, packet, 0);
    // 5–6. 闭包 + 全屏三角形。
    closure(builder).draw_no_vao(FULLSCREEN_TRIANGLE_VERTICES);
    // 7. 渲染。
    pass.render();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::pass_builder::draw_builder::{ACCESS_SAMPLED_IMAGE, BindTexture};
    use crate::render::pass_builder::draw_stream::test_host::MockHost;
    use crate::render::pass_builder::draw_stream::{DrawKind, ShaderReflection};
    use crate::render::pass_builder::framebuffer_binding::AttachmentTexture;
    use alloc::sync::Arc;

    fn att(id: u32, w: u32, h: u32) -> ColorAttachment {
        ColorAttachment::Texture(AttachmentTexture {
            id,
            extent: (w, h),
            format: 44,
            sample_count: 1,
        })
    }

    fn shader_handle() -> Handle<RasterShaderProgram> {
        Handle(
            crate::rhapsody_contract::ManagedHandle(1),
            core::marker::PhantomData,
        )
    }

    #[test]
    fn runs_closure_then_fullscreen_triangle_then_render() {
        let (mut host, log) = MockHost::new();
        let mut r = ShaderReflection::default();
        r.program_name = "fs";
        r.bindings.insert(0x10, (0, 2));
        host.shaders.insert(1, Arc::new(r));
        host.presets.insert(
            5,
            AttachmentTexture {
                id: 9,
                extent: (4, 4),
                format: 1,
                sample_count: 1,
            },
        );
        let mut ran = false;
        raster_single_object::<1, _>(
            "pass",
            &[att(7, 1280, 720)],
            shader_handle(),
            host.boxed(),
            |b| {
                ran = true;
                b.bind_texture_to_draw_builder(
                    BindTexture::Preset(crate::resources::texture_presets::TexturePreset(5)),
                    0x10,
                    ACCESS_SAMPLED_IMAGE,
                    None,
                )
            },
        );
        assert!(ran);
        let l = log.borrow();
        assert_eq!(l.submissions.len(), 1);
        let s = &l.submissions[0];
        assert_eq!(s.draws.len(), 1);
        // 三个实例都是 draw_no_vao(3)；variant 0。
        assert_eq!(s.draws[0].kind, DrawKind::NoVao { vertex_count: 3 });
        assert_eq!(s.draws[0].variant, 0);
        assert_eq!(FULLSCREEN_TRIANGLE_VERTICES, 3);
        // 颜色附件访问（tag0 → 0x36，组 1）先于闭包里的纹理访问。
        assert_eq!(s.accesses[0].resource, 7);
        assert_eq!(s.accesses[0].access, 0x36);
        assert_eq!(s.accesses[1].resource, 9);
        assert_eq!(s.color_count, 1);
        assert_eq!(l.created_passes.len(), 1);
    }

    #[test]
    fn viewport_is_the_first_attachment_extent() {
        let (mut host, log) = MockHost::new();
        let mut r = ShaderReflection::default();
        r.has_uniform_block = true;
        r.uniform_block_size = 8;
        r.uniform_offsets.insert(0x42, 0);
        host.shaders.insert(1, Arc::new(r));
        let mut seen = None;
        raster_single_object::<1, _>(
            "pass",
            &[att(7, 640, 480)],
            shader_handle(),
            host.boxed(),
            |b| {
                let b = b.optional_uniform(0x42, [640u32, 480u32]);
                seen = b.constants_data().map(|d| d.to_vec());
                b
            },
        );
        assert_eq!(
            seen.unwrap(),
            [640u32.to_le_bytes(), 480u32.to_le_bytes()].concat()
        );
        assert_eq!(log.borrow().submissions[0].draws.len(), 1);
    }

    #[test]
    #[should_panic(expected = "The DrawBuilder for fs was not drawn or discarded.")]
    fn closure_that_drops_the_builder_panics_via_drop_check() {
        // 闭包返回前把 builder 丢掉不可能（返回类型要求）；这里验证未消费的 builder 在 pass 里被检出。
        let (mut host, _log) = MockHost::new();
        let mut r = ShaderReflection::default();
        r.program_name = "fs";
        host.shaders.insert(1, Arc::new(r));
        let mut pass = RasterPass::new("p", &[att(1, 2, 2)], None, host.boxed());
        let packet = DrawPacket::new(
            shader_handle(),
            [0, 0],
            crate::render::pass_builder::draw_stream::RasterState(0),
        );
        {
            let _b = DrawBuilder::new(&mut pass, packet, 0);
        }
        pass.render();
    }
}
