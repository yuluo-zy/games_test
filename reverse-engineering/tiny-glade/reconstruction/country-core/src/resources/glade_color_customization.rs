//! glade_color_customization.rs — 谷地自定义配色（WP21）。
//!
//! O：`GladeColorCustomization::default`（0x8c5600，82B）：+0x0=0、+0x4=0、
//! +0x8=0x186a3、+0x18=0x800000000、+0x20=0、+0x40=0(u16)、+0x42=1、+0x48/0x50/0x58=0、
//! +0x60=1.0。字段语义 U（serde 存档）。`StairCustomization`。

use bevy_ecs::resource::Resource;
use serde::{Deserialize, Serialize};

use crate::typed_gap::Gap;

/// `GladeColorCustomization`（O：glade 配色资源；默认布局字节 O）。
#[derive(Resource, Clone, Debug, Serialize, Deserialize)]
pub struct GladeColorCustomization {
    /// +0x0 u32=0（O）。
    pub a: u32,
    /// +0x4 u8=0（O）。
    pub b: u8,
    /// +0x8 u32=0x186a3（O；I 语义）。
    pub c: u32,
    /// +0x18 u64=0x800000000（O）。
    pub d: u64,
    /// +0x20 u32=0（O）。
    pub e: u32,
    /// +0x40 u16=0（O）。
    pub f: u16,
    /// +0x42 u8=1（O）。
    pub g: u8,
    /// +0x48/0x50/0x58 u64=0（O）。
    pub h: u64,
    pub i: u64,
    pub j: u64,
    /// +0x60 f32=1.0（O）。
    pub k: f32,
    /// 未映射区（U）。
    pub _gap: Gap,
}

impl Default for GladeColorCustomization {
    fn default() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0x186a3,
            d: 0x800000000,
            e: 0,
            f: 0,
            g: 1,
            h: 0,
            i: 0,
            j: 0,
            k: 1.0,
            _gap: Gap::default(),
        }
    }
}

/// `StairCustomization`（O 名；阶梯配色，U）。
#[derive(Resource, Clone, Debug, Default, Serialize, Deserialize)]
pub struct StairCustomization(pub Gap);
