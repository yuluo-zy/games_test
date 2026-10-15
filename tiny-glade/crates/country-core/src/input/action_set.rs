//! `action_set.rs` — 动作集（B `input::action_set`）。
//!
//! 证据（`input/ALGORITHMS.md` §1，汇编逐条核对）：
//! - 共 46 个动作：6 个 analog（Vec2）+ 40 个 digital；名字、槽号、串长已逐项与 RNE 串表比对
//!   （第 40 个 digital 槽的名字是 **`cinematic_camera_ctrls`**，22 字符，此前文档写成 21 字符的 `..._ctrl`）；
//! - `DigitalActionState`（8 B，`disc i32 @0` + `val f32 @4`）：`pressed` 0x14093fa70 = `disc < 3`；
//!   `released` 0x14093fa80 = `disc - 3 < 2`（`disc ∈ {3, 4}`）；`just_released` 0x14093fac0 = `disc == 3`；
//!   `just_pressed` 0x14093fa90 = 跳表：`disc 0 → true`、`disc 2 → 0.0 >= val`（NaN 为假）、其余 → 假；
//!   `disc == 4` 是空闲，不会被打包；
//! - `ActionSetAction`（12 B 记录，也是 `deconstruct`/`reconstruct` 的元素）：`kind u8 @0`（0 = Analog、1 = Digital）、
//!   `slot u8 @1`、Analog 的 `Vec2 @4`、Digital 的 `disc u32 @4` + `val f32 @8`；
//!   `just_pressed` 0x1409487b0 / `pressed` 0x1409487e0 / `as_analog` 0x140948800 都先比 `kind` 与 `slot`；
//! - 运行时形态 `MainActionSet<Vec2, DigitalActionState>`（0x170 B）：digital `[8 B; 40]` 在前，analog `[Vec2; 6]` 在后；
//!   serde 形态 `MainActionSet<Vec2, bool>`（0x58 B）字段序相反，两者分别建模；
//! - `deconstruct` 0x140946b80：analog 槽只要不是 `(0, 0)` 就打包（**NaN 也打包**），digital 槽 `disc != 4` 才打包；
//!   `reconstruct` 0x140947c80：全部槽先置 `disc = 4`，再按记录覆盖。
//!
//! 未做：`MainActionSet::just_pressed`（0x140946620）的 46 槽跳表（语义同 `DigitalActionState::just_pressed`，槽号→动作映射未单独核对）。
use glam::Vec2;
use serde::de::{Error as _, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub const ANALOG_SLOTS: usize = 6;
pub const DIGITAL_SLOTS: usize = 40;
/// `deconstruct`/`reconstruct` 的记录大小。
pub const ACTION_RECORD_SIZE: usize = 12;
/// 空闲判别（O：`disc == 4` 不打包）。
pub const DIGITAL_IDLE: u32 = 4;

/// analog 槽名（槽号 = 下标，O）。
pub const ANALOG_NAMES: [&str; ANALOG_SLOTS] = [
    "input_space_cursor_move",
    "hardware_window_space_cursor_move",
    "camera_pan",
    "camera_height",
    "camera_zoom",
    "camera_rotate",
];

/// digital 槽名（槽号 = 下标，O）。
pub const DIGITAL_NAMES: [&str; DIGITAL_SLOTS] = [
    "click",
    "right_click",
    "camera_rotate_mode",
    "camera_pan_mode",
    "tool_building",
    "tool_path",
    "tool_decorator",
    "tool_customize",
    "tool_terrain",
    "tool_eraser",
    "tool_garden",
    "tool_tree",
    "tool_water",
    "photomode",
    "time_of_day",
    "snapping_disable",
    "undo",
    "redo",
    "ui_cancel",
    "ui_confirm",
    "ui_left",
    "ui_right",
    "ui_down",
    "ui_up",
    "ui_tab_prev",
    "ui_tab_next",
    "clear_canvas",
    "main_menu",
    "developer_menu",
    "screenshot",
    "screenshot_grading_reference",
    "screenshot_feedback",
    "hide_ui_photomode",
    "cinematic_camera_ctrls",
    "human_scale_ref_toggle",
    "ivy_growth_toggle",
    "ivy_spawn",
    "camera_snapshot_add",
    "camera_snapshot_restore",
    "toggle_snapping",
];

/// 数字动作状态。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DigitalActionState {
    pub disc: u32,
    pub val: f32,
}

