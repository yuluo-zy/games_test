//! `custom_cursor_pass.rs` — 自定义光标 pass（0x1409d1710，2455 B）。着色器 `custom_cursor`（`/custom_cursor.raster.hlsl`）。
//!
//! O 顺序：puffin scope → `Res<AssetShaderLibrary>` → `get_raster_handle_by_name("custom_cursor"(0xd))`（未命中 `expect` panic）→
//! `Res<WindowSize>`（`+0` 宽 u32、`+4` 高 u32；`as_vec2`）→ 光标资源（TypeId 0x3d7b56e7…）：**`+0xf4` 字节为 0 则整个 pass 不执行**
//! （`je → 结尾`）→ `extent_2d(target)` → `RasterPass::new_impl("Custom cursor"(0xd), [tag0 = target], 无深度, 视口 = target)`
//! → `DrawPacket::new(head = [0, …])`/`DrawBuilder::new(variant 0)` →
//! `write_uniform<[u32;2]>(0x0a5a0625b7860559, [w, h])`（`Vec2` → `f32 as u32` 饱和转换：`cvttss2si` + 与 0 / 4294967040.0 比较的 cmov；
//! 构建器常量块为 `None` 时跳过）→ `bind_texture_to_draw_builder(TextureFromDisk(0xf9f14d14bcc4e780), 绑定名 0x9f0d228ee3dba429, 访问 0x1d, 无 view)` →
//! `read_storage_buffer<CustomCursorSsbo>(槽 "CustomCursorSsbo", 0x19ce7a8f29cef00d)` → 状态 `byte0 &= 0x7f`、
//! `byte3 = (byte3 & 0x9f) | 0x40` → `draw_no_vao(6)` → `render`。

use crate::render::pass_builder::draw_builder::{BindTexture, DrawBuilder, ACCESS_SAMPLED_IMAGE};
use crate::render::pass_builder::draw_stream::{DrawPacket, RasterState};
use crate::render::pass_builder::framebuffer_binding::AttachmentTexture;
use crate::render::pass_builder::pass_resource_access::SsboType;
use crate::resources::texture_presets::TextureFromDisk;
use crate::rhapsody_contract::{Handle, ManagedHandle};
use crate::systems::render_loop::draw_passes::d_main_support::{ColorSpec, DepthSpec, DrawPassCtx};

/// B original shader CustomCursor: pos@0, enabled@8, size_px@12. The
/// constructor@1416146c0 uses 16B, host-visible memory2, no None upload.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CustomCursorSsbo {
    pub pos: [f32; 2],
    pub enabled: u32,
    pub size_px: f32,
}

impl crate::render::ssbo::StorageElement for CustomCursorSsbo {
    const NAME: &'static str = "CustomCursorSsbo";
    const BUFFER_LEN: usize = 1;
}

impl crate::asset_libraries::ssbo_library::SsboType for CustomCursorSsbo {
    type Storage = crate::render::ssbo::TypedStorageBuffer<Self>;

    fn new_storage_buffer(
        device: &dyn crate::render::ssbo::rhapsody_boundary::GraphicsDevice,
        initial: Option<&[u8]>,
    ) -> Self::Storage {
        use crate::render::ssbo::{rhapsody_boundary::BufferDesc, StorageElement};
        if let Some(bytes) = initial {
            // Native order: element divisibility, pointer alignment, capacity.
            assert_eq!(bytes.len() % 16, 0);
            assert_eq!(bytes.as_ptr() as usize % 4, 0);
            assert!(
                bytes.len() < 32,
                "Excessive data size supplied for buffer {}",
                core::any::type_name::<Self>()
            );
        }
        let desc = BufferDesc::new(16, 0x22, 2).alignment(4);
        let buffer = device
            .create_buffer(&desc, Self::NAME, initial)
            .expect("called `Result::unwrap()` on an `Err` value: create_buffer");
        crate::render::ssbo::TypedStorageBuffer::from_created_buffer(buffer)
    }
}

/// Only the final entry of native init_populate_ssbo_library@140909e80.
/// The caller must retain its actual device/buffer owner; the OS consumer
/// resolves and retains a real managed-buffer lease before using this pointer.
/// This does not replace the preceding71 registrations or send a fake event.
pub fn create_custom_cursor_ssbo(
    library: &crate::asset_libraries::ssbo_library::AssetSsboLibrary,
    device: &dyn crate::render::ssbo::rhapsody_boundary::GraphicsDevice,
) -> crate::resources::mouse_capture::CustomCursorSsboCreatedCmd {
    use crate::resources::mouse_capture::CustomCursorSsboCreatedCmd;
    library.add_new::<CustomCursorSsbo>(device);
    let buffer = library
        .get_buffer::<CustomCursorSsbo>()
        .expect("called `Option::unwrap()` on a `None` value");
    let mapped = device
        .mapped_slice_mut(buffer)
        .expect("called `Option::unwrap()` on a `None` value");
    assert_eq!(mapped.as_ptr() as usize % 4, 0);
    assert_eq!(mapped.len() % 16, 0);
    assert_eq!(mapped.len(), 16);
    CustomCursorSsboCreatedCmd(
        core::ptr::NonNull::new(mapped.as_mut_ptr().cast())
            .expect("mapped Vulkan buffer pointer must be non-null"),
    )
}

