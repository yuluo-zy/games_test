
/// Original B GCC/roof wire uses the same two named rim variants.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FlatRoofRimStyle {
    #[default]
    Rim = 0,
    NoRim = 1,
}
impl FlatRoofRimStyle {
    pub fn native_tag(self)->u8 { self as u8 }
}
impl TryFrom<u8> for FlatRoofRimStyle {
    type Error=u8;
    fn try_from(value:u8)->Result<Self,u8> {
        match value {0=>Ok(Self::Rim),1=>Ok(Self::NoRim),_=>Err(value)}
    }
}
