//! timberframe_circle.rs — 圆顶木框架梁构建（B 0x1421b67a0，O；re-analyst-A
//! 契约 `systems-recovery/groupA/timberframe/RE-ANALYSIS.md` §2）。
//!
//! 算法骨架（O）：输入墙形状（ShapeParameters 域）→ `Circle2d::perimeter`
//! （0x140c97970）+ `as_points2_ptcount`/`get_pos_at_u`（0x140c97ba0/0x140c975a0，
//! u 采样梁位置）→ `roof_height_ws_from_wall_shape`（0x1408e3c70）+ 法向旋转
//! （0x140c90a40）→ 梁实例写往 `TimberframeInstanceBuffer`（GPU 侧
//! `WoodenPlankSsbo`）。常量：0.2 间距（0x142a8c6f0 ×2）、-1.0/-0.5
//! （0x14295a0e4）、0.4922/1.0（0x142925874/0x142925904）。构造后刷新
//! `TimberframeBeamCount.current`（+0x2c 读取面）。

use bevy_ecs::prelude::*;
use glam::Vec3;
use utils::geometry::Circle2d;

use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::resources::events::ConstructTimberframeCmd;
use country_core::resources::roofs::roof_shape::roof_height_ws_from_wall_shape;
use country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache;
use country_core::systems::wall::valid_enclosure::{Shape, ShapeParameters};

use crate::timberframe_beam_count_change_audio::TimberframeBeamCount;
use crate::TimberframeInstanceBuffer;

/// 梁间距（O：0x142a8c6f0 = 0.2，圆/矩形共用）。
pub const BEAM_SPACING: f32 = 0.2;
/// 梁长度缩放（O：0x142925874 = 0.4922；re-analyst 记为 0x142925870 区）。
pub const PLANK_LENGTH_SCALE: f32 = 0.4922;
/// 高度符号（O：-1.0/-0.5 @ 0x14295a0e4；语义 I——梁高/偏移方向族）。
pub const HEIGHT_SIGN: f32 = -1.0;
pub const HEIGHT_HALF: f32 = -0.5;

/// 梁实例记录（O：0x20B 条目，写往 WoodenPlankSsbo；字段明细 U——
/// {位置, 朝向, 长度} 语义 I，按机器码输出面）。
#[derive(Clone, Copy, Debug)]
pub struct TimberframePlank {
    /// 世界位置。
    pub position: Vec3,
    /// 朝向（法向旋转；I）。
    pub orientation: Vec3,
    /// 梁长。
    pub length: f32,
    /// 填充（0x20B 对齐；U）。
    pub _pad: [u8; 4],
}

impl Default for TimberframePlank {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            orientation: Vec3::Z,
            length: 1.0,
            _pad: [0; 4],
        }
    }
}

/// `construct_timberframe_circle` @0x1421b67a0（O 面）。
///
/// 参数面（reviewer-A FIX）：param_2 = `Res<TimberframeInstanceBuffer>` **只读**
/// （机器码无 tick 回写；`if (cap == 0) goto` 门控 + 条目基址只读）——梁构建
/// 的写路径非 buffer（re-analyst §3.2：输出路径待补证），本实现按只读门控
/// 落地，输出面登记 GAP。
pub fn construct_timberframe_circle(
    mut constructs: EventReader<ConstructTimberframeCmd>,
    buffer: Res<TimberframeInstanceBuffer>,
    _library: Res<AssetSsboLibrary>,
    _params: Res<TimberframeConstructionParamsCache>,
    mut beams: ResMut<TimberframeBeamCount>,
) {
    // O：`if (*(longlong *)*param_2 == 0) goto …`——cap==0 门控（只读）。
    if buffer.0.is_empty() {
        return;
    }
    for cmd in constructs.read() {
        if cmd.shape.len() < 28 {
            continue;
        }
        let params = ShapeParameters::from_observed(cmd.shape[..28].try_into().unwrap());
        match params.get_shape() {
            Some(Shape::Circle(circle)) => {
                // O：梁实例构建（几何环 0x140c97ba0/0x140c975a0/0x140c9cdc0——
                // 输出路径 re-analyst §3.2 待补证；本处计算后登记，不 push）。
                let _ = build_circle_beams(circle, &cmd.shape);
            }
            _ => {}
        }
        beams.current = buffer.0.len() as i32;
        beams.non_empty = !buffer.0.is_empty();
    }
}

/// 圆周长采样梁（O 骨架：perimeter + u 采样 + 屋顶高 + 法向；输出路径
/// re-analyst §3.2 待补证——返回梁序列供调用方登记）。
fn build_circle_beams(circle: Circle2d, wall_shape: &[u8]) -> Vec<TimberframePlank> {
    let perimeter = circle.perimeter();
    let count = (perimeter / BEAM_SPACING).ceil().max(1.0) as usize;
    // O：屋顶世界高度（0x1408e3c70）。
    let height = roof_height_ws_from_wall_shape(wall_shape);
    let mut out = Vec::with_capacity(count);
    for index in 0..count {
        // I：u 采样（机器码 get_pos_at_u ×3 的采样族；相位细分 U）。
        let u = index as f32 / count as f32;
        let pos = circle.get_pos_at_u(u);
        let normal = circle.get_normal_at_u(u);
        out.push(TimberframePlank {
            position: Vec3::new(pos.x, height * HEIGHT_SIGN, pos.y),
            orientation: Vec3::new(normal.x, 0.0, normal.y),
            length: PLANK_LENGTH_SCALE * circle.radius * 2.0,
            _pad: [0; 4],
        });
    }
    out
}
