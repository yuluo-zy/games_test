//! Observed session theme predicates. Full session/save state is not reconstructed here.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GladeType {
    Olden,
    Flowery,
    Autumn,
    Winter,
    Raw(u8),
}
impl GladeType {
    /// B 0x140a9fa70, discriminant 1.
    pub fn is_olden(self) -> bool {
        matches!(self, Self::Olden | Self::Raw(1))
    }
    /// B 0x14092e6a0, discriminant 2.
    pub fn is_flowery(self) -> bool {
        matches!(self, Self::Flowery | Self::Raw(2))
    }
    /// B 0x14092e6c0, discriminant 3.
    pub fn is_autumn(self) -> bool {
        matches!(self, Self::Autumn | Self::Raw(3))
    }
    /// B 0x140ae8320, discriminant 4.
    pub fn is_winter(self) -> bool {
        matches!(self, Self::Winter | Self::Raw(4))
    }
}
impl From<u8> for GladeType {
    fn from(tag: u8) -> Self {
        match tag {
            1 => Self::Olden,
            2 => Self::Flowery,
            3 => Self::Autumn,
            4 => Self::Winter,
            raw => Self::Raw(raw),
        }
    }
}
