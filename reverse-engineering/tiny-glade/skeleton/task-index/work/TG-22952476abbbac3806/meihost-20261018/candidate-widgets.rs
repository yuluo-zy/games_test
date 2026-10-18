//! B LabelContent / FrameContent, identified through their actual content
//! vtables. See task-index work TG-22952476abbbac3806/meihost-20261018.
//! Public fields carry caller-produced text/shapes; no placeholder UI is made.
use crate::mei::{
    core::{NodeContent, NodeDrawContext, NodePrepareContext},
    layout::LengthPercentage,
};
use tg_text::{RawShapeStream, ShapeStream, TextFragment};

/// Native 0x260-byte owner: 600-byte TextFragment and packed color @0x258.
pub struct LabelContent {
    pub text: TextFragment,
    pub color: u32,
    /// Source adapter: this tg-text API needs the renderer when setting text.
    /// Apply caller-produced text at the original prepare boundary.
    pub pending_text: Option<String>,
}

impl NodeContent for LabelContent {
    fn type_name(&self) -> &'static str {
        "toolbar::mei::widgets::LabelContent"
    }

    /// B 0x14067e510: prepare actual glyphs and return absolute text bounds.
    fn prepare_layout(
        &mut self,
        ctx: &mut NodePrepareContext<'_>,
    ) -> Option<[LengthPercentage; 2]> {
        if let Some(text) = self.pending_text.take() {
            self.text.set_text(ctx.text, text).expect("set label text");
        }
        self.text.prepare(ctx.text).expect("prepare label text");
        Some(self.text.bounds().map(|value| LengthPercentage {
            is_percent: false,
            value,
        }))
    }

    /// B 0x14067e560: tint is the original packed color scaled by opacity.
    fn draw(&mut self, ctx: &mut NodeDrawContext<'_>) {
        let tint = tg_text::scale_u32_packed_srgba_color(self.color, ctx.opacity);
        self.text
            .render(ctx.text, ctx.center.to_array(), ctx.extent.to_array(), tint)
            .expect("render label text");
    }
}

/// Native two-Vec owner: original ShapeStream followed by cached encoded words.
#[derive(Default)]
pub struct FrameContent {
    pub shapes: ShapeStream,
    pub raw_shapes: RawShapeStream,
}

impl NodeContent for FrameContent {
    fn type_name(&self) -> &'static str {
        "toolbar::mei::widgets::FrameContent"
    }

    /// B 0x140680540: clear → append, using the real texture descriptor index.
    fn prepare_layout(
        &mut self,
        ctx: &mut NodePrepareContext<'_>,
    ) -> Option<[LengthPercentage; 2]> {
        self.raw_shapes.clear();
        self.raw_shapes
            .append(&self.shapes, &mut |name| {
                ctx.textures.image_name_to_index(name).0
            })
            .expect("encode frame shape stream");
        Some([LengthPercentage::default(); 2])
    }

    /// B 0x140680590: an empty ShapeStream emits nothing; rect accepts origin.
    fn draw(&mut self, ctx: &mut NodeDrawContext<'_>) {
        if !self.shapes.is_empty() {
            let origin = ctx.center - ctx.extent * 0.5;
            ctx.text
                .rect(
                    origin.to_array(),
                    ctx.extent.to_array(),
                    ctx.opacity,
                    &self.raw_shapes,
                )
                .expect("render frame shape stream");
        }
    }
}

/// B get_content_mut_impl initializes the original concrete content using the
/// shared font table. This is a source-level ownership adapter, not a vtable ABI.
pub trait DefaultNodeContent: NodeContent + Sized {
    fn from_font_table(fonts: std::sync::Arc<tg_text::FontTable>) -> Self;
}
impl DefaultNodeContent for FrameContent {
    fn from_font_table(_fonts: std::sync::Arc<tg_text::FontTable>) -> Self {
        Self::default()
    }
}
impl DefaultNodeContent for LabelContent {
    /// Original140f9da6a uses48.0 and role0(Label), then white packed color.
    fn from_font_table(fonts: std::sync::Arc<tg_text::FontTable>) -> Self {
        Self {
            text: TextFragment::new(48., tg_text::FontFamily::Label, fonts)
                .expect("MEI label font role"),
            color: u32::MAX,
            pending_text: None,
        }
    }
}
impl LabelContent {
    pub fn set_text(&mut self, text: String) {
        self.pending_text = Some(text);
    }
}

