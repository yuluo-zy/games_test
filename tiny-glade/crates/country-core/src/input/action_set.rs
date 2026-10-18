//! 固定槽位动作集：设备值、按键状态和可校验的稀疏记录。
//!
//! 热路径使用定长数组；序列化直接写记录，不为每帧创建中间 Vec。
use glam::Vec2;
use serde::de::{Error as _, MapAccess, Visitor};
use serde::ser::{SerializeMap, SerializeSeq};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub const ANALOG_SLOTS: usize = 6;
pub const DIGITAL_SLOTS: usize = 40;
pub const ACTION_RECORD_SIZE: usize = 12;

/// 槽号就是下标；名字顺序也是文件协议的一部分。
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

/// 按键生命周期。使用枚举防止无效整数进入运行时状态。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u32)]
pub enum DigitalActionPhase {
    JustPressed = 0,
    Held = 1,
    Repeating = 2,
    JustReleased = 3,
    #[default]
    Idle = 4,
}

/// 持续状态记录已按住的时间；连发状态记录距离下一次触发的时间。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DigitalActionState {
    pub phase: DigitalActionPhase,
    pub timer_seconds: f32,
}

impl DigitalActionState {
    pub fn pressed(self) -> bool {
        matches!(
            self.phase,
            DigitalActionPhase::JustPressed
                | DigitalActionPhase::Held
                | DigitalActionPhase::Repeating
        )
    }
    pub fn released(self) -> bool {
        !self.pressed()
    }
    pub fn just_pressed(self) -> bool {
        self.phase == DigitalActionPhase::JustPressed
            || (self.phase == DigitalActionPhase::Repeating && self.timer_seconds <= 0.0)
    }
    pub fn just_released(self) -> bool {
        self.phase == DigitalActionPhase::JustReleased
    }
}

/// 一条动作记录。字节协议固定为 kind、slot、两字节保留位和八字节载荷。
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ActionSetAction {
    Analog { slot: u8, value: Vec2 },
    Digital { slot: u8, state: DigitalActionState },
}

impl ActionSetAction {
    pub fn to_bytes(self) -> [u8; ACTION_RECORD_SIZE] {
        let mut bytes = [0; ACTION_RECORD_SIZE];
        let (slot, first_word, second_word) = match self {
            Self::Analog { slot, value } => (slot, value.x.to_bits(), value.y.to_bits()),
            Self::Digital { slot, state } => {
                bytes[0] = 1;
                (slot, state.phase as u32, state.timer_seconds.to_bits())
            }
        };
        bytes[1] = slot;
        bytes[4..8].copy_from_slice(&first_word.to_le_bytes());
        bytes[8..12].copy_from_slice(&second_word.to_le_bytes());
        bytes
    }

    /// 文件入口严格校验，不把未知 kind 当数字动作，不忽略越界槽。
    pub fn from_bytes(bytes: &[u8; ACTION_RECORD_SIZE]) -> anyhow::Result<Self> {
        anyhow::ensure!(bytes[2..4] == [0, 0], "动作记录保留位必须为零");
        let first = u32::from_le_bytes(bytes[4..8].try_into()?);
        let second = f32::from_le_bytes(bytes[8..12].try_into()?);
        let slot = bytes[1];
        match bytes[0] {
            0 => {
                let value = Vec2::new(f32::from_bits(first), second);
                anyhow::ensure!(usize::from(slot) < ANALOG_SLOTS, "模拟动作槽越界：{slot}");
                anyhow::ensure!(value.is_finite(), "模拟动作必须为有限值");
                Ok(Self::Analog { slot, value })
            }
            1 => {
                anyhow::ensure!(usize::from(slot) < DIGITAL_SLOTS, "数字动作槽越界：{slot}");
                anyhow::ensure!(second.is_finite(), "按键计时必须为有限值");
                let phase = match first {
                    0 => DigitalActionPhase::JustPressed,
                    1 => DigitalActionPhase::Held,
                    2 => DigitalActionPhase::Repeating,
                    3 => DigitalActionPhase::JustReleased,
                    4 => DigitalActionPhase::Idle,
                    _ => anyhow::bail!("未知按键状态：{first}"),
                };
                Ok(Self::Digital {
                    slot,
                    state: DigitalActionState {
                        phase,
                        timer_seconds: second,
                    },
                })
            }
            kind => anyhow::bail!("未知动作类型：{kind}"),
        }
    }
    pub fn just_pressed(&self, id: u8) -> bool {
        matches!(self, Self::Digital {slot, state} if *slot == id && state.just_pressed())
    }
    pub fn pressed(&self, id: u8) -> bool {
        matches!(self, Self::Digital {slot, state} if *slot == id && state.pressed())
    }
    pub fn as_analog(&self, id: u8) -> Option<Vec2> {
        match self {
            Self::Analog { slot, value } if *slot == id => Some(*value),
            _ => None,
        }
    }
}

