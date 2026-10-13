//! `reused` — game-side call surfaces of reused third-party/local dependencies.
//!
//! Per project rules, std / Bevy / glam / serde and registry deps are REUSED,
//! not re-implemented. This module declares the exact surfaces the WP16 skeleton
//! bundles call, so the standalone gate compiles offline even though several of
//! these crates are not in the offline registry cache. The integration owner
//! swaps these for the real crates; none of this is recovered game logic.
//!
//! Covered reused surfaces:
//!   - `watch`     hotwatch::GenericHotwatch + notify backend (MeshWatch/
//!                 ConfigWatch/TextureWatch file watching).
//!   - `rapier`    rapier3d QueryPipeline / ColliderSet + parry3d SharedShape /
//!                 Qbvh (RaycastWorld glue).
//!   - `steam`     steamworks Client / Input (startup_steam / update_steam_inputs).
//!   - `noise`     bracket-noise FastNoise (IvyDirectionProposer).
//!   - `rng`       perchance PerchanceContext (ivy spawn/grow randomness).
//!   - `bvh`       obvhs::cwbvh builder (GpuBlBvh build).
//!   - `tabloid`   pub/sub diff channels (ColliderDiff / DirtyTiles).
//!   - `vec`       append-only-vec (debug_viz queue).
//!   - `line_raster` conservative-line-raster (DirtyTiles stroke).
//!   - `map`       frozen::FrozenMap + nohash-hasher (ssbo/vao registries).
//!   - `heap`      priority-queue (stairs distortion overlap ordering).

#![allow(dead_code)]

// ---------------------------------------------------------------------------
// hotwatch / notify — file-watch surface.
// ---------------------------------------------------------------------------
pub mod watch {
    use std::path::Path;

    /// `hotwatch::GenericHotwatch` (reused). `new_with_custom_delay` / `watch`.
    pub struct GenericHotwatch(pub crate::Gap);

    impl GenericHotwatch {
        pub fn new_with_custom_delay(_delay: core::time::Duration) -> Result<Self, ()> {
            Ok(GenericHotwatch(crate::Gap))
        }
        pub fn watch<P: AsRef<Path>, F: FnMut(std::path::PathBuf) + Send + 'static>(
            &self,
            _path: P,
            _cb: F,
        ) -> Result<(), ()> {
            Ok(())
        }
    }
    /// `notify` watcher backend tag used by hotwatch.
    pub type ReadDirectoryChangesWatcher = ();
}

// ---------------------------------------------------------------------------
// rapier3d / parry3d — physics query surface.
// ---------------------------------------------------------------------------
pub mod rapier {
    use glam::Vec3;

    /// `rapier3d::geometry::ColliderHandle` (u32 newtype).
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct ColliderHandle(pub u32);

    /// `rapier3d::pipeline::QueryPipeline` (reused; owns the QBVH).
    #[derive(Default)]
    pub struct QueryPipeline(pub crate::Gap);

    impl QueryPipeline {
        pub fn intersections_with_shape<F>(&self, _shape: &dyn core::any::Any, _f: F) {}
        pub fn cast_shape<F>(&self, _f: F) {}
    }

    /// `rapier3d::geometry::ColliderSet`.
    #[derive(Default)]
    pub struct ColliderSet(pub crate::Gap);

    impl ColliderSet {
        pub fn get_mut_internal(&self) -> Option<&Collider> {
            None
        }
    }

    /// Opaque collider (reused type).
    pub struct Collider(pub crate::Gap);
    /// `parry3d::shape::SharedShape` — the shared triangle/convex shape the game
    /// wraps in `RaycastMeshData`.
    #[derive(Clone, Debug)]
    pub struct SharedShape(pub crate::Gap);

    /// Isometry placeholder (glam 0.29 has no root `Isometry3`; the game builds
    /// rapier compound parts from translation+rotation — typed gap).
    #[derive(Clone, Copy, Debug)]
    pub struct Isometry(pub crate::Gap);

    impl SharedShape {
        pub fn trimesh(
            _verts: Vec<Vec3>,
            _indices: Vec<[u32; 3]>,
        ) -> Result<Self, ()> {
            Ok(SharedShape(crate::Gap))
        }
        pub fn convex_hull(_verts: Vec<Vec3>) -> Result<Self, ()> {
            Ok(SharedShape(crate::Gap))
        }
        pub fn cuboid(_half: Vec3) -> Self {
            SharedShape(crate::Gap)
        }
        pub fn ball(_r: f32) -> Self {
            SharedShape(crate::Gap)
        }
        pub fn compound(_parts: Vec<(Isometry, Self)>) -> Result<Self, ()> {
            Ok(SharedShape(crate::Gap))
        }
        /// as-any downcast for debug visualization (visualize_shape).
        pub fn as_any(&self) -> &dyn core::any::Any {
            self
        }
    }

