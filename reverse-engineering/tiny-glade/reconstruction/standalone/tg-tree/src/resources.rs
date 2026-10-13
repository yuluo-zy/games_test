//! Resources the tree tool reads but that are owned by other modules of the game
//! (country-core etc.). Only the fields/methods the tree tool touches are mirrored;
//! the real types are UNKNOWN(evidence: tree-process-cmds-analysis.md 剩余unknown).

use bevy_ecs::prelude::*;
use glam::Vec2;

/// ActiveSession[+0x58] state object. Only the three queried methods are
/// evidenced (is_dragging 0x14092e6a0, deny_load 0x140a9fa70, is_corrupted 0x140ae8320).
#[derive(Resource, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ActiveSession {
    pub dragging: bool,
    pub deny_load: bool,
    pub corrupted: bool,
}

/// GameTime resource: delta f32 @+0x28.
#[derive(Resource, Debug, Clone, Copy, PartialEq)]
pub struct GameTime {
    pub delta: f32,
}

/// Square water raster, bilinear sampling; `0.0 < sample(x,z)` means "in water"
/// (bilinear_sample_at_world_xz<RasterBufferRef<512>> 0x140a14df0). The 130.0 world
/// size is evidenced (0x142a7b658); the exact texel-centre mapping is an ASSUMPTION
/// (UNKNOWN(evidence: tree-process-cmds-analysis.md "WaterRaster")).
#[derive(Resource, Debug, Clone)]
pub struct WaterRaster {
    pub res: usize,
    pub data: Vec<f32>,
}

impl WaterRaster {
    pub const WORLD_SIZE: f32 = 130.0;

    pub fn filled(res: usize, v: f32) -> Self {
        Self { res, data: vec![v; res * res] }
    }

    /// Build from a world-space function evaluated at texel centres.
    pub fn from_fn(res: usize, f: impl Fn(f32, f32) -> f32) -> Self {
        let mut data = Vec::with_capacity(res * res);
        for j in 0..res {
            for i in 0..res {
                let x = ((i as f32 + 0.5) / res as f32 - 0.5) * Self::WORLD_SIZE;
                let z = ((j as f32 + 0.5) / res as f32 - 0.5) * Self::WORLD_SIZE;
                data.push(f(x, z));
            }
        }
        Self { res, data }
    }

    /// Water value 1.0 inside an xz rectangle, else 0.0.
    pub fn rect(res: usize, min: Vec2, max: Vec2) -> Self {
        Self::from_fn(res, |x, z| if x >= min.x && x <= max.x && z >= min.y && z <= max.y { 1.0 } else { 0.0 })
    }

    fn at(&self, x: usize, y: usize) -> f32 {
        self.data[y.min(self.res - 1) * self.res + x.min(self.res - 1)]
    }

    pub fn sample(&self, x: f32, z: f32) -> f32 {
        let max = (self.res - 1) as f32;
        let fx = ((x / Self::WORLD_SIZE + 0.5) * self.res as f32 - 0.5).clamp(0.0, max);
        let fz = ((z / Self::WORLD_SIZE + 0.5) * self.res as f32 - 0.5).clamp(0.0, max);
        let (x0, z0) = (fx.floor() as usize, fz.floor() as usize);
        let (tx, tz) = (fx - x0 as f32, fz - z0 as f32);
        let top = self.at(x0, z0) * (1.0 - tx) + self.at(x0 + 1, z0) * tx;
        let bot = self.at(x0, z0 + 1) * (1.0 - tx) + self.at(x0 + 1, z0 + 1) * tx;
        top * (1.0 - tz) + bot * tz
    }
}

/// Terrain height source wrapper (`TerrainHeightsData<TerrainHeightSsbo>`::sample_at_world_xz
/// 0x141246fb0). The real type samples a GPU-side buffer; here a sampling closure.
#[derive(Resource)]
pub struct TerrainHeights {
    sampler: Box<dyn Fn(f32, f32) -> f32 + Send + Sync>,
}

impl TerrainHeights {
    pub fn new(sampler: impl Fn(f32, f32) -> f32 + Send + Sync + 'static) -> Self {
        Self { sampler: Box::new(sampler) }
    }
    pub fn flat(h: f32) -> Self {
        Self::new(move |_, _| h)
    }
    pub fn sample(&self, x: f32, z: f32) -> f32 {
        (self.sampler)(x, z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn water_rasters() {
        assert_eq!(WaterRaster::filled(4, 0.5).sample(1.0, 2.0), 0.5);
        let r = WaterRaster::rect(512, Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));
        assert_eq!(r.sample(5.0, 5.0), 1.0);
        assert_eq!(r.sample(-1.0, 5.0), 0.0);
        let mut g = WaterRaster::filled(4, 0.0);
        g.data[3 * 4 + 3] = 1.0;
        assert_eq!(g.sample(-60.0, -60.0), 0.0);
        assert!(g.sample(64.0, 64.0) > 0.9);
        assert!((WaterRaster::filled(4, 0.25).sample(3.0, -7.0) - 0.25).abs() < 1e-6);
    }

    #[test]
    fn terrain_heights_sampler() {
        assert_eq!(TerrainHeights::flat(2.0).sample(1.0, 1.0), 2.0);
        assert_eq!(TerrainHeights::new(|x, z| x + z).sample(1.0, 2.0), 3.0);
    }
}
