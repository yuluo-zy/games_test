//! B `ui_systems/ui_change_chimney_height.rs` — 烟囱高度调整（0x1407ae2e0，O）。
//!
//! 门控（O）：`dispatcher.state==4 && payload==0`；`is_active` 检查；相位
//! （`bVar20 = (dragged-2)<3 ? dragged-2 : 3`，O）：**释放**（dragged≥4，
//! bVar20∈{2,3}）→ `apply_decorator_op`（0x141276e30，8 参：op/kind_flag/dst/
//! storage/archivist/history/events）+ 推 0x28B 高度操作（kind 0x12：
//! `{seq@0, 0x12@8, ratio@0xc, pos@0x18, f32@0x20, 0@0x24}`）+ 队列计数自增；
//! **拖中**（dragged==3）→ `show_signifiers` + 预览状态更新
//! （`*[+0xc] = 光标高差`，FUN_140c9cde0 位置变换）；**悬停**（dragged==2）→
//! 仅 `show_signifiers`。
//! 装饰视觉查表（O）：`param_7` 哈希表（u32 键 → 实体，0x20B 条目）探测
//! （hashbrown 原始表，重建用 std HashMap）+ 实体字段 @+0x128（视觉位置，I）。
use bevy_ecs::prelude::*;
use country_core::resources::walls::decorator::{DecoratorId, DecoratorInfo};
use country_core::resources::walls::decorator_storage::{DecoratorStorage, DecoratorStorageAddr};
use country_core::systems::ui::resources::ui_state::{UiState, WidgetId};
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;
use country_core::systems::ui::signifiers::UiAppendSignifier;
use glam::Vec3;

use crate::apply_decorator_op::{
    apply_decorator_op, write_change_op_history, DecoratorOp, DecoratorOpKind,
};
use crate::deferred_decorator_history::{
    DeferredDecoratorEditHistory, DeferredDecoratorOp, DeferredDecoratorOpQueue,
};
use crate::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher;
use crate::ui_systems::MoveDecorationInteractionState;

/// `Local<EditingState>`（ui_change_chimney_height 域；I）。
#[derive(Default)]
pub struct EditingState {
    /// 烟囱高度比例（I）。
    pub ratio: f32,
}

/// 烟囱预览状态（param_8 观察：{pos u64@0, f32@+0xc 高度差, f32@+0x14}；I）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct ChimneyPreviewState {
    /// 预览位置（u32 对打包）。
    pub pos: (u32, u32),
    /// 高度差。
    pub height_delta: f32,
}

/// B 0x1407ae2e0（O 结构）：烟囱高度调整主系统。
#[allow(clippy::too_many_arguments)]
pub fn ui_change_chimney_height(
    ui: Res<UiState>,
    mut stream: ResMut<UiSignifierStream>,
    dispatcher: Res<DecoratorAffordanceDispatcher>,
    mut storage: ResMut<DecoratorStorage>,
    mut interaction: ResMut<MoveDecorationInteractionState>,
    visuals: Res<ChimneyVisuals>,
    mut preview: ResMut<ChimneyPreviewState>,
    mut op_queue: ResMut<DeferredDecoratorOpQueue>,
    mut history: ResMut<DeferredDecoratorEditHistory>,
    local: Local<EditingState>,
) {
    if !dispatcher.gated(4) {
        return;
    }
    let id = WidgetId(dispatcher.payload);
    if !ui.widget_context(id).is_active() {
        return;
    }
    let dragged = ui.widget_context(id).dragged();
    if dragged < 2 {
        interaction.reset_interaction();
    }
    let decorator_id = DecoratorId(dispatcher.payload as u32);
    let Some(info) = lookup_info(&storage, decorator_id) else {
        panic!("ui_change_chimney_height: no decorator info (ui_change_chimney_height.rs:40)");
    };
    let visual_pos = visuals.position(decorator_id);
    let phase = if dragged.wrapping_sub(2) < 3 {
        dragged.wrapping_sub(2)
    } else {
        3
    };
    match phase {
        // 释放（O：bVar20∈{2,3} → apply + 推 0x28B 高度 op）。
        2 | 3 => {
            let dst = info_dst_addr(&storage, info);
            let op = DecoratorOp {
                kind: match dst {
                    DecoratorStorageAddr::Wall(w) => w.0,
                    DecoratorStorageAddr::Terrain => 1,
                    DecoratorStorageAddr::Stair => 2,
                },
                info: *info,
                extra: [0; 0x20],
            };
            apply_decorator_op(
                &op,
                u8::from(dragged > 1),
                dst,
                &mut storage,
                None::<&mut country_core::recovered_building::wall_decorator_archivist::DecoratorArchivist<()>>,
                Some(&mut history),
                &Default::default(),
            );
            let ratio = local.ratio.max(0.0).min(1.0);
            let seq = op_queue.seq;
            op_queue.push(DeferredDecoratorOp::chimney_height(
                seq,
                ratio,
                preview.pos,
                0.0,
            ));
            interaction.reset_interaction();
        }
        // 拖中（O：bVar20==1 → show + 预览更新）。
        1 => {
            show_signifiers(visual_pos, decorator_id, &mut stream);
            preview.height_delta = preview_ratio(local.ratio);
        }
        // 悬停（O：bVar20==0 → 仅 show）。
        _ => {
            show_signifiers(visual_pos, decorator_id, &mut stream);
        }
    }
    let _ = write_change_op_history;
    let _ = DecoratorOpKind::Change;
}

