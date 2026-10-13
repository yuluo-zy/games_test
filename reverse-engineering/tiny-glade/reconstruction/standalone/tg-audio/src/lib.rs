//! Tiny Glade's `country_core::systems::audio` event -> FMOD glue (B baseline) on the real
//! `bevy_ecs`/`bevy_app` 0.16.0 and `glam` 0.29.3. FMOD is not reusable here, so all FMOD
//! traffic goes through [`backend::AudioBackend`], held as the [`backend::AudioBackendRes`]
//! resource. Add [`plugin::AudioPlugin`] to an `App`.
//!
//! Sources: `evidence/multi-baseline/audio-events-analysis.md` ("A") and
//! `audio-extra-analysis.md` ("B"). Items the docs mark unknown carry an
//! `UNKNOWN(evidence: ...)` marker and no behavior.
pub mod backend;
pub mod building;
pub mod clutter;
pub mod events;
pub mod math;
pub mod menu;
pub mod music;
pub mod plugin;
pub mod registry;
pub mod resources;
pub mod sheep;
pub mod stairs;
pub mod update;
pub mod wall;
pub mod wall_system;

#[cfg(test)]
mod test_util;
