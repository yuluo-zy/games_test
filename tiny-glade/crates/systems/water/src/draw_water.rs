//! draw_water.rs — 水面绘制 / 水面橡皮（B 0x141ba9130，O；re-analyst-A 契约
//! `systems-recovery/groupA/water/RE-ANALYSIS.md` §2/§3）。
//!
//! 模式分发（O）：`AppMode::Eraser(EraserLayer::All | EraserLayer::Water)`（tag 9）
//! = 水面橡皮；`AppMode::Water`（tag 15）= 画水。其余模式早退。
//! 两者共用尾部：`DrawState::update`（0x140ae4bd0）→ 命令状态机
//! {0 Idle, 1 开始, 2 延伸, 3 提交} → `RecalculateHeightMapCmd` →
//! `ScheduledTerrainReadback::on_terrain_modified`（0x140aabdc0，LOCK INC）→
//! `draw_raster<WaterRasterStagingSsbo>`（0x142260ae0 等价族）。
//!
//! 常量（O，字节直读）：橡皮半径 `t = (eraser - 0.3)/(2.0 - 0.3)` 后 `t*4 + (1-t)`
//! （inverse_lerp 0x140c904a0 无钳制，同 core draw_garden）；画水半径 =
//! `BrushSizes.water`；界内圈半径 = `0.8 * radius`（0x142a16cb8 = 0.8，
//! 与 core `systems/paths.rs::BRUSH_RECT_SCALE` 一致；re-analyst 记 0.15 与字节
//! 直读不符，以字节为准）；橡皮 signifier 缩放 = `0.5 * radius`（0x142925870）；
//! 图钉高度 = `pos.y + 1.5`（0x1429258d0）。

use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use glam::Vec2;
use utils::geometry::Circle2d;

use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::resources::active_raster_stroke::ActiveRasterStrokesStorage;
use country_core::resources::brush_sizes::BrushSizes;
use country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast;
use country_core::resources::glade_border::{
    BorderShape, DisplayOutOfBorder, FeedbackInputOutOfBorder, GladeBorder,
};
use country_core::resources::mouse_terrain_raycast::CameraCursorRay;
use country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed;
use country_core::resources::walls::{decorator_archivist::DecoratorArchivist, EraserSizeMultiplier};
use country_core::resources::water::{WaterDrawEvent, WaterEraseEvent};
use country_core::resources::world_raster_defs::{WaterRaster, WaterRasterStagingSsbo};
use country_core::systems::history::history_process::DebugHistoryMsg;
use country_core::systems::mode_manager::{AppMode, EraserLayer};
use country_core::systems::paths::DrawState;
use country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd;
use country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback;
use country_core::systems::ui::resources::ui_state::UiState;
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;
use country_core::systems::ui::signifiers::UiAppendSignifier;
use country_core::systems::world_raster::{draw_raster, RasterDrawResources, RasterStrokeCmd};

use crate::{water_systems_enabled, record_water_history::RecordWaterEditToHistoryCmd};

/// 橡皮半径 inverse_lerp 端点（O：0x1429258d8 / 0x14292e6f0）。
const ERASER_LERP_MIN: f32 = 0.3;
const ERASER_LERP_MAX: f32 = 2.0;
/// 橡皮半径重映射（O：`x * 0x142934694 + (0x142925904 - x)` = `x*4 + (1-x)`）。
const ERASER_RADIUS_SCALE: f32 = 4.0;
/// 界内圈半径系数（O：0x142a16cb8 = 0.8）。
const WATER_BORDER_CIRCLE_SCALE: f32 = 0.8;
/// 橡皮笔刷指示符缩放（O：0x142925870 = 0.5）。
const ERASER_SIGNIFIER_SCALE: f32 = 0.5;
/// 图钉高度抬升（O：0x1429258d0 = 1.5）。
const PIN_HEIGHT_OFFSET: f32 = 1.5;
/// WaterRaster 的栅格槽位（O：0x140868cf0 `ActiveRasterStrokesStorage::index`，
/// `idx * 0x470`；garden = 5）。
const WATER_RASTER_SLOT: u8 = 4;

