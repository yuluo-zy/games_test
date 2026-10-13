//! The original RNG crates, wrapped as ECS resources (no stand-in generator).
//!
//! critters-behavior-analysis "时序/RNG": goal systems use `perchance::PerchanceContext::uniform_f32`
//! (0x141625ae0), carried in a Perchance state handed to the plugin at build time and never reseeded
//! inside a system. critters-extra-analysis "RNG 二分": boid random turns and wren/fish spawn positions
//! use `fastrand::Rng::f32`; the two are never mixed.
//!
//! Both crates are the exact B versions (perchance 0.4.0, fastrand 1.8.0), so for a given seed the
//! bit-stream is the library's own. `fastrand::Rng` is `!Sync` (it holds a `Cell`), hence the `Mutex`.

use bevy_ecs::prelude::Resource;
use perchance::PerchanceContext;
use std::sync::Mutex;

/// `perchance::PerchanceContext` resource (goal systems).
#[derive(Resource, Clone)]
pub struct PerchanceRes(pub PerchanceContext);

impl PerchanceRes {
    pub fn seeded(seed: u128) -> Self {
        Self(PerchanceContext::new(seed))
    }

    /// `PerchanceContext::uniform_f32`, `[0, 1)`.
    pub fn uniform_f32(&mut self) -> f32 {
        self.0.uniform_f32()
    }
}

/// `fastrand::Rng` resource (boid turns, spawn positions).
#[derive(Resource)]
pub struct FastRng(Mutex<fastrand::Rng>);

impl FastRng {
    pub fn seeded(seed: u64) -> Self {
        Self(Mutex::new(fastrand::Rng::with_seed(seed)))
    }

    /// `fastrand::Rng::f32`, `[0, 1)`.
    pub fn f32(&mut self) -> f32 {
        self.0.get_mut().unwrap_or_else(|e| e.into_inner()).f32()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perchance_stream_is_the_library_stream() {
        let mut a = PerchanceRes::seeded(7);
        let mut b = PerchanceContext::new(7);
        for _ in 0..32 {
            assert_eq!(a.uniform_f32(), b.uniform_f32());
        }
    }

    #[test]
    fn fastrand_stream_is_the_library_stream() {
        let mut a = FastRng::seeded(9);
        let b = fastrand::Rng::with_seed(9);
        for _ in 0..32 {
            assert_eq!(a.f32(), b.f32());
        }
    }

    #[test]
    fn seeds_differ_and_range_holds() {
        let mut a = PerchanceRes::seeded(1);
        let mut b = PerchanceRes::seeded(2);
        assert_ne!(a.uniform_f32(), b.uniform_f32());
        let mut f = FastRng::seeded(3);
        for _ in 0..1000 {
            assert!((0.0..1.0).contains(&f.f32()));
            assert!((0.0..1.0).contains(&a.uniform_f32()));
        }
    }
}
