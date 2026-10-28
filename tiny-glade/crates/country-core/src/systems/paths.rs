//! `paths.rs` — 路径绘制状态（B `systems::paths::DrawState::update`，0x140ae4bd0，汇编逐条核对）。
//!
//! 纠错：原函数有**三个**参数 `(&mut mode, mode_id: u8, raw: u8)`；`raw` 在 `r ∈ {1, 2}` 分支里被写入 `mode`
//! （`movzx edx, r8b; cmovae eax, edx`），文档里"写入 `r`"与机器码不符。
//!
//! 规则（`r = min(mode_id - 2 (u8 回绕，不是饱和减), 3)`，所以 `mode_id` 为 0/1 时 `r = 3`）：
//! - `r == 0`：`mode = 0`；
//! - `r ∈ {1, 2}`：`mode = if mode - 1 (u8 回绕) >= 2 { raw } else { 2 }`；
//! - `r == 3`：`mode = if mode - 1 (u8 回绕) < 2 { 3 } else { 0 }`。
//!
//! 常量（O，`draw_path` 0x140ae4d00）：笔刷缩放 `0.5525`（0x142a95dd0）、笔刷矩形半径系数 `0.8`（0x142a16cb8）。

/// 路径绘制模式字节。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DrawState(pub u8);

impl DrawState {
    /// `DrawState::update`（0x140ae4bd0）。
    pub fn update(&mut self, mode_id: u8, raw: u8) {
        let r = mode_id.wrapping_sub(2).min(3);
        let shifted = self.0.wrapping_sub(1);
        self.0 = match r {
            0 => 0,
            1 | 2 => {
                if shifted >= 2 {
                    raw
                } else {
                    2
                }
            }
            _ => {
                if shifted < 2 {
                    3
                } else {
                    0
                }
            }
        };
    }
}

/// `draw_path` 里 `eraser` 模式的笔刷缩放（O：`0x142a95dd0` fresh read_memory =
/// `0x3fd70a3d` = **1.68**；旧注释写 0.5525 与字节不符，2026-10-17 修正）。
pub const ERASER_BRUSH_SCALE: f32 = 1.68;
/// `draw_path` 笔刷矩形尺寸系数（O：0x142a16cb8）。
pub const BRUSH_RECT_SCALE: f32 = 0.8;

#[cfg(test)]
mod tests {
    use super::*;

    fn run(mode: u8, mode_id: u8, raw: u8) -> u8 {
        let mut s = DrawState(mode);
        s.update(mode_id, raw);
        s.0
    }

    #[test]
    fn only_mode_id_two_resets_to_zero() {
        assert_eq!(run(2, 2, 9), 0);
        assert_eq!(run(3, 2, 9), 0);
    }

    #[test]
    fn mode_ids_zero_and_one_wrap_around_and_behave_like_the_toggle_branch() {
        for id in [0u8, 1] {
            assert_eq!(run(1, id, 9), 3, "mode_id {id}");
            assert_eq!(run(0, id, 9), 0, "mode_id {id}");
        }
    }

    #[test]
    fn mode_ids_three_and_four_take_raw_unless_the_state_is_one_or_two() {
        for id in [3u8, 4] {
            assert_eq!(run(0, id, 9), 9);
            assert_eq!(run(3, id, 9), 9);
            assert_eq!(run(1, id, 9), 2);
            assert_eq!(run(2, id, 9), 2);
        }
    }

    #[test]
    fn mode_ids_five_and_above_toggle_between_three_and_zero() {
        for id in [5u8, 6, 200] {
            assert_eq!(run(1, id, 9), 3);
            assert_eq!(run(2, id, 9), 3);
            assert_eq!(run(0, id, 9), 0);
            assert_eq!(run(3, id, 9), 0);
        }
    }

    #[test]
    fn state_zero_wraps_to_255_and_counts_as_not_one_or_two() {
        assert_eq!(run(0, 3, 7), 7);
        assert_eq!(run(0, 5, 7), 0);
    }
}

// ---------------------------------------------------------------- 组 A 接线

/// 路径栅格资源束（O：`systems::paths::PathRastersResources`；森林/树的
/// 候选评估以 `as_float_grid`（0x140ae4c20）采样四路径栅格。字段 @+0x00/+0x20/
/// +0x40/+0x60（render/nani/instance_updates.rs 注）——语义 = 四栅格引用）。
#[derive(bevy_ecs::system::SystemParam)]
pub struct PathRastersResources<'w> {
    path: bevy_ecs::prelude::ResMut<'w, crate::resources::world_raster_defs::PathRaster>,
    cobblestone1: bevy_ecs::prelude::ResMut<'w, crate::resources::world_raster_defs::Cobblestone1Raster>,
    cobblestone2: bevy_ecs::prelude::ResMut<'w, crate::resources::world_raster_defs::Cobblestone2Raster>,
    cobblestone3: bevy_ecs::prelude::ResMut<'w, crate::resources::world_raster_defs::Cobblestone3Raster>,
}

impl PathRastersResources<'_> {
    /// `as_float_grid`（0x140ae4c20，O fresh 2026-10-17）：**无 index 参数**，一次返回
    /// 全部四个路径栅格的浮点引用。
    ///
    /// 原生把 `PathRastersResources` 四个元素（偏移 +0x00/+0x20/+0x40/+0x60，每个是用
    /// 例的 `WorldRaster` 指针）按 `selector@+0x298` 选 composite（+0x8/+0x10）或
    /// scratch（+0x20/+0x28），把 4×{ptr,len} = 64 B 写入 sret 缓冲。顺序 =
    /// path / cobblestone1 / cobblestone2 / cobblestone3。调用方：
    /// `sample_wall_path_intersections` / `manage_tree_prefabs` / `fish` 等 8 处。
    ///
    /// 旧实现 `as_float_grid(index: u8)` 按索引取一，是自造近似；已按原生签名重写。
    pub fn as_float_grid(&self) -> [crate::resources::world_raster::RasterBufferRef<'_, 512>; 4] {
        [
            (**self.path).as_ref(),
            (**self.cobblestone1).as_ref(),
            (**self.cobblestone2).as_ref(),
            (**self.cobblestone3).as_ref(),
        ]
    }
}

