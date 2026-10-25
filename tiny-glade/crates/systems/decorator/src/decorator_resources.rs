//! Canonical decorator handle name and its widget hash.
//! B14125d180 copies a u64 and a u8 into the name before calling B14128c8e0.
//! That hash helper writes exactly 8 bytes followed by 1 byte into zero-key
//! std SipHasher13. A string or its 0xff Hash terminator changes the key.
use country_core::systems::ui::resources::ui_state::WidgetId;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct DecoratorHandleName(pub u64, pub u8);

impl DecoratorHandleName {
    pub fn to_widget_name_hash(self) -> u64 {
        utils::calculate_hash(&(self.0, self.1))
    }

    pub fn to_widget_id(self) -> WidgetId {
        WidgetId(self.to_widget_name_hash())
    }
}

pub fn to_widget_name_hash(name: DecoratorHandleName) -> u64 {
    name.to_widget_name_hash()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn original_b_handle_hash_goldens() {
        // Filled from execution of the complete original helper, including
        // its actual std SipHasher writes; no hash implementation is copied.
        for (key, kind, expected) in GOLDENS {
            let name = DecoratorHandleName(key, kind);
            assert_eq!(name.to_widget_name_hash(), expected);
            assert_eq!(name.to_widget_id().0, expected);
        }
    }

    const GOLDENS: [(u64, u8, u64); 25] = [
        (0, 0, 7399894860367327684),
        (0, 1, 14472648492582661910),
        (0, 2, 16546596690377609769),
        (0, 63, 9564325984038645752),
        (0, 255, 9445931171247383011),
        (1, 0, 15877719499815107262),
        (1, 1, 14494360753792957044),
        (1, 2, 16482863132411079682),
        (1, 63, 10049648382212962092),
        (1, 255, 9399151345360787321),
        (18446744073709551615, 0, 9701638580726904287),
        (18446744073709551615, 1, 14471024608294704178),
        (18446744073709551615, 2, 2777855660918435627),
        (18446744073709551615, 63, 13773832207269365677),
        (18446744073709551615, 255, 16317043856278656660),
        (1311768467463790320, 0, 17163709260663748781),
        (1311768467463790320, 1, 7347520083513708897),
        (1311768467463790320, 2, 2130212972957674303),
        (1311768467463790320, 63, 291103655440355526),
        (1311768467463790320, 255, 8401560671301275032),
        (4294967298, 0, 22325340119255437),
        (4294967298, 1, 4981777454904418166),
        (4294967298, 2, 12864333453634692062),
        (4294967298, 63, 9921202787296750855),
        (4294967298, 255, 10841342037072990669),
    ];
}
