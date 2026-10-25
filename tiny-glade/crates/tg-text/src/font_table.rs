//! Validated font-role configuration and metric scaling for text fragments.
use crate::{TextError, TextResult};
use serde::Deserialize;
use std::collections::HashMap;

/// Semantic font roles used by interface components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[repr(u8)]
pub enum FontFamily {
    Label = 0,
    MenuButton = 1,
    TextEditor = 2,
    Hand = 3,
}

/// Unscaled font size and line height. Fragment and scaling entry points
/// validate these values before passing them to the shaping engine.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FontMetrics {
    /// Unscaled glyph size; validated entry points require a finite positive value.
    pub font_size: f32,
    /// Unscaled baseline spacing; must be finite and positive when used.
    pub line_height: f32,
}

impl FontMetrics {
    /// Reject nonpositive or nonfinite components without modifying the value.
    pub fn validate(self) -> TextResult<()> {
        if !self.font_size.is_finite()
            || self.font_size <= 0.
            || !self.line_height.is_finite()
            || self.line_height <= 0.
        {
            return Err(TextError::InvalidInput(
                "font metrics must be finite and positive".into(),
            ));
        }
        Ok(())
    }
    /// Store unscaled values. Construction alone does not validate them;
    /// fragment and scaling entry points validate before shaping.
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

/// A font-family match name and multiplier applied to both metric components.
/// Deserialization stores values; call validate when bypassing FontTable::from_ron.
#[derive(Debug, Clone, Deserialize)]
pub struct FontTableEntry {
    /// Name matched against fonts explicitly loaded into TextRenderer.
    pub family: String,
    /// Finite positive metric multiplier; omitted RON values default to 1.
    #[serde(default = "default_scale")]
    pub scale: f32,
}

/// Default multiplier used when a font entry omits its scale.
pub fn default_scale() -> f32 {
    1.0
}

/// Role mappings shared by fragments. A table may omit unused roles.
/// Direct Deserialize does not validate; from_ron validates all configured entries.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct FontTable {
    /// Configured roles; a missing role becomes an error when a fragment selects it.
    pub fonts: HashMap<FontFamily, FontTableEntry>,
}

impl FontTable {
    /// Look up a role without requiring its presence or validating its entry.
    pub fn get(&self, role: FontFamily) -> Option<&FontTableEntry> {
        self.fonts.get(&role)
    }
    /// Borrow an entry or return MissingFontRole; the entry is not validated here.
    pub fn require(&self, role: FontFamily) -> TextResult<&FontTableEntry> {
        self.get(role)
            .ok_or_else(|| TextError::MissingFontRole(format!("{role:?}")))
    }
    /// Validate all configured entries; an empty or partial table is allowed.
    pub fn validate(&self) -> TextResult<()> {
        for entry in self.fonts.values() {
            entry.validate()?;
        }
        Ok(())
    }
    /// Parse RON and validate every configured family/scale before returning it.
    /// Syntax errors become FontConfig; invalid entries return InvalidInput.
    pub fn from_ron(text: &str) -> TextResult<Self> {
        let table: Self = ron::from_str(text).map_err(TextError::from)?;
        table.validate()?;
        Ok(table)
    }
}

/// Scaled metrics borrowing their family name; `FontTableEntry::scaled` validates its output.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScaledFont<'a> {
    pub family: &'a str,
    pub font_size: f32,
    pub line_height: f32,
}

impl FontTableEntry {
    /// Require a nonblank family name and a finite positive multiplier.
    /// This does not prove that matching font bytes have been loaded.
    pub fn validate(&self) -> TextResult<()> {
        if self.family.trim().is_empty() || !self.scale.is_finite() || self.scale <= 0. {
            return Err(TextError::InvalidInput(
                "font family must be nonempty and scale finite and positive".into(),
            ));
        }
        Ok(())
    }
    /// Validate this entry and both input metrics, then apply its multiplier.
    /// Rejects overflow or underflow to zero; the returned family borrows self.
    pub fn scaled(&self, font_size: f32, line_height: f32) -> TextResult<ScaledFont<'_>> {
        self.validate()?;
        FontMetrics::new(font_size, line_height).validate()?;
        let metrics = FontMetrics::new(self.scale * font_size, self.scale * line_height);
        metrics.validate()?;
        Ok(ScaledFont {
            family: &self.family,
            font_size: metrics.font_size,
            line_height: metrics.line_height,
        })
    }
}
