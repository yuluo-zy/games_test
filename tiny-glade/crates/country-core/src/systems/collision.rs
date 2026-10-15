//! `collision.rs` — 碰撞世界的已锚定常量（B `systems::collision::world`）。
//!
//! 证据（`collision-eco/ALGORITHMS.md`）：`SubworldIndex` 共 **17** 个槽（重建里的 16 个常量序错误），其中
//! Terrain 槽 = **5**（掩码 `0x20`、地形命中 `0xb`、映射表三方锚定）；`SubworldIndex::to_bitflag = 1 << 槽号`
//! 在 `iter_shape_overlaps` 族内联使用。`SubworldColliderHandle` 的空值判别是 `0x11`（= 槽数）。
//! 其余 16 个槽的名字与槽号对应关系未锚定（U），这里只提供不依赖名字的 [`SubworldIndex`]。

/// 槽数（O）。
pub const SUBWORLD_SLOT_COUNT: u8 = 17;
/// `SubworldColliderHandle` 的"空"判别（O：0x11）。
pub const EMPTY_COLLIDER_HANDLE_TAG: u8 = 0x11;

/// 子世界槽号。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SubworldIndex(u8);

impl SubworldIndex {
    /// Terrain（O：槽 5，位标志 `0x20`）。
    pub const TERRAIN: Self = Self(5);

    /// 槽号合法（`< 17`）时返回。
    pub fn new(slot: u8) -> Option<Self> {
        (slot < SUBWORLD_SLOT_COUNT).then_some(Self(slot))
    }

    pub fn slot(self) -> u8 {
        self.0
    }

    /// `1 << 槽号`。
    pub fn to_bitflag(self) -> u32 {
        1 << self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terrain_is_slot_five_with_mask_0x20() {
        assert_eq!(SubworldIndex::TERRAIN.slot(), 5);
        assert_eq!(SubworldIndex::TERRAIN.to_bitflag(), 0x20);
    }

    #[test]
    fn there_are_seventeen_slots_and_the_empty_tag_is_the_slot_count() {
        assert!(SubworldIndex::new(16).is_some());
        assert!(SubworldIndex::new(17).is_none());
        assert_eq!(EMPTY_COLLIDER_HANDLE_TAG, SUBWORLD_SLOT_COUNT);
        assert_eq!(SubworldIndex::new(16).unwrap().to_bitflag(), 1 << 16);
    }
}
