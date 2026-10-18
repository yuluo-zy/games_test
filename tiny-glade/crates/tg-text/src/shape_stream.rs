//! Typed shape configuration and transactional encoding into shader words.
pub(crate) mod encoding;
use crate::{TextError, TextResult};
use encoding::{
    ARGUMENT_SHIFT, FILL_OPCODE, IMAGE_INDEX_MASK, IMAGE_OPCODE, MAX_SHAPE_WORDS, RADIUS_OPCODE,
    SCALING_SHIFT,
};
use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[repr(u8)]
/// Texture placement mode understood by the text shader. Numeric discriminants are ABI values.
pub enum ImageScaling {
    Stretch = 0,
    FitHeight = 1,
    BlurExtend = 2,
    Nine = 3,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// An image slot or a name resolved when encoding. RON accepts names only; indices are a Rust API input.
pub enum ImageSource {
    Index(u32),
    Name(String),
}
impl From<u32> for ImageSource {
    fn from(value: u32) -> Self {
        Self::Index(value)
    }
}
impl From<&str> for ImageSource {
    fn from(value: &str) -> Self {
        Self::Name(value.into())
    }
}
impl From<String> for ImageSource {
    fn from(value: String) -> Self {
        Self::Name(value)
    }
}

impl<'de> Deserialize<'de> for ImageSource {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer).map(Self::Name)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
/// One drawing operation; colors are RRGGBBAA and radius is in logical units.
/// Encoding rejects image slots above 65535 and radii outside finite `0..=255`.
pub enum ShapeStreamItem {
    FillColor(u32),
    Image {
        source: ImageSource,
        scaling: ImageScaling,
    },
    CornerRadius(f32),
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
#[serde(transparent)]
/// Ordered drawing operations. Construction/deserialization defers validation to encoding.
pub struct ShapeStream(pub Vec<ShapeStreamItem>);

impl ShapeStream {
    /// Create an empty program; an empty rectangle program uses the solid-color path.
    pub fn new() -> Self {
        Self::default()
    }
    /// Append an sRGB fill operation with an RRGGBBAA color.
    pub fn fill_color(mut self, color: u32) -> Self {
        self.0.push(ShapeStreamItem::FillColor(color));
        self
    }
    /// Append an image reference; named sources require a resolver at encoding time.
    pub fn image(mut self, source: impl Into<ImageSource>, scaling: ImageScaling) -> Self {
        self.0.push(ShapeStreamItem::Image {
            source: source.into(),
            scaling,
        });
        self
    }
    /// Append a radius; encoding checks its range and truncates fractional units.
    pub fn corner_radius(mut self, radius: f32) -> Self {
        self.0.push(ShapeStreamItem::CornerRadius(radius));
        self
    }
    /// Whether no drawing operations have been configured.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    /// Encode without a name resolver. Numeric image slots remain supported.
    /// Returns an error for named images, invalid operations or programs over 255 words.
    pub fn encode_no_images(&self) -> TextResult<RawShapeStream> {
        let mut raw = RawShapeStream::new();
        raw.try_append(self, &mut |name| {
            Err(TextError::NamedImageRequiresResolver(name.into()))
        })?;
        Ok(raw)
    }
    /// Parse RON syntax without resolving images or validating numeric ranges.
    /// Call an encoding method before handing data to the renderer.
    pub fn from_ron(text: &str) -> Result<Self, ron::error::SpannedError> {
        ron::from_str(text)
    }
}

/// Packed shader words. Encoding is validated before modifying the destination.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RawShapeStream(pub Vec<u32>);
impl RawShapeStream {
    /// Create an empty word buffer.
    pub fn new() -> Self {
        Self::default()
    }
    /// Borrow encoded words; callers constructing raw values must preserve the shape ABI.
    pub fn words(&self) -> &[u32] {
        &self.0
    }
    /// Remove encoded words while retaining the backing allocation.
    pub fn clear(&mut self) {
        self.0.clear();
    }
    /// Resolve names in source order. An encoding error leaves the destination unchanged.
    /// Resolver side effects are owned by the caller and cannot be rolled back.
    pub fn append(
        &mut self,
        stream: &ShapeStream,
        resolver: &mut impl FnMut(&str) -> u32,
    ) -> TextResult<()> {
        self.try_append(stream, &mut |name| Ok(resolver(name)))
    }
    /// Encode one program and append it only when every operation and resolver call succeeds.
    /// Errors leave the destination unchanged; resolver side effects cannot be rolled back.
    /// The appended program is at most 255 words; a rectangle also limits its full program to 255.
    pub fn try_append(
        &mut self,
        stream: &ShapeStream,
        resolver: &mut impl FnMut(&str) -> TextResult<u32>,
    ) -> TextResult<()> {
        // A bounded staging buffer preserves atomicity without calling a stateful
        // resolver twice, as a validation/encoding two-pass approach would.
        let mut encoded = Vec::with_capacity(stream.0.len().min(MAX_SHAPE_WORDS));
        for item in &stream.0 {
            match item {
                ShapeStreamItem::FillColor(color) => encoded.extend([FILL_OPCODE, *color]),
                ShapeStreamItem::CornerRadius(radius) => {
                    if !radius.is_finite() || !(0.0..=255.0).contains(radius) {
                        return Err(TextError::InvalidValue {
                            field: "corner radius",
                            value: *radius,
                        });
                    }
                    encoded.push(RADIUS_OPCODE | (u32::from(*radius as u8) << ARGUMENT_SHIFT));
                }
                ShapeStreamItem::Image { source, scaling } => {
                    let index = match source {
                        ImageSource::Index(index) => *index,
                        ImageSource::Name(name) => resolver(name)?,
                    };
                    if index > IMAGE_INDEX_MASK {
                        return Err(TextError::ImageIndexOutOfRange(index));
                    }
                    encoded.push(
                        IMAGE_OPCODE
                            | (index << ARGUMENT_SHIFT)
                            | ((*scaling as u32) << SCALING_SHIFT),
                    );
                }
            }
            if encoded.len() > MAX_SHAPE_WORDS {
                return Err(TextError::ShapeStreamTooLong(encoded.len()));
            }
        }
        self.0.extend(encoded);
        Ok(())
    }
}
