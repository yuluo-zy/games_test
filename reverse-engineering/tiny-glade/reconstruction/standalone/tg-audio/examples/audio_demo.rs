//! Runs a scripted event sequence through a real `bevy_app::App` with `AudioPlugin` and
//! prints the FMOD calls the recording backend saw.
use bevy_app::App;
use glam::Vec3;
use tg_audio::backend::{AudioBackendRes, Listener, RecordingBackend};
use tg_audio::building::{BuildingEntry, BuildingTable};
use tg_audio::events::*;
use tg_audio::music::MusicInputs;
use tg_audio::plugin::AudioPlugin;
use tg_audio::resources::{AppMode, WallVolumeSource};
use tg_audio::update::{AudioSettings, ListenerPose};

fn dump(title: &str, rec: &RecordingBackend) {
    println!("--- {title}");
    for c in rec.take() {
        println!("  {c}");
    }
}

fn main() {
    let rec = RecordingBackend::new();
    let mut app = App::new();
    app.add_plugins(AudioPlugin)
        .insert_resource(AudioBackendRes::new(rec.clone()))
        .insert_resource(AudioSettings { master: 1.0, music: 0.6, sfx: 0.8, amb: 0.5 })
        .insert_resource(AppMode(1))
        .insert_resource(WallVolumeSource(30.0));
    app.world_mut().resource_mut::<BuildingTable>().0.insert(7, BuildingEntry { size_param: 0.25 });

    // Frame 0 runs Startup: boot_audio_update (listener not yet inserted -> runtime update idle) + init systems.
    app.update();
    dump("boot + init (16 events registered, loops created; first frame tails)", &rec);

    let p = Vec3::new(10.0, 0.0, -4.0);
    let w = |kind, field_c, volume| PlayWallAudio { kind, field_c, position: p, volume, ..Default::default() };
    app.world_mut().send_event(w(0, 0.0, 0.9));
    app.world_mut().send_event(w(2, 29.0, 0.9));
    app.update();
    dump("frame 1: wall draw + fence extend (loop kinds)", &rec);

    app.world_mut().send_event(w(3, 0.0, 0.5));
    app.world_mut().send_event(w(1, 7.25, 1.0));
    app.update();
    dump("frame 2: fence appear (one-shot), wall height loop", &rec);

    let w = app.world_mut();
    w.send_event(PlayBuildingAudio { id: 7, position: p, editing_speed: 0.4, kind: 7 });
    w.send_event(PlayStairsAudioCmd { kind: 3, position: p });
    w.send_event(PlayStairsAudioCmd { kind: 0, position: p });
    w.send_event(PlaySheepReactSound { kind: 7 });
    w.send_event(PlayMenuUiSound { kind: 8 });
    app.update();
    dump("frame 3: building attach + stairs place + stairs move + sheep + menu", &rec);

    // Host inputs that switch on the runtime update and music systems.
    app.insert_resource(ListenerPose(Listener { position: Vec3::new(0.0, 5.0, 0.0), forward: Vec3::NEG_Z, up: Vec3::Y }))
        .insert_resource(AppMode(0xa))
        // time_continuous_lerp is UNKNOWN (B 4/5): the default MusicLerp jumps to the target.
        .insert_resource(MusicInputs { time: 12.0, mode_arg1: 0, force_richness_one: false, beat: 0.5 });
    app.update();
    dump("frame 4: audio_update_system + music_audio_system (lerp = identity stand-in)", &rec);
}
