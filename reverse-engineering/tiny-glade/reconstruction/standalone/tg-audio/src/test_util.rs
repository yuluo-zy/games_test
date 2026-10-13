//! Test harness: real `bevy_app::App` + recording FMOD backend resource.
use crate::backend::{AudioBackendRes, Call, RecordingBackend};
use bevy_app::{App, Plugin};
use bevy_ecs::event::Event;

/// App with `plugin` and a recording backend; nothing has run yet.
pub fn build(plugin: impl Plugin) -> (App, RecordingBackend) {
    let rec = RecordingBackend::new();
    let mut app = App::new();
    app.add_plugins(plugin).insert_resource(AudioBackendRes::new(rec.clone()));
    (app, rec)
}

/// `build` + one `update()` (runs Startup init) with the log then drained.
pub fn app_with(plugin: impl Plugin) -> (App, RecordingBackend) {
    let (mut app, rec) = build(plugin);
    app.update();
    rec.take();
    (app, rec)
}

pub fn send<E: Event>(app: &mut App, e: E) {
    app.world_mut().send_event(e).expect("event not registered");
}

/// Send `events`, run one frame, return the calls it made.
pub fn frame<E: Event>(app: &mut App, rec: &RecordingBackend, events: impl IntoIterator<Item = E>) -> Vec<Call> {
    for e in events {
        send(app, e);
    }
    app.update();
    rec.take()
}
