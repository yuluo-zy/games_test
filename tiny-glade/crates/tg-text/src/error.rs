//! Recoverable configuration, encoding and rendering errors.
/// CPU input/configuration failures and explicit atlas recovery states.
/// `AtlasChanged` requires a whole-frame rebuild; `AtlasFull` requires changing
/// the working set or budget. Other errors describe rejected input or I/O failures.
#[derive(Debug, thiserror::Error)]
pub enum TextError {
    #[error("invalid text input: {0}")]
    InvalidInput(String),
    #[error("invalid {field}: {value}; expected a finite value in the supported range")]
    InvalidValue { field: &'static str, value: f32 },
    #[error("font role is not configured: {0}")]
    MissingFontRole(String),
    #[error("invalid font configuration: {0}")]
    FontConfig(#[from] ron::error::SpannedError),
    #[error("invalid atlas configuration: {0}")]
    InvalidAtlas(&'static str),
    #[error("glyph atlas budget exhausted")]
    AtlasFull,
    #[error("glyph atlas changed; begin a new frame and rebuild its primitives")]
    AtlasChanged,
    #[error("image index {0} exceeds the 16-bit shape encoding range")]
    ImageIndexOutOfRange(u32),
    #[error("named image requires a resolver: {0}")]
    NamedImageRequiresResolver(String),
    #[error("invalid shape stream: {0}")]
    InvalidShape(&'static str),
    #[error("shape stream has {0} words; a primitive supports at most 255")]
    ShapeStreamTooLong(usize),
    #[error("invalid render batch: {0}")]
    InvalidBatch(String),
    #[error("text I/O failed: {0}")]
    Io(#[from] std::io::Error),
}

/// Result returned by validated CPU text operations.
pub type TextResult<T> = Result<T, TextError>;
