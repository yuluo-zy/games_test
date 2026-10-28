//! B `systems/audio/mod.rs`: audio-domain event types, the async loaded-audio
//! package and the two registration entry points (`add_audio_events` /
//! `add_audio_systems`).
//!
//! O-level facts (chapter mod.rs.md, audio-misc DIGEST §2.13 +
//! multi-baseline audio-events-analysis §1):
//! - `add_audio_events` 0x140941450 (PDB name add_audio_startup_systems 错位):
//!   16 × `register_event` in a fixed order — PlayTerrainSound / SuppressPathSound /
//!   PlayReplayAudio / PlayDecoratorSoundCmd / PlayStairsAudioCmd / PlayWallUiSound /
//!   PlayWallAudio / RequestWallMaterialAudio / WallBendAudioParamReset /
//!   PlayTimberframeSound / PlaySheepReactSound / PlayMenuUiSound / PlayBuildingAudio /
//!   PlayElevationSupportSoundCmd / PlayClutterSoundCmd / PlayTreeSoundCmd.
//! - `add_audio_systems` 0x1409411a0: music_debug_gui + two `run_if` packages
//!   (water_audio_system, water_ambience_audio_system).
//! - `AsyncLoadedAudioResources::new` 0x140a7abb0: 24B async package wrapping
//!   crossbeam mpmc channels (AudioSamplesLoaded generic instance).

use bevy_ecs::prelude::*;
use glam::Vec3;

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

// ---- Audio events (payload layouts O; see each system chapter) ----

/// `PlayWallAudio` (wall_audio_fmod). Native slot stride 0x28, payload from
/// @+8: kind u8, f32 @+c, position Vec3 @+0x10, volume f32 @+0x1c, flag @+0x20.
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct PlayWallAudio {
    pub kind: u8,
    /// Generic scalar read by parameterized kinds (e.g. WallHeight inverse_lerp input).
    pub value: f32,
    pub position: Vec3,
    /// Volume passed to instantiate_audio_w_position_and_volume / request_play.
    pub volume: f32,
    /// Flag byte (e.g. kind 10 HasBricks: non-zero → 1.0).
    pub flag: u8,
}

/// `PlayBuildingAudio` (building_audio). Payload starts at slot @+0 (no bevy
/// bookmark): id u64 (hash key into the building table), position Vec3, editing_speed f32, kind u8.
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct PlayBuildingAudio {
    pub id: u64,
    pub position: Vec3,
    pub editing_speed: f32,
    pub kind: u8,
}

/// `PlayStairsAudioCmd` (playermade_stairs_audio). Slot stride 0x38, payload
/// from @+8: kind u8, bytes @+9.., f32s, position, more f32s.
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct PlayStairsAudioCmd {
    pub kind: u8,
    pub position: Vec3,
    /// f32 payloads @+0xc/+0x10/+0x14/+0x18.
    pub values: [f32; 4],
    /// f32 payloads @+0x28/+0x2c.
    pub tail: [f32; 2],
    /// payload bytes @+9/+0xa/+0xb.
    pub bytes: [u8; 3],
}

/// `PlayClutterSoundCmd` (clutter_audio). Slot stride 0x18, kind u8 @+0x14,
/// position @+0/+8, context byte @+0x15 (indexes the clutter-context value table).
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct PlayClutterSoundCmd {
    pub kind: u8,
    pub position: Vec3,
    /// O: byte @+0x15 → "clutter-context" value table index {0,1,2}.
    pub context: u8,
}

/// `PlaySheepReactSound` (sheep_audio). Slot stride 0x20; payload (from +8):
/// position Vec3 @+8, pitch f32 @+0x14, kind u8 @+0x18.
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct PlaySheepReactSound {
    pub position: Vec3,
    /// O: pitch f32 @+0x14 (set_parameter("Pitch", value)).
    pub pitch: f32,
    pub kind: u8,
}

/// `PlayMenuUiSound` (menu_audio). Slot stride 0x10, kind u8 @+0x8.
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct PlayMenuUiSound {
    pub kind: u8,
}

