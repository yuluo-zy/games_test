//! `d_main_support.rs` — draw_passes 共用的 pass 宿主层与“列表驱动”运行器（d_main 新增，无独立 PDB 过程）。
//!
//! 原版里每个 pass 都是一个 bevy 系统风格的自由函数：先 `RasterPass::new_impl(name, 颜色…, 深度, 视口, &World, &ctx)`，
//! 再 `Components::get_resource_id(Time)` 取 `Duration::as_secs_f32` 作 `CommonDrawParams.time`，然后逐元素
//! `DrawPacket::new → DrawBuilder::new → bind_common → 状态位改写 → 回调`，最后 `RasterPass::render`。
//! 这些步骤在 wireframe / ui / loading / main / oc2 / late 里是同一段内联代码，差别只在下面 [`ListPass`] 里的参数。
//!
//! A4 的 `RasterPass` 需要一个 `Box<dyn PassHost>`（对应 `PassResources::new_from(world)` + 图录制器）；
//! 本层用 [`DrawPassCtx`] 的工厂闭包按 pass 生成宿主。

use core::time::Duration;

use alloc::boxed::Box;
use bevy_ecs::world::World;

use crate::asset_libraries::color_library::ColorLibrarySsbo;
use crate::render::pass_builder::draw_builder::{CommonDrawFlag, CommonDrawParams, DrawBuilder};
use crate::render::pass_builder::draw_stream::{PassHost, RasterPass};
use crate::render::pass_builder::framebuffer_binding::{
    AttachmentTexture, ColorAttachment, DepthAttachment, DepthClear,
};
use crate::render::shader::RasterShaderProgram;
use crate::rhapsody_contract::Handle;
use crate::systems::render_loop::draw_passes::draw_lists::DrawEntry;

/// `ColorAttachment` 的 tag（O：`RasterPass::new_impl` 读取的 dword）。A4 的枚举只有 tag0/tag1，
/// `main_pass` 的首个 `G-buffer` pass 用了 **tag 2**（O：`mov dword [rbp+0x988], 2`），下面用 [`ColorKind::Tag2`] 表达并在建 pass 后补丁。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorKind {
    /// tag0：访问 0x36。
    Tag0,
    /// tag1：访问 0x2e，附带 16 B 视图字（O：全 0）。
    Tag1,
    /// tag2：访问 0x2e（`(tag == 0) * 8 + 0x2e`），引用在 `+8`。
    Tag2,
}

/// 一个颜色附件说明。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorSpec {
    pub tex: AttachmentTexture,
    pub kind: ColorKind,
}

impl ColorSpec {
    pub fn tag0(tex: AttachmentTexture) -> ColorSpec {
        ColorSpec {
            tex,
            kind: ColorKind::Tag0,
        }
    }
    pub fn tag1(tex: AttachmentTexture) -> ColorSpec {
        ColorSpec {
            tex,
            kind: ColorKind::Tag1,
        }
    }
    pub fn tag2(tex: AttachmentTexture) -> ColorSpec {
        ColorSpec {
            tex,
            kind: ColorKind::Tag2,
        }
    }
}

/// 深度附件说明（O：`DepthAttachment` tag 0 可写 / tag 1 只读 / tag 2 无）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DepthSpec {
    /// tag2。
    None,
    /// tag0；`clear` = `+0x10` 非 0 时的清除值（`+0x14`）。
    Write {
        tex: AttachmentTexture,
        layer: u32,
        clear: Option<f32>,
    },
    /// tag1。
    ReadOnly { tex: AttachmentTexture },
}

impl DepthSpec {
    fn attachment(&self) -> Option<DepthAttachment> {
        match *self {
            DepthSpec::None => None,
            DepthSpec::Write { tex, layer, clear } => Some(DepthAttachment {
                writable: true,
                layer,
                texture: tex,
                clear: clear.map_or(DepthClear::Load, DepthClear::Clear),
            }),
            DepthSpec::ReadOnly { tex } => Some(DepthAttachment {
                writable: false,
                layer: 0,
                texture: tex,
                clear: DepthClear::Load,
            }),
        }
    }
}

/// `AssetShaderLibrary::get_raster_handle_by_name(&str) -> Option<Handle<RasterShaderProgram>>`（0x140937aa0，O：返回 `al` 命中标志 +
/// `rdx` 句柄）。库本体属 `asset_libraries::shader_library`（不在本范围），这里只取调用面。
pub trait ShaderLookup {
    fn raster_handle_by_name(&self, name: &str) -> Option<Handle<RasterShaderProgram>>;
}