/// 游戏实际消费的唯一动作状态；实时输入和回放都写入这里。
#[derive(bevy_ecs::resource::Resource, Clone, Copy, Debug, PartialEq)]
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
    /// 按模拟槽、数字槽顺序生成记录；迭代器不分配堆内存。
    fn records(&self) -> impl Iterator<Item = ActionSetAction> + '_ {
        self.analog
            .iter()
            .enumerate()
            .filter(|(_, value)| **value != Vec2::ZERO)
            .map(|(slot, value)| ActionSetAction::Analog {
                slot: slot as u8,
                value: *value,
            })
            .chain(
                self.digital
                    .iter()
                    .enumerate()
                    .filter(|(_, state)| **state != DigitalActionState::default())
                    .map(|(slot, state)| ActionSetAction::Digital {
                        slot: slot as u8,
                        state: *state,
                    }),
            )
    }
    pub fn deconstruct_into(&self, records: &mut Vec<[u8; ACTION_RECORD_SIZE]>) {
        records.clear();
        records.extend(self.records().map(ActionSetAction::to_bytes));
    }
    pub fn deconstruct(&self) -> Vec<[u8; ACTION_RECORD_SIZE]> {
        self.records().map(ActionSetAction::to_bytes).collect()
    }
    pub fn reconstruct(records: &[[u8; ACTION_RECORD_SIZE]]) -> anyhow::Result<Self> {
        anyhow::ensure!(
            records.len() <= ANALOG_SLOTS + DIGITAL_SLOTS,
            "动作记录数超过槽位数量"
        );
        let mut actions = Self::default();
        let mut seen = [false; ANALOG_SLOTS + DIGITAL_SLOTS];
        for record in records {
            let action = ActionSetAction::from_bytes(record)?;
            let index = match action {
                ActionSetAction::Analog { slot, .. } => usize::from(slot),
                ActionSetAction::Digital { slot, .. } => ANALOG_SLOTS + usize::from(slot),
            };
            anyhow::ensure!(!seen[index], "动作槽重复：{index}");
            seen[index] = true;
            match action {
                ActionSetAction::Analog { slot, value } => {
                    actions.analog[usize::from(slot)] = value
                }
                ActionSetAction::Digital { slot, state } => {
                    actions.digital[usize::from(slot)] = state
                }
            }
        }
        Ok(actions)
    }
    pub fn values(&self) -> MainActionSetSerde {
        MainActionSetSerde {
            analog: self.analog,
            digital: self.digital.map(DigitalActionState::pressed),
        }
    }
    pub fn is_finite(&self) -> bool {
        self.analog.iter().all(|value| value.is_finite())
            && self
                .digital
                .iter()
                .all(|state| state.timer_seconds.is_finite())
    }
}
impl Serialize for MainActionSet {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if !self.is_finite() {
            return Err(serde::ser::Error::custom("动作包含非有限值"));
        }
        let mut sequence = serializer.serialize_seq(Some(self.records().count()))?;
        for action in self.records() {
            sequence.serialize_element(&action.to_bytes())?;
        }
        sequence.end()
    }
}
impl<'de> Deserialize<'de> for MainActionSet {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let records = Vec::<[u8; ACTION_RECORD_SIZE]>::deserialize(deserializer)?;
        Self::reconstruct(&records).map_err(D::Error::custom)
    }
}

/// 设备这一帧提供的值；数字槽尚未计算按下沿或连发。
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
#[derive(bevy_ecs::resource::Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct ActionSetValues {
    pub previous: MainActionSetSerde,
    pub current: MainActionSetSerde,
}
pub type ActionSetStates = MainActionSet;