    /// `parry3d::query::visitors` traversal entry point used by mesh-mesh
    /// intersection and iter_shape_overlaps (reused).
    pub fn traverse_bvtt<F>(_a: &QBvh, _b: &QBvh, _f: F) {}

    /// `parry3d::partitioning::QBvh` (reused).
    pub struct QBvh(pub crate::Gap);
}

// ---------------------------------------------------------------------------
// steamworks — Steam client / input surface.
// ---------------------------------------------------------------------------
pub mod steam {
    /// `steamworks::Client`/`ClientManager` handle (reused).
    #[derive(Clone, Debug, Default)]
    pub struct ClientManager(pub crate::Gap);
    pub struct Input(pub crate::Gap);
    pub struct ActionSetHandle(pub u64);
    pub struct DigitalActionHandle(pub u64);
    pub struct AnalogActionHandle(pub u64);

    /// `steamworks::callback::register_callback<T>` (reused).
    pub fn register_callback<T, F>(_f: F) -> Result<(), ()> {
        Ok(())
    }
    pub mod utils {
        pub struct FloatingGamepadTextInputDismissed;
    }
    pub mod screenshots {
        pub struct ScreenshotRequested;
    }
}

// ---------------------------------------------------------------------------
// bracket-noise — FastNoise field (IvyDirectionProposer).
// ---------------------------------------------------------------------------
pub mod noise {
    /// `bracket_noise::fastnoise::FastNoise` (reused; 2D/3D gradient noise).
    pub struct FastNoise(pub crate::Gap);

    impl FastNoise {
        pub fn seeded(_seed: u64) -> Self {
            FastNoise(crate::Gap)
        }
        pub fn set_fractal_octaves(&mut self, _n: i32) {}
        pub fn set_fractal_gain(&mut self, _g: f32) {}
        pub fn set_seed(&mut self, _seed: u64) {}
        pub fn get_noise3d(&self, _x: f32, _y: f32, _z: f32) -> f32 {
            0.0
        }
        pub fn get_noise2d(&self, _x: f32, _y: f32) -> f32 {
            0.0
        }
    }
}

// ---------------------------------------------------------------------------
// perchance — PerchanceContext randomness (ivy spawn/grow).
// ---------------------------------------------------------------------------
pub mod rng {
    /// `perchance::PerchanceContext` (reused) — the game's small RNG context.
    pub struct PerchanceContext(pub crate::Gap);

    impl PerchanceContext {
        pub fn uniform_f32(&mut self) -> f32 {
            0.0
        }
        pub fn usize_less_than(&mut self, _n: usize) -> usize {
            0
        }
    }
}

// ---------------------------------------------------------------------------
// obvhs — CWBVH builder (GpuBlBvh).
// ---------------------------------------------------------------------------
pub mod bvh {
    /// `obvhs::cwbvh::builder::build_cwbvh_from_tris` (reused local crate).
    pub struct CwbvhBuilder {
        pub node_count: u32,
        pub leaf_count: u32,
    }
    pub fn build_cwbvh_from_tris(
        _tris: &[f32],
        _config: &CwbvhConfig,
    ) -> CwbvhBuilder {
        CwbvhBuilder {
            node_count: 0,
            leaf_count: 0,
        }
    }
    pub struct CwbvhConfig;
}

// ---------------------------------------------------------------------------
// tabloid — pub/sub diff channels.
// ---------------------------------------------------------------------------
pub mod tabloid {
    /// `tabloid::Publisher<T>` / `Subscriber<T>` (reused pub/sub).
    pub struct Publisher<T>(pub crate::Gap, pub core::marker::PhantomData<fn() -> T>);
    impl<T> Publisher<T> {
        pub fn publish(&self, _diff: T) {}
    }
    pub struct Subscriber<T>(pub crate::Gap, pub core::marker::PhantomData<fn() -> T>);
    impl<T> Subscriber<T> {
        pub fn take(&mut self) -> Option<T> {
            None
        }
    }
    /// `publish` free-fn surface used by ColliderDiff / DirtyTiles channels.
    pub fn publish<T>(_publisher: &Publisher<T>, _diff: T) {}
}

