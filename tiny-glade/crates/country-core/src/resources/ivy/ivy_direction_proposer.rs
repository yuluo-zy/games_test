//! B default 0x1412343b0, get_direction 0x141234590. The original
//! bracket-noise 0.8.2 and perchance 0.4.0 own noise and random generation.
use bevy_ecs::prelude::Resource;
use bracket_noise::prelude::{FastNoise, FractalType, NoiseType};
use glam::{Quat, Vec2, Vec3};
use perchance::PerchanceContext;

pub struct IvyDirectionProposer {
    base: FastNoise,
    rotation: FastNoise,
}

/// B insert_ivy_resources 0x140944132..1dc. The threshold at native +78 is
/// consumed by root spawning; leaf spawning has its own -0.05 threshold.
#[derive(Resource)]
pub struct IvySpawnNoise {
    pub noise: FastNoise,
    pub threshold: f32,
}
impl Default for IvySpawnNoise {
    fn default() -> Self {
        Self::new()
    }
}
impl IvySpawnNoise {
    pub fn new() -> Self {
        let mut noise = FastNoise::seeded(45);
        noise.set_noise_type(NoiseType::PerlinFractal);
        noise.set_fractal_type(FractalType::FBM);
        noise.set_fractal_octaves(3);
        noise.set_fractal_gain(2.0);
        noise.set_fractal_lacunarity(2.0);
        noise.set_frequency(0.3);
        noise.set_seed(123);
        Self {
            noise,
            threshold: -0.02,
        }
    }
}

impl Default for IvyDirectionProposer {
    fn default() -> Self {
        let mut base = FastNoise::seeded(45);
        base.set_noise_type(NoiseType::SimplexFractal);
        base.set_fractal_type(FractalType::FBM);
        base.set_fractal_octaves(2);
        base.set_fractal_gain(2.0);
        base.set_fractal_lacunarity(2.0);
        base.set_frequency(1.0);
        base.set_seed(13);

        let mut rotation = FastNoise::seeded(45);
        rotation.set_noise_type(NoiseType::SimplexFractal);
        rotation.set_fractal_type(FractalType::FBM);
        rotation.set_fractal_octaves(3);
        rotation.set_fractal_gain(1.0);
        rotation.set_fractal_lacunarity(1.3);
        rotation.set_frequency(2.0);
        rotation.set_seed(103);
        Self { base, rotation }
    }
}

impl IvyDirectionProposer {
    /// Position is scaled by 2.8. The seed initializes a temporary perchance
    /// stream on every call; it does not advance the storage's spawn RNG.
    pub fn get_direction(&self, position: Vec3, seed: u64) -> Vec2 {
        let position = position / 2.8;
        let nx = self
            .base
            .get_noise3d(-23.0 + position.x, -40.0 + position.y, -32.5 + position.z);
        let height = 3.0 * position.y;
        let height = if 1.0 > height { 1.0 } else { height };
        let height = if 8.0 < height { 8.0 } else { height };
        let x = height * nx;
        let ny = self
            .base
            .get_noise3d(12.03 + position.x, 67.0 + position.y, 785.0 + position.z);
        let bias =
            self.base
                .get_noise3d(-12.03 + position.x, -0.82 + position.y, -785.0 + position.z);
        let y = bias * 0.6 + (1.0 - bias) * 0.35 + ny;
        let direction = Vec2::new(x, y).normalize();

        let mut rng = PerchanceContext::new(u128::from(seed));
        let x = position.x + -24.0 - rng.uniform_f32() * 100.0;
        let y = position.y + 7.0 + rng.uniform_f32() * 100.0;
        let z = position.z + 5.5 - rng.uniform_f32() * 100.0;
        let noise = self.rotation.get_noise3d(x, y, z);
        let blend = (noise + noise + 1.0) * 0.5;
        // SSE maxss/minss return the second operand on NaN; f32::clamp's
        // ordered comparisons also preserve this NaN input.
        let blend = blend.clamp(0.0, 1.0);
        // 80 degrees is the full angle. Native sin/cos consume half-angle;
        // older prose incorrectly reported the half-angle as the rotation.
        let angle = (blend * 80.0 - (1.0 - blend) * 80.0) * f32::from_bits(0x3c8e_fa35);
        (Quat::from_rotation_z(angle) * direction.extend(0.0))
            .truncate()
            .normalize()
    }
}
