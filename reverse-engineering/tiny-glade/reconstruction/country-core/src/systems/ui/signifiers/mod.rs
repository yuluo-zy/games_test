//! `systems/ui/signifiers/mod.rs` — `SignifierEntityBuilder` / `SignifierFactory` /
//! `SignifierAnimation` / `SignifierId` / `SignifierMesh` / etc.
//!
//! Source: B skeleton `crates/country-core/src/systems/ui/signifiers/mod.rs`.
//! PDB named entities: `SignifierAnimation`, `SignifierEntityBuilder`,
//! `SignifierFactory`, `SignifierId`, `SignifierMarkedForDeletion`,
//! `SignifierMesh`, `SignifierMinimumLifespan`, `SignifierParentToVolumeOrigin`.
//!
//! O evidence (Ghidra, B patched EXE):
//! - `SignifierEntityBuilder` = **0x90 B**; `new` @0x14089d9b0 default shader
//!   name string `"vertex_color_shader"` (19 chars) at @0x60..0x70, position
//!   Vec2 @0x7c, and defaults `{0, 1.0f64, 0x100 u16 @0x40, f32 0.2 @0x78}`.
//! - builders: `position` @0x14089dc00 (Vec2 @0x7c), `set_ui_pass` @0x14089dad0
//!   (byte @0x8a ^= 1), `custom_shader` @0x14089db00 (String @0x60..0x70),
//!   `rotation` @0x14089dc30, `scale` @0x14089dc60, `animation_duration`
//!   @0x14089dc90, `minimum_lifespan` @0x14089dcc0, `animation_easing`
//!   @0x14089dcf0, `parent_to_widget` @0x14089dd20.
//! - `build` @0x14089dd50 — mesh selector (`SignifierMesh`, tag in word 2):
//!   0 = named mesh (`AssetMeshLibrary::try_get_handle_by_name`, panic if
//!   missing), 1 = transient mesh (`AssetMeshLibrary::add` via a thread-local
//!   `Cell<(u64,u64)>` instance counter), 2 = `utils::geo::mesh_from_pts`
//!   curve. Then spawns the signifier entity + `SignifierAnimation`.
//! - `SignifierFactory` @0x14089d8b0 = 40 B (copies a slice; 44 callers).

#![allow(dead_code, unused_imports)]

pub mod animation;
pub mod signifier_create;
pub mod signifier_mark_for_delete;

use crate::Gap;

/// `SignifierMesh` (O named entity; mesh selector for `SignifierEntityBuilder`).
#[derive(Clone, Debug, PartialEq)]
pub enum SignifierMesh {
    /// Named mesh via `AssetMeshLibrary::try_get_handle_by_name` (O tag 0).
    Named(u64),
    /// Transient generated mesh via `AssetMeshLibrary::add` (O tag 1).
    Transient,
    /// Curve mesh from points (`utils::geo::mesh_from_pts`, O tag 2).
    Points(Vec<glam::Vec3>),
}

impl Default for SignifierMesh {
    fn default() -> Self {
        SignifierMesh::Transient
    }
}

/// `SignifierAnimation` (O named entity; bevy Component).
#[derive(Clone, Copy, Debug, Default)]
pub struct SignifierAnimation {
    /// `SignifierAnimation::t` (O inline into `adjust_wall_height`).
    pub t: f32,
    pub _gap: Gap,
}

impl SignifierAnimation {
    pub fn t(&self) -> f32 {
        self.t
    }
}

/// `SignifierId` (O named entity; Component).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SignifierId(pub u64);

/// `SignifierMarkedForDeletion` (O named entity; marker Component).
#[derive(Clone, Copy, Debug, Default)]
pub struct SignifierMarkedForDeletion;

/// `SignifierMinimumLifespan` (O named entity; Component).
#[derive(Clone, Copy, Debug, Default)]
pub struct SignifierMinimumLifespan(pub f32);

/// `SignifierParentToVolumeOrigin` (O named entity; marker Component).
#[derive(Clone, Copy, Debug, Default)]
pub struct SignifierParentToVolumeOrigin;

