//! `wall_audio_init_system` (0x14094fb80) and `wall_audio_fmod` (0x140950920), A §3 / B §1.
use crate::backend::{AudioBackend, AudioBackendRes, LoopId};
use crate::events::PlayWallAudio;
use crate::math::inverse_lerp_clamped;
use crate::resources::{AppMode, WallVolumeSource};
use crate::wall::*;
use bevy_app::{App, Plugin, Startup, Update};
use bevy_ecs::prelude::*;
use glam::Vec3;

/// AppMode value for which the volume source is forced to 1.0 (`cmp [rax],0xd`, A §3b).
pub const APP_MODE_FIXED_VOLUME: u32 = 13;
/// Divisor of the stack-arg volume source (`[0x1429d18cc]` = 60.0, B §1).
pub const VOLUME_SOURCE_DIVISOR: f32 = 60.0;

/// Values `wall_audio_fmod` reads from other resources, gathered for the `editing_speed` rule.
#[derive(Clone, Copy, Debug)]
pub struct WallCtx {
    pub app_mode: u32,
    pub volume_source: f32,
}

impl WallCtx {
    /// The `editing_speed` passed to `request_play` (xmm14).
    pub fn editing_speed(&self) -> f32 {
        if self.app_mode == APP_MODE_FIXED_VOLUME { 1.0 } else { self.volume_source / VOLUME_SOURCE_DIVISOR }
    }
}

/// WallAudioState resource: 10 loop players + per-loop "should play" flags.
/// `[state+0x280]` powf volume modulation is UNKNOWN(evidence: B §1 tail) and omitted.
#[derive(Resource, Debug)]
pub struct WallAudioState {
    pub loops: [LoopId; 10],
    pub should_play: [bool; 10],
}

/// `wall_audio_init_system` (Startup).
pub fn wall_audio_init_system(mut commands: Commands, mut backend: ResMut<AudioBackendRes>) {
    let mut loops = [0; 10];
    for (i, (name, _)) in WALL_LOOP_NAMES.iter().enumerate() {
        loops[i] = backend.instantiate_loop(name, LOOP_DESC_COUNT, LOOP_DESC_MAX);
    }
    commands.insert_resource(WallAudioState { loops, should_play: [false; 10] });
}

fn play_oneshot(b: &mut dyn AudioBackend, name: &str, pos: Vec3, volume: f32) {
    // instantiate_audio_w_position_and_volume 0x140aa3590
    let id = b.instantiate_audio(name);
    b.set_position(id, pos);
    b.start_if_not_playing(id);
    b.set_volume(id, volume);
}

/// `wall_audio_fmod`: dispatch events, then frame-tail start/stop of every loop.
pub fn wall_audio_fmod(
    mut backend: ResMut<AudioBackendRes>,
    mut state: ResMut<WallAudioState>,
    mut events: EventReader<PlayWallAudio>,
    mode: Res<AppMode>,
    volume_source: Res<WallVolumeSource>,
) {
    let ctx = WallCtx { app_mode: mode.0, volume_source: volume_source.0 };
    let b = backend.get();
    let state = &mut *state;
    for ev in events.read() {
        let Some(row) = wall_kind(ev.kind) else { continue };
        match row.action {
            WallAction::OneShot { name, .. } => play_oneshot(b, name, ev.position, ev.volume),
            WallAction::Loop { loop_idx, param } => {
                let l = state.loops[loop_idx];
                b.loop_request_play(l, ev.position, ev.volume, ctx.editing_speed());
                state.should_play[loop_idx] = true;
                match param {
                    Some(WallParam::Range { name, lo, hi }) => {
                        b.loop_set_parameter(l, name, inverse_lerp_clamped(lo, hi, ev.field_c))
                    }
                    Some(WallParam::HasBricks) => {
                        b.loop_set_parameter(l, "HasBricks", if ev.has_bricks != 0 { 1.0 } else { 0.0 })
                    }
                    None => {}
                }
            }
        }
    }
    // Frame tail 0x14095145e: start/stop by flag, then clear.
    for i in 0..10 {
        if state.should_play[i] {
            b.loop_start_if_not_playing(state.loops[i]);
        } else {
            b.loop_stop(state.loops[i]);
        }
        state.should_play[i] = false;
    }
}

/// Wall part of the audio plugin.
pub struct WallAudioPlugin;

