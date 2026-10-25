//! water_planes_draw.rs — 水面网格与植被 draw passes（WP7 恢复）。
//!
//! 反编译（O）：`water_planes_draw` = 0x1409d9bc0（size 1522）、`lilypad_draw` =
//! 0x1409da580、`lilypad_flower_draw` = 0x1409da760、`reed_draw` = 0x1409da940。
//!
//! - `water_planes_draw`：demo 守卫；`ActiveSession`（type-id 0x74430dfcc6d8719b，I）
//!   字节@+0x58 水面变体；`DrawBuilder::bind_common` + 纹理（preset 9/10/0x13/0x18/8，
//!   `TextureFromDisk 0x579f39ab91a01865` = water_noise_texture，O）+
//!   `read_storage_buffer<RippleSourceSsbo>` + `<DofAutofocusSsbo>` +
//!   `write_uniform<Vec3>` + `bind_ray_tracing`；`cVar1==4`（冬季）追加雪纹理
//!   `0xc555853f6bd247ac`；`DrawBuilder::draw`。
//! - 植被 draw：`read_storage_buffer<LilypadSsbo / LilypadFlowerSsbo / ReedSsbo>` +
//!   preset 9/10 + `common_includes::bind_water_object_push`（水植被 push 常量，
//!   `water_object_push.inc.hlsl` 嵌入 lilypad/reed raster）+ `multi_draw_indexed_indirect`。

#![allow(dead_code, unused_imports)]

use crate::consume::core::DofAutofocusSsbo;
use crate::render::pass_builder::draw_builder::DrawBuilder;
use crate::render::pass_builder::pass_resource_access::SsboType;
use crate::resources::session::ActiveSession;
use crate::resources::water::RippleSourceSsbo;
use crate::resources::water_mask::WATER_MASK_SAMPLE_SCALE;
use crate::resources::water_mask::WaterMaskSsbo;
use crate::systems::render_loop::compute_passes::compute_water::{
    LilypadFlowerSsbo, LilypadSsbo, ReedSsbo,
};
use crate::typed_gap::Gap;

/// 水/冰面网格 draw（O：0x1409d9bc0）。
/// 按 `ActiveSession` 字节@+0x58 选择水面变体（==4 冬季雪面）；读 `RippleSourceSsbo`
/// + `DofAutofocusSsbo` + 软件 BVH；`DrawBuilder::draw`（水 plane mesh）。
pub fn water_planes_draw(_session: &ActiveSession, _ripples: &RippleSourceSsbo, _builder: DrawBuilder) {
    // O：demo 守卫；bind_common → 纹理绑定 → read_storage_buffer×2 →
    //   write_uniform<Vec3>（水色/参数）→ bind_ray_tracing → draw。
    let _ = (WATER_MASK_SAMPLE_SCALE,);
    unimplemented!()
}

/// lilypad 实例 draw（O：0x1409da580）：`LilypadSsbo` + `bind_water_object_push` +
/// `multi_draw_indexed_indirect`（间接参数源 = W13 写的 `DrawIndexedIndirectBuffer`）。
pub fn lilypad_draw(_builder: DrawBuilder, _instances: &LilypadSsbo) {
    unimplemented!()
}

/// lilypad_flower 实例 draw（O：0x1409da760）。
pub fn lilypad_flower_draw(_builder: DrawBuilder, _instances: &LilypadFlowerSsbo) {
    unimplemented!()
}

/// reed 实例 draw（O：0x1409da940）。
pub fn reed_draw(_builder: DrawBuilder, _instances: &ReedSsbo) {
    unimplemented!()
}