impl Default for DigitalActionState {
    /// `reconstruct` 的默认槽：空闲。
    fn default() -> Self {
        Self {
            disc: DIGITAL_IDLE,
            val: 0.0,
        }
    }
}

impl DigitalActionState {
    /// 0x14093fa70。
    pub fn pressed(self) -> bool {
        self.disc < 3
    }

    /// 0x14093fa80。
    pub fn released(self) -> bool {
        self.disc.wrapping_sub(3) < 2
    }

    /// 0x14093fa90（跳表 0x142ab2c3c）。
    pub fn just_pressed(self) -> bool {
        match self.disc {
            0 => true,
            2 => 0.0 >= self.val,
            _ => false,
        }
    }

    /// 0x14093fac0。
    pub fn just_released(self) -> bool {
        self.disc == 3
    }
}

/// 12 B 动作记录（`deconstruct` 的元素）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ActionSetAction {
    Analog { slot: u8, value: Vec2 },
    Digital { slot: u8, state: DigitalActionState },
}

impl ActionSetAction {
    pub fn to_bytes(self) -> [u8; ACTION_RECORD_SIZE] {
        let mut out = [0u8; ACTION_RECORD_SIZE];
        match self {
            Self::Analog { slot, value } => {
                out[1] = slot;
                out[4..8].copy_from_slice(&value.x.to_le_bytes());
                out[8..12].copy_from_slice(&value.y.to_le_bytes());
            }
            Self::Digital { slot, state } => {
                out[0] = 1;
                out[1] = slot;
                out[4..8].copy_from_slice(&state.disc.to_le_bytes());
                out[8..12].copy_from_slice(&state.val.to_le_bytes());
            }
        }
        out
    }

    pub fn from_bytes(bytes: &[u8; ACTION_RECORD_SIZE]) -> Self {
        let word = |at: usize| u32::from_le_bytes(bytes[at..at + 4].try_into().unwrap());
        if bytes[0] == 0 {
            Self::Analog {
                slot: bytes[1],
                value: Vec2::new(f32::from_bits(word(4)), f32::from_bits(word(8))),
            }
        } else {
            Self::Digital {
                slot: bytes[1],
                state: DigitalActionState {
                    disc: word(4),
                    val: f32::from_bits(word(8)),
                },
            }
        }
    }

    /// 0x1409487b0：`kind == Digital && slot == id` 且状态为 just-pressed。
    pub fn just_pressed(&self, id: u8) -> bool {
        matches!(self, Self::Digital { slot, state } if *slot == id && state.just_pressed())
    }

    /// 0x1409487e0。
    pub fn pressed(&self, id: u8) -> bool {
        matches!(self, Self::Digital { slot, state } if *slot == id && state.disc < 3)
    }

    /// 0x140948800。
    pub fn as_analog(&self, id: u8) -> Option<Vec2> {
        match self {
            Self::Analog { slot, value } if *slot == id => Some(*value),
            _ => None,
        }
    }
}

/// 运行时形态（0x170 B）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MainActionSet {
    pub digital: [DigitalActionState; DIGITAL_SLOTS],
    pub analog: [Vec2; ANALOG_SLOTS],
}

impl Default for MainActionSet {
    fn default() -> Self {
        Self {
            digital: [DigitalActionState::default(); DIGITAL_SLOTS],
            analog: [Vec2::ZERO; ANALOG_SLOTS],
        }
    }
}