/// `PlayTreeSoundCmd` (tree_audio). Slot stride 0x20; payload (from +8):
/// kind u8 @+8, selector byte @+9, extra byte @+0xa, value f32 @+0xc,
/// position Vec3 @+0x10.
///
/// Kind 0 → TreePlace (value = tree_size, byte@+9 = in_water, byte@+0xa =
/// is_naked); kind 1 → the frame-end TreeMove loop (byte@+9 = in_water,
/// value = tree_size, byte@+9 == 2 → stop); kind 2 → TreeDelete.
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct PlayTreeSoundCmd {
    pub kind: u8,
    /// O: payload byte @+9 (in_water / TreeMove selector: 2 stops the loop).
    pub selector: u8,
    /// O: payload byte @+0xa (is_naked for TreePlace).
    pub extra: u8,
    /// O: payload f32 @+0xc (tree_size).
    pub value: f32,
    pub position: Vec3,
}

/// `PlayReplayAudio` (replay_audio). Slot stride 0x10, kind u8 @+0x8:
/// 0 → Undo, 1 → Redo, 2 → DenyInput.
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct PlayReplayAudio {
    pub kind: u8,
}

/// `PlayWallUiSound` (wall_audio_ui). Native slot stride 0x20: kind u8 @+8,
/// selector byte @+9, value f32 @+0xc, position Vec3 @+0x10. Kinds 6/9/0xf/0x10
/// select by the byte@+9; kinds 0/0xd/0xe carry a parameter value@+0xc.
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct PlayWallUiSound {
    pub kind: u8,
    /// Byte @+9 (O: selector for ColorBricks isHalfTimbered, Flags
    /// ToggleDisconnect/Connect, Chimney ToggleSmoke, ChangeWallStyle).
    pub selector: u8,
    /// f32 @+0xc (O: StructureSize, LanternColor, BrightnessValue parameter).
    pub value: f32,
    pub position: Vec3,
}

/// `PlayDecoratorSoundCmd` (decorator_audio). Kind dispatch into the 16-loop
/// state plus one-shot instantiate path.
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct PlayDecoratorSoundCmd {
    pub kind: u8,
    pub position: Vec3,
}

/// `PlayTimberframeSound` (timberframe_audio). Native payload (no bevy
/// bookmark): kind u8 @0, flag byte @+1, speed-source f32 @+4, position Vec3
/// @+8, value f32 @+0x14. Kinds 0/1 arm the Width/Length loops (speed source
/// @+4 divided by the shared editing speed), kind 3 the Move loop (speed
/// source @+4, no division), kinds 2/4/5/6/7 are one-shots.
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct PlayTimberframeSound {
    pub kind: u8,
    /// O: byte @+1 (is_on_ground for the Move loop).
    pub flag: u8,
    /// O: f32 @+4 (inverse_lerp input).
    pub speed_source: f32,
    pub position: Vec3,
    /// O: f32 @+0x14 (BuildingSize instance parameter).
    pub value: f32,
}

/// `PlayElevationSupportSoundCmd` (elevation_supports_audio). Kind selects one
/// of the three support events.
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct PlayElevationSupportSoundCmd {
    pub kind: u8,
    pub position: Vec3,
}

/// `SuppressPathSound` (terrain_stairs_audio): emit when the path audio should
/// be suppressed while terrain stairs are being drawn.
#[derive(Event, Clone, Copy, Debug, Default, PartialEq)]
pub struct SuppressPathSound;

/// `RequestWallMaterialAudio` (wall_audio_orchestrator): request a material-parameter
/// update for the wall identified by `id` (GlobalId hash key).
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct RequestWallMaterialAudio {
    pub id: u64,
    pub position: Vec3,
    pub value: f32,
    pub kind: u8,
}

/// `WallBendAudioParamReset` (wall audio): reset a bend-related loop parameter.
#[derive(Event, Clone, Copy, Debug, Default, PartialEq)]
pub struct WallBendAudioParamReset;

/// `add_audio_events` (0x140941450): register the 16 audio events on `world`.
pub fn add_audio_events(world: &mut World) {
    world.init_resource::<Events<crate::systems::terrain::PlayTerrainSound>>();
    world.init_resource::<Events<SuppressPathSound>>();
    world.init_resource::<Events<PlayReplayAudio>>();
    world.init_resource::<Events<PlayDecoratorSoundCmd>>();
    world.init_resource::<Events<PlayStairsAudioCmd>>();
    world.init_resource::<Events<PlayWallUiSound>>();
    world.init_resource::<Events<PlayWallAudio>>();
    world.init_resource::<Events<RequestWallMaterialAudio>>();
    world.init_resource::<Events<WallBendAudioParamReset>>();
    world.init_resource::<Events<PlayTimberframeSound>>();
    world.init_resource::<Events<PlaySheepReactSound>>();
    world.init_resource::<Events<PlayMenuUiSound>>();
    world.init_resource::<Events<PlayBuildingAudio>>();
    world.init_resource::<Events<PlayElevationSupportSoundCmd>>();
    world.init_resource::<Events<PlayClutterSoundCmd>>();
    world.init_resource::<Events<PlayTreeSoundCmd>>();
}

