//! B brightness selector, 0x140b06770. Variant spellings are not available;
//! preserve the three observed tags rather than inventing a Custom(f32) variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorMenuBrightnessValue(u8);

impl Default for ColorMenuBrightnessValue {
    fn default() -> Self {
        Self(1)
    }
}

impl ColorMenuBrightnessValue {
    pub fn from_tag(tag: u8) -> Option<Self> {
        (tag < 3).then_some(Self(tag))
    }

    pub fn tag(self) -> u8 {
        self.0
    }

    pub fn to_f32(self) -> f32 {
        [0.25, 1.0, 4.0][usize::from(self.0)]
    }

    pub fn default_f32() -> f32 {
        1.0
    }
}