/// 空查找表（任何名字都未命中）。
pub struct NoShaders;
impl ShaderLookup for NoShaders {
    fn raster_handle_by_name(&self, _name: &str) -> Option<Handle<RasterShaderProgram>> {
        None
    }
}

/// pass 运行环境：World、`Res<Time>` 的已流逝时间、以及按 pass 生成宿主的工厂。
pub struct DrawPassCtx<'a> {
    pub world: &'a mut World,
    /// `Res<Time>`（TypeId 0x4bc362195b80f729）`+0` 的 `Duration`（secs u64 @0、nanos u32 @8）。
    pub elapsed: Duration,
    new_host: &'a mut dyn FnMut() -> Box<dyn PassHost>,
    /// `Res<AssetShaderLibrary>` 的按名查找（`get_raster_handle_by_name`）。
    pub shaders: &'a dyn ShaderLookup,
}

impl<'a> DrawPassCtx<'a> {
    pub fn new(
        world: &'a mut World,
        elapsed: Duration,
        new_host: &'a mut dyn FnMut() -> Box<dyn PassHost>,
        shaders: &'a dyn ShaderLookup,
    ) -> DrawPassCtx<'a> {
        DrawPassCtx {
            world,
            elapsed,
            new_host,
            shaders,
        }
    }

    /// `get_raster_handle_by_name(name)`，未命中 → `Option::expect` 失败（各调用点文案见各 pass）。
    pub fn shader(&self, name: &str) -> Option<Handle<RasterShaderProgram>> {
        self.shaders.raster_handle_by_name(name)
    }

    /// 取一个新的 `PassHost`（对应 `RasterPass::new_impl` 的 `&World` + `&ctx` 参数）。
    pub fn host(&mut self) -> Box<dyn PassHost> {
        (self.new_host)()
    }

    /// `CommonDrawParams::new`（inline）：`Duration::as_secs_f32` + 布尔标志。
    pub fn common_params(&self, flag: impl Into<CommonDrawFlag>) -> CommonDrawParams {
        CommonDrawParams::new(self.elapsed, flag)
    }

    /// 建 `RasterPass`（`new_impl`）：颜色/深度/视口；tag2 颜色补丁访问类型。
    pub fn raster_pass(
        &mut self,
        name: &'static str,
        colors: &[ColorSpec],
        depth: DepthSpec,
        viewport: AttachmentTexture,
    ) -> RasterPass {
        let atts: Vec<ColorAttachment> = colors
            .iter()
            .map(|c| match c.kind {
                ColorKind::Tag1 => ColorAttachment::View {
                    view: [0; 4],
                    texture: c.tex,
                },
                _ => ColorAttachment::Texture(c.tex),
            })
            .collect();
        let host = self.host();
        let d = depth.attachment();
        let mut pass =
            RasterPass::new(name, &atts, d.as_ref(), host).with_viewport([viewport.extent.0, viewport.extent.1]);
        for (i, c) in colors.iter().enumerate() {
            if c.kind == ColorKind::Tag2 {
                // O：tag2 的访问 = 0x2e，描述里的 `+4` 覆盖为 2。
                pass.colors[i].tag = 2;
                pass.accesses[i].access = crate::render::pass_builder::framebuffer_binding::COLOR_ACCESS_VIEW;
            }
        }
        pass
    }
}

/// 逐元素的状态位改写模式。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemMode {
    /// 无改写（`ui_pass`、`loading_screen_pass`）。
    Plain,
    /// `bind_common` 之后：`state.byte3 |= 0x80`（`set_polygon_mode`）、`state.byte0 &= 0x7f`（`set_depth_test_enable(false)`）、
    /// 混合 1.0（`wireframe_pass`）。
    Wireframe,
    /// `DrawBuilder::new` 之后读取 `stencil_ref`（`Option::unwrap`，None → panic）写入 `packet.+0xb1`，
    /// `bind_common` 之后 `state.byte1 = (byte1 & 0x8e) | 0x21`（`with_stencil_write`）。
    Stencil,
    /// 在构造包**之前**把状态改成 `(state & 0xffff_ff8f) | 0x20`（`Velocity` / `Velocity OC/2`）。
    Velocity,
    /// `shadow_pass`：`bind_common` 的标志字节是 `CommonDrawFlag`（O：`movzx r14d, byte [params+4]`），这里传 `cascade + 2`
    /// （`ShadowCascade0..2`，A4 已改 u8 枚举，不再需要覆盖写 uniform）；之后 `state.byte0 &= 0xcf`（O：`and byte [..], -0x31`）。
    Shadow { flag: u8 },
    /// 透明 pass：`bind_common` + 额外绑定之后 `state.byte0 &= 0xbf`（`and al, -0x41`）；`byte3_field2` 为真时再
    /// `byte3 = (byte3 & 0x9f) | 0x40`（`pre/post_taa` 有，`refraction_copy` 没有）。
    Transparent { byte3_field2: bool },
}