/// 路径工具笔刷预览组件（O：`SetSizeBasedOnAppModePath`；`path_brush_preview_size`
/// 的 `Query<&mut Transform, With<…>>` 过滤器）。
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, Default)]
pub struct SetSizeBasedOnAppModePath;

/// 路径笔刷预览圆事件（O：`ShowPinnedDecoClutter` 的 circle/height 特化，56 B 复合
/// 事件槽；与 garden 的 `GardenPinnedClutterCircle` 同族，draw_garden 注释已说明该
/// 56 B union 的渲染器未恢复）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug)]
pub struct PathPinnedClutterCircle {
    pub circle: utils::geometry::Circle2d,
    pub height: f32,
}

/// `path_brush_preview_size` 0x140b01f?（系统名；PDB 0x140b1c010）。
///
/// Fresh 反编译（2026-10-17）逐分支：
/// - 门：`AppMode` native tag ∈ {7, 8}（Path / Cobblestone(_)）；其它直接返回；
/// - `brush_size = BrushSizes.path * 1.25`（无 is_finite 断言）；
/// - Query 必须恰好一个匹配实体；空或多个均不写（原608B查询第二项判错）：
///   `scale.x = scale.z = brush_size`，`scale.y = max(brush_size, 1.0)`；
///   Transform 偏移 +0x1c/+0x20/+0x24。
pub mod path_brush_preview_size {
    use super::*;
    use crate::resources::brush_sizes::BrushSizes;
    use crate::systems::mode_manager::AppMode;
    use bevy_ecs::prelude::*;
    use crate::utils::components::transform::Transform;

    pub fn path_brush_preview_size(
        mode: Res<AppMode>,
        mut query: Query<ComponentWrite<Transform>, With<SetSizeBasedOnAppModePath>>,
        brush: Res<BrushSizes>,
    ) {
        // O：`ADD EAX,-7; CMP EAX,1; JA return` → native tag ∈ {7,8}。
        if !matches!(*mode, AppMode::Path | AppMode::Cobblestone(_)) {
            return;
        }
        // O：`MULSS [0x142a8d600]` = 1.25。
        let size = brush.path * 1.25;
        if let Ok(mut transform) = query.single_mut() {
            transform.scale.x = size;
            transform.scale.z = size;
            // O：`MAXSS XMM0, [0x142925904]`（=1.0）：NaN 时返回 1.0（SSE maxss 语义）。
            transform.scale.y = if size > 1.0 { size } else { 1.0 };
        }
    }
}

/// `draw_path` 0x140ae4d00（O，fresh 反编译 + 机器码 2026-10-17）。
///
/// 结构（逐分支对照）：
/// 1. `AppMode` native tag：Path=7 / Cobblestone=8 / Eraser=9，其余返回；`r = tag-5`
///    仅 2/3/4 进入。Eraser layer `& 5 != 0` → 返回（只擦 All/Path）。
/// 2. 半径：Path/Cobblestone = `BrushSizes.path`；Eraser = `EraserSizeMultiplier.0 * 1.68`；
///    三者都 `assert!(brush_size.is_finite())`。
/// 3. `UiState::clearing_canvas_w_layer_and_sound(name, arg, 3)`：
///    name/arg = ("path",3) / ("cobblestone",3) / ("eraser",7)。
/// 4. 圆：`Circle2d::new((x, z), radius * 0.8)`，`GladeBorder::is_shape_inside`；
///    `!draw || inside`（原生 Eraser 分支短路，不构造圆）。
/// 5. 圆外：`DisplayOutOfBorder` 恒发；`widget.dragged() != 2` 时再发
///    `FeedbackInputOutOfBorder`。
/// 6. `DrawState::update` 内联（同 `DrawState::update` 的反编译），决定
///    `discriminant ∈ {0,2,3}`；state 从 1/2 进入 2 且 `draw` 时发 `PathPinnedClutterCircle`
///    （circle 半径 = 全半径，height = pos.y + 1.5 / 无命中时 1.5）。
/// 7. 分派（selector 0..4）：先按固定顺序对**其它三种变体**各调一次 `draw_raster`
///    （命令 `draw=false`、回调全空），再对目标变体调 `draw_raster`：
///    - selector 0 Path：C1, C2, C3 → Path(slot 0)；回调写 PathDraw/EraseEvent +
///      RecordPathEditToHistoryCmd；
///    - 1 C1：Path, C2, C3 → C1(slot 1)；RecordCobblestone1…；
///    - 2 C2：C1, Path, C3 → C2(slot 2)；RecordCobblestone2…；
///    - 3 C3：C1, C2, Path → C3(slot 3)；RecordCobblestone3…；
///    - 4 Eraser：C1, C2, C3 → Path(slot 0)（draw=false，走 on_erase）。
///
/// 登记（U）：`RasterStrokeCmd.strength`（原生两条命令栈槽 +0x8c/+0x9c 从未写入；
/// 读取方 `start_new` 把它当 param_b）。`erase_other_path_variants` 的 3 组闭包
/// （`closure_env_0..8`）载荷未逐字节恢复；本实现按“无事件回调”执行其擦除调用。
pub mod draw_path {
    use super::{DrawState, PathPinnedClutterCircle, BRUSH_RECT_SCALE, ERASER_BRUSH_SCALE};
    use crate::{
        asset_libraries::ssbo_library::AssetSsboLibrary,
        resources::{
            active_raster_stroke::ActiveRasterStrokesStorage,
            brush_sizes::BrushSizes,
            cursor_terrain_raycast::CursorTerrainRaycast,
            glade_border::{
                BorderShape, DisplayOutOfBorder, FeedbackInputOutOfBorder, GladeBorder,
            },
            onboarding::NotifyHintSystemToolWasJustUsed,
            path_stroke::{PathDrawEvent, PathEraseEvent},
            walls::{decorator_archivist::DecoratorArchivist, EraserSizeMultiplier},
            world_raster_defs::{
                Cobblestone1Raster, Cobblestone1RasterStagingSsbo, Cobblestone2Raster,
                Cobblestone2RasterStagingSsbo, Cobblestone3Raster, Cobblestone3RasterStagingSsbo,
                PathRaster, PathRasterStagingSsbo,
            },
        },
        systems::{
            history::history_process::DebugHistoryMsg,
            mode_manager::AppMode,
            paths::record_path_history::{
                RecordCobblestone1EditToHistoryCmd, RecordCobblestone2EditToHistoryCmd,
                RecordCobblestone3EditToHistoryCmd, RecordPathEditToHistoryCmd,
            },
            ui::resources::ui_state::UiState,
            world_raster::{draw_raster, RasterDrawResources, RasterStrokeCmd},
        },
    };
    use bevy_ecs::prelude::*;
    use bevy_ecs::system::SystemParam;
    use glam::Vec2;
    use utils::geometry::Circle2d;