impl MainActionSet {
    /// `deconstruct`（0x140946b80）：先 6 个 analog，再 40 个 digital；顺序即槽号序。
    pub fn deconstruct(&self) -> Vec<[u8; ACTION_RECORD_SIZE]> {
        let mut out = Vec::new();
        for (slot, value) in self.analog.iter().enumerate() {
            // 原版 `x != 0 || y != 0 || NaN`，等价于 `!(x == 0 && y == 0)`。
            if value.x != 0.0 || value.y != 0.0 || value.x.is_nan() || value.y.is_nan() {
                out.push(
                    ActionSetAction::Analog {
                        slot: slot as u8,
                        value: *value,
                    }
                    .to_bytes(),
                );
            }
        }
        for (slot, state) in self.digital.iter().enumerate() {
            if state.disc != DIGITAL_IDLE {
                out.push(
                    ActionSetAction::Digital {
                        slot: slot as u8,
                        state: *state,
                    }
                    .to_bytes(),
                );
            }
        }
        out
    }

    /// `reconstruct`（0x140947c80）：全部槽先置默认；槽号越界的记录在原版里落入跳表之外，这里忽略。
    pub fn reconstruct(records: &[[u8; ACTION_RECORD_SIZE]]) -> Self {
        let mut set = Self::default();
        for record in records {
            match ActionSetAction::from_bytes(record) {
                ActionSetAction::Analog { slot, value } => {
                    if let Some(target) = set.analog.get_mut(usize::from(slot)) {
                        *target = value;
                    }
                }
                ActionSetAction::Digital { slot, state } => {
                    if let Some(target) = set.digital.get_mut(usize::from(slot)) {
                        *target = state;
                    }
                }
            }
        }
        set
    }
}

/// serde 形态（0x58 B）：analog 在前，digital 为 bool。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MainActionSetSerde {
    pub analog: [Vec2; ANALOG_SLOTS],
    pub digital: [bool; DIGITAL_SLOTS],
}

impl Default for MainActionSetSerde {
    fn default() -> Self {
        Self {
            analog: [Vec2::ZERO; ANALOG_SLOTS],
            digital: [false; DIGITAL_SLOTS],
        }
    }
}

