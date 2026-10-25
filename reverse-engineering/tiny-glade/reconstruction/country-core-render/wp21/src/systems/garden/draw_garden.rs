//! systems/garden/draw_garden.rs — 花园画笔系统（WP21）。
//!
//! O：`draw_garden`（0xae4620）：读工具状态（类 6 花丛 / 4 橡皮）；花丛分支：
//! `Circle2d::new` 半径（`stored + 1.5`）、`GladeBorder::is_shape_inside`，不越界则推
//! 事件、置画笔状态字节（0/2/3），最后 `world_raster::draw_raster<GardenRasterStagingSsbo,
//! WorldRaster<512>, c0, c1, c2>` 绘制到花园掩码光栅（世界光栅 512² 域，wp7/10）。
//! 事件/UI/光栅域类型为共享 crate 边界 → typed gap。

use crate::typed_gap::Gap;

/// 花园画笔状态（O：状态字节 0/2/3；橡皮 0 层）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GardenBrushLayer {
    None = 0,
    Flowers = 2,
    Eraser = 3,
}

/// `draw_garden`（O 0xae4620；系统面）。
///
/// `brush_layer` 输出画笔状态；`garden_mask` 为花园掩码光栅（GardenMaskSsbo /
/// WorldRaster<512> 域，U）。
pub fn draw_garden(
    _radius: f32,
    _inside_border: bool,
    _brush_layer: &mut GardenBrushLayer,
    _garden_mask: &mut Gap,
    _events: &mut Gap,
) {
    // 业务逻辑 O（半径 +1.5、边界判定、状态机、draw_raster 接线）；骨架表面。
}
