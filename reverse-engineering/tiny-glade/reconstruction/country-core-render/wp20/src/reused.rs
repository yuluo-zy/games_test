//! `reused` — game-side call surfaces of reused third-party dependencies.
//!
//! Per project rules, std / Bevy family / glam / serde and registry deps are
//! REUSED, not re-implemented. This module declares the exact surfaces the WP20
//! skeleton bundles call that are NOT present in the offline registry cache at
//! the original version (bevy_app/bevy_state/bevy_input/bevy_time at 0.16.0 are
//! represented by bevy_ecs-compatible typed-gap facades; puffin / enum-iterator
//! / append-only-vec / egui / suncalc / hexasphere / time are represented by
//! call-surface stubs). The integration owner swaps these for the real crates;
//! none of this is recovered game logic.

#![allow(dead_code, unused_imports)]

// ---------------------------------------------------------------------------
// bevy_app — SubApp / App plugin surface (0.16.0 not in offline cache).
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
// bevy_state — `State<T>` resource + AppLoadingState transitions (reused).
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
// bevy_input — ButtonInput<KeyCode> / KeyCode (reused; 0.16.0 not offline).
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
// bevy_time — `Time` / `Timer` (reused; 0.16.0 not offline).
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
// puffin — profiling (reused; offline-cache env gap).
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
// enum-iterator — `All<T>` iterator for `GameSystemSet` (reused).
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
// append-only-vec — signifier queue (reused).
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
// egui — input-record-replay UI panel (reused).
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
// suncalc — sun position (reused; `suncalc::get_position`).
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
// time — `OffsetDateTime` (reused; date/time helper for the sun position).
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
// hexasphere — subdivision used by `geometry::sphere` (reused; referenced by
// the world_raster/geometry neighbours only).
// ---------------------------------------------------------------------------
pub mod hexasphere {}

// ---------------------------------------------------------------------------
// dolly — camera rig. NOTE: `dolly` 0.6.0 IS in the offline cache and is used
// directly in `camera_rig`; this section only documents the surface.
// ---------------------------------------------------------------------------
