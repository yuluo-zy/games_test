//! Tiny Glade `critters` and `water` modules ported onto the original B dependencies
//! (`bevy_ecs`/`bevy_app` 0.16.0, `glam` 0.29.3, `perchance` 0.4.0, `fastrand` 1.8.0, `lerp`, `itertools`,
//! `log`, `smart-default`): every business function is a real bevy_ecs system, registered by
//! [`critters::CrittersPlugin`] / [`water::WaterPlugin`].
//!
//! Evidence: `evidence/multi-baseline/{critters-behavior,critters-extra,water-cpu,water-extra}-analysis.md`
//! plus `evidence/toolchain/*`. Anything the evidence leaves open is marked
//! `// UNKNOWN(evidence: <doc>)` and no behaviour is built on it (it is injected through a
//! config resource instead). See `NOTES.md` for the implemented/unknown/test map.

pub mod critters;
pub mod env;
pub mod rng;
pub mod sample;
#[cfg(test)]
mod testing;
pub mod water;
