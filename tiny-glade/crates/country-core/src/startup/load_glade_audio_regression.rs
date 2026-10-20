// Pending insertion as a cfg(test) module under startup/load_glade_audio.rs.
// Exercises the original session loader, including real resource replacement
// order. This must fail before pending-music-load-ownership.patch is applied.
use super::*;
use crate::resources::{audio_instance::PlaybackState, session::SessionMutator};
use std::{
    convert::Infallible,
    sync::{Arc, Mutex},
};

#[derive(Default)]
struct Backend(Arc<Mutex<Vec<String>>>);
struct Event {
    name: String,
    log: Arc<Mutex<Vec<String>>>,
}
impl Drop for Event {
    fn drop(&mut self) {
        self.log
            .lock()
            .unwrap()
            .push(format!("release:{}", self.name));
    }
}
impl MusicEvent for Event {
    type Error = Infallible;
    fn start(&mut self) -> Result<(), Infallible> {
        self.log
            .lock()
            .unwrap()
            .push(format!("start:{}", self.name));
        Ok(())
    }
    fn stop(&mut self, mode: StopMode) -> Result<(), Infallible> {
        self.log
            .lock()
            .unwrap()
            .push(format!("stop:{mode:?}:{}", self.name));
        Ok(())
    }
    fn set_paused(&mut self, _: bool) -> Result<(), Infallible> {
        Ok(())
    }
    fn paused(&self) -> Result<bool, Infallible> {
        Ok(false)
    }
    fn playback_state(&self) -> Result<PlaybackState, Infallible> {
        Ok(PlaybackState::Stopped)
    }
    fn timeline_position(&self) -> Result<i32, Infallible> {
        Ok(0)
    }
    fn install_beat_callback(&mut self) -> Result<(), Infallible> {
        Ok(())
    }
}
impl MusicBackend for Backend {
    type Event = Event;
    fn instantiate(&self, path: &str) -> Result<Option<Event>, Infallible> {
        self.0.lock().unwrap().push(format!("create:{path}"));
        Ok(Some(Event {
            name: path.into(),
            log: self.0.clone(),
        }))
    }
    fn set_global_parameter(&self, name: &str, value: f32) -> Result<(), Infallible> {
        self.0
            .lock()
            .unwrap()
            .push(format!("parameter:{name}:{value}"));
        Ok(())
    }
}

#[test]
fn session_reload_preserves_stop_release_create_order_and_mutator_semantics() {
    let backend = Backend::default();
    let mut state = MusicAudioState::init(&backend);
    state.player.load_tracks(&backend, &["event:/old-track"]);
    state.new_glade_time = 47.0;
    backend.0.lock().unwrap().clear();
    let root = tempfile::tempdir().unwrap();
    // The path spelling must not select Halloween: only SessionMeta does.
    let path = root.path().join("halloween");
    std::fs::create_dir(&path).unwrap();
    std::fs::write(path.join("audio.json"),
        r#"{"music":["event:/new-track"],"ambience":"event:/new-ambience","select_glade_stinger":"event:/choose"}"#).unwrap();
    let mut meta = SessionMeta::default();
    load_session_audio(&mut state, &backend, &path, &meta);
    let calls = backend.0.lock().unwrap().clone();
    assert_eq!(
        calls
            .iter()
            .filter(|v| v.as_str() == "stop:Immediate:event:/old-track")
            .count(),
        1
    );
    let release = calls
        .iter()
        .position(|v| v == "release:event:/old-track")
        .unwrap();
    let create = calls
        .iter()
        .position(|v| v == "create:event:/new-track")
        .unwrap();
    assert!(
        release < create,
        "old track owner must release before creating replacement"
    );
    assert_eq!(state.player.track_names, ["event:/new-track"]);
    assert_eq!(state.new_glade_time, 0.0);
    assert_eq!((state.richness, state.target_richness), (0.0, 0.0));
    let start = calls
        .iter()
        .position(|v| v == "start:event:/choose")
        .unwrap();
    let release = calls
        .iter()
        .position(|v| v == "release:event:/choose")
        .unwrap();
    assert!(
        start < release,
        "stinger starts then releases without stopping"
    );
    assert!(
        !calls
            .iter()
            .any(|v| v.starts_with("stop:") && v.ends_with(":event:/choose"))
    );
    assert!(calls.contains(&"parameter:IsHalloween:0".into()));
    meta.glade = GladeType::Winter;
    meta.mutators.push(SessionMutator::Halloween2025);
    load_session_audio(&mut state, &backend, &path, &meta);
    assert_eq!(state.player.track_names, HALLOWEEN_MUSIC_EVENTS);
    assert_eq!((state.richness, state.target_richness), (0.25, 0.25));
    let calls = backend.0.lock().unwrap();
    assert!(calls.contains(&"parameter:IsHalloween:1".into()));
    assert!(calls.contains(&"parameter:GladeType:4".into()));
}
