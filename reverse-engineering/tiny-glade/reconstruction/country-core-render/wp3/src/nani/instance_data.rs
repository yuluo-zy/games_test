//! instance_data.rs — 每类 nani 实例的 GPU 布局类型（WP3）。
//!
//! 机器码锚点（B patched EXE）：
//! - `CurvedHalfTimberInstanceData::{new_curved_xy, new_curved_xz, new_straight}` 0x14098f690/0x14098f800/0x14098f970。
//! - `impl$122::{register_required_components, clone_behavior}` 0x140990a80/0x140990af0。
//! - `PlantInstanceData::marker` 0x14003bb80（与多 `*_marker` 共址）。
//! - 尺寸证据：`impl$14::copy<T>`（gpu_world.rs 0x1415724f0/5b0/660/760）元素尺寸。
//!
//! 这些 `#[repr(C)]` 结构体是 `_nani_cull_and_bucket{...}` 与 `nani_uber` 光栅 shader 消费的
//! per-class GPU 实例布局（见 evidence/render-passes/INDEX.md §3）。字段名/顺序多为 typed gap；
//! 完整契约见 contracts/instance_data.md。

#![allow(dead_code, unused_imports)]

use core::any::Any;
use glam::{Vec2, Vec3};

use crate::render::nani::gpu_world::NaniInstanceData;

/// Uber 实例（0x40 = 64B，O；`update_instance_data<Uber>` 0x14156db40 写 0x40）。
/// 关联 `NaniUberInstanceFlags` 位域（modular_bitfield 0.11.2 依赖证据）。
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct UberInstanceData {
    /// typed gap：{transform 行/平移, 缩放, 色彩, 位域, ...} 字段 U（对齐 0x40 布局）。
    pub _gap: [u32; 16],
}

/// 纯色实例（0x40 = 64B，O）。
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SolidVertexColorInstanceData {
    pub _gap: [u32; 16],
}

/// 纯色砖实例（0x40，O）。
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SolidVertexColorBricksInstanceData {
    pub _gap: [u32; 16],
}

/// 纯色窗实例（0x40，O）。
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SolidVertexColorWindowInstanceData {
    pub _gap: [u32; 16],
}

/// 窗玻璃实例（0x40，O）；关联 `WindowGlassFlags` 位域。
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SolidVertexColorWindowGlassInstanceData {
    pub _gap: [u32; 16],
}

/// albedo 贴图实例（0x40，O）。
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SolidAlbedoMapInstanceData {
    pub _gap: [u32; 16],
}

/// albedo + 自发光实例（0x40，O）。
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SolidAlbedoMapEmissiveInstanceData {
    pub _gap: [u32; 16],
}

/// 灯笼自发光实例（0x40，O）。
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct LanternEmissiveInstanceData {
    pub _gap: [u32; 16],
}

/// 阳台门实例（0x40，O）。
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct BalconyDoorInstanceData {
    pub _gap: [u32; 16],
}

/// 烟实例（0x40，O）。
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SmokeInstanceData {
    pub _gap: [u32; 16],
}

/// 石地板实例（0x40，O）。
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct StoneFloorInstanceData {
    pub _gap: [u32; 16],
}

/// 布料实例（0x40，O）。
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ClothInstanceData {
    pub _gap: [u32; 16],
}

/// 预置花实例（0x40，O）。
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PreplacedFlowerInstanceData {
    pub _gap: [u32; 16],
}

/// 植物实例（0x30 = 48B，O；`copy<Plant>` 0x1415725b0 写 0x30）。
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PlantInstanceData {
    pub _gap: [u32; 12],
}

impl PlantInstanceData {
    /// 常量 marker（0x14003bb80，1B，O）。
    pub fn marker() -> Self {
        Self { _gap: [0; 12] }
    }
}

/// 楼梯地板实例（0x30，O）。
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct StairsFloorInstanceData {
    pub _gap: [u32; 12],
}

/// 石膏实例（0x30，O）。
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PlasterInstanceData {
    pub _gap: [u32; 12],
}

/// 速度-only 实例（0x60 = 96B，O；`copy<VelocityOnly>` 0x141572760 写 0x60）。
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VelocityOnlyInstanceData {
    /// typed gap：{prev/current transform, ...} 字段 U。
    pub _gap: [u32; 24],
}

/// 鹪鹩实例（0x70 = 112B，O；`copy<Wren>` 0x141572660 写 0x70）。
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WrenInstanceData {
    pub _gap: [u32; 28],
}

/// 哥特窗砖实例（0x70，O；`update_instance_data<Gothic>` 0x14156ebd0 写 0x70）。
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GothicWindowBricksInstanceData {
    pub _gap: [u32; 28],
}

/// 弯曲半木实例（0x40 = 64B，O；构造器见下）。
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CurvedHalfTimberInstanceData {
    /// +0x00..+0x0b：来源复制（color_id / atlas 引用；O）。
    pub source: [u32; 3],
    /// +0x0c：u32 = 0。
    pub pad_c: u32,
    /// +0x10：start 参数（new_curved_xy = `start - half_len*0.5`）。
    pub start_param: f32,
    /// +0x14：end 参数（`mid + half_len*0.5`）。
    pub end_param: f32,
    /// +0x18：高度起点。
    pub height_start: f32,
    /// +0x1c：高度终点。
    pub height_end: f32,
    /// +0x20：段数（new_curved_xy 为 power-of-two；new_straight = caps*2 + 1/2）。
    pub segment_count: i32,
    /// +0x24..+0x2b：打包字段（typed gap；new_straight 写 `0x3dc28f5c00000001`）。
    pub packed_a: u64,
    /// +0x2c..+0x33：打包字段（new_straight 写 `0x100000000`）。
    pub packed_b: u64,
    /// +0x34..+0x3b：u64 = 0。
    pub pad_34: u64,
    /// +0x3c：u32 = 0。
    pub pad_3c: u32,
}