impl Plugin for WallAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayWallAudio>()
            .init_resource::<AppMode>()
            .init_resource::<WallVolumeSource>()
            .add_systems(Startup, wall_audio_init_system.in_set(crate::plugin::AudioInit::Wall))
            .add_systems(Update, wall_audio_fmod.in_set(crate::plugin::AudioFrame::Wall));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::{Call, RecordingBackend};
    use crate::test_util::{app_with, build, frame};

    fn setup() -> (App, RecordingBackend) {
        let (mut app, rec) = app_with(WallAudioPlugin);
        app.insert_resource(AppMode(1)).insert_resource(WallVolumeSource(30.0));
        (app, rec)
    }
    fn ev(kind: u8) -> PlayWallAudio {
        PlayWallAudio { kind, field_c: 0.0, position: Vec3::new(1.0, 2.0, 3.0), volume: 0.7, ..Default::default() }
    }

    #[test]
    fn init_creates_ten_loops_in_order() {
        let (mut app, rec) = build(WallAudioPlugin);
        app.update();
        assert_eq!(app.world().resource::<WallAudioState>().loops, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        for (i, c) in rec.calls().iter().take(10).enumerate() {
            let Call::InstantiateLoop { name, count, max, .. } = c else { panic!() };
            assert_eq!(name, WALL_LOOP_NAMES[i].0);
            assert_eq!((*count, *max), (3, 50));
        }
    }

    #[test]
    fn editing_speed_source() {
        assert_eq!(WallCtx { app_mode: 1, volume_source: 30.0 }.editing_speed(), 0.5);
        assert_eq!(WallCtx { app_mode: 13, volume_source: 99.0 }.editing_speed(), 1.0);
    }

    #[test]
    fn mode_13_forces_editing_speed_one_through_the_system() {
        let (mut app, rec) = setup();
        app.insert_resource(AppMode(13));
        let calls = frame(&mut app, &rec, [ev(0)]);
        assert_eq!(calls[0], Call::LoopRequestPlay { l: 0, pos: Vec3::new(1.0, 2.0, 3.0), volume: 0.7, editing_speed: 1.0 });
    }

    /// One generated test per documented wall kind row.
    macro_rules! wall_kind_test {
        ($($name:ident: $kind:expr),* $(,)?) => {$(
            #[test]
            fn $name() {
                let (mut app, rec) = setup();
                let mut e = ev($kind);
                e.field_c = 100.0;
                e.has_bricks = 1;
                let calls = frame(&mut app, &rec, [e]);
                let row = &WALL_KINDS[$kind];
                match row.action {
                    WallAction::OneShot { name, .. } => {
                        assert_eq!(calls[0], Call::InstantiateAudio { name: name.into(), id: 0 });
                        assert_eq!(calls[1], Call::SetPosition { id: 0, pos: e.position });
                        assert_eq!(calls[2], Call::Start { id: 0 });
                        assert_eq!(calls[3], Call::SetVolume { id: 0, volume: 0.7 });
                        // all ten loops stopped at tail
                        assert_eq!(calls.iter().filter(|c| matches!(c, Call::LoopStop { .. })).count(), 10);
                    }
                    WallAction::Loop { loop_idx, param } => {
                        let l = loop_idx as u32;
                        assert_eq!(calls[0], Call::LoopRequestPlay { l, pos: e.position, volume: 0.7, editing_speed: 0.5 });
                        assert_eq!(param.is_some(), matches!(calls[1], Call::LoopSetParameter { .. }));
                        assert!(calls.contains(&Call::LoopStart { l }));
                        assert_eq!(calls.iter().filter(|c| matches!(c, Call::LoopStop { .. })).count(), 9);
                    }
                }
            }
        )*};
    }
    wall_kind_test!(k0: 0, k1: 1, k2: 2, k3: 3, k4: 4, k5: 5, k6: 6, k7: 7, k8: 8, k9: 9, k10: 10, k11: 11,
        k12: 12, k13: 13, k14: 14, k15: 15, k16: 16, k17: 17, k18: 18, k19: 19, k20: 20, k21: 21, k22: 22, k23: 23);

    #[test]
    fn param_values() {
        let (mut app, rec) = setup();
        let mut e = ev(1);
        e.field_c = 7.25; // (7.25-0.5)/13.5 = 0.5
        let calls = frame(&mut app, &rec, [e]);
        assert_eq!(calls[1], Call::LoopSetParameter { l: 1, name: "WallHeight".into(), value: 0.5 });
        let mut e = ev(7);
        e.field_c = 9.0;
        let calls = frame(&mut app, &rec, [e]);
        assert_eq!(calls[1], Call::LoopSetParameter { l: 3, name: "FenceLength".into(), value: 0.5 });
        let mut e = ev(10);
        e.has_bricks = 0;
        let calls = frame(&mut app, &rec, [e]);
        assert_eq!(calls[1], Call::LoopSetParameter { l: 5, name: "HasBricks".into(), value: 0.0 });
    }

    #[test]
    fn out_of_range_kind_ignored_and_flags_cleared() {
        let (mut app, rec) = setup();
        frame(&mut app, &rec, [ev(24), ev(0)]);
        assert!(app.world().resource::<WallAudioState>().should_play.iter().all(|f| !f));
        // next frame without events stops loop 0
        let calls = frame(&mut app, &rec, std::iter::empty::<PlayWallAudio>());
        assert!(calls.contains(&Call::LoopStop { l: 0 }));
        // kind 24 alone: no request_play at all
        let calls = frame(&mut app, &rec, [ev(24)]);
        assert!(!calls.iter().any(|c| matches!(c, Call::LoopRequestPlay { .. })));
    }

    #[test]
    fn events_are_consumed_once() {
        let (mut app, rec) = setup();
        let calls = frame(&mut app, &rec, [ev(21)]);
        assert!(matches!(calls[0], Call::InstantiateAudio { .. }));
        app.update();
        assert!(!rec.take().iter().any(|c| matches!(c, Call::InstantiateAudio { .. })));
    }
}