    /// 绘画侧可变资源束（同 garden 的 `GardenRasterResources`）。
    #[derive(SystemParam)]
    pub struct PathRasterResources<'w, C: Send + Sync + 'static> {
        pub strokes: ResMut<'w, ActiveRasterStrokesStorage>,
        pub archivist: ResMut<'w, DecoratorArchivist<C>>,
        pub library: ResMut<'w, AssetSsboLibrary>,
        pub tool_used: ResMut<'w, Events<NotifyHintSystemToolWasJustUsed>>,
        pub debug_history: ResMut<'w, Events<DebugHistoryMsg>>,
    }

    /// 四个路径栅格的可变借用（原生 draw_path 的四个 ResMut 参数；本 crate 的 bevy
    /// 系统参数元组上限 16，故与事件写手一并成组）。
    #[derive(SystemParam)]
    pub struct PathRasterSet<'w> {
        pub path: ResMut<'w, PathRaster>,
        pub cobblestone1: ResMut<'w, Cobblestone1Raster>,
        pub cobblestone2: ResMut<'w, Cobblestone2Raster>,
        pub cobblestone3: ResMut<'w, Cobblestone3Raster>,
    }

    /// 路径绘制/擦除/历史记录事件写手（原生的 6 个 EventWriter 参数）。
    #[derive(SystemParam)]
    pub struct PathEventWriters<'w> {
        pub draw: EventWriter<'w, PathDrawEvent>,
        pub erase: EventWriter<'w, PathEraseEvent>,
        pub record_path: EventWriter<'w, RecordPathEditToHistoryCmd>,
        pub record_c1: EventWriter<'w, RecordCobblestone1EditToHistoryCmd>,
        pub record_c2: EventWriter<'w, RecordCobblestone2EditToHistoryCmd>,
        pub record_c3: EventWriter<'w, RecordCobblestone3EditToHistoryCmd>,
    }

    /// 预览圆高度偏移（O：0x1429258d0 = 1.5）。
    const PIN_HEIGHT_OFFSET: f32 = 1.5;
    /// 命令 strength 槽（原生未写，见模块文档登记）。
    const PATH_STRENGTH: f32 = 0.0;

    #[allow(clippy::too_many_arguments)]
    pub fn draw_path<C: Send + Sync + 'static>(
        mode: Res<AppMode>,
        mut resources: PathRasterResources<C>,
        ui: Res<UiState>,
        mut rasters: PathRasterSet,
        mut events: PathEventWriters,
        cursor: Res<CursorTerrainRaycast>,
        border: (
            Res<GladeBorder>,
            EventWriter<DisplayOutOfBorder>,
            EventWriter<FeedbackInputOutOfBorder>,
        ),
        mut state: Local<DrawState>,
        brush: (Res<EraserSizeMultiplier>, Res<BrushSizes>),
        mut pins: EventWriter<PathPinnedClutterCircle>,
    ) {
        // 1+2：模式 → (半径, draw, name, argument, selector)。
        let (radius, draw, name, argument, selector): (f32, bool, &'static str, u8, u8) = match *mode
        {
            AppMode::Path => (brush.1.path, true, "path", 3, 0),
            AppMode::Cobblestone(variant) => {
                // O：`lea rax,[payload-1]; cmp rax,3; jnc panic` → payload ∈ {1,2,3}。
                assert!(
                    (1..=3).contains(&variant),
                    "cobblestone variant out of range"
                );
                (brush.1.path, true, "cobblestone", 3, variant as u8)
            }
            AppMode::Eraser(layer) => {
                // O：`movzx eax, byte [AppMode+4]; test al,5; jnz return`。
                if (layer as u8) & 5 != 0 {
                    return;
                }
                (
                    brush.0 .0 * ERASER_BRUSH_SCALE,
                    false,
                    "eraser",
                    7,
                    4,
                )
            }
            _ => return,
        };
        // O：三处 `assert!(brush_size.is_finite())`（0x140ae57c6/57de/57f6）。
        assert!(radius.is_finite(), "brush_size.is_finite()");

        // 3+4+5：UI 用具 + 边框检查 + 圆外事件（顺序同 0x140ae4f08..0x50bd）。
        let widget = ui.clearing_canvas_w_layer_and_sound(name, argument, 3);
        let position = cursor.position();
        let center = Vec2::new(position.x, position.z);
        let (border, mut display_outside, mut feedback_outside) = border;
        let allowed = !draw
            || border.is_shape_inside(BorderShape::Circle(Circle2d::new(
                center,
                radius * BRUSH_RECT_SCALE,
            )));
        let phase = widget.dragged();
        if !allowed {
            display_outside.write(DisplayOutOfBorder);
            if phase != 2 {
                feedback_outside.write(FeedbackInputOutOfBorder);
            }
        }

        // 6：DrawState::update 内联（0x140ae50d1..0x51e1）。
        let r = phase.wrapping_sub(2).min(3);
        let command: u32;
        if r.wrapping_sub(1) < 2 {
            if state.0.wrapping_sub(1) < 2 {
                state.0 = 2;
                if !allowed {
                    return;
                }
                if draw {
                    pins.write(PathPinnedClutterCircle {
                        circle: Circle2d::new(center, radius),
                        height: if cursor.tag & 1 != 0 {
                            position.y + PIN_HEIGHT_OFFSET
                        } else {
                            PIN_HEIGHT_OFFSET
                        },
                    });
                }
                command = 2;
            } else {
                state.0 = allowed as u8;
                if !allowed {
                    return;
                }
                command = 0;
            }
        } else {
            if r == 0 || state.0.wrapping_sub(1) >= 2 {
                state.0 = 0;
                return;
            }
            state.0 = 3;
            command = 3;
        }

        // 7：分派。命令 draw=false 用于“擦除其它变体”与 Eraser 目标。
        let erase_command = RasterStrokeCmd {
            discriminant: command,
            strength: PATH_STRENGTH,
            radius,
            draw: false,
        };
        let draw_command = RasterStrokeCmd {
            discriminant: command,
            strength: PATH_STRENGTH,
            radius,
            draw,
        };
        let point = position;

        /// 空回调擦除调用（原生 `erase_other_path_variants` 的擦除侧；闭包载荷登记 U）。
        macro_rules! erase_call {
            ($staging:ty, $raster:expr, $slot:expr) => {{
                let mut borrowed = RasterDrawResources {
                    strokes: &mut resources.strokes,
                    archivist: &mut resources.archivist,
                    cursor: &cursor,
                    tool_used: &mut resources.tool_used,
                    debug_history: &mut resources.debug_history,
                };
                draw_raster::<$staging, C>(
                    erase_command,
                    point,
                    $slot,
                    &mut borrowed,
                    &mut $raster,
                    &mut resources.library,
                    |_, _, _| {},
                    |_, _, _| {},
                    || {},
                );
            }};
        }

        macro_rules! target_call {
            ($staging:ty, $raster:expr, $slot:expr, $writer:expr, $record:expr) => {{
                let mut borrowed = RasterDrawResources {
                    strokes: &mut resources.strokes,
                    archivist: &mut resources.archivist,
                    cursor: &cursor,
                    tool_used: &mut resources.tool_used,
                    debug_history: &mut resources.debug_history,
                };
                draw_raster::<$staging, C>(
                    draw_command,
                    point,
                    $slot,
                    &mut borrowed,
                    &mut $raster,
                    &mut resources.library,
                    |position, radius, observed_flag| {
                        events.draw.write(PathDrawEvent {
                            position,
                            radius,
                            observed_flag,
                        });
                    },
                    |position, radius, observed_flag| {
                        events.erase.write(PathEraseEvent {
                            position,
                            radius,
                            observed_flag,
                        });
                    },
                    || {
                        $writer.write($record);
                    },
                );
            }};
        }

        match selector {
            // Path 目标（case 0：0x140ae5311）。
            0 => {
                erase_call!(Cobblestone1RasterStagingSsbo, rasters.cobblestone1, 1);
                erase_call!(Cobblestone2RasterStagingSsbo, rasters.cobblestone2, 2);
                erase_call!(Cobblestone3RasterStagingSsbo, rasters.cobblestone3, 3);
                target_call!(PathRasterStagingSsbo, rasters.path, 0, events.record_path, RecordPathEditToHistoryCmd);
            }
            // Cobblestone1 目标（case 1：0x140ae56b7）。
            1 => {
                erase_call!(PathRasterStagingSsbo, rasters.path, 0);
                erase_call!(Cobblestone2RasterStagingSsbo, rasters.cobblestone2, 2);
                erase_call!(Cobblestone3RasterStagingSsbo, rasters.cobblestone3, 3);
                target_call!(Cobblestone1RasterStagingSsbo, rasters.cobblestone1, 1, events.record_c1, RecordCobblestone1EditToHistoryCmd);
            }
            // Cobblestone2 目标（case 2：0x140ae54f1）。
            2 => {
                erase_call!(Cobblestone1RasterStagingSsbo, rasters.cobblestone1, 1);
                erase_call!(PathRasterStagingSsbo, rasters.path, 0);
                erase_call!(Cobblestone3RasterStagingSsbo, rasters.cobblestone3, 3);
                target_call!(Cobblestone2RasterStagingSsbo, rasters.cobblestone2, 2, events.record_c2, RecordCobblestone2EditToHistoryCmd);
            }
            // Cobblestone3 目标（case 3：0x140ae55d4）。
            3 => {
                erase_call!(Cobblestone1RasterStagingSsbo, rasters.cobblestone1, 1);
                erase_call!(Cobblestone2RasterStagingSsbo, rasters.cobblestone2, 2);
                erase_call!(PathRasterStagingSsbo, rasters.path, 0);
                target_call!(Cobblestone3RasterStagingSsbo, rasters.cobblestone3, 3, events.record_c3, RecordCobblestone3EditToHistoryCmd);
            }
            // Eraser 目标（case 4：0x140ae540a）：draw=false，Path 走 on_erase。
            _ => {
                erase_call!(Cobblestone1RasterStagingSsbo, rasters.cobblestone1, 1);
                erase_call!(Cobblestone2RasterStagingSsbo, rasters.cobblestone2, 2);
                erase_call!(Cobblestone3RasterStagingSsbo, rasters.cobblestone3, 3);
                target_call!(PathRasterStagingSsbo, rasters.path, 0, events.record_path, RecordPathEditToHistoryCmd);
            }
        }
    }
}

