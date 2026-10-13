//! systems/audio/mod.rs — 音频系统注册骨架。
//!
//! PDB 命名实体（O）：`AsyncLoadedAudioResources`、`WallBendAudioParamReset`。
//! 反汇编证据（O，audio-events-analysis §1）：
//! - `add_audio_events`（0x140941450，行 179–196）：16×`register_event`（顺序见下）。
//! - `add_audio_systems`（0x1409411a0，行 95–142）：`music_debug_gui` +
//!   `water_audio_system`/`water_ambience_audio_system`（各带 run_if）+ 0x26 SystemSet。
//! - `add_audio_startup_systems`（内联入 `add_game_startup_systems` 行 144）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::resource::Resource;

use crate::audio_events;
use crate::typed_gap::Gap;

pub mod audio_update;
pub mod building_audio;
pub mod clutter_audio;
pub mod decorator_audio;
pub mod elevation_supports_audio;
pub mod firstperson_steps;
pub mod forest_audio;
pub mod game_loading_audio;
pub mod garden_audio;
pub mod ivy_audio;
pub mod menu_audio;
pub mod music;
pub mod path_audio;
pub mod playermade_stairs_audio;
pub mod replay_audio;
pub mod roof_audio;
pub mod sheep_audio;
pub mod signifier_audio;
pub mod terrain_audio;
pub mod terrain_stairs_audio;
pub mod timberframe_audio;
pub mod tree_audio;
pub mod trigger_audio;
pub mod wall_audio_fmod;
pub mod wall_audio_orchestrator;
pub mod wall_audio_path;
pub mod wall_audio_ui;
pub mod wall_audio_utils;
pub mod water_ambience_audio;
pub mod water_audio;
pub mod weather_audio;

/// bevy_app::App（REUSED 依赖，不还原；此处以 typed-gap 载体现真实签名 `&mut App`）。
#[derive(Debug, Default)]
pub struct App(pub Gap);

impl App {
    /// `App::add_event<T>`（REUSED 边界，不还原）。
    pub fn add_event<T: 'static>(&mut self) -> &mut Self {
        let _ = core::marker::PhantomData::<T>;
        self
    }
    /// `App::add_systems`（REUSED 边界，不还原）。
    pub fn add_systems(&mut self) -> &mut Self {
        self
    }
}

/// `AsyncLoadedAudioResources`（O 名；`new` 0x140a7abb0，24B；mpmc Channel 泛型证据）。
#[derive(Resource, Clone, Debug, Default)]
pub struct AsyncLoadedAudioResources(pub Gap);

impl AsyncLoadedAudioResources {
    /// `new`（0x140a7abb0，O）。
    pub fn new() -> Self {
        Self(Gap)
    }
}

/// `add_audio_startup_systems`（行 144，O）：启动期音频系统。
pub fn add_audio_startup_systems(_app: &mut App) {
    // 启动系统（load_audio / boot_audio_update_system / …）在此注册（O）。
}

/// `add_audio_systems`（0x1409411a0，行 95–142，O）：运行期音频系统。
pub fn add_audio_systems(app: &mut App) {
    // music_debug_gui + water_audio_system/water_ambience_audio_system（run_if）注册（O）。
    app.add_systems();
}

/// `add_audio_events`（0x140941450，行 179–196，O）：16 个音频事件注册。
pub fn add_audio_events(app: &mut App) {
    app.add_event::<audio_events::PlayTerrainSound>()
        .add_event::<audio_events::SuppressPathSound>()
        .add_event::<audio_events::PlayReplayAudio>()
        .add_event::<audio_events::PlayDecoratorSoundCmd>()
        .add_event::<audio_events::PlayStairsAudioCmd>()
        .add_event::<audio_events::PlayWallUiSound>()
        .add_event::<audio_events::PlayWallAudio>()
        .add_event::<audio_events::RequestWallMaterialAudio>()
        .add_event::<audio_events::WallBendAudioParamReset>()
        .add_event::<audio_events::PlayTimberframeSound>()
        .add_event::<audio_events::PlaySheepReactSound>()
        .add_event::<audio_events::PlayMenuUiSound>()
        .add_event::<audio_events::PlayBuildingAudio>()
        .add_event::<audio_events::PlayElevationSupportSoundCmd>()
        .add_event::<audio_events::PlayClutterSoundCmd>()
        .add_event::<audio_events::PlayTreeSoundCmd>();
}
