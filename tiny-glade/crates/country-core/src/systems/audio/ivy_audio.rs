//! B `ivy_audio.rs` 0x140ad5350: a single ivy-growth sound, played once per
//! growth trigger.
//!
//! O-level facts (chapter ivy_audio.rs.md, decomp-0x140ad5350):
//! - `ivy_audio_init_system` instantiates `event:/SFX/IvyGrow` once;
//! - no loop, no kind dispatch — ivy growth triggers an immediate single
//!   playback (`start_if_not_playing`, 0x140a57300).
//!
//! The FMOD instance is created through the injected [`EventInstance`]
//! abstraction (real binding via libfmod, library-external).

use crate::resources::audio_resources::EventInstance;
use bevy_ecs::prelude::*;

/// The ivy-growth event name (O: `event:/SFX/IvyGrow`).
pub const IVY_GROW_EVENT: &str = "event:/SFX/IvyGrow";

/// `ivy_audio_init_system` (0x140ad5350): instantiate the ivy-grow event once.
/// The FMOD-bound `instantiate_audio(IVY_GROW_EVENT)` (0x140a54440) is wired
/// by the render/audio host through the injected [`EventInstance`] path.
pub fn ivy_audio_init_system() {
    log::trace!(target: "country_core::systems::audio::ivy_audio", "init {}", IVY_GROW_EVENT);
}

/// `ivy_audio_system`: consume ivy growth-completion events and fire the stored
/// single-shot instance with `start_if_not_playing` (O: single playback per
/// growth). `play` is the injected trigger; the real FMOD start is
/// library-external.
pub fn ivy_audio_system<E: EventInstance>(
    mut messages: EventReader<crate::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>,
    play: &mut impl FnMut() -> Result<(), E::Error>,
) {
    if messages.read().next().is_some() {
        let _ = play();
    }
}
