//! water.rs — `country_core::resources::water` 水内容资源模块（WP7 恢复）。
//!
//! 源（B skeleton）：`crates/country-core/src/resources/water.rs`；PDB 命名实体经
//! `resources/mod.rs.structures.json.gz` 暴露（`RippleSourceSsbo` / `WaterDrawEvent` /
//! `WaterEraseEvent`，均 `name_and_relations_only`，fields/size U）。
//! CPU 水位状态本体 = `world_raster_defs::WaterRaster`（`WorldRaster<512>` u8 栅格），
//! 由 `system_water::impl$0::build`（0x1406501d0）`insert_resource<WaterRaster>()` 注册（O）。
//! 本模块承载 GPU 侧波纹源 + 水面编辑事件 + `Water` 资源外观（指向水位栅格）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::event::Event;
use bevy_ecs::resource::Resource;

use crate::render::pass_builder::pass_resource_access::SsboType;
use crate::resources::world_raster_defs::WaterRaster;
use crate::typed_gap::Gap;

/// 波纹源 SSBO（O：`resources::water::RippleSourceSsbo`）。
///
/// 消费者（O）：`water_planes_draw`（0x1409d9bc0，`read_storage_buffer<RippleSourceSsbo>`）；
/// `_water_plane.raster.hlsl` / `_waterfall.raster.hlsl`（bin names
/// `RippleSourceSsbo` / `ripple_source_buffer`）。生产者（O，跨 crate）：
/// `system_critters::duck::update_duck_ripples`（0x2314b70，鸭子入水波纹）。
/// 布局 U（typed gap）。
pub struct RippleSourceSsbo(pub Gap);

impl SsboType for RippleSourceSsbo {
    /// typed gap：原值 = `utils::name_hash("…RippleSourceSsbo")`（AssetSsboLibrary 注册键）。
    fn rg_buffer_hash() -> u64 {
        0
    }
}

/// 水面绘制事件（O：`WaterPlugin::build` 0x1406501d0 `register_event<WaterDrawEvent>`）。
/// 生产者：`draw_raster<Water…>` / `replay_raster<Water…>`（O：`EventWriter<WaterDrawEvent>`）。
#[derive(Event, Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct WaterDrawEvent(pub Gap);

/// 水面擦除事件（O：同 build 注册）。载荷 U（typed gap）。
#[derive(Event, Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct WaterEraseEvent(pub Gap);

/// 水内容 CPU 状态资源外观。
///
/// PDB 未列 `water::Water` 结构体（name_only 域仅上面三类）；水位本体是
/// `WaterRaster`（`Res<WaterRaster>`，O）。`Water` 作为承载水面内容状态的
/// `Resource` 外观，引用水位栅格 + 变体选择（`GladeType` 判别值，I：@+0x58）。
#[derive(Resource, Clone, Debug, Default)]
pub struct Water {
    /// 水位栅格本体（O：`Res<WaterRaster>`；`bilinear_sample_at_world_xz<512>` 采样）。
    pub raster: WaterRaster,
    /// typed gap：其余字段（波纹引用、水面变体字节、配置常数）U。
    pub _gap: Gap,
}