/// `record_path_history` 0x140b1c270（O，fresh 反编译 2026-10-17）。
///
/// 四个同构块：每个 EventReader 若**有未读事件**（原生先把游标写成 events 总计数，
/// 即完整消费），就对 `ActiveRasterStrokesStorage` 对应槽做
/// `take_history_stroke()` + `History::add(EditType::…)`：
/// 块序 Path(slot 0) / Cobblestone1(1) / Cobblestone2(2) / Cobblestone3(3)
/// （原生槽基址 +0x48/+0x4b8/+0x928/+0xd98，stride 0x470）。
/// 事件载荷被忽略。
pub mod record_path_history {
    use crate::resources::{
        active_raster_stroke::ActiveRasterStrokesStorage, history::{EditType, History},
    };
    use crate::systems::world_raster::record_raster_history;
    use bevy_ecs::prelude::*;
    use bevy_ecs::system::SystemParam;

    #[derive(Event, Clone, Copy, Debug)]
    pub struct RecordPathEditToHistoryCmd;
    #[derive(Event, Clone, Copy, Debug)]
    pub struct RecordCobblestone1EditToHistoryCmd;
    #[derive(Event, Clone, Copy, Debug)]
    pub struct RecordCobblestone2EditToHistoryCmd;
    #[derive(Event, Clone, Copy, Debug)]
    pub struct RecordCobblestone3EditToHistoryCmd;

