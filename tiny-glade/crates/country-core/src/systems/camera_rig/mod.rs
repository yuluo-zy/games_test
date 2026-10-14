//! Only the closed camera-schema leaf is included. Camera mode controllers,
//! projection adapters and the incomplete standalone camera package are excluded.

/// B `default_vertical_fov_degrees` 0x140911e30 (9 bytes), returned f32 = 45.
/// Source: reconstruction/camera/schema.rs, Git 8107b475;
/// evidence/parallel-camera/B-decompile/140911e30.c.
pub fn default_vertical_fov_degrees() -> f32 {
    45.0
}