/// 额外的 bind 步骤（`bind_common` 之后、状态改写之前）。
pub type ExtraBind<'s> = &'s dyn for<'p> Fn(DrawBuilder<'p>) -> DrawBuilder<'p>;

/// `bind_common` 里 `write_uniform<u32>(…, flag)` 的 uniform 名哈希（O：`movabs -0x3a5498c04f79ae0b`）。
pub const BIND_COMMON_FLAG_INT_UNIFORM: u64 = 0xc5ab_673f_b086_51f5;

/// `Option::unwrap` 失败文案（原 `unwrap_failed` 的固定文本）。
pub const UNWRAP_NONE: &str = "called `Option::unwrap()` on a `None` value";

/// `with_stencil_write`（main_pass.rs:22）：`set_stencil_test_enable` + `set_stencil_pass_op`，
/// 机器码为 `byte1 = (byte1 & 0x8e) | 0x21`（O：`and al, -0x72; or al, 0x21`）。
pub fn with_stencil_write(state: u32) -> u32 {
    let b1 = (state >> 8) & 0xff;
    let nb1 = (b1 & 0x8e) | 0x21;
    (state & !0xff00) | (nb1 << 8)
}

/// `Velocity` 状态改写（O：`and esi, -0x71; or esi, 0x20`）。
pub fn velocity_state(state: u32) -> u32 {
    (state & 0xffff_ff8f) | 0x20
}

/// `wireframe_pass` 状态改写（O：`or byte [+0xab], 0x80; and byte [+0xa8], 0x7f`）。
pub fn wireframe_state(state: u32) -> u32 {
    (state | 0x8000_0000) & !0x80
}

/// 一次“列表驱动 pass”的全部参数。
pub struct ListPass<'s> {
    pub name: &'static str,
    pub colors: &'s [ColorSpec],
    pub depth: DepthSpec,
    /// 视口取自哪张纹理的 `extent_2d`。
    pub viewport: AttachmentTexture,
    /// `CommonDrawParams` 的布尔标志（O：`mov byte [..+0x89c], 1/0`）。
    pub flag: bool,
    pub mode: ItemMode,
    /// `DrawBuilder::new` 的着色器变体（O：`xor r9d, r9d` = 0；`shadow_pass` 为 `mov r9b, 1`）。
    pub variant: u8,
    /// `bind_common` 之后的额外纹理绑定（`refraction_copy` / `pre_taa`）。
    pub extra: Option<ExtraBind<'s>>,
}

/// 运行一个列表驱动 pass（O：见模块头）。
pub fn run_list_pass(ctx: &mut DrawPassCtx<'_>, spec: &ListPass<'_>, items: &[DrawEntry]) {
    let mut pass = ctx.raster_pass(spec.name, spec.colors, spec.depth, spec.viewport);
    let mut params = ctx.common_params(spec.flag);
    if let ItemMode::Shadow { flag } = spec.mode {
        params.flag = CommonDrawFlag::from_byte(flag).expect("shadow cascade flag out of range");
    }
    for (item, func) in items {
        let mut state = pass.host.default_raster_state();
        if spec.mode == ItemMode::Velocity {
            state.0 = velocity_state(state.0);
        }
        let packet = item.to_draw_packet(state);
        let mut builder = DrawBuilder::new(&mut pass, packet, spec.variant);
        if spec.mode == ItemMode::Stencil {
            builder.packet_mut().unknown_b1 = item.stencil_ref.expect(UNWRAP_NONE);
        }
        let mut builder = builder.bind_common::<ColorLibrarySsbo>(&params, &item.params.transform);
        if let Some(extra) = spec.extra {
            builder = extra(builder);
        }
        match spec.mode {
            ItemMode::Transparent { byte3_field2 } => {
                let p = builder.packet_mut();
                p.state.0 &= !0x40;
                if byte3_field2 {
                    p.state = p.state.set_byte3_field_2();
                }
            }
            ItemMode::Wireframe => {
                let p = builder.packet_mut();
                p.state.0 = wireframe_state(p.state.0);
                p.blend = 1.0;
            }
            ItemMode::Stencil => {
                let p = builder.packet_mut();
                p.state.0 = with_stencil_write(p.state.0);
            }
            ItemMode::Shadow { .. } => {
                let p = builder.packet_mut();
                p.state.0 &= !0x30;
            }
            _ => {}
        }
        func(builder, &item.params, &mut *ctx.world);
    }
    pass.render();
}