    /// `RasterHistoryRecordResources`（O：`country_core::systems::world_raster`；
    /// 原生布局 = `ResMut<ActiveRasterStrokesStorage>` + `ResMut<History>`）。
    #[derive(SystemParam)]
    pub struct RasterHistoryRecordResources<'w> {
        pub strokes: ResMut<'w, ActiveRasterStrokesStorage>,
        pub history: ResMut<'w, History>,
    }

    pub fn record_path_history(
        mut resources: RasterHistoryRecordResources,
        mut path: EventReader<RecordPathEditToHistoryCmd>,
        mut cobblestone1: EventReader<RecordCobblestone1EditToHistoryCmd>,
        mut cobblestone2: EventReader<RecordCobblestone2EditToHistoryCmd>,
        mut cobblestone3: EventReader<RecordCobblestone3EditToHistoryCmd>,
    ) {
        if path.read().last().is_some() {
            let _ = record_raster_history(&mut resources.strokes, 0, &mut resources.history, EditType::Path);
        }
        if cobblestone1.read().last().is_some() {
            let _ = record_raster_history(
                &mut resources.strokes,
                1,
                &mut resources.history,
                EditType::Cobblestone1,
            );
        }
        if cobblestone2.read().last().is_some() {
            let _ = record_raster_history(
                &mut resources.strokes,
                2,
                &mut resources.history,
                EditType::Cobblestone2,
            );
        }
        if cobblestone3.read().last().is_some() {
            let _ = record_raster_history(
                &mut resources.strokes,
                3,
                &mut resources.history,
                EditType::Cobblestone3,
            );
        }
    }
}

/// `systems::paths` 插件 build（O：`impl$0::build` 0x140b68e40，fresh 2026-10-17）。
///
/// 原生：
/// 1. `add_systems(Update, record_path_history.in_set(GameSystemSet))`（`chain` +
///    `in_set<GameSystemSet>`）；crate 无 bevy_app 也无 `GameSystemSet`，本实现把系统
///    加到传入的 `Schedule`（宿主按 Update/组顺序接入）。
/// 2. `insert_resource<PathRaster>` / `Cobblestone1/2/3Raster`（各自 `WorldRaster::new<512>`）
///    + `insert_resource<BrushSizes>`。
/// 3. `register_event`：RecordPath/RecordCobblestone1/2/3EditToHistoryCmd、
///    PathDrawEvent、PathEraseEvent。
pub struct PathsPlugin;
impl PathsPlugin {
    pub fn build(world: &mut bevy_ecs::world::World, schedule: &mut bevy_ecs::schedule::Schedule) {
        use crate::resources::{
            brush_sizes::{BrushSizes, GARDEN_DEFAULT, PATH_DEFAULT, WATER_DEFAULT},
            path_stroke::{PathDrawEvent, PathEraseEvent},
            world_raster::DirtyTilePublisher,
            world_raster_defs::{
                Cobblestone1Raster, Cobblestone2Raster, Cobblestone3Raster, PathRaster,
            },
        };
        use bevy_ecs::prelude::*;
        // 1. 系统装配（原生 `record_path_history.in_set(GameSystemSet)`）。
        schedule.add_systems(record_path_history::record_path_history);
        // 2. 资源。
        world.insert_resource(PathRaster::new(DirtyTilePublisher::unique()));
        world.insert_resource(Cobblestone1Raster::new(DirtyTilePublisher::unique()));
        world.insert_resource(Cobblestone2Raster::new(DirtyTilePublisher::unique()));
        world.insert_resource(Cobblestone3Raster::new(DirtyTilePublisher::unique()));
        world.insert_resource(BrushSizes {
            path: PATH_DEFAULT,
            water: WATER_DEFAULT,
            garden: GARDEN_DEFAULT,
        });
        // 3. 事件。
        world.init_resource::<Events<record_path_history::RecordPathEditToHistoryCmd>>();
        world.init_resource::<Events<record_path_history::RecordCobblestone1EditToHistoryCmd>>();
        world.init_resource::<Events<record_path_history::RecordCobblestone2EditToHistoryCmd>>();
        world.init_resource::<Events<record_path_history::RecordCobblestone3EditToHistoryCmd>>();
        world.init_resource::<Events<PathDrawEvent>>();
        world.init_resource::<Events<PathEraseEvent>>();
        world.init_resource::<Events<PathPinnedClutterCircle>>();
    }
}

