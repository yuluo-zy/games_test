//! FMOD owner and recovered audio-domain scheduling for the real gameplay App.
//! The loading App shares this same Studio; it never creates a second engine.
use std::path::Path;
use bevy_app::{App, PreStartup, Startup, Update};
use bevy_ecs::prelude::*;
use country_core::{resources::{audio_resources::{AudioCore, AudioSamplesLoadedPromise},
    native_audio::{NativeStudio, NativeEvent, NativeMusicBackend, create_audio_resources}},
    systems::audio};
use crate::run_game::GameSystemSet;

#[derive(Resource)]
struct Installed;

/// Original run_game captures CliArgs+0x149 in audio_init's closure. The CLI
/// defaults to synchronous FMOD Studio updates; the flag opts into async.
#[derive(Resource, Clone, Copy, Debug)]
pub struct AudioStartupOptions { pub asynchronous: bool }

/// Both Apps retain the real engine. Native events keep it alive until their
/// own release, even when the loading App's resources are dropped on swap.
pub fn share_core(core: &AudioCore<NativeStudio>) -> AudioCore<NativeStudio> {
    AudioCore { studio: core.studio.clone(), banks: core.banks.clone() }
}

pub fn install(app: &mut App, installed: &Path) -> anyhow::Result<()> {
    if app.world().contains_resource::<Installed>() { return Ok(()); }
    app.insert_resource(Installed);
    if !app.world().contains_resource::<AudioCore<NativeStudio>>() {
        let asynchronous = app.world().resource::<AudioStartupOptions>().asynchronous;
        let mut startup = create_audio_resources(installed, asynchronous);
        app.insert_resource(startup.core);
        if let Some(samples) = startup.samples.take() { app.insert_resource(samples); }
        if let Some(loader) = startup.load_remaining.take() {
            std::thread::Builder::new().name("FMOD bank samples".into()).spawn(loader)?;
        }
    }
    // 鸭子的构造工厂持有同一 FMOD 引擎；不再丢弃原 Pitch/Flap 音频链。
    let critter_audio = system_critters::audio::CritterAudioCore::from_audio_core(
        share_core(app.world().resource::<AudioCore<NativeStudio>>()),
    );
    app.insert_resource(critter_audio);
    let music = NativeMusicBackend(app.world().resource::<AudioCore<NativeStudio>>().studio.clone());
    app.insert_resource(music);
    app.add_systems(PreStartup, receive_samples);
    // Startup follows the original PreStartup sample wait, so constructors
    // resolve real events from the actual metadata banks before Update.
    app.add_systems(Startup, (
        audio::building_audio::building_audio_init_system::<NativeStudio>,
        audio::decorator_audio::decorator_audio_init_system::<NativeStudio>,
        audio::forest_audio::forest_audio_init_system::<NativeStudio>,
        audio::garden_audio::garden_audio_init_system::<NativeStudio>,
        audio::menu_audio::menu_audio_init_system::<NativeStudio>,
        audio::path_audio::path_audio_init_system::<NativeStudio>,
        audio::playermade_stairs_audio::stairs_audio_init_system::<NativeStudio>,
        audio::replay_audio::replay_audio_init_system::<NativeStudio>,
        audio::roof_audio::roof_audio_init_system::<NativeStudio>,
        audio::sheep_audio::sheep_audio_init_system::<NativeStudio>,
        audio::timberframe_audio::timberframe_audio_init_system::<NativeStudio>,
        audio::tree_audio::tree_audio_init_system::<NativeStudio>,
        audio::wall_audio_fmod::wall_audio_init_system::<NativeStudio>,
        audio::game_loading_audio::init_game_loading_audio::<NativeStudio>,
        audio::music::music_system::music_audio_init_system::<NativeMusicBackend>,
        audio::trigger_audio::trigger_audio_init_system::<NativeStudio>,
    ));
    // Register through App so original event lifecycles advance each frame.
    app.add_event::<audio::PlayBuildingAudio>();
    app.add_event::<audio::PlayClutterSoundCmd>();
    app.add_event::<audio::PlayDecoratorSoundCmd>();
    app.add_event::<audio::PlayElevationSupportSoundCmd>();
    app.add_event::<audio::PlayMenuUiSound>();
    app.add_event::<audio::PlayStairsAudioCmd>();
    app.add_event::<audio::PlayReplayAudio>();
    app.add_event::<audio::PlaySheepReactSound>();
    app.add_event::<audio::PlayTimberframeSound>();
    app.add_event::<audio::PlayTreeSoundCmd>();
    app.add_event::<audio::PlayWallAudio>();
    app.add_event::<audio::PlayWallUiSound>();
    app.add_event::<audio::roof_audio::RoofEditSound>();
    app.add_event::<audio::path_audio::PathDrawEvent>();
    app.add_event::<audio::path_audio::PathEraseEvent>();
    app.add_event::<audio::path_audio::PathEditEvent>();
    app.add_event::<country_core::systems::backend::photomode::PhotoModeAudioParameter>();
    app.add_systems(Update, (
        audio::building_audio::building_audio::<NativeStudio>,
        audio::clutter_audio::clutter_audio_system::<NativeStudio>,
        audio::decorator_audio::decorator_audio_system::<NativeStudio>,
        audio::elevation_supports_audio::elevation_supports_audio::<NativeStudio>,
        audio::forest_audio::forest_audio_system::<NativeStudio>,
        audio::garden_audio::garden_audio_system::<NativeStudio>,
        audio::menu_audio::menu_audio::<NativeStudio>,
        audio::path_audio::path_audio_system::<NativeStudio>,
        audio::playermade_stairs_audio::stairs_audio_system::<NativeStudio>,
        audio::replay_audio::replay_audio_system::<NativeStudio>,
        audio::roof_audio::roof_audio_system::<NativeStudio>,
        audio::sheep_audio::sheep_audio_system::<NativeStudio>,
        audio::timberframe_audio::timberframe_audio::<NativeStudio>,
        audio::tree_audio::tree_audio_system::<NativeStudio>,
        audio::wall_audio_fmod::wall_audio_fmod::<NativeStudio>,
        audio::wall_audio_ui::wall_audio_ui::<NativeStudio>,
        audio::weather_audio::weather_audio_system::<NativeStudio>,
    ).in_set(GameSystemSet(38)));
    app.add_systems(Update, audio::trigger_audio::trigger_audio_system::<NativeStudio>
        .in_set(GameSystemSet(38)));
    app.add_systems(Update, country_core::systems::time_of_day::time_of_day::<NativeStudio>
        .in_set(GameSystemSet(36)).run_if(crate::run_game::is_gameplay_active));
    app.add_systems(Update, (
        audio::audio_update::audio_update_system::<NativeStudio>,
        audio::game_loading_audio::loading_audio::<NativeStudio>,
        audio::music::music_system::music_audio_system::<NativeMusicBackend>,
        audio::music::music_debug_gui::music_debug_gui::<NativeEvent>,
    ).in_set(GameSystemSet(38)));
    app.add_systems(Update, photo_audio.in_set(GameSystemSet(8))
        .after(country_core::systems::backend::photomode::process_photomode_transitions)
        .run_if(bevy_state::condition::in_state(country_core::app_loading_state::AppLoadingState::Game)));
    app.add_systems(bevy_state::state::OnEnter(country_core::app_loading_state::AppLoadingState::Game),
        load_session_music.after(country_core::startup::startup_session::load_game_session));
    Ok(())
}

fn receive_samples(
    samples: Option<Res<AudioSamplesLoadedPromise<()>>>,
    mut commands: Commands,
) {
    if let Some(samples) = samples {
        country_core::startup::load_audio::load_audio::<NativeEvent>(&samples, &mut commands);
    }
}
fn photo_audio(mut events: EventReader<country_core::systems::backend::photomode::PhotoModeAudioParameter>, core: Res<AudioCore<NativeStudio>>) {
    for event in events.read() {
        let _ = core.set_parameter(country_core::systems::backend::photomode::PHOTOMODE_AUDIO_PARAMETER, event.0);
    }
}
fn load_session_music(
    session: Res<country_core::resources::session::ActiveSession>,
    mut state: ResMut<audio::music::music_system::MusicAudioState<NativeEvent>>,
    backend: Res<NativeMusicBackend>,
) {
    country_core::startup::load_glade_audio::load_session_audio(
        &mut state, &*backend, &paths::installed().join(session.assets_root()), &session.meta,
    );
}
