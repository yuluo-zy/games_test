/// Wall identity (B 0x140b34f20): zero is invalid.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct WallId(pub u64);
impl WallId {
    pub fn get_inner(self) -> u64 {
        self.0
    }
    pub fn is_valid(self) -> bool {
        self.0 != 0
    }
}

/// Native `resources::walls::impl$0::fmt` (0x140b34ec0): `"WallId({})"`.
/// Used by the `wall_height_edit_` signifier names and the
/// `"Tried deleting roof {} during Replay"` panic.
impl std::fmt::Display for WallId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WallId({})", self.0)
    }
}

