//! Music: init 0x140a71f20 (A §6a), music_audio_system 0x140a72810 (B §4).
use crate::backend::{AudioBackendRes, InstanceId};
use crate::resources::AppMode;
use bevy_app::{App, Plugin, Startup, Update};
use bevy_ecs::prelude::*;

pub const MUSIC_EVENT: &str = "event:/Music/mus_GladePreview";
pub const AMB_EVENT: &str = "event:/Amb/GladePreview";
pub const PARAM_MUSIC_RICHNESS: &str = "MusicRichness";
pub const PARAM_NEW_GLADE_TIME: &str = "NewGladeTime";
/// Parameter name present in the string table, usage UNKNOWN(evidence: B §4).
pub const PARAM_PREVIEW_GLADE_TYPE: &str = "PreviewGladeType";

pub const RICHNESS_TIME_DIV: f32 = 120.0; // 0x1429cd0f8
pub const NEW_GLADE_TIME_MAX: f32 = 0.8; // 0x142a16cb8
pub const MODE_FORCE_RICHNESS_ONE: u32 = 0x13;

/// Snap values away from exactly 0.25/0.5/0.75 (B §4).
pub fn snap_off(x: f32) -> f32 {
    if (0.24..0.25).contains(&x) {
        x.max(0.251)
    } else if (0.49..=0.5).contains(&x) {
        x.max(0.501)
    } else if (0.74..=0.75).contains(&x) {
        x.max(0.751)
    } else {
        x
    }
}

/// MusicAudioState fields @+0x1c0/+0x1c4/+0x1c8.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct MusicFields {
    pub smoothed: f32,
    pub accum: f32,
    pub new_glade_time: f32,
}

/// Per-frame resource inputs (B §4 table, arg roles). AppMode (arg4) is the shared
/// [`AppMode`] resource. The host inserts this resource; `music_audio_system` is skipped while
/// it is absent.
#[derive(Resource, Clone, Copy, Debug)]
pub struct MusicInputs {
    /// time resource +0x28.
    pub time: f32,
    /// arg1 +0 (== 0x13 forces richness target 1.0).
    pub mode_arg1: u32,
    /// arg6 +0x100: forces the emitted MusicRichness to 1.0.
    pub force_richness_one: bool,
    /// arg7 +0x58: beat parameter for MusicPlayer::update.
    pub beat: f32,
}

/// `time_continuous_lerp` (0x140c904e0) as `fn(prev_smoothed, target) -> next`.
/// UNKNOWN(evidence: B §4/§5): k (0.99?) / formula. Default: jump straight to the target.
#[derive(Resource, Clone, Copy)]
pub struct MusicLerp(pub fn(f32, f32) -> f32);

impl Default for MusicLerp {
    fn default() -> Self {
        MusicLerp(|_prev, target| target)
    }
}

#[derive(Resource, Clone, Copy, Debug)]
pub struct MusicHandles {
    pub music: InstanceId,
    pub amb: InstanceId,
}

/// `music_audio_init_system`: instantiate music + ambience events and start them.
/// UNKNOWN(evidence: A §6a): the third instance (a "short name" copy of mus_GladePreview) and
/// which instance is the MusicPlayer event; we treat `music` as the MusicPlayer target.
pub fn music_audio_init_system(mut commands: Commands, mut backend: ResMut<AudioBackendRes>) {
    let music = backend.instantiate_audio(MUSIC_EVENT);
    let amb = backend.instantiate_audio(AMB_EVENT);
    backend.start_if_not_playing(music);
    backend.start_if_not_playing(amb);
    commands.insert_resource(MusicHandles { music, amb });
    commands.insert_resource(MusicFields::default());
}

/// `music_audio_system`. [`MusicLerp`] stands in for `time_continuous_lerp` (UNKNOWN).
pub fn music_audio_system(
    mut backend: ResMut<AudioBackendRes>,
    h: Res<MusicHandles>,
    mut st: ResMut<MusicFields>,
    inp: Res<MusicInputs>,
    mode: Res<AppMode>,
    lerp: Res<MusicLerp>,
) {
    let accum = snap_off((st.accum + inp.time / RICHNESS_TIME_DIV).clamp(0.0, 1.0));
    st.accum = accum;
    let target = if inp.mode_arg1 == MODE_FORCE_RICHNESS_ONE { 1.0 } else { accum };
    st.smoothed = snap_off((lerp.0)(st.smoothed, target).clamp(0.0, 1.0));
    let richness = if inp.force_richness_one { 1.0 } else { st.smoothed };
    backend.set_parameter(h.music, PARAM_MUSIC_RICHNESS, richness);

    st.new_glade_time = (inp.time + st.new_glade_time).clamp(0.0, NEW_GLADE_TIME_MAX);
    backend.set_parameter(h.music, PARAM_NEW_GLADE_TIME, st.new_glade_time);

    if (mode.0 | 8) == 0xa {
        backend.music_player_update(inp.beat);
    } else {
        backend.music_player_pause();
    }
}