impl SsboType for CustomCursorSsbo {
    /// O：`get_rg_buffer<CustomCursorSsbo>`（0x1412c7550）里的 `SsboId` 立即数 `-0x2f0f7d81800dd867`。
    fn rg_buffer_hash() -> u64 {
        0xd0f0_827e_7ff2_2799
    }
}

pub const SHADER_NAME: &str = "custom_cursor";
pub const PASS_NAME: &str = "Custom cursor";
pub const WINDOW_SIZE_UNIFORM: u64 = 0x0a5a_0625_b786_0559;
pub const CURSOR_TEXTURE_FILE: u64 = 0xf9f1_4d14_bcc4_e780;
pub const CURSOR_TEXTURE_BINDING: u64 = 0x9f0d_228e_e3db_a429;
pub const CURSOR_SSBO_BINDING: u64 = 0x19ce_7a8f_29ce_f00d;
pub const CURSOR_SSBO_SLOT: &str = "CustomCursorSsbo";
pub const CURSOR_VERTEX_COUNT: u32 = 6;

/// `WindowSize::as_vec2` → `write_uniform<[u32;2]>` 的饱和转换（O：`cvtsi2ss` 后 `cvttss2si` + cmov 夹到 `[0, u32::MAX]`）。
pub fn window_uniform(width: u32, height: u32) -> [u32; 2] {
    [(width as f32) as u32, (height as f32) as u32]
}

/// 状态改写（O：`and byte [+0xa8], 0x7f; byte3 = (byte3 & 0x9f) | 0x40`），UI 叠加类 pass 共用。
pub fn overlay_state(state: RasterState) -> RasterState {
    RasterState(state.clear_byte0_bit7().0).set_byte3_field_2()
}

/// `custom_cursor_pass`。`window` = `Res<WindowSize>`，`cursor_visible` = 光标资源 `+0xf4` 字节是否非 0。
pub fn custom_cursor_pass(
    ctx: &mut DrawPassCtx<'_>,
    target: &AttachmentTexture,
    window: (u32, u32),
    cursor_visible: bool,
) {
    let shader = ctx
        .shader(SHADER_NAME)
        .expect("called `Option::expect()` on a `None` value");
    if !cursor_visible {
        return;
    }
    let mut pass = ctx.raster_pass(
        PASS_NAME,
        &[ColorSpec::tag0(*target)],
        DepthSpec::None,
        *target,
    );
    let state = pass.host.default_raster_state();
    let packet = DrawPacket::new(shader, [0, 0], state);
    let [w, h] = window_uniform(window.0, window.1);
    let mut builder = DrawBuilder::new(&mut pass, packet, 0)
        .uniform(WINDOW_SIZE_UNIFORM, [w, h])
        .bind_texture_to_draw_builder(
            BindTexture::FromDisk(TextureFromDisk(CURSOR_TEXTURE_FILE)),
            CURSOR_TEXTURE_BINDING,
            ACCESS_SAMPLED_IMAGE,
            None,
        )
        .read_storage_buffer::<CustomCursorSsbo>(CURSOR_SSBO_SLOT, CURSOR_SSBO_BINDING);
    let p = builder.packet_mut();
    p.state = overlay_state(p.state);
    builder.draw_no_vao(CURSOR_VERTEX_COUNT);
    pass.render();
}

#[allow(dead_code)]
fn _unused(_: Handle<crate::render::shader::RasterShaderProgram>, _: ManagedHandle) {}

/// World-owned WindowSize feeds the existing pass without manually built dims.
/// Visibility still belongs to the original GameConfig cursor producer.
pub fn custom_cursor_pass_from_world(
    ctx: &mut DrawPassCtx<'_>,
    target: &AttachmentTexture,
    world: &bevy_ecs::world::World,
    cursor_visible: bool,
) {
    let window = crate::utils::get_res::<crate::resources::window::WindowSize>(world);
    custom_cursor_pass(ctx, target, (window.width, window.height), cursor_visible);
}


/// Render-thread extraction: what this module reads from the World.
pub fn render_extract(m: &mut impl crate::render::extract::RenderExtract) {
    m.resource::<crate::resources::window::WindowSize>();
}
