//! B 0x140a71f20 / 0x140a72810. The PDB parameters distinguish GameTime from
//! bevy Time<()>, and EscMenuState from AppMode.
use super::music_player::{MusicBackend, MusicEvent, MusicInstance, MusicPlayer, StopMode};
use crate::systems::audio::music::parameters::advance_new_glade_time;
use crate::systems::audio::music::parameters::snap_off_boundaries;
use crate::resources::gui::DebugGui;
use crate::resources::gui::EscMenuState;
use crate::resources::gui::PhotomodeScreen;
use crate::resources::session::ActiveSession;
use crate::resources::session::GladeType;
use crate::resources::time::GameTime;
use crate::startup::init_photomode_state::PhotomodeState;
use crate::systems::mode_manager::AppMode;
use bevy_ecs::prelude::*;

#[derive(Resource)]
pub struct MusicAudioState<E: MusicEvent> {
    pub player: MusicPlayer<E>,
    pub ambience: MusicInstance<E>,
    pub preview_music: MusicInstance<E>,
    pub preview_ambience: MusicInstance<E>,
    pub richness: f32,
    pub target_richness: f32,
    pub new_glade_time: f32,
    pub preview_glade: Option<GladeType>,
}
impl<E: MusicEvent> MusicAudioState<E> {
    pub fn init<B: MusicBackend<Event = E>>(backend: &B) -> Self {
        let mut ambience = MusicInstance::create(backend, "event:/Amb/Ambience");
        ambience.start();
        let preview_music = MusicInstance::create(backend, "event:/Music/mus_GladePreview");
        let preview_ambience = MusicInstance::create(backend, "event:/Amb/GladePreview");
        let player = MusicPlayer::init(backend);
        Self {
            player,
            ambience,
            preview_music,
            preview_ambience,
            richness: 0.0,
            target_richness: 0.0,
            new_glade_time: 0.0,
            preview_glade: None,
        }
    }
}
pub fn music_audio_init_system<B: MusicBackend + Resource>(
    backend: Res<B>,
    mut commands: Commands,
) {
    commands.insert_resource(MusicAudioState::init(&*backend));
}

/// Connect the original ActiveSession owner and clocks to music, then its GUI.
pub fn configure_music_consumers<B>(schedule: &mut Schedule)
where
    B: MusicBackend + Resource,
{
    schedule.add_systems(
        (
            music_audio_system::<B>,
            super::music_debug_gui::music_debug_gui::<B::Event>,
        )
            .chain(),
    );
}

/// Table @0x142ad67f0.
pub fn preview_glade_parameter(theme: GladeType) -> f32 {
    theme.audio_id()
}
fn set_parameter<B: MusicBackend>(backend: &B, name: &str, value: f32) {
    // AudioCoreInner::set_parameter logs a warning and continues on FMOD errors.
    if let Err(error) = backend.set_global_parameter(name, value) {
        log::warn!("Could not set audio parameter {name}: {error:?}");
    }
}

/// Original ActiveSession's metadata theme at +0x58.
#[allow(clippy::too_many_arguments)]
pub fn music_audio_system<B: MusicBackend + Resource>(
    mode: Res<AppMode>,
    backend: Res<B>,
    mut state: ResMut<MusicAudioState<B::Event>>,
    menu: Res<EscMenuState>,
    session: Res<ActiveSession>,
    debug: Res<DebugGui>,
    game_time: Res<GameTime>,
    time: Res<bevy_time::Time>,
    screen: Res<PhotomodeScreen>,
    photo: Res<PhotomodeState>,
) {
    update_music_audio(
        &mut state,
        &*backend,
        session.meta.glade,
        &mode,
        &menu,
        &debug,
        game_time.delta,
        time.delta_secs(),
        screen.enabled,
        photo.camera_mode,
    );
}

#[allow(clippy::too_many_arguments)]
pub fn update_music_audio<B: MusicBackend>(
    state: &mut MusicAudioState<B::Event>,
    backend: &B,
    active_theme: GladeType,
    mode: &AppMode,
    menu: &EscMenuState,
    debug: &DebugGui,
    game_delta: f32,
    real_delta: f32,
    photomode_enabled: bool,
    camera_mode: crate::resources::photo_mode::PhotoModeCameraMode,
) {
    state.target_richness =
        snap_off_boundaries((state.target_richness + game_delta / 120.0).clamp(0.0, 1.0));
    let target = if mode.native_tag() == 19 {
        1.0
    } else {
        state.target_richness
    };
    // utils::time_continuous_lerp 0x140c904e0, rate=1 at this call site.
    let smooth = state.richness + (target - state.richness) * (1.0 - (-game_delta).exp());
    state.richness = snap_off_boundaries(if smooth > 0.99 { 1.0 } else { smooth });
    set_parameter(
        backend,
        "MusicRichness",
        if debug.force_music_richness {
            1.0
        } else {
            state.richness
        },
    );
    advance_new_glade_time(game_delta, &mut state.new_glade_time);
    set_parameter(backend, "NewGladeTime", state.new_glade_time);
    // Native tests only the menu tag, including Photomode(true).
    if menu.view().tag | 8 == 10 {
        state.player.update(real_delta);
        if photomode_enabled {
            if camera_mode as u8 == 2 {
                state.player.photomode.start_if_not_playing();
                state.player.first_person.stop(StopMode::Immediate);
            } else {
                state.player.first_person.start_if_not_playing();
                state.player.photomode.stop(StopMode::Immediate);
            }
        } else {
            state.player.stop_photomode_snapshots();
        }
        state.preview_music.set_paused(true);
        state.preview_ambience.stop(StopMode::AllowFadeout);
    } else {
        state.player.pause();
        state.player.stop_photomode_snapshots();
        let theme = match state.preview_glade {
            Some(theme) => {
                state.preview_ambience.start_if_not_playing();
                theme
            }
            None => {
                state.preview_ambience.stop(StopMode::AllowFadeout);
                active_theme
            }
        };
        set_parameter(backend, "PreviewGladeType", preview_glade_parameter(theme));
        state.preview_music.start_if_not_playing();
        if state
            .preview_music
            .event()
            .is_some_and(|e| e.paused().unwrap())
        {
            state.preview_music.set_paused(false);
        }
    }
}
