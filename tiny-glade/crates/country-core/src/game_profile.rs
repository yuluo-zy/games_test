//! B `game_profile.rs` — 玩家档案（0x208B=520B；ron）。
//!
//! `serialize<ron>` 0x140a658a0 只写 **5 字段**：tutorial / tutorial_cooldowns /
//! game_installation_id / key_bindings / new_glade_count；`player_id` **不序列化**，
//! 但 `deserialize` 0x1404b0ed0 接受（6 名：tutorial/tutorial_cooldowns/player_id/
//! key_bindings/new_glade_count/game_installation_id，旧档兼容/内部身份）。
//! `default` 0x140916420 = `Uuid::new_v4`（player_id）+ 默认键位。
//!
//! 真实消费方 = `load_game` 0x14146d030（第 3 参 `Res<GameProfile>`）/ save_game /
//! startup_session（`load_game_profile` 0x140916480 / `save_game_profile` 0x140917230）。
//! **load_completion_checker 不含 GameProfile**（re-analyst 澄清：skeleton 引用是 load_game
//! tuple 注释串）。
use bevy_ecs::resource::Resource;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 12B 键位槽（① r6-gamekeybindings-modkey 契约 §1，O：`{key u32@0, value u64@4}`，
/// 槽 stride 0xc = **packed 压缩布局**，③ 复审要求 `#[repr(C, packed)]` 使结构真为 12B，
/// 否则 Rust 默认布局 16B 注释冒充）。`value` 动作 id 语义待①（默认 0 = 未绑定）。
/// serde/Debug 手工实现（packed 字段不可被引用）。
#[repr(C, packed)]
#[derive(Clone, Copy, Default, PartialEq)]
pub struct KeyBinding {
    /// 键码（u32 @0；bevy 风格 KeyCode）。
    pub key: u32,
    /// 动作 id（u64 @4；语义待①，默认 0 = 未绑定）。
    pub value: u64,
}

impl core::fmt::Debug for KeyBinding {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let key = self.key;
        let value = self.value;
        f.debug_struct("KeyBinding")
            .field("key", &key)
            .field("value", &value)
            .finish()
    }
}

impl serde::Serialize for KeyBinding {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // packed 字段按值读取（Copy），不产生未对齐引用。
        let key = self.key;
        let value = self.value;
        serde::Serialize::serialize(&(key, value), serializer)
    }
}

impl<'de> serde::Deserialize<'de> for KeyBinding {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let (key, value): (u32, u64) = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self { key, value })
    }
}

/// 键位映射（原 game_key_bindings；**40 槽 × 12B 固定数组**）。
/// ① r6-gamekeybindings-modkey 契约 §1（O：capstone default 构建器 0x14089b110）：
/// 槽 0..35 键码 + 槽 36 `[0x1b0]=0xc3`（未绑哨兵）+ 槽 37/38/39 鼠标绑定；`value`
/// 动作 id 语义待①。
/// serde 经 Vec 中转（`[KeyBinding; 40]` 无标准 serde/Default impl，N>32 限制）。
#[derive(Clone, Debug, PartialEq)]
pub struct GameKeyBindings {
    /// 40 槽 × 12B（键/动作）。
    pub bindings: [KeyBinding; 40],
}

impl Default for GameKeyBindings {
    fn default() -> Self {
        // ① r6 契约 §2 default 键码（capstone 0x14089b110 逐槽；与 BindingsTable
        // translate_input.rs KEYS 一致）：槽 0..35 键码 + 槽 36=0xc3（未绑）；
        // 槽 37/38/39 鼠标绑定 value 待①。
        const KEYS: [u32; 40] = [
            0x2a, 0x26, 0x14, 0x17, 0x18, 0x24, 0xc3, 0xc3, 0xc3, 0xc3, // 0..9
            0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x23, // 10..19
            0x27, 0x2d, 0x2b, 0xc3, 0x38, 0xab, 0xa5, 0x1b, 0x49, 0x3a, // 20..29
            0x51, 0x52, 0x50, 0x53, 0x4f, 0x4e, 0xc3, 0, 0, 0, // 30..39
        ];
        Self {
            bindings: KEYS.map(|key| KeyBinding { key, value: 0 }),
        }
    }
}

