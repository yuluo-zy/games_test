//! `ui_edit_terrain.rs` — 地形编辑 UI（B 0x140aef1f0）。
//!
//! 证据：fresh 反编译（restorer-terrain-clutter/jobs/decomp/140aef1f0）+ 反汇编/常量直读。
//! 逐语句对照：
//! - 门 `*(int*)*in_RCX == 10`；
//! - 逐**笔画**注册 widget：遍历 `TerrainEditingData.strokes`（bucket stride 0x30）；
//!   `format_inner` 名字 = `"terrain_edit_{}__canvas"`（字面片 @0x142ae39f2 "terrain_edit_" +
//!   @0x142ae39ff "__canvas"）；**type 字节 = `(hovered_id != stroke_id) * 3 + 4`**
//!   （disasm 源 decomp 行 351-354：`(iVar37 != *(int*)*pauVar45)*3+4`；当
//!   `CursorTerrainRaycast.tag != 0` 时固定 7）；`UiState::add<String>`；
//! - `iVar37` = hovered stroke id：`if (*in_stack_00000078 != 0) → *(int*)(...+8)`（资源
//!   tag 非零才读 id@+8）；
//! - `iVar37 != -1 && cursor.tag == 1` → 写 `WindowFocusEvent(1)`（stride 0x10）；
//! - **每个 widget** 的 subtype = `widget.dragged()`（`native_tag≥2 ∧ widget id 匹配 →
//!   native_phase`，否则 2；decomp 行 371-410），subtype switch：
//!   - case 0（subtype 2）：无动作；
//!   - case 1（subtype 3）：`WindowFocusEvent(0)` + hover 点/朝向写入宿主 UI 状态
//!     （本 crate 以 [`TerrainEditUiState`] 承载 O 槽位）；
//!   - case 2（subtype 4）：写 8 B unit 事件（原生 `WriteSettingsFileCmd`，crate 以
//!     [`RecalculateHeightMapCmd`] 承载）；`0.0 < |cursor_moved|` 时写 stairs 桶记录
//!     （`indexmap::Bucket<(DecoratorId,DecoratorId),AlignKind>`，0x28 B；`tag` 2 / 0
//!     两分支，见 [`stairs_record`]），并累加 `|dx*0.002|`，> 0.3 再写 `WindowFocusEvent(1)`；
//!   - case 3（subtype 5）：**TerrainEdit 历史哨兵 `0x8000000000000001`**；写完后校验
//!     之前状态 == deferred 哨兵 `0x8000000000000002`，否则 `panic`（decomp 行 718-729；
//!     与 fresh 顺序一致：先写新状态再检查旧值）；
//!   - case 4（subtype 6）：`&DAT_142ae405c` 子跳表（fresh UNRECOVERED_JUMPTABLE；旧
//!     ReVa 定点恢复见文件末注释，宿主 UI 名队列接入后按名分发）；
//!   - case 6/8/0xc/0xf：响应表字节写（`(&UNK_142ae404d)[idx] = 4/5/6/0` +
//!     `*pcVar39 = 0`），以 [`TerrainWidgetResponseTable`] 承载。
//!
//! 早期 Rust 的 `MouseActivityTimer / 16 位脏掩码→EditTerrainStrokeCmd/SetTerrainStrokePtsCmd`
//! 系**自造近似**（原版脏判定 = 事件队列非空，不产这些命令），已删。
use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};

use crate::{
    resources::{
        cursor_terrain_raycast::CursorTerrainRaycast,
        terrain_editing::{StrokeId, TerrainEditingData},
        window::CursorMovedScreenSpace,
    },
    systems::{
        mode_manager::AppMode,
        terrain::RecalculateHeightMapCmd,
        ui::resources::{
            ui_state::UiState,
            widget_params::{WidgetParams, WidgetVolumeShape},
        },
    },
};

use super::terrain_hover_idx::HoveredTerrainStrokeId;

