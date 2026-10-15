//! Only the closed camera-schema leaf is included. Camera mode controllers,
//! projection adapters and the incomplete standalone camera package are excluded.
//! `main_camera` / `free_camera` hold the pieces of `MainCamera` verified against the
//! B EXE (see `R9_camera.md`); the per-mode bodies and `main_camera_update` are still open.

pub mod free_camera;
pub mod main_camera;
pub mod normal_body;

/// B `default_vertical_fov_degrees` 0x140911e30 (9 bytes), returned f32 = 45.
/// Source: reconstruction/camera/schema.rs, Git 8107b475;
/// evidence/parallel-camera/B-decompile/140911e30.c.
pub fn default_vertical_fov_degrees() -> f32 {
    45.0
}
