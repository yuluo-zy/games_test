//! draw_water.rs — 水面绘制 / 水面橡皮（B 0x141ba9130，O；fresh 2026-10-17 逐语句重建）。
//!
//! 模式分发（O）：`sel = mode_tag - 5`；`sel==4`（tag 9 = 橡皮，层掩码 `&3==0` →
//! `EraserLayer::All | Water`）、`sel==10`（tag 15 = 水）；其余早退。
//! 共用尾部（O）：`DrawState::update`（0x140ae4bd0）→ 4 相位 switch（0x142eebcb0
//! jumptable：0→return、1→begin(0)、2→extend(2)、3→commit(3)）→
//! `RecalculateHeightMapCmd` 事件 → `ScheduledTerrainReadback::on_terrain_modified`
//! （0x140aabdc0）→ `draw_raster<WaterRasterStagingSsbo, WorldRaster<512>>`（0x142260ae0）。
//!
//! 常量（O，字节直读）：橡皮半径 `t = inverse_lerp(0.3, 2.0, eraser_size)`（0x1429258d8 /
//! 0x14292e6f0，无钳制）后 `t*4 + (1-t)`（0x142934694 / 0x142925904）；水半径 =
//! `BrushSizes.water`；界内圈半径 = `0.8 * brush_size`（0x142a16cb8）；橡皮指示符缩放 =
//! `0.5 * brush_size`（0x142925870）；图钉高度 = `pos.y + 1.5`（0x1429258d0）；
//! 图钉载荷标志 `0x100`。`assert!(brush_size.is_finite())`（panic 串 0x142eebba0 =
//! `'assertion failed: brush_size.is_finite()'`）。
//!
//! ## `closure$0`（0x141ba9700，2026-10-17 晚 restorer-consume-debugline 落库）
//! 橡皮指示符的 `SignifierFactory` 闭包（PDB `closure_env$0`）。fresh disasm 逐指令：
//! env = ArcInner data `{position: Vec3 @0, scale: f32 @0xc}`（读 `[RDX]`/`[RDX+8]`/
//! `[RDX+0xc]`）；体为 `"brush_circle_scribble"`（0x15 B）→
//! `SignifierEntityBuilder::new(payload {0x8000000000000000, len, ptr, len, 0, 0})` →
//! `animation_duration(0.0)` → `position(env.position)`（0x14089dc00 写 `+0x7c/+0x84`）→
//! `scale(env.scale)`（0x14089dc60 写 `+0x50=1/+0x54`）→ `set_ui_pass(0)` → `build` →
//! 返回 1 元素 `Vec<Entity>`。**更正**：PDB `closure_env$0` 的
//! `(AssetMeshLibrary, AssetShaderLibrary, Commands)` 是闭包**实参**（vtable call 面），
//! 不是捕获；真实捕获只有 `{position, scale}`。

use std::sync::Arc;

use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use glam::{Vec2, Vec3};
use utils::geometry::Circle2d;

use country_core::asset_libraries::mesh_library::AssetMeshLibrary;
use country_core::asset_libraries::shader_library::AssetShaderLibrary;
use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::resources::active_raster_stroke::ActiveRasterStrokesStorage;
use country_core::resources::brush_sizes::BrushSizes;
use country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast;
use country_core::resources::glade_border::{
    BorderShape, DisplayOutOfBorder, FeedbackInputOutOfBorder, GladeBorder,
};
use country_core::resources::mouse_terrain_raycast::CameraCursorRay;
use country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed;
use country_core::resources::walls::{
    EraserSizeMultiplier, decorator_archivist::DecoratorArchivist,
};
use country_core::resources::water::{WaterDrawEvent, WaterEraseEvent};
use country_core::resources::world_raster_defs::{WaterRaster, WaterRasterStagingSsbo};
use country_core::systems::history::history_process::DebugHistoryMsg;
use country_core::systems::mode_manager::{AppMode, EraserLayer};
use country_core::systems::paths::DrawState;
use country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd;
use country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback;
use country_core::systems::ui::resources::ui_state::UiState;
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;
use country_core::systems::ui::signifiers::{SignifierEntityBuilder, SignifierFactory};
use country_core::systems::world_raster::{RasterDrawResources, RasterStrokeCmd, draw_raster};