/// Music part of the audio plugin.
pub struct MusicAudioPlugin;

impl Plugin for MusicAudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AppMode>()
            .init_resource::<MusicLerp>()
            .add_systems(Startup, music_audio_init_system.in_set(crate::plugin::AudioInit::Music))
            .add_systems(
                Update,
                music_audio_system
                    .in_set(crate::plugin::AudioFrame::Music)
                    .run_if(resource_exists::<MusicInputs>.and(resource_exists::<MusicHandles>)),
            );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::Call;
    use crate::test_util::{app_with, build};

    fn inp() -> MusicInputs {
        MusicInputs { time: 12.0, mode_arg1: 0, force_richness_one: false, beat: 0.5 }
    }

    /// App after init, with AppMode 0xa and `i` as the inputs; log drained.
    fn setup(i: MusicInputs) -> (App, crate::backend::RecordingBackend) {
        let (mut app, rec) = app_with(MusicAudioPlugin);
        app.insert_resource(AppMode(0xa)).insert_resource(i);
        (app, rec)
    }
    fn fields(app: &App) -> MusicFields {
        *app.world().resource::<MusicFields>()
    }

    #[test]
    fn init_names() {
        let (mut app, rec) = build(MusicAudioPlugin);
        app.update();
        let calls = rec.calls();
        assert_eq!(calls[0], Call::InstantiateAudio { name: MUSIC_EVENT.into(), id: 0 });
        assert_eq!(calls[1], Call::InstantiateAudio { name: AMB_EVENT.into(), id: 1 });
        assert_eq!(calls.len(), 4);
        // no MusicInputs yet: music_audio_system is skipped
        assert!(!calls.iter().any(|c| matches!(c, Call::SetParameter { .. })));
    }

    #[test]
    fn snap_boundaries() {
        assert_eq!(snap_off(0.245), 0.251);
        assert_eq!(snap_off(0.25), 0.25); // outside [0.24,0.25)
        assert_eq!(snap_off(0.5), 0.501);
        assert_eq!(snap_off(0.495), 0.501);
        assert_eq!(snap_off(0.75), 0.751);
        assert_eq!(snap_off(0.3), 0.3);
        assert_eq!(snap_off(0.9), 0.9);
    }

    #[test]
    fn richness_accumulates_by_time_over_120() {
        let (mut app, rec) = setup(inp());
        app.update();
        let st = fields(&app);
        assert!((st.accum - 0.1).abs() < 1e-6);
        assert_eq!(rec.calls()[0], Call::SetParameter { id: 0, name: "MusicRichness".into(), value: st.smoothed });
    }

    #[test]
    fn mode_0x13_targets_one_and_flag_forces_one() {
        let mut i = inp();
        i.mode_arg1 = 0x13;
        let (mut app, rec) = setup(i);
        app.update();
        assert_eq!(fields(&app).smoothed, 1.0);
        let (mut app, rec2) = setup(MusicInputs { mode_arg1: 0, force_richness_one: true, ..i });
        let _ = rec;
        app.update();
        assert_eq!(rec2.calls()[0], Call::SetParameter { id: 0, name: "MusicRichness".into(), value: 1.0 });
        assert!(fields(&app).smoothed < 1.0); // state itself not forced
    }

    #[test]
    fn accum_clamped_to_one() {
        let (mut app, _rec) = setup(inp());
        app.world_mut().resource_mut::<MusicFields>().accum = 0.99;
        app.update();
        assert_eq!(fields(&app).accum, 1.0);
    }

    #[test]
    fn new_glade_time_caps_at_0_8() {
        let (mut app, rec) = setup(inp());
        app.update();
        assert_eq!(fields(&app).new_glade_time, 0.8);
        assert_eq!(rec.calls()[1], Call::SetParameter { id: 0, name: "NewGladeTime".into(), value: 0.8 });
        let (mut app, _rec) = setup(MusicInputs { time: 0.3, ..inp() });
        app.update();
        assert!((fields(&app).new_glade_time - 0.3).abs() < 1e-6);
    }

    #[test]
    fn custom_lerp_resource_is_used() {
        let (mut app, _rec) = setup(inp());
        app.insert_resource(MusicLerp(|prev, target| prev + (target - prev) * 0.5));
        app.update();
        // accum 0.1 -> smoothed 0.05
        assert!((fields(&app).smoothed - 0.05).abs() < 1e-6);
    }

    #[test]
    fn app_mode_gate_update_vs_pause() {
        for (mode, update) in [(0xa, true), (0x2, true), (0x3, false)] {
            let (mut app, rec) = setup(inp());
            app.insert_resource(AppMode(mode));
            app.update();
            let calls = rec.calls();
            let last = calls.last().unwrap();
            assert_eq!(matches!(last, Call::MusicUpdate { beat } if *beat == 0.5), update, "mode {mode}");
            assert_eq!(matches!(last, Call::MusicPause), !update);
        }
    }
}
