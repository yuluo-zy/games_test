//! `foliage_exclusion_mask.rs` — W7 植被排除遮罩光栅（`rasterize_foliage_exclusion_mask` 0x1409cb7b0，2743 B）。
//!
//! O：取资源 R（TypeId 0x1e9ac9b6…，`Option`，有则为楼梯植被排除数据）。逻辑两条路径，共用一个全局脏标志
//! （静态 bool @0x143634cf8）：
//! - **R 缺失**：全局为 0 → 直接返回；为 1 → `AssetTextureLibrary::get_rg_texture(预设 0x10)` → `extent_2d` →
//!   `RasterPass::new_impl("Foliage exclusion"(0x11), [tag1（视图字全 0）= 遮罩纹理], 无深度, 视口 = 遮罩 extent)` →
//!   **不画任何东西直接 `render`（只是清空）** → 全局置 0。
//! - **R 存在**：`get_raster_handle_by_name("stairs_foliage_exclusion"(0x18))`（未命中 `expect`）→ 同样的 pass →
//!   `DrawPacket::new` / `DrawBuilder::new(variant 0)` → `bind_external_storage_buffer(-0x6d6cf4f29e010e92, R 的缓冲)` →
//!   `bind_texture_to_draw_builder(TexturePreset(9), 0x61b08b5e0470c5f2, 0x1d)` →
//!   `bind_texture_to_draw_builder(TexturePreset(0xa), 0x297d1e4cdae8f657, 0x1d)` → `draw_no_vao(R.+0x28)` → `render` → 全局置 1。
//! 全局标志的初值 U（静态区，默认 0）。

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::doc_lazy_continuation)]
use core::sync::atomic::{AtomicBool, Ordering};

use crate::render::pass_builder::draw_builder::{ACCESS_SAMPLED_IMAGE, BindTexture, DrawBuilder};
use crate::render::pass_builder::draw_stream::DrawPacket;
use crate::resources::texture_presets::TexturePreset;
use crate::rhapsody_contract::ManagedHandle;
use crate::systems::render_loop::draw_passes::d_main_support::{ColorSpec, DepthSpec, DrawPassCtx};

pub const PASS_NAME: &str = "Foliage exclusion";
pub const SHADER_NAME: &str = "stairs_foliage_exclusion";
/// 遮罩 RG 纹理预设（O：`mov r8b, 0x10`）。
pub const MASK_PRESET: u8 = 0x10;
pub const STAIRS_BUFFER_BINDING: u64 = 0x9293_0b0d_61fe_f16e; // -0x6d6cf4f29e010e92
pub const PRESET_A: u8 = 9;
pub const PRESET_A_BINDING: u64 = 0x61b0_8b5e_0470_c5f2;
pub const PRESET_B: u8 = 0xa;
pub const PRESET_B_BINDING: u64 = 0x297d_1e4c_dae8_f657;

/// 全局脏标志（O：静态 bool 0x143634cf8）。
pub static FOLIAGE_EXCLUSION_DIRTY: AtomicBool = AtomicBool::new(false);

/// 资源 R 被读取的部分：缓冲句柄与 `+0x28` 的顶点数。
#[derive(Clone, Debug)]
pub struct StairsFoliageExclusion {
    pub buffer: ManagedHandle,
    pub vertex_count: u32,
}

/// `rasterize_foliage_exclusion_mask(ctx, world)`。
pub fn rasterize_foliage_exclusion_mask(
    ctx: &mut DrawPassCtx<'_>,
    stairs: Option<&StairsFoliageExclusion>,
) {
    let Some(stairs) = stairs else {
        if !FOLIAGE_EXCLUSION_DIRTY.load(Ordering::Relaxed) {
            return;
        }
        let mask = ctx.host().rg_texture(MASK_PRESET);
        let pass = ctx.raster_pass(PASS_NAME, &[ColorSpec::tag1(mask)], DepthSpec::None, mask);
        pass.render();
        FOLIAGE_EXCLUSION_DIRTY.store(false, Ordering::Relaxed);
        return;
    };
    let shader = ctx
        .shader(SHADER_NAME)
        .expect("called `Option::expect()` on a `None` value");
    let mask = ctx.host().rg_texture(MASK_PRESET);
    let mut pass = ctx.raster_pass(PASS_NAME, &[ColorSpec::tag1(mask)], DepthSpec::None, mask);
    let state = pass.host.default_raster_state();
    let packet = DrawPacket::new(shader, [1, 0], state);
    DrawBuilder::new(&mut pass, packet, 0)
        .bind_external_storage_buffer(STAIRS_BUFFER_BINDING, &stairs.buffer)
        .bind_texture_to_draw_builder(
            BindTexture::Preset(TexturePreset(PRESET_A)),
            PRESET_A_BINDING,
            ACCESS_SAMPLED_IMAGE,
            None,
        )
        .bind_texture_to_draw_builder(
            BindTexture::Preset(TexturePreset(PRESET_B)),
            PRESET_B_BINDING,
            ACCESS_SAMPLED_IMAGE,
            None,
        )
        .draw_no_vao(stairs.vertex_count);
    pass.render();
    FOLIAGE_EXCLUSION_DIRTY.store(true, Ordering::Relaxed);
}