#[cfg(test)]
mod world_raster_tests {
    use super::*;
    use crate::asset_libraries::ssbo_library::AssetSsboLibrary;
    use crate::resources::history::transactions::ObservedHistoryLedger;
    use crate::resources::active_raster_stroke::ActiveRasterStrokesStorage;
    use crate::resources::brush_sizes::BrushSizes;
    use crate::resources::glade_border::BorderShape;
    use crate::resources::glade_border::DisplayOutOfBorder;
    use crate::resources::glade_border::FeedbackInputOutOfBorder;
    use crate::resources::glade_border::GladeBorder;
    use crate::resources::history::EditType;
    use crate::resources::history::History;
    use crate::resources::path_stroke::PathDrawEvent;
    use crate::resources::path_stroke::PathEraseEvent;
    use crate::resources::walls::decorator_archivist::DecoratorArchivist;
    use crate::resources::walls::decorator_archivist::DecoratorCullPolicy;
    use crate::resources::walls::EraserSizeMultiplier;
    use crate::resources::world_raster::DirtyTilePublisher;
    use crate::resources::world_raster_defs::Cobblestone1Raster;
    use crate::resources::world_raster_defs::Cobblestone1RasterStagingSsbo;
    use crate::resources::world_raster_defs::Cobblestone2Raster;
    use crate::resources::world_raster_defs::Cobblestone2RasterStagingSsbo;
    use crate::resources::world_raster_defs::Cobblestone3Raster;
    use crate::resources::world_raster_defs::Cobblestone3RasterStagingSsbo;
    use crate::resources::world_raster_defs::PathRaster;
    use crate::resources::world_raster_defs::PathRasterStagingSsbo;
    use crate::systems::mode_manager::AppMode;
    use crate::systems::mode_manager::EraserLayer;
    use crate::systems::paths::record_path_history::RecordCobblestone1EditToHistoryCmd;
    use crate::systems::paths::record_path_history::RecordPathEditToHistoryCmd;
    use crate::systems::ui::resources::ui_state::UiInteraction;
    use crate::systems::ui::resources::ui_state::UiState;
    use crate::systems::ui::resources::ui_state::WidgetId;
    use bevy_ecs::prelude::*;
    use glam::Vec2;
    use crate::resources::cursor_terrain_raycast::CursorTerrainRaycast;
    use crate::utils::components::transform::Transform;
    use std::collections::{HashMap, HashSet};
    use utils::geometry::Circle2d;

    fn archivist() -> DecoratorArchivist<()> {
        DecoratorArchivist {
            walls: HashMap::new(),
            special1: None,
            special2: None,
            collision_a: HashMap::new(),
            collision_b: HashMap::new(),
            culling: DecoratorCullPolicy {
                allowed_ids: HashSet::new(),
                allow_any: false,
            },
        }
    }

    fn library_with_path_staging() -> AssetSsboLibrary {
        let library = AssetSsboLibrary::new();
        let device = crate::render::test_device::TestDevice(0);
        library.add_new::<PathRasterStagingSsbo>(&device);
        library.add_new::<Cobblestone1RasterStagingSsbo>(&device);
        library.add_new::<Cobblestone2RasterStagingSsbo>(&device);
        library.add_new::<Cobblestone3RasterStagingSsbo>(&device);
        library
    }

    fn history_started() -> History {
        let mut history: History = History::from_observed(ObservedHistoryLedger {
            entries: vec![],
            cursor: -1,
            undo_anchor: -1,
            resolved_snapshots: vec![],
            observed_count_58: 0,
            mutation_nonce: 0,
        });
        history.start_input_recording(17).unwrap();
        history
    }

    fn base_world(mode: AppMode, widget: WidgetId) -> World {
        let mut world = World::new();
        world.insert_resource(mode);
        world.insert_resource(UiState::new(UiInteraction {
            native_tag: 2,
            native_phase: 3,
            widget,
        }));
        world.insert_resource(BrushSizes {
            path: 0.8,
            water: 2.0,
            garden: 3.0,
        });
        world.insert_resource(EraserSizeMultiplier(1.0));
        world.insert_resource(GladeBorder(BorderShape::Circle(Circle2d::new(
            Vec2::ZERO,
            10.0,
        ))));
        let mut cursor = CursorTerrainRaycast::new();
        cursor.update(Some(glam::Vec3::new(-2.0, 0.0, 0.0)));
        world.insert_resource(cursor);
        world.insert_resource(ActiveRasterStrokesStorage::default());
        world.insert_resource(archivist());
        world.insert_resource(library_with_path_staging());
        world.insert_resource(history_started());
        world.init_resource::<Events<DisplayOutOfBorder>>();
        world.init_resource::<Events<FeedbackInputOutOfBorder>>();
        world.init_resource::<Events<crate::resources::onboarding::NotifyHintSystemToolWasJustUsed>>();
        world.init_resource::<Events<crate::systems::history::history_process::DebugHistoryMsg>>();
        world
    }

    fn set_phase(world: &mut World, widget: WidgetId, phase: u8) {
        world
            .resource_mut::<UiState>()
            .replace_interaction(UiInteraction {
                native_tag: 2,
                native_phase: phase,
                widget,
            });
    }