fn lookup_info<'a>(
    storage: &'a DecoratorStorage,
    id: DecoratorId,
) -> Option<&'a DecoratorInfo> {
    let addr = storage.lookup_storage(id)?;
    storage.get(addr)?.get(id)
}

/// 目的地址（I：DecoratorInfo 目的 tag → 存储地址；disc {0,1}=wall）。
fn info_dst_addr(storage: &DecoratorStorage, info: &DecoratorInfo) -> DecoratorStorageAddr {
    match info.dst_tag() {
        0 | 1 => DecoratorStorageAddr::Wall(
            country_core::resources::walls::WallId(info.dst_wall),
        ),
        2 => DecoratorStorageAddr::Terrain,
        3 => DecoratorStorageAddr::Stair,
        _ => {
            let _ = storage;
            DecoratorStorageAddr::Terrain
        }
    }
}

/// 预览高度比例（I：光标高差/参考高度；精确公式 U）。
fn preview_ratio(_local_ratio: f32) -> f32 {
    0.0
}

/// 装饰视觉表（I：u32 键 → 视觉实体位置；原生 0x20B 条目表，重建 std HashMap）。
#[derive(Resource, Clone, Debug, Default)]
pub struct ChimneyVisuals {
    pub positions: std::collections::HashMap<u32, Vec3>,
}

impl ChimneyVisuals {
    pub fn position(&self, id: DecoratorId) -> Vec3 {
        self.positions.get(&id.0).copied().unwrap_or_default()
    }
}

/// B 0x1407ae910（O 结构）：烟囱高度指示符（两条：'chimney_height_' 0x142a833b0 /
/// 'height_gizmo_' 0x142a83410）。记录 48B {1, 1, enc_id, id, pos, scale} →
/// `UiSignifierStream::write`（0x140b2cf90 拷贝 40B）；mesh 名经
/// SignifierFactory::new（0x14089d8b0，宿主接线，U）。
pub fn show_signifiers(
    visual_pos: Vec3,
    id: DecoratorId,
    stream: &mut UiSignifierStream,
) {
    let encoded = (u64::from(id.0)).rotate_right(32);
    for (name, dy) in [("chimney_height_", 1.2f32), ("height_gizmo_", -1.2f32)] {
        let _ = name;
        stream.write(UiAppendSignifier {
            tag: 1,
            kind: 1,
            position: visual_pos + Vec3::new(0.0, dy, 0.0),
            scale: 1.0,
            // 装饰域 name 槽 = 编码 id（I：原生记录 {enc_id@0x10, id@0x18}）。
            name: encoded,
        });
    }
}
