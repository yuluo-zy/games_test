use serde::Deserialize;
use std::collections::HashMap;

/// Semantic names observed in the B font-table enum visitor and shipped RON.
/// Type name is preserved in B PDB namespace metadata; key variants are confirmed by the enum visitor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[repr(u8)]
pub enum FontFamily {
    Label = 0,
    MenuButton = 1,
    TextEditor = 2,
    Hand = 3,
}

/// Recovered two-f32 metric value held by TextFragment (+0x240/+0x244).
/// The original Rust type name is absent from the retained PDB records.
/// The emitted From<f32> at 0x140b906c0 sets both components to the scalar.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FontMetrics {
    pub font_size: f32,
    pub line_height: f32,
}

impl FontMetrics {
    pub fn new(font_size: f32, line_height: f32) -> Self {
        Self {
            font_size,
            line_height,
        }
    }
}

impl From<f32> for FontMetrics {
    fn from(value: f32) -> Self {
        Self::new(value, value)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct FontTableEntry {
    pub family: String,
    #[serde(default = "default_scale")]
    pub scale: f32,
}

pub fn default_scale() -> f32 {
    1.0
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct FontTable {
    pub fonts: HashMap<FontFamily, FontTableEntry>,
}

impl FontTable {
    pub fn get(&self, role: FontFamily) -> Option<&FontTableEntry> {
        self.fonts.get(&role)
    }
    pub fn from_ron(text: &str) -> Result<Self, ron::error::SpannedError> {
        ron::from_str(text)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScaledFont<'a> {
    pub family: &'a str,
    pub font_size: f32,
    pub line_height: f32,
}

impl FontTableEntry {
    /// 原 0x140b92090：scale 作为第一操作数（SSE mulss 传播 scale 的 NaN 符号），
    /// 两分量均无 clamp。
    pub fn scaled(&self, font_size: f32, line_height: f32) -> ScaledFont<'_> {
        ScaledFont {
            family: &self.family,
            font_size: self.scale * font_size,
            line_height: self.scale * line_height,
        }
    }
}
