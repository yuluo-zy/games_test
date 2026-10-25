#![doc = include_str!("../README.md")]
pub mod error;
pub use error::{TextError, TextResult};
pub mod font_table;
pub mod shape_stream;
pub use font_table::{FontFamily, FontMetrics, FontTable, FontTableEntry, ScaledFont};
pub use shape_stream::{ImageScaling, ImageSource, RawShapeStream, ShapeStream, ShapeStreamItem};

pub mod fragment;
pub mod renderer;
pub use cosmic_text::{
    Action, Align, Color, Cursor, Edit, Editor, Metrics, Motion, Selection, Weight,
};
pub use fragment::TextFragment;
pub use renderer::{
    AtlasAllocUsage, AtlasConfig, AtlasDescriptor, Glyph, PendingGlyphUpload, Primitive,
    RenderBatch, TextRenderStream, TextRenderer, TextStats, init_font_system,
};
pub mod validation;

#[cfg(feature = "gpu")]
pub mod gpu;

pub mod color;
pub use color::scale_u32_packed_srgba_color;
