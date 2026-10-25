//! `leaf_particles.rs` — W18 leaf particles (0x1408f4d00).
//!
//! URI `/leaf_particles.cs.hlsl`. `utils::get_res` + `Ord::min` + uniform +
//! dispatch (O). `LeafParticleSsbo` / `ParticleEmitterSsbo` /
//! `LoadedParticleEmitterCount` (O names; layout U, `resources::` domain).

use crate::typed_gap::Gap;

pub struct LeafParticleSsbo(pub Gap);
pub struct ParticleEmitterSsbo(pub Gap);
pub struct LoadedParticleEmitterCount(pub Gap);

pub fn leaf_particles_compute(world: &mut Gap, _emitters: &Gap, _max: u32) {
    let _ = world;
    unimplemented!()
}