use crate::{record_water_history::RecordWaterEditToHistoryCmd, water_systems_enabled};

/// 橡皮半径 inverse_lerp 端点（O：0x1429258d8 = 0.3 / 0x14292e6f0 = 2.0）。
const ERASER_LERP_MIN: f32 = 0.3;
const ERASER_LERP_MAX: f32 = 2.0;
/// 橡皮半径重映射（O：`x * 0x142934694 + (0x142925904 - x)` = `x*4 + (1-x)`）。
const ERASER_RADIUS_SCALE: f32 = 4.0;
/// 界内圈半径系数（O：0x142a16cb8 = 0.8）。
const WATER_BORDER_CIRCLE_SCALE: f32 = 0.8;
/// 橡皮笔刷指示符缩放（O：0x142925870 = 0.5）。
const ERASER_SIGNIFIER_SCALE: f32 = 0.5;
/// 橡皮指示符名（O：原版 `format!` 生成，模板 "eraser_water_"@0x142eebc2d，13 字节；
/// 经 `SignifierFactory::new`(0x14089d8b0) 把名字串拷贝进 40 B 记录
/// `{name_len, name_ptr, name_len, &ArcInner, vtable}`）。crate 侧以
/// `SignifierFactory::from_closure` 承载同名记录（闭包 = [`closure_0`]）。
const ERASER_SIGNIFIER_NAME: &str = "eraser_water_";
/// `closure$0` 的网格名（O：0x15 = 21 B，@0x142eebc98 拷贝源）。
const BRUSH_CIRCLE_SCRIBBLE: &str = "brush_circle_scribble";
/// 图钉高度抬升（O：0x1429258d0 = 1.5）。
const PIN_HEIGHT_OFFSET: f32 = 1.5;
/// WaterRaster 的栅格槽位（O：`ActiveRasterStrokesStorage::index`，`idx * 0x470`；
/// water = 4，与 `ORDERED_NAMES` 第 5 位一致）。
const WATER_RASTER_SLOT: u8 = 4;

/// Water 的圈/高图钉变体（O：机器码事件写入 0x142260a60 载荷 = {Circle2d, height,
/// union 标志 0x100}）。原版事件为 core `ui_clutter::ShowPinnedDecoClutter`（56 B union）
/// 的圈/高变体（BASELINE 参数面 `EventWriter<ShowPinnedDecoClutter>`）；core ui_clutter
/// 未恢复完整 union，按 core garden 域惯例以本 crate 专名表达 O 载荷（差登记 RESTORE.md）。
#[derive(Event, Clone, Copy, Debug)]
pub struct WaterPinnedClutterCircle {
    pub circle: Circle2d,
    pub height: f32,
    /// O：union 载荷标志（0x100 = 256）。
    pub flag: u16,
}

/// SystemParam 束（同 core `GardenRasterResources` 模式；原版为 `RasterDrawResources`
/// SystemParam——core 侧仅登记类型面，SystemParam impl 待接线，本 crate 以派生束 +
/// 借用组装等价落地；`library` 与槽位原版内嵌于该 SystemParam，本 crate 显式传入）。
#[derive(SystemParam)]
pub struct WaterRasterResources<'w, C: Send + Sync + 'static> {
    strokes: ResMut<'w, ActiveRasterStrokesStorage>,
    archivist: ResMut<'w, DecoratorArchivist<C>>,
    library: ResMut<'w, AssetSsboLibrary>,
    tool_used: ResMut<'w, Events<NotifyHintSystemToolWasJustUsed>>,
    debug_history: ResMut<'w, Events<DebugHistoryMsg>>,
}