/// Water 的圈/高图钉变体（O：事件记录 = {Circle2d, height, union 载荷 0x100 标志}——
/// 机器码低 u16 = 0x100（reviewer FIX §3.1：载荷带出标志，不再只登记 U）。
///
/// 原版事件为 `ShowPinnedDecoClutter`（56B union）的圈/高变体（BASELINE 参数面
/// `EventWriter<enum2$<ShowPinnedDecoClutter>>`）；core `ui_clutter` 域未恢复完整
/// union，按 core garden 域惯例（`GardenPinnedClutterCircle`）以本 crate 专名表达
/// O 载荷。机器码事件写入 = 0x142260a60（re-analyst 记为 WaterDrawEvent 写入，
/// 与 BASELINE 参数面 ShowPinnedDecoClutter 不符——以 BASELINE 参数面为准，此处
/// 登记分歧）。
#[derive(Event, Clone, Copy, Debug)]
pub struct WaterPinnedClutterCircle {
    pub circle: Circle2d,
    pub height: f32,
    /// O：union 载荷标志（0x100 = 256）。
    pub flag: u16,
}

/// SystemParam 束（同 core `GardenRasterResources` 模式；原版为
/// `RasterDrawResources` SystemParam——core 侧 `init_state` 0x140958040 已登记
/// 类型面，SystemParam impl 待接线，本 crate 以派生束 + 借用组装等价落地）。
#[derive(SystemParam)]
pub struct WaterRasterResources<'w, C: Send + Sync + 'static> {
    strokes: ResMut<'w, ActiveRasterStrokesStorage>,
    archivist: ResMut<'w, DecoratorArchivist<C>>,
    library: ResMut<'w, AssetSsboLibrary>,
    tool_used: ResMut<'w, Events<NotifyHintSystemToolWasJustUsed>>,
    debug_history: ResMut<'w, Events<DebugHistoryMsg>>,
}

