use serde::Deserialize;
use std::collections::HashMap;

/// Semantic names observed in the B font-table enum visitor and shipped RON.
/// Type name is preserved in B PDB namespace metadata; key variants are confirmed by the enum visitor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub enum FontFamily {
    Label,
    MenuButton,
    TextEditor,
    Hand,
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
