//! Stage 5: update_duck_ripples (0x142314b70, duck.rs 735-796) — GPU glue.
//! Pure part: the two retention factors `1 - exp(-2.5 dt)` / `1 - exp(-5.0 dt)` (expf x2, L746/747).
//! UNKNOWN(evidence: critters-behavior-analysis 1.5): the FilterMap closure (which ducks), the 0x30
//! record layout and what the SSBO shader does with the factors; `DuckState == 2` ducks are collected
//! and no SSBO records are built (`DuckRipples` stands for the data handed to the upload).

use super::DuckState;
use crate::critters::components::Transform;
use crate::env::Time;
use bevy_ecs::prelude::*;
use glam::Vec3;

pub const RIPPLE_DECAY_A: f32 = -2.5;
pub const RIPPLE_DECAY_B: f32 = -5.0;

pub fn ripple_blend_factors(dt: f32) -> (f32, f32) {
    (1.0 - (RIPPLE_DECAY_A * dt).exp(), 1.0 - (RIPPLE_DECAY_B * dt).exp())
}

/// Output of the system: the blend factors and the ripple source ducks (`collect_into Vec<u32>`).
#[derive(Resource, Clone, Debug, Default, PartialEq)]
pub struct DuckRipples {
    pub factors: (f32, f32),
    pub sources: Vec<(Entity, Vec3)>,
}

pub fn update_duck_ripples(
    time: Res<Time>,
    ducks: Query<(Entity, &Transform, &DuckState)>,
    mut ripples: ResMut<DuckRipples>,
) {
    ripples.factors = ripple_blend_factors(time.delta_secs());
    ripples.sources = ducks
        .iter()
        .filter(|(_, _, state)| **state == DuckState::FREE)
        .map(|(e, t, _)| (e, t.translation))
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::critters::duck::{DuckBundle, DuckConfig};
    use crate::testing::base_app;
    use bevy_app::Update;

    #[test]
    fn factors() {
        let (a, b) = ripple_blend_factors(0.0);
        assert_eq!((a, b), (0.0, 0.0));
        let (a, b) = ripple_blend_factors(1.0);
        assert!((a - (1.0 - (-2.5f32).exp())).abs() < 1e-6);
        assert!(b > a && b < 1.0);
    }

    #[test]
    fn system_collects_free_ducks_and_factors() {
        let mut app = base_app(1.0);
        app.init_resource::<DuckRipples>().add_systems(Update, update_duck_ripples);
        let cfg = DuckConfig::default();
        let free = app.world_mut().spawn(DuckBundle::new(Vec3::X, DuckState::FREE, &cfg)).id();
        app.world_mut().spawn(DuckBundle::new(Vec3::Y, DuckState::TRAJ_FORWARD, &cfg));
        app.update();
        let r = app.world().resource::<DuckRipples>();
        assert_eq!(r.sources, vec![(free, Vec3::X)]);
        assert_eq!(r.factors, ripple_blend_factors(1.0));
    }
}
