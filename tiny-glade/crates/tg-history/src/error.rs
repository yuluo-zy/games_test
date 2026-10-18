use std::{error::Error, fmt};

/// A document structure failure or an error from the JSON codec.
#[derive(Debug)]
pub enum DecodeErrorKind {
    /// A missing or malformed field, with the expected type or shape.
    Expected(&'static str),
    /// The original JSON syntax, typed-decoding, or serialization error.
    Json(serde_json::Error),
}

/// A decoding failure with an RFC 6901 JSON Pointer and the original cause.
/// The empty pointer identifies the complete document. The error owns its
/// location and cause, so callers can retain it after dropping the document.
#[derive(Debug)]
pub struct DecodeError {
    path: String,
    kind: DecodeErrorKind,
}

impl DecodeError {
    /// Borrows the failing value's RFC 6901 pointer, or `""` for the document.
    /// Typed roof errors identify the `/Roof` payload containing the failure.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Borrows the failure classification without discarding codec details.
    /// JSON failures also expose their original cause through [`Error::source`].
    pub fn kind(&self) -> &DecodeErrorKind {
        &self.kind
    }

    pub(crate) fn expected(path: impl Into<String>, expected: &'static str) -> Self {
        Self {
            path: path.into(),
            kind: DecodeErrorKind::Expected(expected),
        }
    }

    pub(crate) fn json(path: impl Into<String>, source: serde_json::Error) -> Self {
        Self {
            path: path.into(),
            kind: DecodeErrorKind::Json(source),
        }
    }
}

impl fmt::Display for DecodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let path = if self.path.is_empty() {
            "<document>"
        } else {
            &self.path
        };
        match &self.kind {
            DecodeErrorKind::Expected(expected) => write!(formatter, "{path}: expected {expected}"),
            DecodeErrorKind::Json(source) => write!(formatter, "{path}: {source}"),
        }
    }
}

impl Error for DecodeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.kind {
            DecodeErrorKind::Expected(_) => None,
            DecodeErrorKind::Json(source) => Some(source),
        }
    }
}

pub(crate) fn escape_pointer_segment(segment: &str) -> String {
    segment.replace('~', "~0").replace('/', "~1")
}