/// `SignifierFactory` (O named entity; 40 B, 44 callers).
#[derive(Clone, Debug, Default)]
pub struct SignifierFactory(pub Gap);

impl SignifierFactory {
    /// `SignifierFactory::new` @0x14089d8b0 (O; 44 callers).
    pub fn new(_data: &[u8]) -> Self {
        SignifierFactory(Gap)
    }
}

/// `SignifierEntityBuilder` (O named entity; 0x90 B builder).
#[derive(Clone, Debug)]
pub struct SignifierEntityBuilder {
    /// mesh selector (O).
    pub mesh: SignifierMesh,
    /// custom shader name (O; default `"vertex_color_shader"`).
    pub custom_shader: String,
    /// position (O @0x7c).
    pub position: glam::Vec2,
    /// rotation (O).
    pub rotation: f32,
    /// scale (O).
    pub scale: f32,
    /// animation duration (O).
    pub animation_duration: f32,
    /// minimum lifespan (O).
    pub minimum_lifespan: f32,
    /// animation easing (O).
    pub animation_easing: f32,
    /// ui-pass flag (O @0x8a).
    pub ui_pass: bool,
    pub _gap: Gap,
}

impl Default for SignifierEntityBuilder {
    fn default() -> Self {
        SignifierEntityBuilder {
            mesh: SignifierMesh::Transient,
            custom_shader: "vertex_color_shader".to_string(),
            position: glam::Vec2::ZERO,
            rotation: 0.0,
            scale: 1.0,
            animation_duration: 1.0,
            minimum_lifespan: 0.0,
            animation_easing: 0.2,
            ui_pass: false,
            _gap: Gap,
        }
    }
}

impl SignifierEntityBuilder {
    /// `new` @0x14089d9b0 (O; 35 callers).
    pub fn new(_factory: &SignifierFactory) -> Self {
        SignifierEntityBuilder::default()
    }
    /// `set_ui_pass` @0x14089dad0 (O).
    pub fn set_ui_pass(mut self, pass: bool) -> Self {
        self.ui_pass = pass;
        self
    }
    /// `position` @0x14089dc00 (O; 24 callers).
    pub fn position(mut self, pos: glam::Vec2) -> Self {
        self.position = pos;
        self
    }
    /// `rotation` @0x14089dc30 (O; 13 callers).
    pub fn rotation(mut self, r: f32) -> Self {
        self.rotation = r;
        self
    }
    /// `scale` @0x14089dc60 (O; 21 callers).
    pub fn scale(mut self, s: f32) -> Self {
        self.scale = s;
        self
    }
    /// `animation_duration` @0x14089dc90 (O; 35 callers).
    pub fn animation_duration(mut self, d: f32) -> Self {
        self.animation_duration = d;
        self
    }
    /// `minimum_lifespan` @0x14089dcc0 (O).
    pub fn minimum_lifespan(mut self, l: f32) -> Self {
        self.minimum_lifespan = l;
        self
    }
    /// `animation_easing` @0x14089dcf0 (O).
    pub fn animation_easing(mut self, e: f32) -> Self {
        self.animation_easing = e;
        self
    }
    /// `parent_to_widget` @0x14089dd20 (O; 1 caller).
    pub fn parent_to_widget(mut self) -> Self {
        self
    }
    /// `custom_shader` @0x14089db00 (O; 2 callers).
    pub fn custom_shader(mut self, name: String) -> Self {
        self.custom_shader = name;
        self
    }
    /// `build` @0x14089dd50 (O; 91 callers): spawn the signifier entity.
    pub fn build(self) {
        todo!("recover SignifierEntityBuilder::build (mesh lookup + signifier spawn + SignifierAnimation)")
    }
}

/// `UiAppendSignifier`（WP20；O：经 `append_only_vec::AppendOnlyVec` 由
/// `ui_camera_mode_gizmos` push；布局 U）。
#[derive(Clone, Debug, Default)]
pub struct UiAppendSignifier(pub crate::Gap);
