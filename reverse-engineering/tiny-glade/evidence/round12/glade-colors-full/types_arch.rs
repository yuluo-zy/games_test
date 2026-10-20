
/// Original B GCC serializer141584b90 and its field visitor use this enum.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ArchType {
    #[default]
    FollowTerrain = 0,
    ConstantMaxHeight = 1,
}
impl ArchType {
    pub fn native_tag(self) -> u8 { self as u8 }
}
impl TryFrom<u8> for ArchType {
    type Error=u8;
    fn try_from(value:u8)->Result<Self,u8> {
        match value { 0=>Ok(Self::FollowTerrain), 1=>Ok(Self::ConstantMaxHeight), _=>Err(value) }
    }
}

impl serde::Serialize for WallStyle {
    fn serialize<S:serde::Serializer>(&self, serializer:S)->Result<S::Ok,S::Error> {
        match self.0 {
            0=>serializer.serialize_unit_variant("WallStyle",0,"Stone"),
            1=>serializer.serialize_unit_variant("WallStyle",1,"StoneWithSkirts"),
            2=>serializer.serialize_unit_variant("WallStyle",2,"HalfTimbered"),
            invalid=>Err(serde::ser::Error::custom(format_args!("invalid native WallStyle {invalid}"))),
        }
    }
}
