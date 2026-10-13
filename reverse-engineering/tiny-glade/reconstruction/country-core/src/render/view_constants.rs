//! `ViewConstants` / `ViewportConstantBuilder` — per-view GPU constant matrices
//! (WP4-owned; written to the `frame_constants` UBO set1/b0 convention, see
//! `evidence/render-passes/INDEX.md`).
//!
//! Skeleton bundle: `skeleton/B/crates/country-core/src/render/view_constants.rs`
//! (source_sha256 `3c00f8e4…`, 2 procs). Decompiles `A_0x140b18af0.c`,
//! `A_0x140b18f60.c`.

use glam::{Mat4, Vec3};

/// Per-view GPU constants (0x320+ bytes, O: `ViewportConstantBuilder::build`
/// writes up to +0x31c).
///
/// Exact per-matrix column mapping is partially inferred (I): the builder emits
/// view, projection, view-projection, inverse matrices, the previous
/// view-projection (temporal reprojection) and the pixel-offset translation.
#[derive(Clone, Copy, Debug)]
pub struct ViewConstants {
    /// +0x00: view matrix (O).
    pub view: Mat4,
    /// +0x40: projection matrix (O).
    pub proj: Mat4,
    /// +0x80: inverse view matrix (O).
    pub inverse_view: Mat4,
    /// +0xc0: inverse projection (I).
    pub inverse_proj: Mat4,
    /// +0x100: view-projection (I).
    pub view_proj: Mat4,
    /// +0x140: inverse view-projection (I).
    pub inverse_view_proj: Mat4,
    /// +0x180: previous view-projection for temporal reprojection (I).
    pub prev_view_proj: Mat4,
    /// +0x1c0: previous inverse view-projection (I).
    pub prev_inverse_view_proj: Mat4,
    /// +0x200: pixel offset (O: `set_pixel_offset` stores offset + viewport).
    pub pixel_offset: (f32, f32),
    /// +0x210: viewport size in pixels (O).
    pub viewport_size: (u32, u32),
}

impl ViewConstants {
    /// `set_pixel_offset(&mut self, offset, viewport)` (O: 0x140b18af0, 1131B):
    /// computes `ndc = offset * 2 / viewport`, rebuilds the projection matrix
    /// columns (shear on the x/y translation), and recomputes the inverse
    /// projection and view-projection accordingly.
    pub fn set_pixel_offset(&mut self, pixel_offset: (f32, f32), viewport_size: (u32, u32)) {
        let w = viewport_size.0.max(1) as f32;
        let h = viewport_size.1.max(1) as f32;
        let (ox, oy) = (pixel_offset.0 * 2.0 / w, pixel_offset.1 * 2.0 / h);

        // Standard jitter: translate projection translation columns.
        self.proj = self.proj * Mat4::from_translation(glam::Vec3::new(ox, oy, 0.0));
        // O: the routine also rebuilds inverse proj + view-proj.
        self.inverse_proj = self.proj.inverse();
        self.view_proj = self.proj * self.view;
        self.inverse_view_proj = self.view_proj.inverse();

        self.pixel_offset = pixel_offset;
        self.viewport_size = viewport_size;
    }
}

/// Builder that accumulates camera state and produces `ViewConstants`.
#[derive(Clone, Copy, Debug)]
pub struct ViewportConstantBuilder {
    pub eye_position: Vec3,
    pub prev_eye_position: Vec3,
    pub fov_y_radians: f32,
    pub near_plane: f32,
    pub view: Mat4,
    pub proj: Mat4,
    pub prev_view_proj: Mat4,
    pub pixel_offset: (f32, f32),
    pub viewport_size: (u32, u32),
}

impl ViewportConstantBuilder {
    /// `builder()` entry (O: inline host `ViewConstants::builder`).
    pub fn new(eye_position: Vec3, fov_y_radians: f32, near_plane: f32) -> Self {
        ViewportConstantBuilder {
            eye_position,
            prev_eye_position: eye_position,
            fov_y_radians,
            near_plane,
            view: Mat4::IDENTITY,
            proj: Mat4::IDENTITY,
            prev_view_proj: Mat4::IDENTITY,
            pixel_offset: (0.0, 0.0),
            viewport_size: (0, 0),
        }
    }

    /// `pixel_offset` (O name; trivial setter).
    pub fn pixel_offset(mut self, offset: (f32, f32)) -> Self {
        self.pixel_offset = offset;
        self
    }

    /// `build` — produce `ViewConstants` from the accumulated camera matrices
    /// (O: 0x140b18f60, 2339B). Applies the pixel offset as part of the chain.
    pub fn build(&self) -> ViewConstants {
        let view_proj = self.proj * self.view;
        let inverse_view = self.view.inverse();
        let inverse_proj = self.proj.inverse();
        let inverse_view_proj = view_proj.inverse();

        let mut out = ViewConstants {
            view: self.view,
            proj: self.proj,
            inverse_view,
            inverse_proj,
            view_proj,
            inverse_view_proj,
            prev_view_proj: self.prev_view_proj,
            prev_inverse_view_proj: self.prev_view_proj.inverse(),
            pixel_offset: self.pixel_offset,
            viewport_size: self.viewport_size,
        };
        if self.pixel_offset != (0.0, 0.0) {
            out.set_pixel_offset(self.pixel_offset, self.viewport_size);
        }
        out
    }

    /// `eye_position` accessor (O name).
    pub fn eye_position(&self) -> Vec3 {
        self.eye_position
    }
    /// `prev_eye_position` accessor (O name).
    pub fn prev_eye_position(&self) -> Vec3 {
        self.prev_eye_position
    }
    /// `fov_y_radians` accessor (O name).
    pub fn fov_y_radians(&self) -> f32 {
        self.fov_y_radians
    }
    /// `near_plane` accessor (O name).
    pub fn near_plane(&self) -> f32 {
        self.near_plane
    }
}