/// `add_audio_systems` (0x1409411a0): the audio systems registered here are the
/// music debug GUI (generic over the music [`MusicBackend`]) and the two water
/// systems behind `run_if` gates (generic over the FMOD [`Studio`] binding).
/// `S` is the injected FMOD `Studio`; `M` the music backend — the host supplies
/// the real libfmod adapters at registration time.
///
/// ⑤轮 ③ 复审修复（round5/REVIEW.md §3）：**F3 闭合**——`garden_audio_init_system` +
/// `garden_audio_system`（T1 已复审通过）补进生产 Schedule；**G6 对齐**——① a 契约 §1
/// G6（0x140b6bd50）= timberframe / elevation_supports / clutter / forest / tree /
/// firstperson_steps 6 域 + crate 其余音频域系统一并补入（native 音频组 tag 0x26=38 系统；
/// water×2 属 water 子组）。各系统依赖的状态资源由对应 init 系统插入或宿主提供。
#[allow(clippy::type_complexity)]
pub fn add_audio_systems<S, M>(schedule: &mut Schedule)
where
    S: crate::resources::audio_resources::Studio + Send + Sync + 'static,
    M: crate::systems::audio::music::music_player::MusicBackend + Resource,
{
    use crate::systems::audio::music::music_debug_gui::music_debug_gui;
    use bevy_ecs::schedule::common_conditions::{not, resource_exists};
    // Keep each existing domain owner across frames; commit its one-time
    // initialization before its first consumer instead of replacing it each Update.
    schedule.add_systems(music_debug_gui::<M::Event>);
    schedule.add_systems((
        water_audio::water_audio_system::<S>,
        water_ambience_audio::water_ambience_audio_system::<S>,
    ));
    // ① a 契约 §1 G6（0x140b6bd50）+ F3 garden_audio（bevy tuple ≤20 系统，分三批）。
    schedule.add_systems((
        // G6 六域（① a 契约 §1）：timberframe / elevation_supports / clutter /
        // forest / tree / firstperson_steps。
        (
            timberframe_audio::timberframe_audio_init_system::<S>
                .run_if(not(resource_exists::<timberframe_audio::TimberframeAudioState<S::Event>>)),
            timberframe_audio::timberframe_audio::<S>,
        )
            .chain(),
        elevation_supports_audio::elevation_supports_audio::<S>,
        clutter_audio::clutter_audio_system::<S>,
        (
            forest_audio::forest_audio_init_system::<S>
                .run_if(not(resource_exists::<forest_audio::ForestAudioState<S::Event>>)),
            forest_audio::forest_audio_system::<S>,
        )
            .chain(),
        (
            tree_audio::tree_audio_init_system::<S>
                .run_if(not(resource_exists::<tree_audio::TreeAudioState<S::Event>>)),
            tree_audio::tree_audio_system::<S>,
        )
            .chain(),
        (
            firstperson_steps::firstperson_steps_init_system::<S>
                .run_if(not(resource_exists::<firstperson_steps::FirstPersonViewAudioState<S::Event>>)),
            firstperson_steps::firstperson_steps::<S>,
        )
            .chain(),
        // F3：garden_audio（T1）。
        (
            garden_audio::garden_audio_init_system::<S>
                .run_if(not(resource_exists::<garden_audio::GardenAudioState<S::Event>>)),
            garden_audio::garden_audio_system::<S>,
        )
            .chain(),
        // crate 其余音频域（init + main），批 1。
        audio_update::audio_update_system::<S>,
        audio_update::boot_audio_update_system::<S>,
        (
            building_audio::building_audio_init_system::<S>
                .run_if(not(resource_exists::<building_audio::BuildingAudioState<S::Event>>)),
            building_audio::building_audio::<S>,
        )
            .chain(),
        (
            decorator_audio::decorator_audio_init_system::<S>
                .run_if(not(resource_exists::<decorator_audio::DecoratorAudioState<S::Event>>)),
            decorator_audio::decorator_audio_system::<S>,
        )
            .chain(),
        (
            game_loading_audio::init_game_loading_audio::<S>
                .run_if(not(resource_exists::<game_loading_audio::GameLoadingAudioState<S::Event>>)),
            game_loading_audio::loading_audio::<S>,
        )
            .chain(),
    ));
    schedule.add_systems((
        (
            menu_audio::menu_audio_init_system::<S>
                .run_if(not(resource_exists::<menu_audio::MenuAudioState<S::Event>>)),
            menu_audio::menu_audio::<S>,
        )
            .chain(),
        (
            path_audio::path_audio_init_system::<S>
                .run_if(not(resource_exists::<path_audio::PathAudioState<S::Event>>)),
            path_audio::path_audio_system::<S>,
        )
            .chain(),
        (
            playermade_stairs_audio::stairs_audio_init_system::<S>
                .run_if(not(resource_exists::<playermade_stairs_audio::StairsAudioState<S::Event>>)),
            playermade_stairs_audio::stairs_audio_system::<S>,
        )
            .chain(),
        (
            replay_audio::replay_audio_init_system::<S>
                .run_if(not(resource_exists::<replay_audio::ReplayAudioState<S::Event>>)),
            replay_audio::replay_audio_system::<S>,
        )
            .chain(),
        (
            sheep_audio::sheep_audio_init_system::<S>
                .run_if(not(resource_exists::<sheep_audio::SheepAudioState<S::Event>>)),
            sheep_audio::sheep_audio_system::<S>,
        )
            .chain(),
        (
            signifier_audio::signifier_audio_init_system::<S>
                .run_if(not(resource_exists::<signifier_audio::SignifierAudioState<S::Event>>)),
            signifier_audio::signifier_audio_system::<S>,
        )
            .chain(),
        (
            terrain_stairs_audio::terrain_stairs_audio_init_system::<S>
                .run_if(not(resource_exists::<terrain_stairs_audio::TerrainStairsAudioState<S::Event>>)),
            terrain_stairs_audio::terrain_stairs_audio_system::<S>,
        )
            .chain(),
        (
            trigger_audio::trigger_audio_init_system::<S>
                .run_if(not(resource_exists::<trigger_audio::TriggerAudioState<S::Event>>)),
            trigger_audio::trigger_audio_system::<S>,
        )
            .chain(),
        (
            wall_audio_fmod::wall_audio_init_system::<S>
                .run_if(not(resource_exists::<wall_audio_fmod::WallAudioState<S::Event>>)),
            wall_audio_fmod::wall_audio_fmod::<S>,
        )
            .chain(),
    ));
    terrain_audio::add_terrain_audio_systems::<S>(schedule);
    schedule.add_systems(weather_audio::weather_audio_system::<S>);
    // G6 补全（① audio-G6-alignment 契约 §3 其余未注册音频系统）。
    schedule.add_systems((
        (
            roof_audio::roof_audio_init_system::<S>
                .run_if(not(resource_exists::<roof_audio::RoofAudioState<S::Event>>)),
            roof_audio::roof_audio_system::<S>,
        )
            .chain(),
        wall_audio_ui::wall_audio_ui::<S>,
        wall_audio_orchestrator::wall_audio_orchestrator::<S>,
        wall_audio_path::wall_audio_path::<S>,
        ivy_audio::ivy_audio_init_system,
    ));
}

/// `AsyncLoadedAudioResources::new` (0x140a7abb0): the 24B async package that
/// crosses the FMOD bank/sample-loading boundary. `SamplesLoaded` carries the
/// [`crate::resources::audio_resources::AudioSamplesLoadedPromise`] generic.
#[derive(Resource, Debug, Default)]
pub struct AsyncLoadedAudioResources {
    /// Channel delivering loaded-bank completion (library-internal mpmc).
    pub samples_loaded: Option<crossbeam_channel::Sender<()>>,
    /// Mirror receiver used by the library (recv_timeout 1s; see the promise).
    pub receiver: Option<crossbeam_channel::Receiver<()>>,
}

impl AsyncLoadedAudioResources {
    /// `new` (0x140a7abb0): construct the channel pair.
    pub fn new() -> Self {
        let (sender, receiver) = crossbeam_channel::bounded(1);
        AsyncLoadedAudioResources {
            samples_loaded: Some(sender),
            receiver: Some(receiver),
        }
    }
}

pub mod wall_kinds;

