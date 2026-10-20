
/// All four native tags, directly executed through GCC serializer14085ee90.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StairRailingType {
    #[default]
    None=0,
    Low=1,
    High=2,
    Wooden=3,
}
impl StairRailingType {
    pub fn native_tag(self)->u8 { self as u8 }
}
impl TryFrom<u8> for StairRailingType {
    type Error=u8;
    fn try_from(value:u8)->Result<Self,u8> {
        match value {0=>Ok(Self::None),1=>Ok(Self::Low),2=>Ok(Self::High),3=>Ok(Self::Wooden),_=>Err(value)}
    }
}
