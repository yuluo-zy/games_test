use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[repr(u8)]
pub enum ImageScaling {
    Stretch = 0,
    FitHeight = 1,
    BlurExtend = 2,
    Nine = 3,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

// The original PDB inline chain delegates to serde String::deserialize.
// Numeric sources are supplied through the Rust builder, not the RON format.
impl<'de> Deserialize<'de> for ImageSource {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer).map(Self::Name)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
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
pub struct ShapeStream(pub Vec<ShapeStreamItem>);

impl ShapeStream {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn fill_color(mut self, color: u32) -> Self {
        self.0.push(ShapeStreamItem::FillColor(color));
        self
    }
    pub fn image(mut self, source: impl Into<ImageSource>, scaling: ImageScaling) -> Self {
        self.0.push(ShapeStreamItem::Image {
            source: source.into(),
            scaling,
        });
        self
    }
    pub fn corner_radius(mut self, radius: f32) -> Self {
        self.0.push(ShapeStreamItem::CornerRadius(radius));
        self
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn encode_no_images(&self) -> RawShapeStream {
        let mut raw = RawShapeStream::new();
        raw.append(self, &mut |_: &str| -> u32 {
            unreachable!("image name in encode_no_images")
        });
        raw
    }
    pub fn from_ron(text: &str) -> Result<Self, ron::error::SpannedError> {
        ron::from_str(text)
    }
}

/// Raw words are the B GPU-consumer input format; using them does not execute a renderer.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RawShapeStream(pub Vec<u32>);
impl RawShapeStream {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn words(&self) -> &[u32] {
        &self.0
    }
    pub fn clear(&mut self) {
        self.0.clear();
    }
    /// Append preserves the existing prefix, resolves names in order, and never deduplicates.
    pub fn append(&mut self, stream: &ShapeStream, resolver: &mut impl FnMut(&str) -> u32) {
        for item in &stream.0 {
            match item {
                ShapeStreamItem::FillColor(color) => self.0.extend([0, *color]),
                ShapeStreamItem::CornerRadius(radius) => self
                    .0
                    .push(2 | ((radius.clamp(0.0, 255.0) as u8 as u32) << 8)),
                ShapeStreamItem::Image { source, scaling } => {
                    let index = match source {
                        ImageSource::Index(index) => *index,
                        ImageSource::Name(name) => resolver(name),
                    };
                    self.0
                        .push(1 | ((index & 0xffff) << 8) | ((*scaling as u32) << 24));
                }
            }
        }
    }
}
