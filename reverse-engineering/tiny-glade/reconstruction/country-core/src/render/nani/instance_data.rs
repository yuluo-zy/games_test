//! instance_data.rs — 每类 nani 实例的 GPU 布局类型（B 阶段按机器码重写，n_world）。
//!
//! 机器码锚点（B patched EXE cf048e27…）：
//! * `CurvedHalfTimberInstanceData::{new_curved_xy, new_curved_xz, new_straight}` 0x14098f690 / 0x14098f800 / 0x14098f970
//!   （逐字节核对 0x40 的写入偏移，O）。
//! * `impl$122::{register_required_components, clone_behavior}` 0x140990a80 / 0x140990af0 = bevy 的
//!   `#[derive(Component, Clone)]` 展开（`component_clone_via_clone<CurvedHalfTimberInstanceData>`，O）。
//! * `UberInstanceData::marker` 0x14003bb80（1B `ret`，与其他 ZST 返回函数 ICF 合并；返回 `InstanceDataMarker<Uber…>`）。
//! * 元素尺寸（O，逐个 `update_instance_data<T>` 的最大写偏移 + `impl$14::copy` 的元素步长核对）：
//!   0x40：SolidVertexColor / Bricks / Window / WindowGlass / SolidAlbedoMap / SolidAlbedoMapEmissive / LanternEmissive /
//!   BalconyDoor / Smoke / StoneFloor / Cloth / Uber / CurvedHalfTimber；
//!   0x30：Plant / Plaster / StairsFloor / **PreplacedFlower**（旧骨架误写 0x40）；
//!   0x60：VelocityOnly；0x70：Wren / GothicWindowBricks。
//!
//! 除 `CurvedHalfTimberInstanceData` 外的字段布局属于 `nani_update_*` 系统（systems.rs，n_sys）与 shader 侧，
//! 这里只保证尺寸、对齐、`StorageElement` 名字（= `TypedStorageBuffer::new<T>` 的调试名，O）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::component::Component;
use bevy_ecs::world::World;
use core::any::Any;
use glam::Vec3;

use crate::render::nani::gpu_world::InstanceDataMarker;
use crate::render::ssbo::StorageElement;