/// crate 本地 WindowFocusEvent 承载（I：原生 `country_core::resources::window::
/// WindowFocusEvent`，0x10B 步长事件，载荷字节 0/1；crate 事件类型存在，
/// 但 terrain 域以本地类型登记载荷语义）。
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct TerrainWindowFocus(pub u8);

/// 台阶放置数据（O：decomp `indexmap::Bucket<(DecoratorId,DecoratorId), AlignKind>`，
/// 0x28B 元素）——terrain→stairs/decorator 消费链的 CPU 侧接口。
///
/// 原生写偏移按生产者区分（同一泛型被当作手写记录使用，hash 槽位被复用）：
/// - `edit_terrain_stroke`：`+0x00` 事件计数、`+0x08` tag(0/1/2/3/4)、`+0x0c..0x14` 位置、
///   `+0x18/+0x1c` 值、`+0x20` 幅度；
/// - `ui_edit_terrain` case 2：`+0x00` 判别(2/0)、`+0x08` tag(0/2)、`+0x0c..0x18`
///   载荷、`+0x1c` widget id、`+0x20` 事件计数。
///
/// crate 以规范化条目承载（无宿主 consumer，字段语义按原生偏移登记）。
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct StairsPlacementEntry {
    /// +0x08：tag 判别。
    pub tag: u32,
    /// +0x0c：载荷 a。
    pub decorator_a: u32,
    /// +0x10：载荷 b。
    pub decorator_b: u32,
    /// +0x14：载荷 c。
    pub sub: u32,
    /// +0x18：值 a。
    pub value_a: u32,
    /// +0x1c：值 b（ui_edit case 2 = widget id）。
    pub value_b: u32,
    /// +0x20：幅度 / 判别载荷（f32）。
    pub magnitude: f32,
}

/// 台阶放置 Bucket（I：terrain→stairs/decorator 消费链的收集面；宿主/stairs 域
/// 消费后清空）。
#[derive(Resource, Clone, Debug, Default)]
pub struct StairsPlacementBucket {
    pub entries: Vec<StairsPlacementEntry>,
}

/// The native drawing and editing systems share this one owning pending edit.
/// Keep the previous public name as an alias for existing host code.
pub type TerrainEditHistoryPending = super::deferred_terrain_history::PendingTerrainHistory;

/// 地形编辑历史哨兵（O：case 3 `0x8000000000000001`；区别于 deferred 的 `…0002`）。
pub const TERRAIN_EDIT_SENTINEL: u64 = 0x8000_0000_0000_0001;

/// UI 状态槽（O：case 1 写 `puVar25[0..4]`、`*(local_130+0x1e0)=1`、`*pfVar22=0.0`；
/// `puVar25`/`local_130`/`pfVar22` 为宿主 UI 资源，crate 以本适配资源承载）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct TerrainEditUiState {
    /// case 1 写入的 hover 点/朝向（cursor 命中时 = `CursorTerrainRaycast` 位置；否则 0）。
    pub hover: Vec3,
    /// `+0x10` 槽（O：stroke/host 字段写入）。
    pub slot_10: f32,
    /// `+0x1e0` 字节置 1（O：进入拖拽状态）。
    pub active: u8,
    /// 拖动累加器（O：`*pfVar22`；case 1 清 0、case 2 累加 `|dx*0.002|`）。
    pub accum: f32,
    /// Original drag capture: height then falloff are saved before editing.
    pub previous_height_ws: f32,
    pub previous_falloff: f32,
}

/// 响应表（O：`&UNK_142ae404d` 4 槽字节 + `*pcVar39` 响应字节；case 6/8/0xc/0xf 写）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct TerrainWidgetResponseTable {
    /// `(&UNK_142ae404d)[uVar56]`（uVar56 = subtype-2 截断索引 0..3）。
    pub slots: [u8; 4],
    /// `*pcVar39`（写 0）。
    pub response: u8,
}