/// `draw_water` @0x141ba9130（O）。
#[allow(clippy::too_many_arguments)]
pub fn draw_water<C: Send + Sync + 'static>(
    mode: Res<AppMode>,
    mut resources: WaterRasterResources<C>,
    ui: Res<UiState>,
    mut water: ResMut<WaterRaster>,
    mut draws: EventWriter<WaterDrawEvent>,
    mut erases: EventWriter<WaterEraseEvent>,
    mut record: EventWriter<RecordWaterEditToHistoryCmd>,
    mut recalc: EventWriter<RecalculateHeightMapCmd>,
    readback: Res<ScheduledTerrainReadback>,
    cursor: Res<CursorTerrainRaycast>,
    mut border: (
        Res<GladeBorder>,
        EventWriter<DisplayOutOfBorder>,
        EventWriter<FeedbackInputOutOfBorder>,
    ),
    mut state: Local<DrawState>,
    mut signifiers: ResMut<UiSignifierStream>,
    camera_ray: Res<CameraCursorRay>,
    brush: (Res<EraserSizeMultiplier>, Res<BrushSizes>),
    mut pins: EventWriter<WaterPinnedClutterCircle>,
) {
    // 门禁（O：`*0x14362c098 != 0 → return`）。
    if !water_systems_enabled() {
        return;
    }
    let (radius, draw, canvas, argument_79, layer) = match *mode {
        AppMode::Eraser(EraserLayer::All | EraserLayer::Water) => {
            // O：inverse_lerp(0.3, 2.0, eraser_size) 无钳制（0x140c904a0），
            // 再 `t*4 + (1-t)`。同 core draw_garden 橡皮半径。
            let t = (brush.0 .0 - ERASER_LERP_MIN) / (ERASER_LERP_MAX - ERASER_LERP_MIN);
            (
                t * ERASER_RADIUS_SCALE + (1.0 - t),
                false,
                "eraser",
                6u8,
                0u8,
            )
        }
        AppMode::Water => (brush.1.water, true, "water", 5u8, 4u8),
        _ => return,
    };
    // O：panic 串 'assertion failed: brush_size.is_finite()'（0x142eebba0）。
    assert!(radius.is_finite(), "water brush radius must be finite");
    let widget = ui.clearing_canvas_w_layer_and_sound(canvas, argument_79, layer);
    let position = cursor.position();

    let inside = if draw {
        // 画水分支（O）：界内圈 = Circle2d(center, 0.8 * radius)；
        // `GladeBorder::is_shape_inside`（0x140ad2970）。
        let center = Vec2::new(position.x, position.z);
        border
            .0
            .is_shape_inside(BorderShape::Circle(Circle2d::new(
                center,
                WATER_BORDER_CIRCLE_SCALE * radius,
            )))
    } else {
        true
    };

    // 画水分支的界外反馈（O）：圈外 → DisplayOutOfBorder；未释放（机器码
    // 0x14092e6b0 = `byte[record] != 2`，record 布局 U；同 garden 的
    // `phase != 2` 语义）→ FeedbackInputOutOfBorder。
    if !inside {
        border.1.write(DisplayOutOfBorder);
        if widget.dragged() != 2 {
            border.2.write(FeedbackInputOutOfBorder);
        }
    }

    // 尾部状态机（O：0x140ae4bd0 `DrawState::update` + switch）。
    let phase = widget.dragged();
    state.update(phase, u8::from(inside));
    let command = match state.0 {
        0 => return,
        1 => 0, // 开始（记录圈数据；draw_raster discriminant 0 = begin）
        2 => {
            // 延伸要求界内（O：`if uVar13 == 0 return`；uVar13 = inside）。
            if !inside {
                return;
            }
            if draw {
                // O：事件写入 0x142260a60，记录 = {Circle2d, height = pos.y + 1.5,
                // union 载荷 0x100 标志（U）}。
                pins.write(WaterPinnedClutterCircle {
                    circle: Circle2d::new(Vec2::new(position.x, position.z), radius),
                    height: position.y + PIN_HEIGHT_OFFSET,
                    flag: 0x100,
                });
            }
            2
        }
        _ => 3, // 提交
    };

    // 橡皮分支：笔刷大小 signifier（O：记录名 'eraser_water_'（0x142eebc40
    // fat），位置 = xz 平面交点（`intersect_with_xz_plane(0.0)`，0x1408ef180），
    // 缩放 = 0.5 * radius；`UiSignifierStream::write` 0x140b2cf90）。
    if !draw {
        let point = camera_ray.intersect_with_xz_plane(0.0);
        // O：0x20B 记录族 = {1, 1, Vec3, f32}（40B 布局中央裁决：
        // tag@0/kind@8/pos@0x10/scale@0x1c）；scale = 0.5×radius（0x142925870）。
        signifiers.write(UiAppendSignifier {
            tag: 1,
            kind: 1,
            position: point,
            scale: ERASER_SIGNIFIER_SCALE * radius,
            name: 0,
        });
    }

    // O 尾部：RecalculateHeightMapCmd → 地形回读调度 → draw_raster。
    recalc.write(RecalculateHeightMapCmd);
    readback.on_terrain_modified();
    let mut borrowed = RasterDrawResources {
        strokes: &mut resources.strokes,
        archivist: &mut resources.archivist,
        cursor: &cursor,
        tool_used: &mut resources.tool_used,
        debug_history: &mut resources.debug_history,
    };
    draw_raster::<WaterRasterStagingSsbo, C>(
        RasterStrokeCmd {
            discriminant: command,
            strength: 1.0, // I：橡皮分支记录常数 1；water 分支 U
            radius,
            draw,
        },
        position,
        WATER_RASTER_SLOT,
        &mut borrowed,
        &mut **water,
        &mut resources.library,
        |point, radius, _observed_flag| {
            draws.write(WaterDrawEvent {
                position: point,
                radius,
            });
        },
        |point, radius, _observed_flag| {
            erases.write(WaterEraseEvent {
                position: point,
                radius,
            });
        },
        || {
            record.write(RecordWaterEditToHistoryCmd);
        },
    );
}
