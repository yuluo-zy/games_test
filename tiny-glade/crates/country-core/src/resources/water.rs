//! water.rs — `country_core::resources::water` 水内容资源模块（组 A restorer 接线）。
//!
//! 源（B skeleton）：`crates/country-core/src/resources/water.rs`；PDB 命名实体
//! `RippleSourceSsbo` / `WaterDrawEvent` / `WaterEraseEvent`（章节
//! `chapters/resources/water.rs.md`：name_only，fields/size U）。
//! CPU 水位状态本体 = `world_raster_defs::WaterRaster`（`WorldRaster<512>` u8 栅格），
//! 由 `system_water::impl$0::build`（0x1406501d0）`insert_resource<WaterRaster>()` 注册（O）。
//!
//! 事件载荷（O，`draw_raster<Water>` 0x142260ae0 的实时写入面 + 回放闭包
//! 0x140a01560 的 20 字节复制面）：`{position: Vec3, radius: f32,
//! replay: bool}`。实时写入把末字节设 0；回放调用点把同一字节设 1。
//! core 音频域只读取 position/radius；本模块把事件
//! 落位到 PDB 的原始模块，替代音频域临时本地副本）。
//! 本模块承载 GPU 侧波纹源（typed gap）+ 水面编辑事件。

use bevy_ecs::event::Event;
use glam::Vec3;

use crate::render::pass_builder::pass_resource_access::SsboType as PassSsboType;

/// 波纹源 SSBO（O：`resources::water::RippleSourceSsbo`）。
///
/// 消费者（O）：`water_planes_draw`（0x1409d9bc0，`read_storage_buffer<RippleSourceSsbo>`）；
/// `_water_plane.raster.hlsl` / `_waterfall.raster.hlsl`（bin names
/// `RippleSourceSsbo` / `ripple_source_buffer`）。生产者（O，跨 crate）：
/// `system_critters::duck::update_duck_ripples`（0x2314b70，鸭子入水波纹）。
/// B ctor@1416150a0 accepts exactly one 528-byte aggregate. Raw words retain
/// its real layout without inventing fields for the unresolved CPU producer.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RippleSourceSsbo {
    pub first_words: [u32; 4],
    pub remaining_words: [[u32; 4]; 32],
}

impl PassSsboType for RippleSourceSsbo {
    fn rg_buffer_hash() -> u64 {
        crate::asset_libraries::ssbo_library::SsboId::of::<Self>()
            .0
            .0
    }
}

impl crate::render::ssbo::StorageElement for RippleSourceSsbo {
    const NAME: &'static str = "RippleSourceSsbo";
    const BUFFER_LEN: usize = 1;
}

impl crate::asset_libraries::ssbo_library::SsboType for RippleSourceSsbo {
    type Storage = crate::render::ssbo::TypedStorageBuffer<Self>;
    fn new_storage_buffer(
        device: &dyn crate::render::ssbo::rhapsody_boundary::GraphicsDevice,
        initial: Option<&[u8]>,
    ) -> Self::Storage {
        use crate::render::ssbo::rhapsody_boundary::BufferDesc;
        if let Some(bytes) = initial {
            assert_eq!(bytes.len() % 0x210, 0);
            assert_eq!(bytes.as_ptr() as usize % 4, 0);
            assert!(
                bytes.len() < 0x420,
                "Excessive data size supplied for RippleSourceSsbo"
            );
        }
        // This one-element storage is still usage0x22; the ordinary uniform
        // default path would change the native descriptor and None policy.
        let buffer = device
            .create_buffer(
                &BufferDesc::new(0x210, 0x22, 1).alignment(4),
                "RippleSourceSsbo",
                initial,
            )
            .expect("creating original RippleSourceSsbo");
        crate::render::ssbo::TypedStorageBuffer::from_created_buffer(buffer)
    }
}

/// 水面绘制事件（O：`WaterPlugin::build` 0x1406501d0 `register_event<WaterDrawEvent>`）。
/// 生产者：`system_water::draw_water::draw_water`（0x141ba9130，`draw_raster` 的
/// on_draw 回调，闭包 `closure$1` 行 161）。事件载荷为 20 字节；
/// `replay` 在实时绘制时为 false，在回放反馈时为 true。
#[derive(Event, Clone, Copy, Debug)]
pub struct WaterDrawEvent {
    pub position: Vec3,
    pub radius: f32,
    pub replay: bool,
}

/// 水面擦除事件（O：同 build 注册；生产者 `draw_water` 的 on_erase 回调，
/// `closure$2` 行 164）。
#[derive(Event, Clone, Copy, Debug)]
pub struct WaterEraseEvent {
    pub position: Vec3,
    pub radius: f32,
    pub replay: bool,
}