/// `draw_water` @0x141ba9130（O，fresh 重建）。
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
    // 门禁（O：`cmp byte ptr [0x14362c098], 0`；非 0 → 早退 0x141ba9640）。
    if !water_systems_enabled() {
        return;
    }
    // 模式分派（O：sel = tag - 5；`sel==4` 橡皮、`sel==10` 水、其余 return）。
    let (brush_size, draw, widget, point, inside) = match *mode {
        AppMode::Eraser(EraserLayer::All | EraserLayer::Water) => {
            // O：`utils::inverse_lerp(0.3, 2.0, eraser_size)` 无钳制（0x140c904a0），
            //     再 `t*4 + (1-t)`。随后 `assert!(brush_size.is_finite())`
            //     （0x141ba967c：panic 串 0x142eebba0）。
            let brush_size = {
                let t = (brush.0.0 - ERASER_LERP_MIN) / (ERASER_LERP_MAX - ERASER_LERP_MIN);
                t * ERASER_RADIUS_SCALE + (1.0 - t)
            };
            assert!(brush_size.is_finite());
            // O：`UiState::clearing_canvas("eraser", argument=6, layer=0)`
            //     （0x142eebc08，R8=6、R9=0）。
            let widget = ui.clearing_canvas_w_layer_and_sound("eraser", 6, 0);
            // O：`CursorTerrainRaycast::position()`（0x140a12650）——死调用，
            //     结果被下一句 `intersect_with_xz_plane` 覆盖（0x141ba930a→0x141ba9318）。
            cursor.position();
            // O：`CameraCursorRay::intersect_with_xz_plane(0.0)`（0x1408ef180，XMM2=0）。
            let point = camera_ray.intersect_with_xz_plane(0.0);
            // O：`UiSignifierStream::write`（0x140b2cf90）写 40 B 记录。原版：
            //    `format!` 生成名字串 "eraser_water_"（13 字节）→ `SignifierFactory::new`
            //    （0x14089d8b0）= `{name_len, name_ptr(拷贝), name_len, &ArcInner, vtable}`；
            //    ArcInner<closure_env$0> = `{strong=1@0, weak=1@8, data@+0x10}`（alloc 0x20），
            //    data = `{position: Vec3 @+0x10, scale: f32 @+0x1c = 0.5×brush_size}`
            //    （fresh disasm 0x141ba9384-0x141ba93ce）；临时名字串随后 `__rust_dealloc`。
            //    crate 侧以 `from_closure` 表达同一记录，`signifier_create` 唤醒
            //    [`closure_0`] 装配 `brush_circle_scribble` 实体。
            let scale = ERASER_SIGNIFIER_SCALE * brush_size;
            let factory = SignifierFactory::from_closure(
                ERASER_SIGNIFIER_NAME,
                Arc::new(
                    move |mesh_library: &AssetMeshLibrary,
                          shader_library: &AssetShaderLibrary,
                          commands: &mut Commands| {
                        closure_0(point, scale, mesh_library, shader_library, commands)
                    },
                ),
            );
            signifiers.write_factory(&factory);
            (brush_size, false, widget, point, true)
        }
        AppMode::Water => {
            // O：`brush_size = BrushSizes.water`（Res 束 +0x20 链，偏移 4）。
            let brush_size = brush.1.water;
            assert!(brush_size.is_finite());
            // O：`clearing_canvas("watereraser_water_", argument=5, layer=4)`
            //     （0x142eebc28，R8=5、R9=4）。
            let widget = ui.clearing_canvas_w_layer_and_sound("watereraser_water_", 5, 4);
            // O：两次 `position()`——第一次（RBP+0x18）为 draw_raster 点，
            //     第二次（RBP+0x28）为界内圈圆心。
            let point = cursor.position();
            let center = {
                let pos = cursor.position();
                Vec2::new(pos.x, pos.z)
            };
            // O：界内圈 = `Circle2d::new(center, 0.8*brush_size)`（0x140c97540，
            //     XMM3 = 0x142a16cb8*brush_size），`GladeBorder::is_shape_inside`
            //     （0x140ad2970）。
            let inside = border.0.is_shape_inside(BorderShape::Circle(Circle2d::new(
                center,
                WATER_BORDER_CIRCLE_SCALE * brush_size,
            )));
            // O：界外 → `DisplayOutOfBorder`（0x141ba9432 事件 push）；随后
            //     `into_drag_state().is_some()`（0x14092df10→0x14092e6b0，= `dragged()!=2`）
            //     拖拽中 → `FeedbackInputOutOfBorder`（0x141ba9480 push）。
            if !inside {
                border.1.write(DisplayOutOfBorder);
                if widget.dragged() != 2 {
                    border.2.write(FeedbackInputOutOfBorder);
                }
            }
            (brush_size, true, widget, point, inside)
        }
        _ => return,
    };

    // O：`DrawState::update(mode_id = dragged(), raw = inside)`（0x140ae4bd0，
    //     R14=&state、EDX=dragged、R8D=inside）。
    let phase = widget.dragged();
    state.update(phase, u8::from(inside));

    // O：4 相位 switch（0x142eebcb0 jumptable，case0→0x141ba9640 return、
    //     case1→discriminant 0、case2→extend、case3→commit）。
    let command = match state.0 {
        0 => return,
        1 => 0,
        2 => {
            // O：`test bl,bl; je 0x141ba9640`（!inside → return）。
            if !inside {
                return;
            }
            // O：`test r12b,r12b; je 0x141ba9658`（!draw → 仅 discriminant=2）。
            if draw {
                // O：重取 `cursor.position()`（0x141ba9521）+ `Circle2d::new`（0x141ba9537，
                //     半径 = brush_size）+ 高度 = pos.y+1.5（0x141ba954d，RIP+0xd7c37b
                //     = 0x1429258d0）+ union 载荷 0x100（0x141ba9569），
                //     `EventWriter::write<ShowPinnedDecoClutter>`（0x142260a60）。
                let pos = cursor.position();
                pins.write(WaterPinnedClutterCircle {
                    circle: Circle2d::new(Vec2::new(pos.x, pos.z), brush_size),
                    height: pos.y + PIN_HEIGHT_OFFSET,
                    flag: 0x100,
                });
            }
            2
        }
        _ => 3,
    };

    // O 尾部：`RecalculateHeightMapCmd` 单元事件 push（0x141ba9599）→
    // `ScheduledTerrainReadback::on_terrain_modified`（0x140aabdc0）→
    // `draw_raster<WaterRasterStagingSsbo, WorldRaster<512>>`（0x142260ae0）。
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
            // O：discriminant 0/2/3（begin/extend/commit）；原版该结构无 strength 字段
            //   （case1 只写 discriminant/radius/draw，strength@-0x2c 未初始化）——
            //   core 当前 RasterStrokeCmd 带 strength，按 begin 记录常数 1.0（差登记）。
            discriminant: command,
            strength: 1.0,
            radius: brush_size,
            draw,
        },
        point,
        WATER_RASTER_SLOT,
        &mut borrowed,
        &mut water,
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

