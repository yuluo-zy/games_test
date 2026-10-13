//! Game-side resources the critter and water systems read (all supplied by country-core / bevy in
//! the original). They are plain `Resource`s here so a real `bevy_app::App` can drive the systems.
//!
//! UNKNOWN(evidence: critters-behavior-analysis 5): the real `Time` (bevy_time is not a direct
//! dependency of `system-critters`; the doc only gives `*Res<Time> + 0x28 = delta f32`) and
//! `Transform` (bevy_transform, same reason) are therefore minimal local types, see NOTES.md.

use bevy_app::{App, Plugin};
use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};

/// Frame time: the `f32` at `*Res<Time> + 0x28`.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct Time {
    pub delta_secs: f32,
}

impl Time {
    pub fn from_delta(delta_secs: f32) -> Self {
        Self { delta_secs }
    }

    pub fn delta_secs(&self) -> f32 {
        self.delta_secs
    }
}

/// `_DEV_OVERRIDE_IS_DEMO_MODE` (byte @0x143634d90).
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DemoMode(pub bool);

/// `BackupSaveFileKind::is_corrupted()` gate of spawn_ducks.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BackupSaveFileKind {
    pub corrupted: bool,
}

/// `MainCamera`: `mode() == 3` is the first-person follow mode (`eye_position` / `eye_forward`).
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct MainCamera {
    pub mode: u32,
    pub eye: Vec3,
    pub forward: Vec3,
}

impl Default for MainCamera {
    fn default() -> Self {
        Self { mode: 0, eye: Vec3::ZERO, forward: Vec3::Z }
    }
}

impl MainCamera {
    pub const FIRST_PERSON_MODE: u32 = 3;

    pub fn is_mode3(&self) -> bool {
        self.mode == Self::FIRST_PERSON_MODE
    }
}

/// `CursorTerrainRaycast::position()`.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct CursorTerrainRaycast(pub Option<Vec3>);

/// `TerrainHeightsData::sample_at_world_xz<TerrainHeightSsbo>` (0x141246fb0); flat terrain.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct TerrainHeights {
    pub ground_y: f32,
}

impl TerrainHeights {
    pub fn sample_at_world_xz(&self, _x: f32, _z: f32) -> f32 {
        self.ground_y
    }
}

/// `GameConfig::is_within_glade_shape` (0x14090dfe0) / `GladeBorder::is_shape_inside`.
/// UNKNOWN(evidence: critters-behavior-analysis 1.3): the real glade shape; a disc is used.
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct GladeShape {
    pub radius: f32,
}

impl Default for GladeShape {
    fn default() -> Self {
        Self { radius: 65.0 }
    }
}

impl GladeShape {
    pub fn is_within_glade(&self, x: f32, z: f32) -> bool {
        Vec2::new(x, z).length() <= self.radius
    }

    /// `GladeBorder::is_shape_inside(Circle2d { centre, radius })`.
    pub fn is_shape_inside(&self, centre: Vec3, radius: f32) -> bool {
        Vec2::new(centre.x, centre.z).length() + radius <= self.radius
    }
}

/// `RaycastWorld`: sphere obstacles for `legacy_spherecast` (0x1408ccc90) / `Raycasting::intersects_any`.
#[derive(Resource, Clone, Debug, Default)]
pub struct RaycastWorld {
    /// `(centre, radius)`.
    pub spheres: Vec<(Vec3, f32)>,
}

impl RaycastWorld {
    /// Distance to the first hit along `from -> to`.
    pub fn spherecast(&self, from: Vec3, to: Vec3) -> Option<f32> {
        let d = to - from;
        let len = d.length();
        self.spheres
            .iter()
            .filter_map(|&(c, r)| {
                let t = if len > 0.0 { ((c - from).dot(d) / (len * len)).clamp(0.0, 1.0) } else { 0.0 };
                let closest = from + d * t;
                (closest.distance(c) <= r).then(|| from.distance(c) - r)
            })
            .map(|d| d.max(0.0))
            .reduce(f32::min)
    }

    /// `Raycasting::intersects_any(ray from -> to)`.
    pub fn intersects_any(&self, from: Vec3, to: Vec3) -> bool {
        self.spherecast(from, to).is_some()
    }
}

