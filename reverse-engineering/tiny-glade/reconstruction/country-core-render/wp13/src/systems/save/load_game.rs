//! `systems::save::load_game` — 读档系统（B 骨架对齐，WP13）。
//!
//! O 证据（合同见 `contracts/save-systems.md`）：
//!   - `load_game` 0x14146d030 / `parse_saved_game_data` 0x141470d00（反编译 +
//!     字符串：版本迁移/命令跳过/旧版 window color / arrow slots）。
//!   - `LoadGameManager` / `LoadGameSource` / `ParsedGameData` /
//!     `PostGameLoadCommand` / `LoadingGameComplete` / `StartupRevealStarted/Finished`。

use crate::typed_gap::Gap;

/// O：`LoadGameManager`（读档状态机；`new`；布局 U）。
#[derive(Clone, Debug, Default)]
pub struct LoadGameManager(pub Gap);

/// O：`LoadGameSource`（读档来源：起始建筑/存档；布局 U）。
#[derive(Clone, Debug, Default)]
pub struct LoadGameSource(pub Gap);

/// O：`ParsedGameData`（解析后数据；布局 U）。
#[derive(Clone, Debug, Default)]
pub struct ParsedGameData(pub Gap);

/// O：`PostGameLoadCommand`（读档后命令；enum，drop 出现）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PostGameLoadCommand(pub Gap);

/// O：`LoadingGameComplete`（读档完成事件）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LoadingGameComplete(pub Gap);

/// O：`StartupRevealStarted/Finished`（开场揭示事件）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StartupRevealStarted(pub Gap);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StartupRevealFinished(pub Gap);

/// O：`parse_saved_game_data` 0x141470d00（反序列化各节；节序/字段 U）。
pub fn parse_saved_game_data(_raw: &[u8]) -> Result<ParsedGameData, anyhow::Error> {
    anyhow::bail!("save parsing schema not recovered")
}

/// O：`apply_parsed_game_data`（把解析数据应用到世界；语义 U）。
pub fn apply_parsed_game_data(_data: &ParsedGameData) {}

/// O：`apply_backward_compatibility_overrides`（版本迁移覆盖）。
pub fn apply_backward_compatibility_overrides(_data: &mut ParsedGameData) {}

/// O：`extract_max_wall_id_from_history`（读档后恢复最大墙 id）。
pub fn extract_max_wall_id_from_history(_data: &ParsedGameData) -> u64 {
    0
}

/// O：`upgrade_window_color`（旧版 window color 升级日志）。
pub fn upgrade_window_color(_data: &mut ParsedGameData) {}

/// O：`load_game` 0x14146d030 主入口（反编译）；完整参数/阶段 U。
pub fn load_game(_manager: &mut LoadGameManager) {}
