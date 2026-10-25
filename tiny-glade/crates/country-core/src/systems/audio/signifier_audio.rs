//! Original signifier audio dispatcher B1409d6b90/6319B. It consumes canonical
//! events; UiSignifierStream is shared visual data and is never drained here.
//! Native tables and source SHA are recorded in module-recovery-2026-10-23.
use crate::{
    resources::{
        audio_resources::{AudioCore, AudioInstance, EventInstance, Studio},
        events::{FinishAdjWallHeightMode, InitAdjWallHeightMode, ModeJustChanged},
        walls::{
            public_walls::{OnWallChanged, PublicWalls},
            WallId,
        },
    },
    systems::{
        mode_manager::{AppMode, EditSubject, LoadingGameComplete},
        ui::resources::widget_params::{PlayPhotomodeSound, PlayUiSound},
    },
    utils::audio_utils::AudioLoop,
};
use bevy_ecs::prelude::*;

pub const FIRST_PERSON_OFF: &str = "event:/UI/camera/FirstPersonOff";
pub const FIRST_PERSON_ON: &str = "event:/UI/camera/FirstPersonOn";
pub const UI_TOOL_CHANGE_EVENT: &str = "event:/UI/UI Tool Change";
pub const TOOL_PARAM: &str = "Tool";
/// Original init order: four cached one-shots, then the rotating-gizmo ring.
pub const SIGNIFIER_EVENTS: [&str; 4] = [
    "event:/UI/UI Boop",
    "event:/Cursor/Grab",
    "event:/Cursor/Release",
    "event:/UI/HoldTimeSiginfier",
];
pub const SIGNIFIER_LOOP_EVENT: &str = "event:/UI/UiRotateGizmoMovingBack";

/// Original sound jump table142ac2ce8, exactly36 variants. The two no-op
/// variants and the two parameterized variants are dispatched separately.
pub const UI_SOUND_EVENTS: [Option<&str>; 36] = [
    None,
    None,
    Some("event:/Cursor/Grab"),
    Some("event:/Cursor/Release"),
    Some("event:/UI/DenyInput"),
    Some("event:/SFX/Rooftip_Grab"),
    Some("event:/SFX/Rooftip_Release"),
    Some("event:/SFX/WindowHover"),
    Some("event:/UI/ColorMenu/Open"),
    Some("event:/UI/ColorMenu/Close"),
    Some("event:/UI/Ui2dLockedHover"),
    Some("event:/UI/Ui2dLockedClickNew"),
    Some("event:/UI/ColorMenu/Hover"),
    Some("event:/UI/ColorMenu/Select"),
    Some("event:/UI/Ui2dClick"),
    Some("event:/UI/Ui2dHover"),
    Some("event:/UI/Toolbar/ToolbarExpand"),
    Some("event:/UI/Toolbar/ToolbarShrink"),
    Some("event:/UI/Toolbar/ToolbarTutorialButtonReveal"),
    None,
    Some("event:/UI/Tutorial/HintAppear"),
    Some("event:/UI/Tutorial/HintDisappear"),
    Some("event:/UI/Tutorial/HintProgressStep"),
    Some("event:/SFX/Tree/TreeGizmoHover"),
    Some("event:/SFX/Tree/TreeGizmoGrab"),
    Some("event:/SFX/Tree/TreeGizmoRelease"),
    Some("event:/UI/UiSliderStartDrag"),
    Some("event:/UI/UiSliderEndDrag"),
    Some("event:/UI/UiSliderTick"),
    Some("event:/UI/camera/TakePicture"),
    Some("event:/UI/TimeOfDay/SliderTick"),
    Some("event:/UI/TimeOfDay/SliderFollow"),
    Some("event:/UI/TimeOfDay/Lock"),
    Some("event:/UI/TimeOfDay/Unlock"),
    None,
    Some("event:/UI/UiRotateGizmoStop"),
];

#[derive(Resource)]
pub struct SignifierAudioState<E: EventInstance> {
    pub boop: AudioInstance<E>,
    pub last_changed_wall: Option<WallId>,
    pub cursor_grab: AudioInstance<E>,
    pub cursor_release: AudioInstance<E>,
    pub hold_time: AudioInstance<E>,
    pub rotate_moving_back: AudioLoop<AudioInstance<E>>,
}
impl<E: EventInstance> SignifierAudioState<E> {
    pub fn new<S: Studio<Event = E>>(core: &AudioCore<S>) -> Self {
        let [boop, cursor_grab, cursor_release, hold_time] =
            SIGNIFIER_EVENTS.map(|name| instantiate_checked(core, name));
        Self {
            boop,
            last_changed_wall: None,
            cursor_grab,
            cursor_release,
            hold_time,
            rotate_moving_back: core.instantiate_loop(
                &crate::resources::audio_resources::AudioLoopDesc::new(SIGNIFIER_LOOP_EVENT),
            ),
        }
    }
}
pub fn signifier_audio_init_system<S: Studio + Send + Sync + 'static>(
    core: Res<AudioCore<S>>,
    mut commands: Commands,
) {
    commands.insert_resource(SignifierAudioState::<S::Event>::new(&core));
}
fn instantiate_checked<S: Studio>(core: &AudioCore<S>, name: &str) -> AudioInstance<S::Event> {
    let instance = core.instantiate_audio(name);
    if let AudioInstance::Err(error) = &instance {
        panic!("instantiate original signifier audio {name}: {error}");
    }
    instance
}
fn one_shot<S: Studio>(core: &AudioCore<S>, name: &str) {
    // Original1409d8e50 instantiates at no world position, then starts and
    // releases its temporary wrapper; camera events use this same sequence.
    let _ = instantiate_checked(core, name).start_if_not_playing();
}