/// cursor-moved 位移（O：ui_edit 首事件队列 stride 0x10，payload Vec2 @+8/+0xc =
/// [`CursorMovedScreenSpace`]）。返回最后一帧未读事件的 x/y。
fn last_cursor_delta(reader: &mut EventReader<CursorMovedScreenSpace>) -> Vec2 {
    reader
        .read()
        .last()
        .map(|event| event.0)
        .unwrap_or(Vec2::ZERO)
}

/// stairs 桶记录（O：ui_edit case 2 两条分支的机器写偏移）。
///
/// 分支 A（`|dy| <= |dx|`）：`+0x00=2`(判别)、`+0x04=dx*0.002`、`+0x08=0`(tag)、
/// `+0x0c..0x14` 位置、`+0x18=slot_10`、`+0x1c=widget id`、`+0x20=tick`。
/// 分支 B（否则，命中笔画）：`+0x00=0`、`+0x08=2`(tag)、`+0x0c=高度增量`、
/// `+0x10..0x18` 位置、`+0x1c=widget id`、`+0x20=tick`。
fn stairs_record(
    branch_b: bool,
    position: Vec3,
    slot_10: f32,
    widget_id: u32,
    height_delta: f32,
    dx: f32,
) -> StairsPlacementEntry {
    if branch_b {
        StairsPlacementEntry {
            tag: 2,
            decorator_a: height_delta.to_bits(),
            decorator_b: position.x.to_bits(),
            sub: position.y.to_bits(),
            value_a: position.z.to_bits(),
            value_b: widget_id,
            magnitude: height_delta,
        }
    } else {
        StairsPlacementEntry {
            tag: 0,
            decorator_a: dx.to_bits(),
            decorator_b: position.x.to_bits(),
            sub: position.y.to_bits(),
            value_a: position.z.to_bits(),
            value_b: widget_id,
            magnitude: slot_10,
        }
    }
}