impl Serialize for MainActionSetSerde {
    /// 46 个槽按表序，只写非默认值（analog: `(0, 0)` 为默认，digital: `false` 为默认；O：0x1400ed690）。
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        for (name, value) in ANALOG_NAMES.iter().zip(&self.analog) {
            if value.x != 0.0 || value.y != 0.0 {
                map.serialize_entry(name, value)?;
            }
        }
        for (name, value) in DIGITAL_NAMES.iter().zip(&self.digital) {
            if *value {
                map.serialize_entry(name, value)?;
            }
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for MainActionSetSerde {
    /// 键名按 46 槽匹配，未知键报错（O：0x1416f92d0 的 unknown 哨兵 0x2e）；缺省槽为默认值。
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct SetVisitor;
        impl<'de> Visitor<'de> for SetVisitor {
            type Value = MainActionSetSerde;
            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("a MainActionSet map")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut set = MainActionSetSerde::default();
                while let Some(key) = map.next_key::<String>()? {
                    if let Some(slot) = ANALOG_NAMES.iter().position(|n| *n == key) {
                        set.analog[slot] = map.next_value()?;
                    } else if let Some(slot) = DIGITAL_NAMES.iter().position(|n| *n == key) {
                        set.digital[slot] = map.next_value()?;
                    } else {
                        return Err(A::Error::unknown_field(&key, &[]));
                    }
                }
                Ok(set)
            }
        }
        deserializer.deserialize_map(SetVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_tables_have_the_original_counts_and_lengths() {
        assert_eq!(ANALOG_NAMES.len() + DIGITAL_NAMES.len(), 46);
        assert_eq!(ANALOG_NAMES[0].len(), 23);
        assert_eq!(ANALOG_NAMES[1].len(), 33);
        assert_eq!(DIGITAL_NAMES[0], "click");
        assert_eq!(DIGITAL_NAMES[33], "cinematic_camera_ctrls");
        assert_eq!(DIGITAL_NAMES[33].len(), 22);
        assert_eq!(DIGITAL_NAMES[30], "screenshot_grading_reference");
        assert_eq!(DIGITAL_NAMES[39], "toggle_snapping");
    }

    #[test]
    fn digital_state_predicates_follow_the_machine_code() {
        let s = |disc, val| DigitalActionState { disc, val };
        for (disc, pressed, released) in [
            (0, true, false),
            (1, true, false),
            (2, true, false),
            (3, false, true),
            (4, false, true),
            (5, false, false),
        ] {
            assert_eq!(s(disc, 0.0).pressed(), pressed, "pressed {disc}");
            assert_eq!(s(disc, 0.0).released(), released, "released {disc}");
        }
        assert!(s(0, 9.0).just_pressed());
        assert!(!s(1, 0.0).just_pressed());
        assert!(s(2, 0.0).just_pressed());
        assert!(s(2, -1.0).just_pressed());
        assert!(!s(2, 0.5).just_pressed());
        assert!(!s(2, f32::NAN).just_pressed());
        assert!(!s(3, 0.0).just_pressed() && !s(4, 0.0).just_pressed());
        assert!(s(3, 0.0).just_released() && !s(4, 0.0).just_released());
    }

    #[test]
    fn default_set_packs_nothing() {
        assert!(MainActionSet::default().deconstruct().is_empty());
        assert_eq!(MainActionSet::reconstruct(&[]), MainActionSet::default());
    }

    #[test]
    fn deconstruct_orders_analog_before_digital_and_packs_nan() {
        let mut set = MainActionSet::default();
        set.digital[1] = DigitalActionState { disc: 0, val: 1.0 };
        set.analog[2] = Vec2::new(0.0, f32::NAN);
        set.analog[0] = Vec2::new(1.0, 0.0);
        let records = set.deconstruct();
        assert_eq!(records.len(), 3);
        assert_eq!(records[0][..2], [0, 0]);
        assert_eq!(records[1][..2], [0, 2]);
        assert_eq!(records[2][..2], [1, 1]);
    }

    #[test]
    fn records_round_trip_through_reconstruct() {
        let mut set = MainActionSet::default();
        set.analog[5] = Vec2::new(0.25, -3.0);
        set.digital[39] = DigitalActionState { disc: 2, val: 0.75 };
        set.digital[0] = DigitalActionState { disc: 3, val: 0.0 };
        assert_eq!(MainActionSet::reconstruct(&set.deconstruct()), set);
    }

    #[test]
    fn action_queries_check_kind_and_slot() {
        let digital = ActionSetAction::Digital {
            slot: 3,
            state: DigitalActionState { disc: 0, val: 0.0 },
        };
        assert!(digital.just_pressed(3) && digital.pressed(3));
        assert!(!digital.just_pressed(4) && !digital.pressed(4));
        assert_eq!(digital.as_analog(3), None);
        let analog = ActionSetAction::Analog {
            slot: 2,
            value: Vec2::new(1.0, 2.0),
        };
        assert_eq!(analog.as_analog(2), Some(Vec2::new(1.0, 2.0)));
        assert_eq!(analog.as_analog(1), None);
        assert!(!analog.pressed(2) && !analog.just_pressed(2));
    }

    #[test]
    fn serde_form_writes_only_non_default_slots_in_table_order() {
        let mut set = MainActionSetSerde::default();
        assert_eq!(serde_json::to_string(&set).unwrap(), "{}");
        set.digital[1] = true;
        set.analog[2] = Vec2::new(1.0, 0.5);
        set.digital[0] = true;
        assert_eq!(
            serde_json::to_string(&set).unwrap(),
            r#"{"camera_pan":[1.0,0.5],"click":true,"right_click":true}"#
        );
    }

    #[test]
    fn serde_form_reads_known_keys_and_rejects_unknown_ones() {
        let set: MainActionSetSerde =
            serde_json::from_str(r#"{"undo":true,"camera_zoom":[0.5,0.0]}"#).unwrap();
        assert!(set.digital[16]);
        assert_eq!(set.analog[4], Vec2::new(0.5, 0.0));
        assert!(serde_json::from_str::<MainActionSetSerde>(r#"{"nope":true}"#).is_err());
    }
}