/// `draw_water::closure$0` @0x141ba9700（O，fresh disasm 0x141ba9700..0x141ba9853）：
/// 橡皮笔刷指示符的唤醒闭包。环境 = `{position: Vec3, scale: f32}`（原生 ArcInner data
/// 0x10 B）；唤醒实参 = `(&AssetMeshLibrary, &AssetShaderLibrary, &mut Commands)`
/// （vtable call 面，**非捕获**）。返回 1 元素 `Vec<Entity>`。
///
/// 逐指令对应：
/// 1. `alloc 0x15` + 拷贝 `"brush_circle_scribble"`（21 B，0x142eebc98 源）；
/// 2. 载荷 `[0x8000000000000000, 0x15, name_ptr, 0x15, ?, ?]` → `new`
///    （0x14089d9b0；尾 2 qword 原生为栈未初始化——取 0，I）；
/// 3. `animation_duration(0.0)`（0x14089dc90，XORPS XMM2）；
/// 4. `position(env+0)`（0x14089dc00：`+0x7c = [pos]`（x,y）、`+0x84 = [pos+8]`（z））；
/// 5. `scale(env+0xc)`（0x14089dc60：`+0x50 = 1`、`+0x54 = scale`）；
/// 6. `set_ui_pass(0)`（0x14089dad0，R8D=0）→ `build`（0x14089dd50）；
/// 7. sret 写 `{cap=1, ptr=heap(Entity), len=1}`。
///
/// Both actual libraries are passed through the original factory invocation.
pub fn closure_0(
    position: Vec3,
    scale: f32,
    meshes: &AssetMeshLibrary,
    shaders: &AssetShaderLibrary,
    commands: &mut Commands,
) -> Vec<Entity> {
    vec![
        SignifierEntityBuilder::from_mesh_name(BRUSH_CIRCLE_SCRIBBLE)
            .animation_duration(0.0)
            .position(position)
            .scale(scale)
            .set_ui_pass(0)
            .build(meshes, shaders, commands),
    ]
}
