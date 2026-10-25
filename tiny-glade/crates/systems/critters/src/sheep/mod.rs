//! sheep 子系统（B 主基线）。
//! 插件面：`SheepPlugin::build`（0x14067d810，fresh decomp/14067d810.c）：
//! `add_systems` + `chain_ignore_deferred` + `run_if<fn(Res<GameTime>,
//! Res<State<AppLoadingState>>) -> bool>`；`register_event`：
//! `PlaySheepPetAnimation`、`CmdCheckForSheepInsideBuilding`、`CmdUnfreezeSheep`。
//!
//! 注：本 crate lib.rs 当前装配事件名为 `PlaySheepPet` / `SheepInputReplayCmd`（宿主/旧名，
//! 需与二进制 `PlaySheepPetAnimation` 对齐，待 lib.rs 同步后改）；各系统证据见子模块。

use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;
use bevy_state::state::State;

use country_core::app_loading_state::AppLoadingState;
use country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding;
use country_core::resources::time::GameTime;
use country_core::systems::debug::sheep_controls::sheep_controls;
use country_core::systems::history::finish_replay::CmdUnfreezeSheep;

pub mod sheep_behavior;
pub mod sheep_floatie;
pub mod sheep_input_record_replay;
pub mod sheep_pet_animation;
pub mod sheep_react_to_being_inside_buildings;
pub mod sheep_terrain;
pub mod sheep_umbrella;
pub mod sheep_unstuck;
pub mod sheep_walk_animation;
pub mod ui_pet_sheep;

/// `country_core::systems::is_gameplay_active`（B 0x140a90e00，22 B）：`GameTime.ran && State == Game`。
/// 原版条件函数住在 country_core::systems；本仓库该函数目前只在宿主 `run_game.rs`（crate 不可依赖宿主），
/// 且为避免与其他 agent 同时改 country-core，这里按同一机器码语义做 crate 私有镜像。
fn is_gameplay_active(time: Res<GameTime>, state: Res<State<AppLoadingState>>) -> bool {
    time.ran && *state.get() == AppLoadingState::Game
}

/// `<system_critters::sheep::SheepPlugin as bevy_app::Plugin>::build`（B 0x14067d810，357 B）。
///
/// | 调用点 | 动作 | 内容 |
/// |---|---|---|
/// | 0x14067d828 | `chain_ignore_deferred`（辅助 0x140f960c0，tuple2） | `(sheep_controls, <tuple5>)`；`sheep_controls` = `country_core::systems::debug::sheep_controls::sheep_controls`；tuple5（0x141dc6c10，普通 tuple，成员无序）= `(sheep_walk_animation, ui_pet_sheep, sheep_pet_animation, react_to_being_inside_buildings, sheep_reaction_emoji)` |
/// | 0x14067d838 | `ScheduleConfigs::run_if(country_core::systems::is_gameplay_active)`（条件串 0x140f7f8a3） | 整个链 |
/// | 0x14067d843 | `SubApp::add_systems(Update)`（无 `in_set`） | |
/// | 0x14067d8a3 / 0x14067d8ff / 尾跳 0x140f936a0 | 3× `register_event`（`get_resource_id` 幂等门） | `PlaySheepPetAnimation`（本 crate 名 [`PlaySheepPet`](sheep_pet_animation::PlaySheepPet)）、`CmdCheckForSheepInsideBuilding`、`CmdUnfreezeSheep` |
///
/// 无资源插入。
pub struct SheepPlugin;

impl Plugin for SheepPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                sheep_controls,
                (
                    sheep_walk_animation::sheep_walk_animation,
                    ui_pet_sheep::ui_pet_sheep,
                    sheep_pet_animation::sheep_pet_animation,
                    sheep_react_to_being_inside_buildings::react_to_being_inside_buildings,
                    sheep_react_to_being_inside_buildings::sheep_reaction_emoji,
                ),
            )
                .chain_ignore_deferred()
                .run_if(is_gameplay_active),
        );
        app.add_event::<sheep_pet_animation::PlaySheepPet>();
        app.add_event::<CmdCheckForSheepInsideBuilding>();
        app.add_event::<CmdUnfreezeSheep>();
    }
}