/// Native Tool global parameter, including special wall state and all
/// decorator discriminants. None means leave the existing global unchanged.
fn tool_parameter(mode: AppMode, walls: &PublicWalls) -> Option<f32> {
    use AppMode::*;
    Some(match mode {
        Editing(EditSubject::Terrain(_)) | Terrain => 3.,
        Editing(EditSubject::Wall(id)) => match walls.get(WallId(id)) {
            Some(wall) if wall.authority.source_flag_65() != 0 => 14.,
            Some(wall) if wall.shape().tag() < 2 => 0.,
            _ => 15.,
        },
        Editing(_) => 0.,
        Wall(shape) => {
            if shape as u8 == 0 {
                15.
            } else {
                0.
            }
        }
        Path | Cobblestone(_) => 1.,
        Eraser(_) => 2.,
        Decorator(kind) => match kind.native_tag().wrapping_sub(56) {
            0..=2 => 5.,
            3 => 9.,
            4 => 11.,
            5 => 10.,
            7 => 12.,
            _ => return None,
        },
        Color => 6.,
        Replay => return None,
        LoadGame => 4.,
        Water => 7.,
        Tree => 13.,
        Garden => 8.,
        Fences => 14.,
        Photomode => 16.,
        CloneClutter => return None,
    })
}

/// Native reads only the last unread sound, all photomode/mode notices, then
/// drains loading/init/finish notices. Its original unused CmdSuppressMusic
/// writer is omitted: the6319B body never writes to that channel.
#[allow(clippy::too_many_arguments)]
pub fn signifier_audio_system<S: Studio + Send + Sync + 'static>(
    core: Res<AudioCore<S>>,
    mut state: ResMut<SignifierAudioState<S::Event>>,
    mut walls_changed: EventReader<OnWallChanged>,
    mut modes: EventReader<ModeJustChanged>,
    mut loading: EventReader<LoadingGameComplete>,
    mut begin_height: EventReader<InitAdjWallHeightMode>,
    mut finish_height: EventReader<FinishAdjWallHeightMode>,
    mut sounds: EventReader<PlayUiSound>,
    mut photomode: EventReader<PlayPhotomodeSound>,
    walls: Res<PublicWalls>,
) {
    if let Some(event) = walls_changed.read().last() {
        state.last_changed_wall = Some(event.0);
    }
    let mut rotate_requested = false;
    if let Some(sound) = sounds.read().last() {
        match sound.kind {
            0 | 19 => {}
            1 => {
                let context = u64::from_le_bytes(sound._payload[4..12].try_into().unwrap());
                let _ = core.set_parameter("BoopContext", context as f32);
                let _ = state.boop.start_if_not_playing();
            }
            34 => {
                rotate_requested = true;
                let _ = state.rotate_moving_back.start_if_not_playing();
                let position = [4usize, 8, 12].map(|offset| {
                    f32::from_le_bytes(sound._payload[offset..offset + 4].try_into().unwrap())
                });
                let speed = f32::from_le_bytes(sound._payload[..4].try_into().unwrap());
                let instance = state.rotate_moving_back.current_mut();
                let _ = instance.set_position(position);
                let _ = instance.set_parameter("speed", speed);
            }
            kind => one_shot(
                &core,
                UI_SOUND_EVENTS[kind as usize].expect("valid original UI sound variant"),
            ),
        }
    }
    if !rotate_requested {
        let _ = state.rotate_moving_back.stop_now();
    }
    for event in photomode.read() {
        one_shot(
            &core,
            if event.0 == 0 {
                FIRST_PERSON_ON
            } else {
                FIRST_PERSON_OFF
            },
        );
    }
    let mut changed_tool = false;
    for event in modes.read() {
        if event.current == AppMode::Replay || event.previous == AppMode::Replay {
            continue;
        }
        changed_tool = true;
        if let Some(value) = tool_parameter(event.current, &walls) {
            let _ = core.set_parameter(TOOL_PARAM, value);
        }
    }
    if changed_tool {
        one_shot(&core, UI_TOOL_CHANGE_EVENT);
    }
    // Original loading notification is consumed without another side effect.
    loading.read().for_each(drop);
    if begin_height.read().count() != 0 {
        let _ = state.cursor_grab.start_if_not_playing();
    }
    if finish_height.read().count() != 0 {
        let _ = state.cursor_release.start_if_not_playing();
    }
}

#[cfg(test)]
#[path = "signifier_audio_tests.rs"]
mod tests;
