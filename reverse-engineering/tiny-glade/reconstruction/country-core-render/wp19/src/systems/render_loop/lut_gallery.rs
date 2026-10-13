//! `lut_gallery.rs` — photomode LUT gallery thumbnails.
//!
//! Business logic (contracts/lut.md §3):
//!   - `LutGalleryPlugin::build` 0x1408b04c0: alloc `LutGalleryState` (default
//!     layout O) + `insert_resource<LutGalleryState>()` + PreStartup `startup`.
//!   - `render_photomode_lut_gallery` 0x1408b0560: demo-mode guard; AppMode==10
//!     (photography) + byte@+0x08==1; lock GradingLuts + LutGalleryState; for each
//!     grading LUT draw an icon+grading-lut textured quad with
//!     `lut_gallery_thumbnail` shader (O string); grid positions from
//!     `resources::gui::impl$6::positions`.

use crate::consume::pass_builder::{DrawBuilder, RasterPass};
use crate::consume::core::{AssetShaderLibrary, AssetTextureLibrary, GradingLuts};
use crate::typed_gap::Gap;

/// LUT 画廊 bevy Plugin。
pub struct LutGalleryPlugin(pub Gap);

impl LutGalleryPlugin {
    /// O：0x1408b04c0（`impl$0::build`）。
    /// `insert_resource<LutGalleryState>()`（默认布局 O）+
    /// `add_systems<PreStartup, startup>`（签名 `(Res<GraphicsDeviceResource>,
    /// Res<AssetShaderLibrary>, Res<ShaderWatch>)`）。
    pub fn build(&mut self) {
        // *alloc = {+0x00=1, +0x08=1, +0x10=0, +0x14=0, +0x18..+0x30=0,
        //          +0x35=0, +0x3d=0}  (LutGalleryState, O offsets)
        // insert_resource<LutGalleryState>(); add_systems<PreStartup, startup>
        unimplemented!()
    }
}

/// O：`startup`（inline 宿主，行 41）——签名同 painterly/vermeer startup；
/// `load_shader_into_library`（I：`lut_gallery_thumbnail`）。
pub fn startup(_device: &Gap, _shaders: &AssetShaderLibrary, _watch: &Gap) {
    unimplemented!()
}

/// `render_photomode_lut_gallery::PreviewParams`（O 名称；字段 U）。
pub struct PreviewParams(pub Gap);

/// O：0x1408b0560。摄影模式 LUT 画廊系统。
pub fn render_photomode_lut_gallery(world: &mut Gap) {
    // O:
    // 1. _DEV_OVERRIDE_IS_DEMO_MODE != 0 → return
    // 2. Res<GradingLuts>(hash 0xaca0d6d443d3ae2e) + Res<AppMode>(hash
    //    0xd38dae617fb79c7a): state==10 && byte@+0x08==1 gate
    // 3. lock GradingLuts (+0x4e==7 guard); AssetTextureLibrary(hash
    //    0x4a142e09642375b9) name-map lookups (icon / LUT textures)
    // 4. get_raster_handle_by_name("lut_gallery_thumbnail")
    // 5. Res(hash 0xd387f1aea6230aea); from_iter<Vec2, Map<Range<u32>,
    //    gui::positions::closure_env_0>> → grid positions
    // 6. per cell DrawBuilder: bind icon texture + 2×TextureFromDisk (incl.
    //    NameHash::compute) + optional_uniform<[u32;2]> + uniforms
    //    f32/Vec2/Vec2/bool/bool/bool → draw_no_vao
    // 7. RasterPass::render
    let _ = world;
    unimplemented!()
}

/// `render_photomode_lut_gallery::closure$0`（inline @100，O）：单格绘制闭包。
pub fn lut_cell_closure(_cell: u32) {
    unimplemented!()
}