/// A wall segment from `WallSpatialHash::query` / `PublicWalls::get`.
/// UNKNOWN(evidence: critters-behavior-analysis 5): the real 0x38-byte wall record; only the 2D
/// segment and `max_y_gable` (PublicWallState::max_y_gable 0x140a91ef0) are modelled.
#[derive(Clone, Copy, Debug)]
pub struct WallSegment {
    pub a: Vec2,
    pub b: Vec2,
    pub max_y_gable: f32,
}

#[derive(Resource, Clone, Debug, Default)]
pub struct WallSpatialHash {
    pub walls: Vec<WallSegment>,
}

impl WallSpatialHash {
    /// Walls whose segment lies within `radius` of `p` (`WallSpatialHash::query`).
    pub fn query(&self, p: Vec2, radius: f32) -> Vec<WallSegment> {
        self.walls.iter().copied().filter(|w| seg_dist(p, w.a, w.b) <= radius).collect()
    }
}

fn seg_dist(p: Vec2, a: Vec2, b: Vec2) -> f32 {
    let ab = b - a;
    let l2 = ab.length_squared();
    let t = if l2 > 0.0 { ((p - a).dot(ab) / l2).clamp(0.0, 1.0) } else { 0.0 };
    p.distance(a + ab * t)
}

/// Installs default values for every game-side resource that is still missing (idempotent, never
/// overrides a resource the caller inserted first).
pub struct EnvPlugin;

impl Plugin for EnvPlugin {
    fn is_unique(&self) -> bool {
        false
    }

    fn build(&self, app: &mut App) {
        app.init_resource::<Time>()
            .init_resource::<DemoMode>()
            .init_resource::<BackupSaveFileKind>()
            .init_resource::<MainCamera>()
            .init_resource::<CursorTerrainRaycast>()
            .init_resource::<TerrainHeights>()
            .init_resource::<GladeShape>()
            .init_resource::<RaycastWorld>()
            .init_resource::<WallSpatialHash>()
            .init_resource::<crate::water::WorldRaster>();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_and_camera() {
        assert_eq!(Time::from_delta(0.5).delta_secs(), 0.5);
        assert!(!MainCamera::default().is_mode3());
        assert!(MainCamera { mode: 3, ..Default::default() }.is_mode3());
    }

    #[test]
    fn glade_and_terrain() {
        let g = GladeShape::default();
        assert!(g.is_within_glade(10.0, 10.0));
        assert!(!g.is_within_glade(100.0, 0.0));
        assert!(g.is_shape_inside(Vec3::ZERO, 60.0));
        assert!(!g.is_shape_inside(Vec3::new(60.0, 0.0, 0.0), 10.0));
        assert_eq!(TerrainHeights { ground_y: 2.0 }.sample_at_world_xz(5.0, 5.0), 2.0);
    }

    #[test]
    fn spherecast_and_walls() {
        let mut w = RaycastWorld::default();
        w.spheres.push((Vec3::new(5.0, 0.0, 0.0), 1.0));
        assert!((w.spherecast(Vec3::ZERO, Vec3::new(10.0, 0.0, 0.0)).unwrap() - 4.0).abs() < 1e-5);
        assert_eq!(w.spherecast(Vec3::ZERO, Vec3::new(-10.0, 0.0, 0.0)), None);
        assert!(w.intersects_any(Vec3::ZERO, Vec3::new(10.0, 0.0, 0.0)));
        assert_eq!(w.spherecast(Vec3::ZERO, Vec3::ZERO), None);
        let mut h = WallSpatialHash::default();
        h.walls.push(WallSegment { a: Vec2::new(0.0, 5.0), b: Vec2::new(10.0, 5.0), max_y_gable: 3.0 });
        assert_eq!(h.query(Vec2::new(5.0, 4.0), 2.0).len(), 1);
        assert!(h.query(Vec2::new(5.0, -20.0), 2.0).is_empty());
    }

    #[test]
    fn env_plugin_does_not_override() {
        let mut app = App::new();
        app.insert_resource(TerrainHeights { ground_y: 9.0 }).add_plugins(EnvPlugin);
        assert_eq!(app.world().resource::<TerrainHeights>().ground_y, 9.0);
        assert!(app.world().contains_resource::<Time>());
    }
}