// ---------------------------------------------------------------------------
// append-only-vec — debug_viz queue.
// ---------------------------------------------------------------------------
pub mod vec {
    /// `append_only_vec::AppendOnlyVec<T>` (reused) — the debug-line queue.
    pub struct AppendOnlyVec<T>(pub crate::Gap, pub core::marker::PhantomData<fn() -> T>);
    impl<T> AppendOnlyVec<T> {
        pub fn push(&self, _item: T) {}
        pub fn iter(&self) -> core::slice::Iter<'_, T> {
            [].iter()
        }
    }
}

// ---------------------------------------------------------------------------
// conservative-line-raster — stroke cells.
// ---------------------------------------------------------------------------
pub mod line_raster {
    use glam::Vec2;
    /// `conservative_line_raster::conservative_plot_line(a, b, cb)` — plot the
    /// cells a conservative line covers (reused local crate).
    pub fn conservative_plot_line<F>(_a: Vec2, _b: Vec2, _cb: F) {}
}

// ---------------------------------------------------------------------------
// frozen + nohash-hasher — write-once registries.
// ---------------------------------------------------------------------------
pub mod map {
    /// `frozen::FrozenMap<K,V>`：见 `crate::frozen_map`（可工作的占位，ssbo 库已使用）。
    pub use crate::frozen_map::FrozenMap;
    /// `nohash_hasher::NoHashHasher<T>` (u64 pre-hashed ids).
    pub struct NoHashHasher<T>(pub crate::Gap, pub core::marker::PhantomData<fn() -> T>);
}

// ---------------------------------------------------------------------------
// priority-queue — stairs distortion ordering.
// ---------------------------------------------------------------------------
pub mod heap {
    /// `priority_queue::PriorityQueue<K, OrderedFloat<f32>>` (reused).
    pub struct PriorityQueue<K>(pub crate::Gap, pub core::marker::PhantomData<fn() -> K>);
    impl<K> PriorityQueue<K> {
        pub fn push(&mut self, _k: K, _prio: super::ordered_float::OrderedFloat<f32>) {}
        pub fn pop(&mut self) -> Option<(K, super::ordered_float::OrderedFloat<f32>)> {
            None
        }
    }
}

// Re-export helper for the above (OrderedFloat used in collision world).
pub mod ordered_float {
    #[derive(Clone, Copy, PartialEq, PartialOrd)]
    pub struct OrderedFloat<T>(pub T);
}

/// `utils::calculate_hash` (game-owned generic hash helper, referenced by
/// collision world / components). Implemented with `std::hash` (game-owned).
pub fn calculate_hash<T: core::hash::Hash>(t: &T) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hasher;
    let mut h = DefaultHasher::new();
    t.hash(&mut h);
    h.finish()
}

// ---------------------------------------------------------------------------
// WP20: bevy family surfaces absent from the offline registry cache at 0.16.0
// (bevy_app / bevy_state / bevy_input / bevy_time) + egui / enum-iterator /
// append-only-vec / puffin / suncalc / time. Game-side call surfaces only —
// none of this is recovered game logic; the integration owner swaps these for
// the real crates when they become available offline.
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// bevy_app — SubApp / App plugin surface.
// ---------------------------------------------------------------------------
pub mod bevy_app {
    /// `bevy_app::App` (reused; typed-gap surface for plugin `build`).
    #[derive(Default)]
    pub struct App(pub crate::Gap);
    impl App {
        pub fn add_systems(&mut self, _systems: impl Sized) {}
        pub fn insert_resource(&mut self, _r: impl Sized) {}
    }
}

// ---------------------------------------------------------------------------
// bevy_state — `State<T>` resource + AppLoadingState transitions.
// ---------------------------------------------------------------------------
pub mod bevy_state {
    use crate::Gap;
    /// `bevy_state::state::resources::State<S>` (reused; typed-gap).
    pub struct State<S> {
        _marker: core::marker::PhantomData<fn() -> S>,
        pub raw: Gap,
    }
}

// ---------------------------------------------------------------------------
// bevy_input — ButtonInput<KeyCode> / KeyCode.
// ---------------------------------------------------------------------------
pub mod bevy_input {
    use crate::Gap;
    use serde::{Deserialize, Serialize};

