//! draw_stream.rs — RasterPass / DrawPacket / DrawPassRenderCheck / TextureBindingKey。
//!
//! 机器码锚点（contracts/draw_stream.md）：
//! - `DrawPacket::new` 0x140867850（行 106）；`TextureBindingKey::with_view` 0x140867940（行 147）；
//! - `RasterPass::new_impl` 0x140867970（行 352）；`RasterPass::render` 0x140868370（行 450）；
//! - `DrawPassRenderCheck` drop 0x140896d80（行 260）；`to_texture_binding_key` 0x1408eefd0（行 201）。
//!
//! 接入说明（assembly）：rhapsody managed-handle / render-pass / texture surface 统一走
//! `crate::rhapsody_contract`（`Handle` / `RenderPass` / `RenderPassDesc` / `Texture` /
//! `TextureViewDesc` / `ReadOnlyHandle`）；`RasterShaderProgram` = WP4 `render::shader`。

use alloc::vec::Vec;
use hashbrown::HashMap;

use crate::render::pass_builder::framebuffer_binding::ColorAttachment;
use crate::render::shader::RasterShaderProgram;
use crate::rhapsody_contract::{self, Handle, ReadOnlyHandle, RenderPass, RenderPassDesc, Texture};

/// draw 命令判别（DrawBuilder mode@+0x40 的枚举载体）。
/// 观测值：1（句柄）/3（gl_draw_flag）/4（multi_instanced）/5（multi_draw_indirect）/
/// 7（multi_draw_indexed_indirect|indexed_indirect_raw）/8（indexed_indirect_count）。
/// Rust 枚举名未恢复 —— typed gap。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrawKind {
    Kind1 = 1,
    Kind3 = 3,
    Kind4 = 4,
    Kind5 = 5,
    Kind7 = 7,
    Kind8 = 8,
}

/// 绘制包：shader 句柄 + RandomState + blend（+0xac = 0x3f800000 = 1.0）。
#[derive(Clone, Debug)]
pub struct DrawPacket {
    pub shader: Handle<RasterShaderProgram>,
    pub blend: f32,
    pub texture_bindings: HashMap<TextureBindingKey, ()>,
}

impl DrawPacket {
    /// `new`（0x140867850，行 106）：默认 shader（`shader::impl$7::default()`）+ 两 RandomState + blend 1.0。
    pub fn new() -> DrawPacket {
        DrawPacket {
            shader: RasterShaderProgram::default_handle(),
            blend: 1.0,
            texture_bindings: HashMap::new(),
        }
    }

    /// `variant`（inline 1）：变体判别。
    pub fn variant(&self) -> u32 {
        0 // typed gap
    }

    /// `stencil_ref`（inline 3）。
    pub fn stencil_ref(&mut self, _value: u8) {}
}

impl Default for DrawPacket {
    fn default() -> Self {
        Self::new()
    }
}

/// 纹理绑定键（texture handle + view desc hash）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextureBindingKey {
    pub texture: u32,
    pub view_hash: u64,
}

impl TextureBindingKey {
    /// `with_view`（0x140867940，行 147）：view 非 0 时并入 TextureViewDesc 的 hash。
    pub fn with_view(texture: u32, view: Option<&crate::rhapsody_contract::TextureViewDesc>) -> Self {
        let view_hash = match view {
            Some(v) => crate::render::utils::calculate_hash(v),
            None => 0,
        };
        TextureBindingKey { texture, view_hash }
    }
}

/// 纹理 → 纹理绑定键。
pub trait ToTextureDrawStreamBindingKey {
    fn to_texture_binding_key(&self) -> TextureBindingKey;
}

impl ToTextureDrawStreamBindingKey for &Texture {
    fn to_texture_binding_key(&self) -> TextureBindingKey {
        TextureBindingKey {
            texture: self.handle(),
            view_hash: 0,
        }
    }
}

impl ToTextureDrawStreamBindingKey for &Handle<Texture> {
    fn to_texture_binding_key(&self) -> TextureBindingKey {
        TextureBindingKey {
            texture: self.handle(),
            view_hash: 0,
        }
    }
}

impl ToTextureDrawStreamBindingKey for &ReadOnlyHandle<Texture> {
    fn to_texture_binding_key(&self) -> TextureBindingKey {
        TextureBindingKey {
            texture: self.handle(),
            view_hash: 0,
        }
    }
}

/// 渲染检查（drop 时断言 pass 已 render，否则 panic_fmt）。
#[derive(Clone, Debug)]
pub struct DrawPassRenderCheck(pub bool);

impl Drop for DrawPassRenderCheck {
    fn drop(&mut self) {
        // 0x140896d80：若未渲染 → panic。
        assert!(self.0, "RasterPass dropped without render");
    }
}

/// RasterPass（约 0x3e8B）。
#[derive(Clone, Debug)]
pub struct RasterPass {
    /// 附件数组（每项 0xcB，+0x40）。
    pub(crate) color_attachments: [ColorAttachment; 5],
    /// 附件计数（+0x7c）。
    pub(crate) color_count: u32,
    /// 深度标志（+0x1c；==2 表示无深度附件路径）。
    pub(crate) depth_flag: i32,
    /// 渲染 pass 去重表（HashMap<RenderPassDesc, RenderPass>）。
    pub(crate) render_passes: HashMap<RenderPassDesc, RenderPass>,
    /// 渲染检查。
    pub(crate) rendered: DrawPassRenderCheck,
    // typed gap：+0x3e8 内其余字段（shader 句柄、纹理视图描述等）。
    pub(crate) packet: DrawPacket,
}

impl RasterPass {
    /// `new`（inline 42）→ `new_impl`（0x140867970，行 352）。
    /// 至多 5 个 ColorAttachment 转 RasterPassColorAttachment（arrayvec 上限 5）；
    /// PassResources::new_from；clone 2 DescriptorSetBinding；两个 RandomState map。
    pub fn new(
        _attachments: &[ColorAttachment],
        _device: &crate::render::GraphicsDeviceResource,
        _world: &bevy_ecs::world::World,
    ) -> RasterPass {
        RasterPass {
            color_attachments: [Default::default(); 5],
            color_count: 0,
            depth_flag: 0,
            render_passes: HashMap::new(),
            rendered: DrawPassRenderCheck(false),
            packet: DrawPacket::new(),
        }
    }

    /// `get_render_pass`（inline 1）：取/建本 pass 的 RenderPass。
    pub fn get_render_pass(
        &mut self,
        _device: &crate::rhapsody_contract::GraphicsDevice,
    ) -> Handle<RenderPass> {
        // HashMap::rustc_entry 去重 → GraphicsDevice::create_render_pass。
        todo!("render pass dedupe via rustc_entry (wired in integration)")
    }

    /// `render`（0x140868370，行 450）：
    /// 置已渲染；构造 RenderPassAttachmentDesc 列表（≤0x80）；按 +0x1c 加深度附件（可选 clear_input）；
    /// HashMap<RenderPassDesc,RenderPass> 去重；GraphicsDevice::create_render_pass；
    /// PassBuilder::render<PassResources, render::closure_env_0>。
    pub fn render(mut self, _device: &crate::rhapsody_contract::GraphicsDevice) {
        self.rendered.0 = true;
        // ... 附件→RenderPassAttachmentDesc；创建/复用 RenderPass；注册图 pass。
    }
}

/// DrawStream（高层封装；无独立过程，聚合 DrawPacket + RasterPass）。
#[derive(Clone, Debug, Default)]
pub struct DrawStream {
    pub packets: Vec<DrawPacket>,
}