impl serde::Serialize for GameKeyBindings {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serde::Serialize::serialize(&self.bindings.as_slice(), serializer)
    }
}

impl<'de> serde::Deserialize<'de> for GameKeyBindings {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let bindings: Vec<KeyBinding> = serde::Deserialize::deserialize(deserializer)?;
        let mut array = [KeyBinding::default(); 40];
        for (i, binding) in bindings.into_iter().enumerate().take(40) {
            array[i] = binding;
        }
        Ok(Self { bindings: array })
    }
}

/// 教学冷却状态（20B @+0x1e0..+0x1f4；字段序待取证）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TutorialCooldowns {}

/// 玩家档案（0x208B；ron）。`player_id` 不序列化（`#[serde(skip_serializing)]`）。
/// 字段声明序按原文 ron serialize 序：tutorial → tutorial_cooldowns →
/// game_installation_id → key_bindings → new_glade_count（RE-DOMAIN-TYPES §4②，
/// REVIEW4 复核）。
///
/// `Resource`（O：native `Res<GameProfile>`，load_game 0x14146d030 第 3 参 /
/// startup_session load_game_session 第 3 参，① T2 契约 §1；crate 以 bevy Resource
/// 承载，宿主经 `load_game_profile` 0x140916480 装配）。
#[derive(Resource, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GameProfile {
    /// @+0x1f4（bool）
    pub tutorial: bool,
    /// @+0x1e0
    pub tutorial_cooldowns: TutorialCooldowns,
    /// @+0x1c8（Uuid 16B）
    pub game_installation_id: Uuid,
    /// @+0x000
    pub key_bindings: GameKeyBindings,
    /// @+0x1d8（u32）
    pub new_glade_count: u32,
    /// 内部身份，不写盘（旧档兼容）。
    #[serde(skip_serializing, default = "default_player_id")]
    pub player_id: Uuid,
}

fn default_player_id() -> Uuid {
    Uuid::new_v4()
}

impl Default for GameProfile {
    fn default() -> Self {
        Self {
            tutorial: false,
            tutorial_cooldowns: TutorialCooldowns::default(),
            game_installation_id: Uuid::new_v4(),
            key_bindings: GameKeyBindings::default(),
            new_glade_count: 0,
            player_id: Uuid::new_v4(),
        }
    }
}

impl GameProfile {
    /// 新 glade 计数消费（load_game 0x14146d030 第 3 参 `Res<GameProfile>`，I）。
    pub fn increment_new_glade_count(&mut self) {
        self.new_glade_count = self.new_glade_count.wrapping_add(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profile_ron_skips_player_id_but_roundtrips_others() {
        let profile = GameProfile {
            key_bindings: GameKeyBindings {
                bindings: vec![(1, 2)],
            },
            game_installation_id: Uuid::nil(),
            new_glade_count: 3,
            tutorial_cooldowns: TutorialCooldowns {},
            tutorial: true,
            player_id: Uuid::new_v4(),
        };
        let text = ron::to_string(&profile).unwrap();
        // player_id 不写盘；其余 5 字段存在。
        assert!(!text.contains("player_id"));
        assert!(text.contains("key_bindings"));
        assert!(text.contains("new_glade_count"));
        let back: GameProfile = ron::from_str(&text).unwrap();
        assert_eq!(back.new_glade_count, 3);
        assert_eq!(back.key_bindings, profile.key_bindings);
    }

    #[test]
    fn default_profile_has_a_generated_player_id() {
        let profile = GameProfile::default();
        assert_ne!(profile.player_id, Uuid::nil());
        assert_eq!(profile.new_glade_count, 0);
    }
}
