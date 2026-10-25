//! raster_single.rs — `raster_single_object` (single-color-target raster pass helper).
//! O/I/U tags per `contracts/raster_single.md`.

/// WP1-owned `pass_builder` boundary. Layout belongs to WP1; WP2 only references the entry points.
pub mod pass_builder_boundary {
    /// O — `ColorAttachment` enum discriminator observed as `2` for the single-attachment case.
    #[derive(Clone, Copy, Debug)]
    pub enum ColorAttachment {
        /// U — exact variant not recovered.
        Color,
    }

    /// WP1 `draw_stream::RasterPass`.
    pub struct RasterPass;
    impl RasterPass {
        pub fn new_impl(_attachments: &[ColorAttachment], _extent: (u32, u32)) -> Self {
            Self
        }
        pub fn render<F: FnOnce(&mut DrawBuilder)>(&mut self, _f: F) {}
    }

    /// WP1 `draw_builder::DrawBuilder`.
    pub struct DrawBuilder;
    impl DrawBuilder {
        pub fn new() -> Self {
            Self
        }
        pub fn bind_texture_to_draw_builder(&mut self, _texture: &dyn AnyTexture) {}
        pub fn draw_no_vao(&mut self, _vertex_count: u32, _instance_count: u32) {}
    }
    pub trait AnyTexture {}
    impl AnyTexture for () {}
}

use pass_builder_boundary::{ColorAttachment, DrawBuilder, RasterPass};

/// O — `raster_single_object<1, [ColorAttachment; 1], CLOSURE>` 0x1409cdec0 / 0x1409ce780 / 0x1409d00a0.
/// Builds a single-color-attachment raster pass over `extent`, binds the pass texture,
/// issues one `draw_no_vao`, and renders. The closure carries the pass-specific parameters.
/// The three monomorphizations (mei_pass / final_post / resolve_to_swapchain) share this shape.
pub fn raster_single_object<F>(
    extent: (u32, u32),
    color_attachment: ColorAttachment,
    _closure: F,
) where
    F: FnOnce(&mut DrawBuilder),
{
    // puffin scope (O) — elided.
    // Framebuffer extent via TextureDesc::extent_2d (O) — elided through the boundary.
    let mut pass = RasterPass::new_impl(&[color_attachment], extent);
    let mut builder = DrawBuilder::new();
    // O: shader::default() + RandomState keys for the hash maps (inside DrawBuilder::new).
    // O: bind_texture_to_draw_builder<TextureFromDisk>(…) then <&Handle<Texture>>(…).
    builder.bind_texture_to_draw_builder(&());
    builder.draw_no_vao(0, 1); // O: draw_no_vao issued; counts are closure-driven (I)
    pass.render(move |b| _closure(b));
}
