//! Shared inputs the audio systems read from other game resources.
use bevy_ecs::prelude::Resource;

/// `AppMode` as read by the audio systems (raw discriminant; the enum itself is not recovered
/// here, UNKNOWN(evidence: A §3b/B §3d)). Real game: a `bevy_state` state.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AppMode(pub u32);

/// Stack-arg volume source of `wall_audio_fmod`.
/// UNKNOWN(evidence: A §3b/§9): real type of the source.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct WallVolumeSource(pub f32);
