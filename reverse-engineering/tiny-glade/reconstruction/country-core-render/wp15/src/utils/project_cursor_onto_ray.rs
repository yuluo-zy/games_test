//! `utils/project_cursor_onto_ray.rs` — `ProportionalScreenSpace` /
//! `ProportionalScreenSpaceCoord` (screen-space cursor → ray projection).
//!
//! Source: B skeleton `crates/country-core/src/utils/project_cursor_onto_ray.rs`.
//! PDB named entities: `ProportionalScreenSpace`, `ProportionalScreenSpaceCoord`.
//! O evidence (Ghidra, B patched EXE):
//! - `ProportionalScreenSpace::from_window_size` @0x140ac8420 — 16 B
//!   `{ width, height, aspect = w/h, 1.0 }`.
//! - `transform_screen_space` @0x140ac8450 — `utils::mouse_ss_to_cs`.
//! - `project_onto_ray` @0x140ac84a0 — transforms two depth points through the
//!   camera (4×4 view/proj, perspective divide by w), lerps, computes the ray
//!   intersection parameter with a **min clamp of `1e-10`**, and returns the
//!   world point. (Consumed by `ui_raise_shape`, `ui_move_shapes`, ...)

#![allow(dead_code, unused_imports)]

/// `ProportionalScreenSpace` (O: 16 B).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProportionalScreenSpace {
    pub width: f32,
    pub height: f32,
    pub aspect: f32,
    pub one: f32,
}

impl ProportionalScreenSpace {
    /// `from_window_size` @0x140ac8420 (O).
    pub fn from_window_size(w: u32, h: u32) -> ProportionalScreenSpace {
        ProportionalScreenSpace {
            width: w as f32,
            height: h as f32,
            aspect: w as f32 / h as f32,
            one: 1.0,
        }
    }
    /// `transform_screen_space` @0x140ac8450 (O: mouse → canvas space).
    pub fn transform_screen_space(&self, _mouse_ss: glam::Vec2) -> ProportionalScreenSpaceCoord {
        todo!("recover transform_screen_space (utils::mouse_ss_to_cs)")
    }
}

/// `ProportionalScreenSpaceCoord` (O named entity).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ProportionalScreenSpaceCoord(pub glam::Vec2);

impl ProportionalScreenSpaceCoord {
    /// `from_screen_space` (O inline host).
    pub fn from_screen_space(_coord: glam::Vec2) -> ProportionalScreenSpaceCoord {
        ProportionalScreenSpaceCoord(_coord)
    }
    /// `project_onto_ray` @0x140ac84a0 (O): project onto the camera ray,
    /// clamping the ray parameter to `>= 1e-10`.
    pub fn project_onto_ray(
        &self,
        _ray_origin: glam::Vec3,
        _ray_dir: glam::Vec3,
        _camera: glam::Mat4,
        _min_dist: f32,
        _max_dist: f32,
    ) -> glam::Vec3 {
        todo!("recover project_onto_ray (two-depth transform + perspective divide + lerp + 1e-10 clamp)")
    }
}
