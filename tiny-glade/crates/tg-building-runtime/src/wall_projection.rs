//! Borrowed access to wall publication fields absent from the core state API.

/// Read-only view of the serialized publication record; no Rust layout is assumed.
/// Known fields use `PublicWallBuildingState` instead of duplicating its accessors.
pub(crate) struct WallProjection<'a>(&'a [u8; 672]);

impl<'a> WallProjection<'a> {
    pub(crate) fn new(bytes: &'a [u8; 672]) -> Self {
        Self(bytes)
    }

    // The consumer's float is known, but its full business meaning is not.
    pub(crate) fn observed_f32_0x274(&self) -> f32 {
        f32::from_le_bytes(self.0[0x274..0x278].try_into().expect("fixed record field"))
    }

    pub(crate) fn clutter_version(&self) -> u8 {
        self.0[0x295]
    }
}