/// B ImageContent (vtable142baf630); the named-asset path used by the startup
/// splash. `file:` cache/upload is a separate original branch, not implemented
/// by this named-asset adapter.
pub struct ImageContent {
    pub image: String,
    pub height: f32,
    pub fit_parent_height: bool,
    pub stretch_width: bool,
    pub color: u32,
    pub size: glam::Vec2,
    pub raw_shapes: RawShapeStream,
}
impl DefaultNodeContent for ImageContent {
    fn from_font_table(_fonts: std::sync::Arc<tg_text::FontTable>) -> Self {
        Self {
            image: String::new(),
            height: 0.,
            fit_parent_height: false,
            stretch_width: false,
            color: u32::MAX,
            size: glam::Vec2::ZERO,
            raw_shapes: RawShapeStream::new(),
        }
    }
}
impl NodeContent for ImageContent {
    fn type_name(&self) -> &'static str {
        "toolbar::mei::widgets::ImageContent"
    }
    fn prepare_layout(
        &mut self,
        ctx: &mut NodePrepareContext<'_>,
    ) -> Option<[LengthPercentage; 2]> {
        assert!(
            !self.image.starts_with("file:"),
            "MEI file-image upload/cache is not connected to the production TextureHost"
        );
        let (handle, texture) = ctx
            .textures
            .get_handle_and_texture_by_name(country_core::name_hash::NameHash::compute(&self.image))
            .unwrap_or_else(|| panic!("MEI image texture is missing: {}", self.image));
        let extent = glam::Vec2::new(texture.width as f32, texture.height as f32);
        let index = ctx.textures.handle_to_bindless_index(handle).0;
        self.size = extent * (self.height / extent.y);
        let scaling = if self.stretch_width {
            tg_text::ImageScaling::BlurExtend
        } else {
            tg_text::ImageScaling::Stretch
        };
        let shapes = ShapeStream::new()
            .image(index, scaling)
            .fill_color(self.color);
        self.raw_shapes.clear();
        self.raw_shapes
            .append(&shapes, &mut |_| {
                unreachable!("image index already resolved")
            })
            .expect("encode MEI image");
        Some(if self.fit_parent_height {
            [LengthPercentage::default(); 2]
        } else {
            self.size.to_array().map(LengthPercentage::absolute)
        })
    }
    fn draw(&mut self, ctx: &mut NodeDrawContext<'_>) {
        let mut size = self.size;
        if self.fit_parent_height {
            size *= ctx.extent.y / size.y.max(1.);
        }
        if self.stretch_width {
            size.x = ctx.extent.x;
        }
        ctx.text
            .rect(
                (ctx.center - size * 0.5).to_array(),
                size.to_array(),
                ctx.opacity,
                &self.raw_shapes,
            )
            .expect("draw MEI image");
    }
}
impl crate::mei::core::NodeContext {
    /// Original child<&str>: hash with the native u64 TypeId high word.
    pub fn child(&self, name: &str) -> Self {
        let value = crate::mei::core::calculate_hash(&(name, ()));
        let local = crate::mei::core::calculate_hash(&(value, 0xa79b7268a2a968d9u64));
        self.child_with_global_id(crate::mei::core::GlobalId(
            crate::mei::core::calculate_hash(&(self.id.0, local)),
        ))
    }
    /// Original image<&str>140f9fca0 creates a distinct image-content identity.
    pub fn image(&self, name: &str, image: &str, height: f32) -> Self {
        let value = crate::mei::core::calculate_hash(&(name, ()));
        let local = crate::mei::core::calculate_hash(&(value, 0x01f6d60c8858b8ffu64));
        let child = self.child_with_global_id(crate::mei::core::GlobalId(
            crate::mei::core::calculate_hash(&(self.id.0, local)),
        ));
        let content = child.get_content_mut_impl::<ImageContent>(false);
        content.image = image.into();
        content.height = height;
        content.fit_parent_height = false;
        child
    }
}
