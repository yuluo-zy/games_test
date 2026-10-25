//! transparency_pass.rs — W44 折射 / W45 水/冰表面 / W46+W53 透明（WP7 水部分）。
//!
//! 反编译（O）：0x1409bf320（refraction_copy_pass）、0x1409bf800（water_surface_pass，
//! size 3099）、0x1409c08d0（ice_surface_pass，size 8874）、0x1409c3a80（pre_taa）、
//! 0x1409c40b0（post_taa）。分支：折射+水/冰由 `\*0x14362c098 == 0` 门；水/冰由
//! `GladeType::is_winter`（0x140ae8320）派发（frame-wiring §3）。
//!
//! **W45 水表面**（O）：`create_raw_resource`（buffer `new_gpu_only` + texture
//! `extent_2d`）+ `clear_rg_buffer`；`RasterPass` 跑 `_water_plane.raster`（draw stream
//! 0x80B/条经 0x70 跳表闭包，读 `Time` type-id 0x4bc362195b80f729、`RippleSourceSsbo`）；
//! compute `_water_calc_deferred_rt_args.cs`（write_rg_storage_buffer + dispatch_1d）+
//! `_water_deferred_rt.cs`（read_rg_storage_buffer×2 + `bind_ray_tracing` +
//! `dispatch_indirect_raw`，软件 BVH）。
//! **W45 冰表面**（O）：同水 + `_water_ice_{refl_march,refl_resolve,refr_resolve,
//! deferred_rt}.cs` + `_ice_plane` + `_water_ice_cracks.raster`；`write_uniform<bool>`（雪/冰标志）。

#![allow(dead_code, unused_imports)]

use crate::consume::core::RtBvhSsbo;
use crate::render::pass_builder::draw_builder::DrawBuilder;
use crate::render::pass_builder::pass_resource_access::SsboType;
use crate::resources::session::GladeType;
use crate::resources::water::RippleSourceSsbo;
use crate::typed_gap::Gap;

/// 水面变体（W45 派发结果；O：`GladeType::is_winter` 选冰）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WaterSurfaceKind {
    Water,
    Ice,
}

/// W45 表面选择（O：`render_world` 0x1404ec110 按 `GladeType::is_winter`（0x140ae8320）
/// 间接派发 water_surface_pass ↔ ice_surface_pass）。
pub fn select_water_surface(glade: GladeType) -> WaterSurfaceKind {
    if glade.is_winter() {
        WaterSurfaceKind::Ice
    } else {
        WaterSurfaceKind::Water
    }
}

/// W44 — 折射拷贝（O：0x1409bf320，门 `\*0x14362c098 == 0`）。
/// `_water_plane_refraction_copy.raster` + `_waterfall_refraction_copy.raster`。
pub fn refraction_copy_pass(world: &mut Gap, _view: &Gap, _lists: &Gap) {
    let _ = world;
    unimplemented!()
}

/// W45 — 非冬季水表面：`_water_plane.raster` + `_water_deferred_rt.cs` +
/// `_water_calc_deferred_rt_args.cs`（O：0x1409bf800）。
/// 读 `RippleSourceSsbo`（`DrawBuilder::read_storage_buffer`）、`Time`（波纹动画）、
/// 水 mask / water_noise_texture / half_depth_tex；`dispatch_indirect_raw` + `bind_ray_tracing`。
pub fn water_surface_pass(world: &mut Gap, _view: &Gap, _bvh: &RtBvhSsbo, _lists: &Gap) {
    let _ = world;
    let _ = (select_water_surface,);
    unimplemented!()
}

/// W45 — 冬季冰表面：`_water_ice_{deferred_rt,refl_march,refl_resolve,refr_resolve}.cs`
/// + `_ice_plane` + `_water_ice_cracks.raster`（O：0x1409c08d0）。
pub fn ice_surface_pass(world: &mut Gap, _view: &Gap, _bvh: &RtBvhSsbo, _lists: &Gap) {
    let _ = world;
    unimplemented!()
}

/// W46 — pre-TAA 透明（窗玻璃/布料，blend）。
pub fn pre_taa_transparency_pass(world: &mut Gap, _view: &Gap, _lists: &Gap) {
    let _ = world;
    unimplemented!()
}

/// W53 — post-TAA 透明（`Vec::is_empty` 守卫）。
pub fn post_taa_transparency_pass(world: &mut Gap, _view: &Gap, _lists: &Gap) {
    let _ = world;
    unimplemented!()
}
