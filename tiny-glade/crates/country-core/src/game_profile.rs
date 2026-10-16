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
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 键位映射（原 game_key_bindings；键/动作 ID 语义待取证，默认空）。
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct GameKeyBindings {
    /// 键→动作映射；键/动作 ID 语义待取证。
    #[serde(default)]
    pub bindings: Vec<(u32, u32)>,
}

/// 教学冷却状态（20B @+0x1e0..+0x1f4；字段序待取证）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TutorialCooldowns {}

/// 玩家档案（0x208B；ron）。`player_id` 不序列化（`#[serde(skip_serializing)]`）。
/// 字段声明序按原文 ron serialize 序：tutorial → tutorial_cooldowns →
/// game_installation_id → key_bindings → new_glade_count（RE-DOMAIN-TYPES §4②，
/// REVIEW4 复核）。
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