/// `ui_edit_terrain` @0x140aef1f0（逐语句重建）。
#[allow(clippy::too_many_arguments)]
pub fn ui_edit_terrain(
    mode: Res<AppMode>,
    cursor: Res<CursorTerrainRaycast>,
    hovered: Res<HoveredTerrainStrokeId>,
    ui_state: Res<UiState>,
    terrain: Res<TerrainEditingData>,
    mut cursor_moved: EventReader<CursorMovedScreenSpace>,
    mut pending: ResMut<TerrainEditHistoryPending>,
    mut stairs: ResMut<StairsPlacementBucket>,
    mut ui: ResMut<TerrainEditUiState>,
    mut response: ResMut<TerrainWidgetResponseTable>,
    mut window_focus: EventWriter<TerrainWindowFocus>,
    mut recalculate: EventWriter<RecalculateHeightMapCmd>,
) {
    if mode.native_tag() != 10 {
        return;
    }
    // hovered 笔画 id（O：`if (*in_stack_00000078 != 0) iVar37 = *(int*)(...+8)`；
    // 资源 tag 非零才读，id@+8）。
    let hovered_id: i32 = if hovered.tag != 0 {
        hovered.id as i32
    } else {
        -1
    };
    // cursor tag 门（O：`*pcVar23 == 1` → WindowFocusEvent(1)）。
    if hovered_id != -1 && cursor.tag == 1 {
        window_focus.write(TerrainWindowFocus(1));
    }
    // 最后一帧 cursor-moved（O：首事件队列，取最后未读；fVar75/fVar76）。
    let delta = last_cursor_delta(&mut cursor_moved);
    let delta_len = (delta.y * delta.y + delta.x * delta.x).sqrt();

    // 逐笔画 widget 注册 + 各自的 subtype switch（O：`do { … } while (puVar48 != 0)`；
    // `UiState::add` 返回值即该 widget 的上下文）。
    let stroke_ids: Vec<StrokeId> = terrain.strokes.keys().copied().collect();
    for id in stroke_ids {
        let ty = if cursor.tag == 0 {
            (hovered_id != id.0 as i32) as u8 * 3 + 4
        } else {
            // O：默认 7（`local_1a0 = '\a'`），cursor.tag != 0 时不按 id 区分。
            7
        };
        let widget = ui_state.add(
            &format!("terrain_edit_{}__canvas", id.0),
            WidgetParams::new(
                WidgetVolumeShape {
                    tag: ty,
                    layer: 0,
                    size: Vec2::splat(130.0),
                },
                [0; 2],
            ),
        );
        let subtype = widget.dragged();
        match subtype {
            3 => {
                // case 1：WindowFocusEvent(0) + hover 点/朝向写入（O decomp 行 422-458）。
                window_focus.write(TerrainWindowFocus(0));
                let (pos, slot_10) = if (cursor.tag & 1) != 0 {
                    (cursor.pos, delta.y)
                } else {
                    (Vec3::ZERO, 0.0)
                };
                ui.hover = pos;
                if let Some(stroke) = terrain.strokes.get(&id) {
                    ui.previous_height_ws = stroke.height_ws;
                    ui.previous_falloff = stroke.falloff;
                }
                ui.slot_10 = slot_10;
                ui.active = 1;
                ui.accum = 0.0;
            }
            4 => {
                // case 2：8 B unit 事件 + stairs 桶记录（O decomp 行 459-675）。
                recalculate.write(RecalculateHeightMapCmd);
                if 0.0 < delta_len {
                    if delta.y.abs() <= delta.x.abs() {
                        // 分支 A：写 tag 0 记录 + 累加器。
                        stairs.entries.push(stairs_record(
                            false, ui.hover, ui.slot_10, id.0, 0.0, delta.x,
                        ));
                        ui.accum += (delta.x * 0.002).abs();
                        if 0.3 < ui.accum {
                            window_focus.write(TerrainWindowFocus(1));
                        }
                    } else if let Some(stroke) = terrain.strokes.get(&id) {
                        // 分支 B：命中笔画 → 高度增量记录（O：`(h/20)^2 - dy*0.02/20`
                        // clamp[0,1] → `sqrt(v)*20 - h`）。
                        let height = stroke.height_ws;
                        let mut v = (height / 20.0) * (height / 20.0) - (delta.y * 0.02) / 20.0;
                        if v < 0.0 {
                            v = 0.0;
                        }
                        if 1.0 < v {
                            v = 1.0;
                        }
                        let height_delta = v.sqrt() * 20.0 - height;
                        stairs.entries.push(stairs_record(
                            true,
                            ui.hover,
                            ui.slot_10,
                            id.0,
                            height_delta,
                            delta.x,
                        ));
                    } else {
                        // O：哈希未中 → log（`&UNK_142ae3aa0`/`&UNK_142ae3a68`）。
                        log::error!(
                            "Was requested to edit a terrain stroke {} that doesnt exist",
                            id.0
                        );
                    }
                }
            }
            5 => {
                window_focus.write(TerrainWindowFocus(0));
                let stroke = terrain
                    .strokes
                    .get(&id)
                    .expect("original terrain editing widget stroke assertion");
                pending.stage(tg_terrain::schema::TerrainEdit::EditTerrainStroke {
                    index: id,
                    prev_height_ws: ui.previous_height_ws,
                    new_height_ws: stroke.height_ws,
                    prev_falloff: ui.previous_falloff,
                    new_falloff: stroke.falloff,
                });
            }
            6 => {
                // case 4：`&DAT_142ae405c` 子跳表（fresh UNRECOVERED_JUMPTABLE）。
                // 旧 ReVa 定点恢复（impl-terrain-deep，2026-10-17）：按 UI 参数/widget 名
                // ev_shift / contrast / saturation / bypass_color 写响应状态字节；
                // 宿主 UI 名队列未接入，登记不伪造。
            }
            8 | 12 | 15 => {
                // case 6/8/0xc/0xf：响应表字节（O decomp 行 760-793）。
                let idx = subtype.wrapping_sub(2).min(3) as usize;
                response.slots[idx] = match subtype {
                    8 => 4,
                    12 => 5,
                    _ => 6,
                };
                response.response = 0;
            }
            _ => {
                // case 0（subtype 2）及其余：无动作。
            }
        }
    }
}
