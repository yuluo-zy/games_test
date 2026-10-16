//! display_glade_border.rs — 谷地边界调试可视化（B 0x141be0ad0，O；re-analyst-A
//! 契约 `systems-recovery/groupA/glade-border/RE-ANALYSIS.md` §2）。
//!
//! 语义（O）：`DisplayOutOfBorder` 事件（EventReader 游标推进后事件数非 0）→
//! 边界曲线（`BorderShape` → `Circle2d::as_curve2`，0x1411ff2b0 实例）→
//! `DebugLine::new`（0x140b04e70）→ `color(Red=1)`（0x140b04eb0）→
//! `line_width(4.0)`（0x140b04f40）→ `set_ui_pass(false)`（0x140b04ee0）→
//! `show`（0x140b04f70，标签映射跳表，宿主 debug 渲染接线）。进程全局
//! `*0x14362c098 == 1` 时发射 `ShowDemoBorderOverlay`（param_7）。配置字节
//! `+0x121 == 0` 时写边界 perimeter signifier 到 `UiSignifierStream`（param_3；
//! 记录 = {tag 1, 位置源 `*param_5 + 0x2c`（I）, perimeter 0x1411ff640}）。

use bevy_ecs::prelude::*;
use glam::Vec3;

use country_core::game_config::GameConfig;
use country_core::resources::glade_border::{
    BorderShape, DisplayOutOfBorder, GladeBorder, ShowDemoBorderOverlay,
};
use country_core::systems::camera_rig::main_camera::MainCamera;
use country_core::systems::mode_manager::AppMode;
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;
use country_core::systems::ui::signifiers::UiAppendSignifier;
use country_core::utils::debug_viz::{DebugColor, DebugLine};

/// 进程全局门禁（O：`*0x14362c098 == 1` 时发射 overlay；建模同 water crate 的
/// 门禁常量族——本 crate 独立常量）。

/// 边界曲线采样步长（I：机器码 as_curve2 的 max_step 常量未显式解出；以相邻域
/// 0.2 间距族为参考，待 re-analyst 补证）。
const BORDER_CURVE_STEP: f32 = 0.2;

/// 边界 debug 输出（O：机器码 param_4 = 输出 Vec（12 B 条目 = Vec3 曲线点）；
/// DebugLine 0x48B 构建完成体。宿主 debug 渲染消费）。
#[derive(Resource, Clone, Debug, Default)]
pub struct GladeBorderDebugLines {
    /// O：边界曲线点（12 B 条目）。
    pub points: Vec<Vec3>,
    /// O：构建的边界线（color=Red、width=4.0、ui_pass=false）。
    pub line: Option<DebugLine>,
}

/// `display_glade_border` @0x141be0ad0（O）。
///
/// 参数面按机器码 8 参序（reviewer-A FIX：删自造 `_heights/_game_time`——
/// 机器码无任何 GameTime/TerrainHeightsData 读取面）；param_5 = 位置源
/// `*param_5 + 0x2c`（I：MainCamera raw +0x2c 假设，reviewer 候选）。
#[allow(clippy::too_many_arguments)]
pub fn display_glade_border(
    mode: Res<AppMode>,
    border: Res<GladeBorder>,
    mut signifiers: ResMut<UiSignifierStream>,
    mut lines: ResMut<GladeBorderDebugLines>,
    camera: Res<MainCamera>,
    mut display: EventReader<DisplayOutOfBorder>,
    mut overlay: EventWriter<ShowDemoBorderOverlay>,
    config: Res<GameConfig>,
) {
    // 门禁（O：`*(int *)*param_1 == 0x13` = Photomode 早退）。
    if mode.native_tag() == AppMode::Photomode.native_tag() {
        return;
    }
    // 事件门（O：EventReader 游标推进到双缓冲末尾——一帧全量消费；
    // `uVar2-uVar15 != 0 || uVar8-uVar11 != 0` 双缓冲任一非空）。
    if display.read().last().is_none() {
        return;
    }
    let points: Vec<Vec3> = match border.0 {
        BorderShape::Circle(circle) => circle
            .as_curve2(BORDER_CURVE_STEP)
            .points
            .into_iter()
            .map(|p| Vec3::new(p.x, 0.0, p.y))
            .collect(),
        BorderShape::Rectangle(rect) => rect
            .as_points2()
            .into_iter()
            .map(|p| Vec3::new(p.x, 0.0, p.y))
            .collect(),
    };
    // O：DebugLine 构建链（0x140b04e70/eb0/f40/ee0/f70）。
    let line = DebugLine::new(points.clone())
        .color(DebugColor::Red)
        .line_width(4.0)
        .set_ui_pass(false);
    lines.points = points;
    lines.line = Some(line);
    // O：进程全局 `*0x14362c098 == 1` → ShowDemoBorderOverlay。
    if border_overlay_enabled() {
        overlay.write(ShowDemoBorderOverlay);
    }
    // O：`*(char *)(*param_8 + 0x121) == 0` → 边界 perimeter signifier。
    if !config.border_signifier_enabled {
        let perimeter = match border.0 {
            BorderShape::Circle(circle) => circle.perimeter(),
            BorderShape::Rectangle(rect) => rect.perimeter(),
        };
        // O：0x20B 规范记录 {1, 1, Vec3, perimeter}（40B 布局中央裁决）。
        // 位置源 = `*param_5 + 0x2c`（I：MainCamera raw 布局 +0x2c Vec3 假设，
        // reviewer 候选——未证实，待 re-analyst）。
        let position = Vec3::new(
            camera.read_f32(0x2c),
            camera.read_f32(0x30),
            camera.read_f32(0x34),
        );
        signifiers.write(UiAppendSignifier {
            tag: 1,
            kind: 1,
            position,
            scale: perimeter,
            name: 0,
        });
    }
}

/// 进程全局门禁（O：`*0x14362c098 == 1` 时发射 overlay；建模同 water crate 的
/// 门禁常量族——本 crate 独立常量）。
pub fn border_overlay_enabled() -> bool {
    BORDER_OVERLAY_ENABLED
}
const BORDER_OVERLAY_ENABLED: bool = false;