    #[test]
    fn path_begin_extend_commit_records_the_path_slot_once() {
        let widget = WidgetId::clearing_canvas("path");
        let mut world = base_world(AppMode::Path, widget);
        let mut schedule = Schedule::default();
        PathsPlugin::build(&mut world, &mut schedule);
        schedule.add_systems(draw_path::draw_path::<()>);

        schedule.run(&mut world); // Begin（discriminant 0，state 0 → 1）。
        assert_eq!(
            world.resource::<UiState>().clearing_canvas_params(widget).unwrap().argument_79,
            3
        );
        assert!(world.resource::<Events<PathDrawEvent>>().is_empty());
        assert_eq!(world.resource::<ActiveRasterStrokesStorage>()[0].resampled().len(), 1);

        set_phase(&mut world, widget, 4);
        world
            .resource_mut::<CursorTerrainRaycast>()
            .update(Some(glam::Vec3::new(2.0, 0.0, 0.0)));
        schedule.run(&mut world); // Extend（discriminant 2）：draw + pin 事件。
        assert_eq!(world.resource::<Events<PathDrawEvent>>().len(), 1);
        assert_eq!(world.resource::<Events<PathPinnedClutterCircle>>().len(), 1);
        assert!(world
            .resource::<PathRaster>()
            .scratch()
            .iter()
            .any(|&value| value != 0));

        set_phase(&mut world, widget, 5);
        schedule.run(&mut world); // Commit（discriminant 3）：发记录命令。
        schedule.run(&mut world); // 记录系统消费命令（下一 run 保证已入队）。
        let pending = world.resource::<History>().pending().unwrap();
        assert_eq!(pending.edits.len(), 1);
        let EditType::Path(stroke) = &pending.edits[0] else {
            panic!("wrong edit owner")
        };
        // begin 命令 discriminant = 0 → flags 0；strength 槽原生未写（登记 U）= 0.0；
        // 半径 = BrushSizes.path；draw 字节 = 1。
        assert_eq!(
            (stroke.flags, stroke.draw, stroke.radius),
            (0, true, 0.8)
        );
        assert_eq!(stroke.strength, 0.0);
        assert!(stroke.points.len() >= 2);
        assert!(world
            .resource::<PathRaster>()
            .composite()
            .iter()
            .any(|&value| value != 0));
    }

    #[test]
    fn cobblestone_targets_their_own_slot_and_widget_argument() {
        let widget = WidgetId::clearing_canvas("cobblestone");
        let mut world = base_world(AppMode::Cobblestone(2), widget);
        let mut schedule = Schedule::default();
        PathsPlugin::build(&mut world, &mut schedule);
        schedule.add_systems(draw_path::draw_path::<()>);
        schedule.run(&mut world);
        assert_eq!(
            world.resource::<UiState>().clearing_canvas_params(widget).unwrap().argument_79,
            3
        );
        // 原生 begin 帧对四个变体各调一次 draw_raster（3 个“擦除其它变体”+ 1 个目标），
        // 因此四个槽都会 start_new（resampled 长度 1）；目标槽 2 的 draw 字节为 true。
        let storage = world.resource::<ActiveRasterStrokesStorage>();
        for slot in 0..4 {
            assert_eq!(storage[slot].resampled().len(), 1, "slot {slot}");
        }
        assert_eq!(storage[2].mode(), 1);
    }

    #[test]
    fn eraser_layer_gate_and_erase_target() {
        // Eraser(Wall) → layer & 5 != 0 → 直接返回，不触碰任何槽。
        let widget = WidgetId::clearing_canvas("eraser");
        let mut world = base_world(AppMode::Eraser(EraserLayer::Wall), widget);
        let mut schedule = Schedule::default();
        PathsPlugin::build(&mut world, &mut schedule);
        schedule.add_systems(draw_path::draw_path::<()>);
        schedule.run(&mut world);
        assert!(!world
            .resource::<UiState>()
            .clearing_canvas_params(widget)
            .is_some());

        // Eraser(All) → 半径 = 1.0 * 1.68，draw=false。
        let mut world = base_world(AppMode::Eraser(EraserLayer::All), widget);
        let mut schedule = Schedule::default();
        PathsPlugin::build(&mut world, &mut schedule);
        schedule.add_systems(draw_path::draw_path::<()>);
        schedule.run(&mut world);
        assert!(world
            .resource::<UiState>()
            .clearing_canvas_params(widget)
            .is_some());
        let stroke = &world.resource::<ActiveRasterStrokesStorage>()[0];
        assert_eq!(stroke.brush_size(), 1.0 * ERASER_BRUSH_SCALE);
        assert_eq!(stroke.mode(), 0); // draw=false
    }

    #[test]
    fn preview_size_sets_uniform_scale_with_a_minimum_height() {
        let mut world = World::new();
        world.insert_resource(AppMode::Path);
        world.insert_resource(BrushSizes {
            path: 0.8,
            water: 0.0,
            garden: 0.0,
        });
        let entity = world.spawn((SetSizeBasedOnAppModePath, Transform::identity())).id();
        let mut schedule = Schedule::default();
        schedule.add_systems(path_brush_preview_size::path_brush_preview_size);
        schedule.run(&mut world);
        // 0.8 * 1.25 = 1.0 → y = max(1.0, 1.0)。
        let transform = world.get::<Transform>(entity).unwrap();
        assert_eq!(transform.scale, glam::Vec3::splat(1.0));

        world.insert_resource(AppMode::Path);
        world.insert_resource(BrushSizes {
            path: 2.0,
            water: 0.0,
            garden: 0.0,
        });
        schedule.run(&mut world);
        let transform = world.get::<Transform>(entity).unwrap();
        assert_eq!(transform.scale, glam::Vec3::splat(2.5));

        // 非 Path/Cobblestone 模式不改写。
        world.insert_resource(AppMode::Water);
        schedule.run(&mut world);
        assert_eq!(world.get::<Transform>(entity).unwrap().scale, glam::Vec3::splat(2.5));
    }

