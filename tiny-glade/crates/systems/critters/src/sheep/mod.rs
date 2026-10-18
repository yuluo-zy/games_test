//! sheep 子系统（B 主基线）。
//! 插件面：`SheepPlugin::build`（0x14067d810，fresh decomp/14067d810.c）：
//! `add_systems` + `chain_ignore_deferred` + `run_if<fn(Res<GameTime>,
//! Res<State<AppLoadingState>>) -> bool>`；`register_event`：
//! `PlaySheepPetAnimation`、`CmdCheckForSheepInsideBuilding`、`CmdUnfreezeSheep`。
//!
//! 注：本 crate lib.rs 当前装配事件名为 `PlaySheepPet` / `SheepInputReplayCmd`（宿主/旧名，
//! 需与二进制 `PlaySheepPetAnimation` 对齐，待 lib.rs 同步后改）；各系统证据见子模块。

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