    /// `bevy_input::keyboard::KeyCode` (reused). Only the codes WP20 systems
    /// read are declared; the rest are typed gaps.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum KeyCode {
        KeyL,
        KeyO,
        KeyE,
        Escape,
        ShiftLeft,
        ControlLeft,
        AltLeft,
    }

    /// `bevy_input::button_input::ButtonInput<T>` (reused).
    #[derive(bevy_ecs::resource::Resource, Clone, Default)]
    pub struct ButtonInput<T> {
        _marker: core::marker::PhantomData<fn() -> T>,
        pub raw: Gap,
    }
    impl<T: Copy + Eq + core::hash::Hash> ButtonInput<T> {
        pub fn just_pressed(&self, _key: T) -> bool {
            false
        }
        pub fn pressed(&self, _key: T) -> bool {
            false
        }
        pub fn any_just_pressed(&self, _keys: impl IntoIterator<Item = T>) -> bool {
            false
        }
        pub fn reset_all(&mut self) {}
    }
}

// ---------------------------------------------------------------------------
// bevy_time — `Time` / `Timer`.
// ---------------------------------------------------------------------------
pub mod bevy_time {
    /// `bevy_time::Time` (reused; typed-gap).
    #[derive(Clone, Default)]
    pub struct Time(pub crate::Gap);
    impl Time {
        pub fn delta_secs(&self) -> f32 {
            0.0
        }
    }
}

// ---------------------------------------------------------------------------
// puffin — profiling (offline-cache env gap).
// ---------------------------------------------------------------------------
pub mod puffin {
    pub struct Scope;
    impl Scope {
        pub fn new(_name: &'static str) -> Self {
            Scope
        }
    }
    pub fn are_scopes_on() -> bool {
        false
    }
}

// ---------------------------------------------------------------------------
// enum-iterator — `All<T>` iterator for `GameSystemSet`.
// ---------------------------------------------------------------------------
pub mod enum_iterator {
    pub struct All<T> {
        _marker: core::marker::PhantomData<fn() -> T>,
    }
    impl<T> Iterator for All<T> {
        type Item = T;
        fn next(&mut self) -> Option<T> {
            None
        }
    }
    pub trait Sequence {
        const CARDINALITY: usize = 0;
        fn all() -> All<Self>
        where
            Self: Sized,
        {
            All {
                _marker: core::marker::PhantomData,
            }
        }
    }
}

// ---------------------------------------------------------------------------
// append-only-vec — signifier queue.
// ---------------------------------------------------------------------------
pub mod append_only_vec {
    use crate::Gap;
    #[derive(bevy_ecs::resource::Resource, Default)]
    pub struct AppendOnlyVec<T> {
        _marker: core::marker::PhantomData<fn() -> T>,
        pub raw: Gap,
    }
    impl<T> AppendOnlyVec<T> {
        pub fn push(&self, _value: T) {}
    }
}

// ---------------------------------------------------------------------------
// egui — input-record-replay UI panel.
// ---------------------------------------------------------------------------
pub mod egui {
    use crate::Gap;
    pub struct Ui(pub Gap);
    impl Ui {
        pub fn checkbox(&mut self, _label: &str, _checked: &mut bool) {}
        pub fn label(&mut self, _text: &str) {}
        pub fn button(&mut self, _label: &str) -> bool {
            false
        }
        pub fn horizontal_with_main_wrap_dyn<F: FnOnce(&mut Ui)>(&mut self, _f: F) {}
    }
}

// ---------------------------------------------------------------------------
// suncalc — sun position (`suncalc::get_position`).
// ---------------------------------------------------------------------------
pub mod suncalc {
    pub struct SunPosition {
        pub elevation: f64,
        pub azimuth: f64,
    }
    /// `suncalc::get_position` (reused) — returns elevation/azimuth (radians)
    /// for the given date/time and coordinates.
    pub fn get_position(
        _date_time: &super::time::OffsetDateTime,
        _latitude: f64,
        _longitude: f64,
    ) -> SunPosition {
        SunPosition {
            elevation: 0.0,
            azimuth: 0.0,
        }
    }
}

// ---------------------------------------------------------------------------
// time — `OffsetDateTime` (date/time helper for the sun position).
// ---------------------------------------------------------------------------
pub mod time {
    /// `time::OffsetDateTime::unix_timestamp` (reused; typed-gap).
    #[derive(Clone, Copy, Debug)]
    pub struct OffsetDateTime(pub i64);
    impl OffsetDateTime {
        pub fn unix_timestamp(self) -> i64 {
            self.0
        }
    }
}

// ---------------------------------------------------------------------------
// hexasphere — subdivision used by `geometry::sphere` (referenced only).
// ---------------------------------------------------------------------------
pub mod hexasphere {}