    #[derive(Resource, Default)]
    struct GridProbe(Vec<u8>);

    fn probe_grids(rasters: PathRastersResources, mut out: ResMut<GridProbe>) {
        out.0 = rasters
            .as_float_grid()
            .iter()
            .map(|grid| {
                assert_eq!(grid.data.len(), 512 * 512);
                grid.data[0]
            })
            .collect();
    }

    #[test]
    fn as_float_grid_returns_all_four_current_buffers_in_path_order() {
        let mut world = World::new();
        let mut path = PathRaster::new(DirtyTilePublisher::unique());
        let mut c1 = Cobblestone1Raster::new(DirtyTilePublisher::unique());
        let mut c2 = Cobblestone2Raster::new(DirtyTilePublisher::unique());
        let mut c3 = Cobblestone3Raster::new(DirtyTilePublisher::unique());
        path.composite_mut()[0] = 11;
        c1.composite_mut()[0] = 22;
        c2.composite_mut()[0] = 33;
        c3.composite_mut()[0] = 44;
        world.insert_resource(path);
        world.insert_resource(c1);
        world.insert_resource(c2);
        world.insert_resource(c3);
        world.init_resource::<GridProbe>();
        let mut schedule = Schedule::default();
        schedule.add_systems(probe_grids);
        schedule.run(&mut world);
        assert_eq!(world.resource::<GridProbe>().0, vec![11, 22, 33, 44]);
    }

    #[test]
    fn record_path_history_records_each_slot_only_when_its_reader_has_events() {
        let mut world = World::new();
        let mut strokes = ActiveRasterStrokesStorage::default();
        strokes[0].start_new(glam::Vec3::ZERO, 1, 0.8, 0, 0.0);
        strokes[1].start_new(glam::Vec3::X, 1, 0.8, 0, 0.0);
        world.insert_resource(strokes);
        world.insert_resource(history_started());
        world.init_resource::<Events<RecordPathEditToHistoryCmd>>();
        world.init_resource::<Events<RecordCobblestone1EditToHistoryCmd>>();
        world.init_resource::<Events<record_path_history::RecordCobblestone2EditToHistoryCmd>>();
        world.init_resource::<Events<record_path_history::RecordCobblestone3EditToHistoryCmd>>();
        let mut schedule = Schedule::default();
        schedule.add_systems(record_path_history::record_path_history);
        world
            .resource_mut::<Events<RecordPathEditToHistoryCmd>>()
            .send(RecordPathEditToHistoryCmd);
        schedule.run(&mut world);
        let pending = world.resource::<History>().pending().unwrap();
        assert_eq!(pending.edits.len(), 1);
        assert!(matches!(pending.edits[0], EditType::Path(_)));
        assert!(world.resource::<ActiveRasterStrokesStorage>()[0].resampled().is_empty());
        assert!(!world.resource::<ActiveRasterStrokesStorage>()[1].resampled().is_empty());
        // PathEraseEvent 定义为本模块生产的事件类型（编译期合同）。
        let _ = std::any::type_name::<PathEraseEvent>();
        let _ = std::any::type_name::<DisplayOutOfBorder>();
        let _ = std::any::type_name::<FeedbackInputOutOfBorder>();
        let _ = std::any::type_name::<RecordCobblestone1EditToHistoryCmd>();
    }
}

#[cfg(test)]
mod native_preview_cardinality_tests {
    use super::*;
    use crate::{
        resources::brush_sizes::BrushSizes, systems::mode_manager::AppMode,
        utils::components::transform::Transform,
    };
    use bevy_ecs::prelude::*;
    #[test]
    fn complete_original64_cardinality_mode_and_float_cases() {
        let fixtures:serde_json::Value=serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/tests/fixtures/original/round12/brush-startup/native-path-preview-fixtures.json"))).unwrap();
        for case in fixtures.as_array().unwrap() {
            let mut world = World::new();
            let mode = match case["mode"].as_u64().unwrap() {
                7 => AppMode::Path,
                8 => AppMode::Cobblestone(Default::default()),
                _ => AppMode::default(),
            };
            world.insert_resource(mode);
            world.insert_resource(BrushSizes {
                path: f32::from_bits(case["size_bits"].as_u64().unwrap() as u32),
                water: 2.,
                garden: 3.,
            });
            let ids: Vec<_> = (0..case["count"].as_u64().unwrap())
                .map(|_| {
                    world
                        .spawn((
                            SetSizeBasedOnAppModePath,
                            Transform {
                                translation: glam::Vec3::new(1., 2., 3.),
                                scale: glam::Vec3::splat(2.),
                                ..Transform::identity()
                            },
                        ))
                        .id()
                })
                .collect();
            let mut schedule = Schedule::default();
            schedule.add_systems(path_brush_preview_size::path_brush_preview_size);
            schedule.run(&mut world);
            for (id, expected) in ids
                .into_iter()
                .zip(case["native"]["transforms"].as_array().unwrap())
            {
                let t = world.get::<Transform>(id).unwrap();
                let bits: Vec<_> = t
                    .rotation
                    .to_array()
                    .into_iter()
                    .chain(t.translation.to_array())
                    .chain(t.scale.to_array())
                    .map(f32::to_bits)
                    .collect();
                let expected: Vec<_> = expected
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|x| x.as_u64().unwrap() as u32)
                    .collect();
                assert_eq!(bits, expected, "case={case}");
            }
        }
    }
}

use crate::render::extract::ComponentWrite;
