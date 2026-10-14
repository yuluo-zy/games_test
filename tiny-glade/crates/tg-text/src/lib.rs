//! B tg-text 行为候选：字体角色、编辑/整形、字形 atlas、48B primitive 和 shape 流。
//! 原 Cosmic/etagere 处理通用算法；可选 gpu 适配复用 rhapsody 和发行 SPIR-V。
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
    AtlasAllocUsage, AtlasDescriptor, Glyph, PendingGlyphUpload, Primitive, RenderBatch,
    TextRenderStream, TextRenderer, TextStats, init_font_system,
};

#[cfg(feature = "gpu")]
pub mod gpu;

pub mod color;
pub use color::scale_u32_packed_srgba_color;