/// 定义一个按 `$words` 个 u32 对齐的纯数据实例类型（字段 U）。
macro_rules! raw_instance_data {
    ($($(#[$doc:meta])* $name:ident, $words:expr;)*) => {$(
        $(#[$doc])*
        #[repr(C)]
        #[derive(Clone, Copy, Debug, Default, PartialEq)]
        pub struct $name {
            /// 字段布局 U（只有总尺寸是 O）。
            pub raw: [u32; $words],
        }

        impl StorageElement for $name {
            /// `TypedStorageBuffer::new<T>` 的调试名（O）。
            const NAME: &'static str = stringify!($name);
        }
    )*};
}

raw_instance_data! {
    /// Uber 实例（0x40，O；`update_instance_data<Uber>` 0x14156db40）。关联 `NaniUberInstanceFlags`。
    UberInstanceData, 16;
    /// 纯色顶点色实例（0x40，O）。
    SolidVertexColorInstanceData, 16;
    /// 纯色砖实例（0x40，O）。
    SolidVertexColorBricksInstanceData, 16;
    /// 纯色窗实例（0x40，O）。
    SolidVertexColorWindowInstanceData, 16;
    /// 窗玻璃实例（0x40，O）；关联 `WindowGlassFlags`。
    SolidVertexColorWindowGlassInstanceData, 16;
    /// albedo 贴图实例（0x40，O）。
    SolidAlbedoMapInstanceData, 16;
    /// albedo + 自发光实例（0x40，O）。
    SolidAlbedoMapEmissiveInstanceData, 16;
    /// 灯笼自发光实例（0x40，O）。
    LanternEmissiveInstanceData, 16;
    /// 阳台门实例（0x40，O）。
    BalconyDoorInstanceData, 16;
    /// 烟实例（0x40，O）。
    SmokeInstanceData, 16;
    /// 石地板实例（0x40，O）。
    StoneFloorInstanceData, 16;
    /// 布料实例（0x40，O）。
    ClothInstanceData, 16;
    /// 预置花实例（**0x30**，O；`update_instance_data<PreplacedFlower>` 0x141571de0 最大写偏移 0x20+0x10）。
    PreplacedFlowerInstanceData, 12;
    /// 植物实例（0x30，O）。
    PlantInstanceData, 12;
    /// 楼梯地板实例（0x30，O）。
    StairsFloorInstanceData, 12;
    /// 石膏实例（0x30，O）。
    PlasterInstanceData, 12;
    /// 速度-only 实例（0x60，O；`copy<VelocityOnly>` 0x141572760 步长 `3 << 5`）。
    VelocityOnlyInstanceData, 24;
    /// 鹪鹩实例（0x70，O；`copy` 0x141572660 `imul 0x70`）。
    WrenInstanceData, 28;
    /// 哥特窗砖实例（0x70，O；`update_instance_data<Gothic…>` 0x14156ebd0 `imul 0x70`）。
    GothicWindowBricksInstanceData, 28;
}

impl UberInstanceData {
    /// `UberInstanceData::marker`（0x14003bb80，O：1B `ret`）：ZST marker 组件。
    pub fn marker() -> InstanceDataMarker<Self> {
        InstanceDataMarker::default()
    }
}

/// 弯曲半木实例（0x40，O）。唯一以 ECS 组件存储的实例数据：`nani_update_curved_halftimber` 查询
/// `&mut CurvedHalfTimberInstanceData` 直写（O，systems.rs）。`#[derive(Component, Clone)]` 对应
/// `impl$122::{register_required_components, clone_behavior}`（`component_clone_via_clone`，O）。
///
/// 布局（O，来自三个构造器的逐偏移写入）：
/// | 偏移 | 字段 | xy | xz | straight |
/// |---|---|---|---|---|
/// | 0x00 | `position: Vec3`（12B） | 拷贝 | 拷贝 | 拷贝 |
/// | 0x0c | `pad_0c` | 0 | 0 | 0 |
/// | 0x10 | `x0` | `center - half` | 同 xy | 参数 a |
/// | 0x14 | `x1` | `center + half` | 同 xy | 参数 b |
/// | 0x18 | `y0` | 参数 | 参数 | 参数 |
/// | 0x1c | `y1` | 参数 | 参数 | 参数 |
/// | 0x20 | `w20: u32` | 0 | 0 | `2n + 1 + has_mid_cap` |
/// | 0x24 | `w24: u32` | 0 | 1 | 1 |
/// | 0x28 | `w28: f32` | 参数 | 参数 | `0x3dc28f5c` = 0.095 |
/// | 0x2c | `w2c: u32` | 0 | `u8` 参数 | 0 |
/// | 0x30 | `w30: u32` | `ilog2(next_pow2(n)).clamp(2, 5)` | 同 xy | 1 |
/// | 0x34/0x38/0x3c | 0 | 0 | 0 | 0 |
///
/// 字段语义名（x0/x1/y0/y1/w*）为 I；偏移与值为 O。
#[repr(C)]
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct CurvedHalfTimberInstanceData {
    pub position: Vec3,
    pub pad_0c: u32,
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
    pub w20: u32,
    pub w24: u32,
    pub w28: f32,
    pub w2c: u32,
    pub w30: u32,
    pub w34: u32,
    pub w38: u32,
    pub w3c: u32,
}

impl StorageElement for CurvedHalfTimberInstanceData {
    const NAME: &'static str = "CurvedHalfTimberInstanceData";
}

/// 弯曲段的细分等级（O：`new_curved_xy`/`xz` 共用的内联序列）：
/// `raw = (|y1 - y0| * x1 + 1.0) / 0.6`（`0.6f32 = 0x3f19999a`）→ `ceilf`（IAT `ucrt ceilf`，O）→
/// 饱和转 u32（`cvttss2si` + `ucomiss` 钳制；NaN/负数 → 0）→ `next_power_of_two`（0/1 → 1）→
/// `ilog2`（结果为 0 时走 `panic_for_nonpositive_argument`，对应 n > 2^31 溢出）→ 下限 2、上限 5
/// （`or eax, 0x20; tzcnt` = `min(ilog2, 5)`；`cmp eax, 3; cmovae` = `max(.., 2)`）。
fn curved_subdivision_level(x1: f32, y0: f32, y1: f32) -> u32 {
    let raw = ((y1 - y0).abs() * x1 + 1.0) / 0.6f32;
    let n = raw.ceil() as u32; // Rust 的 `as u32` 与 `cvttss2si` + 钳制序列的饱和语义一致。
    let pow2 = if n < 2 {
        1u32
    } else {
        (u32::MAX >> (n - 1).leading_zeros()).wrapping_add(1)
    };
    assert!(pow2 > 0, "argument of integer logarithm must be positive");
    pow2.ilog2().clamp(2, 5)
}

impl CurvedHalfTimberInstanceData {
    /// xy 平面弯曲构造（0x14098f690，O）。参数（Win64：rdx = `&position`；xmm2/xmm3；栈上 3 个 f32）：
    /// `center`（xmm2）、`length`（xmm3）、`w28`（[rsp+0xb0]）、`y0`（[rsp+0xb8]）、`y1`（[rsp+0xc0]）。
    pub fn new_curved_xy(
        position: Vec3,
        center: f32,
        length: f32,
        w28: f32,
        y0: f32,
        y1: f32,
    ) -> Self {
        let half = length * 0.5;
        let x1 = half + center;
        Self {
            position,
            pad_0c: 0,
            x0: center - half,
            x1,
            y0,
            y1,
            w20: 0,
            w24: 0,
            w28,
            w2c: 0,
            w30: curved_subdivision_level(x1, y0, y1),
            w34: 0,
            w38: 0,
            w3c: 0,
        }
    }

    /// xz 平面弯曲构造（0x14098f800，O）：与 xy 的差别只有 `w24 = 1`、`w2c = flag`（第 7 参数，u8）。
    pub fn new_curved_xz(
        position: Vec3,
        center: f32,
        length: f32,
        w28: f32,
        y0: f32,
        y1: f32,
        flag: u8,
    ) -> Self {
        let half = length * 0.5;
        let x1 = half + center;
        Self {
            position,
            pad_0c: 0,
            x0: center - half,
            x1,
            y0,
            y1,
            w20: 0,
            w24: 1,
            w28,
            w2c: u32::from(flag),
            w30: curved_subdivision_level(x1, y0, y1),
            w34: 0,
            w38: 0,
            w3c: 0,
        }
    }

    /// 直段构造（0x14098f970，O）：`w20 = 2 * n + (has_mid_cap ? 2 : 1)`；
    /// `w24 = 1`、`w28 = 0x3dc28f5c`（0.095）、`w30 = 1`，其余 0（常量 `0x142ab9500` = `(0,1,0,0)` 覆盖 +0x2c..+0x3c）。
    pub fn new_straight(
        position: Vec3,
        x0: f32,
        x1: f32,
        y0: f32,
        y1: f32,
        has_mid_cap: bool,
        n: u8,
    ) -> Self {
        let n2 = 2 * u32::from(n);
        Self {
            position,
            pad_0c: 0,
            x0,
            x1,
            y0,
            y1,
            w20: if has_mid_cap { n2 + 2 } else { n2 + 1 },
            w24: 1,
            w28: f32::from_bits(0x3dc2_8f5c),
            w2c: 0,
            w30: 1,
            w34: 0,
            w38: 0,
            w3c: 0,
        }
    }
}

/// 弯曲半木端帽（O 名称，字段 U）。
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CurvedHalfTimberCaps {
    pub raw: [u32; 4],
}

/// Uber 位域（modular_bitfield 0.11.2；字段 U）。
#[derive(Clone, Copy, Debug, Default)]
pub struct NaniUberInstanceFlags {
    pub bits: u8,
}

/// 窗玻璃位域（字段 U）。
#[derive(Clone, Copy, Debug, Default)]
pub struct WindowGlassFlags {
    pub bits: u8,
}

/// 组件注册（O：`impl$36::register_required_components<T>` ×19，0x141572d80 系）：为每个实例数据类型注册
/// `InstanceDataMarker<T>`。原版每个类型一个单态化；PDB 里没有 `PreplacedFlowerInstanceData` 的那一份
/// （其 marker 在 systems 里作过滤器使用，注册由首次 `With<>` 查询触发，I），这里仍一并注册，结果等价。
pub fn register_required_components(world: &mut World) {
    macro_rules! register {
        ($($t:ty),* $(,)?) => {$( world.register_component::<InstanceDataMarker<$t>>(); )*};
    }
    register!(
        BalconyDoorInstanceData,
        ClothInstanceData,
        CurvedHalfTimberInstanceData,
        GothicWindowBricksInstanceData,
        LanternEmissiveInstanceData,
        PlantInstanceData,
        PlasterInstanceData,
        PreplacedFlowerInstanceData,
        SmokeInstanceData,
        SolidAlbedoMapEmissiveInstanceData,
        SolidAlbedoMapInstanceData,
        SolidVertexColorBricksInstanceData,
        SolidVertexColorInstanceData,
        SolidVertexColorWindowGlassInstanceData,
        SolidVertexColorWindowInstanceData,
        StairsFloorInstanceData,
        StoneFloorInstanceData,
        UberInstanceData,
        VelocityOnlyInstanceData,
        WrenInstanceData,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, size_of};

    fn words(d: &CurvedHalfTimberInstanceData) -> [u32; 16] {
        unsafe { core::mem::transmute::<CurvedHalfTimberInstanceData, [u32; 16]>(*d) }
    }

    #[test]
    fn element_sizes_match_exe_strides() {
        assert_eq!(size_of::<UberInstanceData>(), 0x40);
        assert_eq!(size_of::<SolidVertexColorInstanceData>(), 0x40);
        assert_eq!(size_of::<SolidVertexColorWindowGlassInstanceData>(), 0x40);
        assert_eq!(size_of::<CurvedHalfTimberInstanceData>(), 0x40);
        assert_eq!(size_of::<PlantInstanceData>(), 0x30);
        assert_eq!(size_of::<PlasterInstanceData>(), 0x30);
        assert_eq!(size_of::<StairsFloorInstanceData>(), 0x30);
        assert_eq!(
            size_of::<PreplacedFlowerInstanceData>(),
            0x30,
            "旧骨架 0x40 是错的"
        );
        assert_eq!(size_of::<VelocityOnlyInstanceData>(), 0x60);
        assert_eq!(size_of::<WrenInstanceData>(), 0x70);
        assert_eq!(size_of::<GothicWindowBricksInstanceData>(), 0x70);
        assert_eq!(align_of::<CurvedHalfTimberInstanceData>(), 4);
    }

    #[test]
    fn storage_names_are_type_names() {
        assert_eq!(
            <UberInstanceData as StorageElement>::NAME,
            "UberInstanceData"
        );
        assert_eq!(
            <WrenInstanceData as StorageElement>::NAME,
            "WrenInstanceData"
        );
        assert_eq!(
            <CurvedHalfTimberInstanceData as StorageElement>::NAME,
            "CurvedHalfTimberInstanceData"
        );
    }

    #[test]
    fn straight_layout_is_byte_exact() {
        let d = CurvedHalfTimberInstanceData::new_straight(
            Vec3::new(1.0, 2.0, 3.0),
            0.5,
            4.5,
            -1.0,
            6.0,
            true,
            3,
        );
        let w = words(&d);
        assert_eq!(
            w[0..3],
            [1.0f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits()]
        );
        assert_eq!(w[3], 0);
        assert_eq!(
            w[4..8],
            [
                0.5f32.to_bits(),
                4.5f32.to_bits(),
                (-1.0f32).to_bits(),
                6.0f32.to_bits()
            ]
        );
        assert_eq!(w[8], 2 * 3 + 2, "has_mid_cap → 2n + 2");
        // 0x3dc28f5c00000001 @0x24；(0,1,0,0) @0x2c..；0 @0x3c
        assert_eq!(w[9], 1);
        assert_eq!(w[10], 0x3dc2_8f5c);
        assert_eq!(w[11..16], [0, 1, 0, 0, 0]);
        let no_cap =
            CurvedHalfTimberInstanceData::new_straight(Vec3::ZERO, 0.0, 1.0, 0.0, 1.0, false, 3);
        assert_eq!(no_cap.w20, 7, "无中间帽 → 2n + 1");
        assert!((f32::from_bits(0x3dc2_8f5c) - 0.095).abs() < 1e-6);
    }

    #[test]
    fn curved_xy_layout_and_pow2_level() {
        // center = 1, length = 2 → half = 1, x0 = 0, x1 = 2；|y1-y0| = 3 → raw = (6+1)/0.6 = 11.67 → ceil 12 → pow2 16 → ilog2 4。
        let d = CurvedHalfTimberInstanceData::new_curved_xy(
            Vec3::new(9.0, 8.0, 7.0),
            1.0,
            2.0,
            0.25,
            1.0,
            4.0,
        );
        assert_eq!(d.position, Vec3::new(9.0, 8.0, 7.0));
        assert_eq!((d.x0, d.x1, d.y0, d.y1), (0.0, 2.0, 1.0, 4.0));
        assert_eq!((d.w20, d.w24, d.w2c), (0, 0, 0));
        assert_eq!(d.w28, 0.25);
        assert_eq!(d.w30, 4);
        assert_eq!((d.w34, d.w38, d.w3c, d.pad_0c), (0, 0, 0, 0));
    }

    #[test]
    fn curved_level_clamps_to_2_and_5() {
        // 极小：raw → ceil(1/0.6)=2 → pow2 2 → ilog2 1 → 下限 2。
        assert_eq!(curved_subdivision_level(0.0, 0.0, 0.0), 2);
        // 极大：n 很大 → ilog2 > 5 → 上限 5。
        assert_eq!(curved_subdivision_level(1000.0, 0.0, 100.0), 5);
        // 边界：n = 9 → pow2 16 → 4；n = 32 → pow2 32 → 5；n = 33 → pow2 64 → 钳 5。
        // raw = (dy*x1+1)/0.6：取 x1 = 1，dy = 0.6*n - 1。
        assert_eq!(curved_subdivision_level(1.0, 0.0, 0.6 * 9.0 - 1.0), 4);
        assert_eq!(curved_subdivision_level(1.0, 0.0, 0.6 * 33.0 - 1.0), 5);
        // NaN → 0 → pow2 1 → 钳到 2（`ucomiss` 无序 → cmovae 不触发 → edx = 0）。
        assert_eq!(curved_subdivision_level(f32::NAN, 0.0, 1.0), 2);
    }

    #[test]
    fn curved_xz_differs_only_in_w24_and_w2c() {
        let xy = CurvedHalfTimberInstanceData::new_curved_xy(Vec3::ONE, 1.0, 2.0, 0.5, 0.0, 3.0);
        let xz =
            CurvedHalfTimberInstanceData::new_curved_xz(Vec3::ONE, 1.0, 2.0, 0.5, 0.0, 3.0, 0xab);
        assert_eq!(xz.w24, 1);
        assert_eq!(xz.w2c, 0xab);
        let (mut a, mut b) = (xy, xz);
        (a.w24, a.w2c, b.w24, b.w2c) = (0, 0, 0, 0);
        assert_eq!(a, b);
    }

    #[test]
    fn registers_markers_in_world() {
        let mut world = World::new();
        register_required_components(&mut world);
        assert!(
            world
                .components()
                .component_id::<InstanceDataMarker<UberInstanceData>>()
                .is_some()
        );
        assert!(
            world
                .components()
                .component_id::<InstanceDataMarker<WrenInstanceData>>()
                .is_some()
        );
        assert!(
            world
                .components()
                .component_id::<InstanceDataMarker<PlantInstanceData>>()
                .is_some()
        );
        let _ = UberInstanceData::marker();
    }
}
