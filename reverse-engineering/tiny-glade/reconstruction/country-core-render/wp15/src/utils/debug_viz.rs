//! `utils/debug_viz.rs` — `DebugLine` / `DebugSphere` / `DebugRectangle2d/3d` /
//! `DebugColor` / `DebugTransform` (debug overlay draw commands).
//!
//! Source: B skeleton `crates/country-core/src/utils/debug_viz.rs`.
//! PDB named entities: `DebugColor`, `DebugLine`, `DebugRectangle2d`,
//! `DebugRectangle3d`, `DebugSphere`, `DebugTransform`.
//! O evidence (Ghidra, B patched EXE): builder-style draw commands with
//! `new`/`color`/`line_width`/`dashed`/`solid`/`set_ui_pass`/`show`
//! (@0x140b02490..@0x140b04f70). Rendered through the `_dashed_line` /
//! `_custom_cursor`-family UI draw passes (`evidence/render-passes/INDEX.md`).
//! Layouts U (typed gap).

#![allow(dead_code, unused_imports)]

use crate::Gap;

/// `DebugColor` (O named entity).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DebugColor(pub glam::Vec4);

impl DebugColor {
    /// `DebugColor::as_array` (O inline).
    pub fn as_array(&self) -> [f32; 4] {
        self.0.to_array()
    }
}

/// `DebugTransform` (O named entity; marker).
#[derive(Clone, Copy, Debug, Default)]
pub struct DebugTransform(pub Gap);

/// `DebugLine` (O named entity).
#[derive(Clone, Debug)]
pub struct DebugLine {
    pub color: DebugColor,
    pub line_width: f32,
    pub dashed: bool,
    pub ui_pass: bool,
    pub _gap: Gap,
}

impl Default for DebugLine {
    fn default() -> Self {
        DebugLine {
            color: DebugColor(glam::Vec4::ONE),
            line_width: 1.0,
            dashed: false,
            ui_pass: false,
            _gap: Gap,
        }
    }
}

impl DebugLine {
    /// `new` @0x140b04e70 (O; 17 callers).
    pub fn new(_from: glam::Vec3, _to: glam::Vec3) -> DebugLine {
        DebugLine::default()
    }
    /// `color` @0x140b04eb0 (O).
    pub fn color(mut self, c: DebugColor) -> Self {
        self.color = c;
        self
    }
    /// `dashed` @0x140b04f10 (O).
    pub fn dashed(mut self) -> Self {
        self.dashed = true;
        self
    }
    /// `line_width` @0x140b04f40 (O).
    pub fn line_width(mut self, w: f32) -> Self {
        self.line_width = w;
        self
    }
    /// `set_ui_pass` @0x140b04ee0 (O).
    pub fn set_ui_pass(mut self, pass: bool) -> Self {
        self.ui_pass = pass;
        self
    }
    /// `show` @0x140b04f70 (O; 40 callers).
    pub fn show(self) {
        todo!("recover DebugLine::show")
    }
}

/// `DebugSphere` (O named entity).
#[derive(Clone, Debug)]
pub struct DebugSphere {
    pub color: DebugColor,
    pub radius: f32,
    pub line_width: f32,
    pub solid: bool,
    pub _gap: Gap,
}

impl Default for DebugSphere {
    fn default() -> Self {
        DebugSphere {
            color: DebugColor(glam::Vec4::ONE),
            radius: 1.0,
            line_width: 1.0,
            solid: false,
            _gap: Gap,
        }
    }
}

impl DebugSphere {
    /// `new` @0x140b02490 (O; 5 callers).
    pub fn new(_center: glam::Vec3, _radius: f32) -> DebugSphere {
        DebugSphere {
            radius: _radius,
            ..DebugSphere::default()
        }
    }
    pub fn color(mut self, c: DebugColor) -> Self {
        self.color = c;
        self
    }
    /// `radius` @0x140b024e0 (O).
    pub fn radius(mut self, r: f32) -> Self {
        self.radius = r;
        self
    }
    /// `solid` @0x140b02500 (O).
    pub fn solid(mut self) -> Self {
        self.solid = true;
        self
    }
    /// `line_width` @0x140b02510 (O).
    pub fn line_width(mut self, w: f32) -> Self {
        self.line_width = w;
        self
    }
    /// `show` @0x140b02520 (O; 14 callers).
    pub fn show(self) {
        todo!("recover DebugSphere::show")
    }
}

/// `DebugRectangle3d` (O named entity).
#[derive(Clone, Debug)]
pub struct DebugRectangle3d {
    pub color: DebugColor,
    pub line_width: f32,
    pub dashed: bool,
    pub _gap: Gap,
}

impl Default for DebugRectangle3d {
    fn default() -> Self {
        DebugRectangle3d {
            color: DebugColor(glam::Vec4::ONE),
            line_width: 1.0,
            dashed: false,
            _gap: Gap,
        }
    }
}

impl DebugRectangle3d {
    /// `new` @0x140b02e10 (O; 4 callers).
    pub fn new(_min: glam::Vec3, _max: glam::Vec3) -> DebugRectangle3d {
        DebugRectangle3d::default()
    }
    pub fn color(mut self, c: DebugColor) -> Self {
        self.color = c;
        self
    }
    pub fn line_width(mut self, w: f32) -> Self {
        self.line_width = w;
        self
    }
    /// `dashed_line` @0x140b02ea0 (O).
    pub fn dashed_line(mut self) -> Self {
        self.dashed = true;
        self
    }
    /// `show` @0x140b02ed0 (O; 5 callers).
    pub fn show(self) {
        todo!("recover DebugRectangle3d::show")
    }
}

/// `DebugRectangle2d` (O named entity).
#[derive(Clone, Debug)]
pub struct DebugRectangle2d {
    pub _gap: Gap,
}

impl Default for DebugRectangle2d {
    fn default() -> Self {
        DebugRectangle2d { _gap: Gap }
    }
}

impl DebugRectangle2d {
    /// `new` (O inline into spatial_hash_debug).
    pub fn new() -> DebugRectangle2d {
        DebugRectangle2d::default()
    }
    /// `show` @0x140b04640 (O).
    pub fn show(self) {
        todo!("recover DebugRectangle2d::show")
    }
}
