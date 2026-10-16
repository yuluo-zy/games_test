//! water.rs — `country_core::resources::water` 水内容资源模块（组 A restorer 接线）。
//!
//! 源（B skeleton）：`crates/country-core/src/resources/water.rs`；PDB 命名实体
//! `RippleSourceSsbo` / `WaterDrawEvent` / `WaterEraseEvent`（章节
//! `chapters/resources/water.rs.md`：name_only，fields/size U）。
//! CPU 水位状态本体 = `world_raster_defs::WaterRaster`（`WorldRaster<512>` u8 栅格），
//! 由 `system_water::impl$0::build`（0x1406501d0）`insert_resource<WaterRaster>()` 注册（O）。
//!
//! 事件载荷（O，`systems/audio/water_audio.rs` 消费面 + `draw_water` 0x141ba9130 的
//! `EventWriter` 面）：`{position: Vec3, radius: f32}`，与花园绘制事件的形状一致
//! （core 音频域已按此实现 `WaterDrawEvent/WaterEraseEvent` 的读取；本模块把事件
//! 落位到 PDB 的原始模块，替代音频域临时本地副本）。
//! 本模块承载 GPU 侧波纹源（typed gap）+ 水面编辑事件。

use bevy_ecs::event::Event;
use glam::Vec3;

use crate::render::pass_builder::pass_resource_access::SsboType as PassSsboType;
use crate::typed_gap::Gap;

/// 波纹源 SSBO（O：`resources::water::RippleSourceSsbo`）。
///
/// 消费者（O）：`water_planes_draw`（0x1409d9bc0，`read_storage_buffer<RippleSourceSsbo>`）；
/// `_water_plane.raster.hlsl` / `_waterfall.raster.hlsl`（bin names
/// `RippleSourceSsbo` / `ripple_source_buffer`）。生产者（O，跨 crate）：
/// `system_critters::duck::update_duck_ripples`（0x2314b70，鸭子入水波纹）。
/// 布局 U（typed gap）。
#[derive(Clone, Copy, Debug, Default)]
pub struct RippleSourceSsbo(pub Gap);

impl PassSsboType for RippleSourceSsbo {
    /// typed gap：原值 = `utils::name_hash("…RippleSourceSsbo")`（AssetSsboLibrary 注册键）。
    fn rg_buffer_hash() -> u64 {
        0
    }
}

impl crate::render::ssbo::StorageElement for RippleSourceSsbo {
    const NAME: &'static str = "RippleSourceSsbo";
    // BUFFER_LEN/MAPPED_MEMORY：布局 U（typed gap），保持 trait 默认（0 / 上传种类）。
}

/// 水面绘制事件（O：`WaterPlugin::build` 0x1406501d0 `register_event<WaterDrawEvent>`）。
/// 生产者：`system_water::draw_water::draw_water`（0x141ba9130，`draw_raster` 的
/// on_draw 回调，闭包 `closure$1` 行 161）。载荷 `{position, radius}`（O：core
/// `systems/audio/water_audio.rs` 消费面，与花园绘制事件形状一致；无
/// `observed_flag`——audio 域证据未列该字段）。
#[derive(Event, Clone, Copy, Debug)]
pub struct WaterDrawEvent {
    pub position: Vec3,
    pub radius: f32,
}

/// 水面擦除事件（O：同 build 注册；生产者 `draw_water` 的 on_erase 回调，
/// `closure$2` 行 164）。
#[derive(Event, Clone, Copy, Debug)]
pub struct WaterEraseEvent {
    pub position: Vec3,
    pub radius: f32,
}