impl CurvedHalfTimberInstanceData {
    /// xy 平面弯曲构造（0x14098f690，行 343，O）：
    /// `segments = next_power_of_two(ceil((ABS(height_end - height_start) * (mid + half_len*0.5) + 1.0) / 0.6))`。
    pub fn new_curved_xy(
        _source: &[u32; 3],
        mid: f32,
        half_len: f32,
        start: f32,
        end: f32,
        subdiv: u32,
    ) -> Self {
        let f5 = mid + half_len * 0.5;
        let raw = ((end - start).abs() * f5 + 1.0) / 0.6;
        let seg = (raw.ceil().max(0.0)) as u32;
        let count = seg.max(2).next_power_of_two();
        Self {
            source: *_source,
            pad_c: 0,
            start_param: start - half_len * 0.5,
            end_param: f5,
            height_start: start,
            height_end: end,
            segment_count: count as i32,
            packed_a: 0,
            packed_b: 0,
            pad_34: 0,
            pad_3c: subdiv,
        }
    }

    /// xz 平面弯曲构造（0x14098f800，行 375，O，对称）。
    pub fn new_curved_xz(
        _source: &[u32; 3],
        _mid: f32,
        _half_len: f32,
        _start: f32,
        _end: f32,
        _subdiv: u32,
    ) -> Self {
        todo!("recover new_curved_xz (363B)")
    }

    /// 直段构造（0x14098f970，行 407，O）：
    /// `segment_count = caps * 2 + (has_mid_cap ? 2 : 1)`；packed_a=`0x3dc28f5c00000001`；packed_b=`0x100000000`。
    pub fn new_straight(
        _source: &[u32; 3],
        start: f32,
        end: f32,
        height_start: f32,
        height_end: f32,
        caps: u8,
        has_mid_cap: bool,
    ) -> Self {
        Self {
            source: *_source,
            pad_c: 0,
            start_param: start,
            end_param: end,
            height_start,
            height_end,
            segment_count: (caps as i32) * 2 + if has_mid_cap { 2 } else { 1 },
            packed_a: 0x3dc28f5c00000001,
            packed_b: 0x100000000,
            pad_34: 0,
            pad_3c: 0,
        }
    }

    /// mesh atlas part 索引（`curved_halftimber_mesh_atlas_part_idx`，PDB LF_FUNC_ID 证据）。
    pub fn curved_halftimber_mesh_atlas_part_idx(&self) -> u32 {
        todo!()
    }
}

/// 弯曲半木端帽（O 名称，字段 U）。
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CurvedHalfTimberCaps {
    pub _gap: [u32; 4],
}

/// Uber 位域（modular_bitfield 0.11.2；字段 U）。
#[derive(Clone, Copy, Default)]
pub struct NaniUberInstanceFlags {
    pub bits: u8,
}

/// 窗玻璃位域（字段 U）。
#[derive(Clone, Copy, Default)]
pub struct WindowGlassFlags {
    pub bits: u8,
}

// --- NaniInstanceData trait 的每个类型实现（尺寸证据 O；行为见 gpu_world.rs impl$14）---

macro_rules! impl_nani_instance_data {
    ($($t:ty,)*) => {$(
        impl NaniInstanceData for $t {
            fn type_name(&self) -> &'static str { stringify!($t) }
            fn as_any(&self) -> &dyn Any { self }
            fn as_any_mut(&mut self) -> &mut dyn Any { self }
            fn gpu_flush(&self) {}
            fn rg_handle(&self) -> u64 { todo!() }
            fn import_to_rg(&self, _rg: &mut rhapsody_rg::render_graph::RenderGraph) { todo!() }
            fn export_from_rg(&self, _rg: &mut rhapsody_rg::render_graph::RenderGraph) { todo!() }
            fn copy(&self, _src: u32, _dst: u32) { todo!("element size = size_of::<Self>(); CAS min/max") }
        }
    )*};
}

impl_nani_instance_data! {
    UberInstanceData,
    SolidVertexColorInstanceData,
    SolidVertexColorBricksInstanceData,
    SolidVertexColorWindowInstanceData,
    SolidVertexColorWindowGlassInstanceData,
    SolidAlbedoMapInstanceData,
    SolidAlbedoMapEmissiveInstanceData,
    LanternEmissiveInstanceData,
    BalconyDoorInstanceData,
    SmokeInstanceData,
    StoneFloorInstanceData,
    ClothInstanceData,
    PreplacedFlowerInstanceData,
    PlantInstanceData,
    StairsFloorInstanceData,
    PlasterInstanceData,
    VelocityOnlyInstanceData,
    WrenInstanceData,
    GothicWindowBricksInstanceData,
    CurvedHalfTimberInstanceData,
}

/// 组件注册（0x140990a80，行 284，O）：为 instance_data 类型注册 `InstanceDataMarker<T>`。
pub fn register_required_components(_world: &mut bevy_ecs::world::World) {
    todo!("bevy component registration; see contracts/instance_data.md §3")
}