impl Serialize for MainActionSetSerde {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if self.analog.iter().any(|value| !value.is_finite()) {
            return Err(serde::ser::Error::custom("模拟动作必须为有限值"));
        }
        let mut map = serializer.serialize_map(None)?;
        for (name, value) in ANALOG_NAMES.iter().zip(&self.analog) {
            if *value != Vec2::ZERO {
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
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // 字段名字由 serde 的借用 visitor 接收，避免每个键创建 String。
        enum Field {
            Analog(usize),
            Digital(usize),
        }
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                struct FieldVisitor;
                impl Visitor<'_> for FieldVisitor {
                    type Value = Field;
                    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        f.write_str("动作名称")
                    }
                    fn visit_str<E: serde::de::Error>(self, key: &str) -> Result<Field, E> {
                        if let Some(slot) = ANALOG_NAMES.iter().position(|name| *name == key) {
                            return Ok(Field::Analog(slot));
                        }
                        if let Some(slot) = DIGITAL_NAMES.iter().position(|name| *name == key) {
                            return Ok(Field::Digital(slot));
                        }
                        Err(E::custom(format_args!("未知动作名称：{key}")))
                    }
                }
                deserializer.deserialize_identifier(FieldVisitor)
            }
        }
        struct SetVisitor;
        impl<'de> Visitor<'de> for SetVisitor {
            type Value = MainActionSetSerde;
            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("动作名称和值的映射")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut values = MainActionSetSerde::default();
                let mut seen = [false; ANALOG_SLOTS + DIGITAL_SLOTS];
                while let Some(field) = map.next_key::<Field>()? {
                    let index = match field {
                        Field::Analog(slot) => slot,
                        Field::Digital(slot) => ANALOG_SLOTS + slot,
                    };
                    if seen[index] {
                        return Err(A::Error::custom("动作名称重复"));
                    }
                    seen[index] = true;
                    match field {
                        Field::Analog(slot) => {
                            let value: Vec2 = map.next_value()?;
                            if !value.is_finite() {
                                return Err(A::Error::custom("模拟动作必须为有限值"));
                            }
                            values.analog[slot] = value;
                        }
                        Field::Digital(slot) => values.digital[slot] = map.next_value()?,
                    }
                }
                Ok(values)
            }
        }
        deserializer.deserialize_map(SetVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn records_preserve_edges_timers_and_all_slots() {
        let mut actions = MainActionSet::default();
        actions.analog.fill(Vec2::new(1.0, -2.0));
        actions.digital.fill(DigitalActionState {
            phase: DigitalActionPhase::Repeating,
            timer_seconds: -0.01,
        });
        assert_eq!(
            MainActionSet::reconstruct(&actions.deconstruct()).unwrap(),
            actions
        );
        let json = serde_json::to_string(&actions).unwrap();
        assert_eq!(
            serde_json::from_str::<MainActionSet>(&json).unwrap(),
            actions
        );
    }
    #[test]
    fn malformed_and_duplicate_records_are_rejected() {
        let valid = ActionSetAction::Analog {
            slot: 0,
            value: Vec2::ONE,
        }
        .to_bytes();
        assert!(MainActionSet::reconstruct(&[valid, valid]).is_err());
        for (offset, value) in [(0, 2), (1, 6), (2, 1)] {
            let mut invalid = valid;
            invalid[offset] = value;
            assert!(MainActionSet::reconstruct(&[invalid]).is_err());
        }
        let nan = ActionSetAction::Analog {
            slot: 0,
            value: Vec2::splat(f32::NAN),
        }
        .to_bytes();
        assert!(MainActionSet::reconstruct(&[nan]).is_err());
    }
    #[test]
    fn named_values_reject_unknown_and_duplicate_fields() {
        assert!(
            serde_json::from_str::<MainActionSetSerde>(r#"{"undo":true,"undo":false}"#).is_err()
        );
        assert!(serde_json::from_str::<MainActionSetSerde>(r#"{"unknown":true}"#).is_err());
        let values: MainActionSetSerde =
            serde_json::from_str(r#"{"undo":true,"camera_zoom":[0.5,0.0]}"#).unwrap();
        assert!(values.digital[16]);
        assert_eq!(values.analog[4], Vec2::new(0.5, 0.0));
    }
}
