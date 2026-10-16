//! hint_orchestrator（B 主基线；16 业务）。
//!
//! 证据（skeleton PDB）：
//! - hint_orchestrator 0x14227fc30（L0）：lang_lib::lookup（0x1425e5b70）+ driftsort_main +
//!   EscMenuState::is_open 门控 + Commands::spawn + EntityCommands::insert + format_inner
//!   —— 把 DisplayHintRequest 转成提示实体（文本经 lang-lib 本地化；同类型去重；
//!   esc 菜单打开时跳过）。
//! - filter_to_mode_impl 0x1422808d0（L375）；BindingHintDisplay::alternate/adjacent/
//!   together/steam/is_some/filter_to_mode（组合子实例，见 lib.rs 枚举）。
//! - impl$13/21/23/25/27/31::register_required_components（0x1422809d0..0x142282f40）：
//!   Bevy 注册胶水（不计业务）。

use bevy_ecs::prelude::*;
use country_core::resources::gui::EscMenuState;
use lang_lib::Localization;

use crate::{
    BindingHintDisplay, DisplayHintRequest, HintBundle, HintPlacement, HintText, HintType,
    LingerHintRequest,
};

/// 编排状态（资源）。
#[derive(Resource, Clone, Debug, Default)]
pub struct HintOrchestratorState {
    /// 当前显示的提示类型（去重）。
    pub active: Vec<HintType>,
    pub rollercoaster_timer: f32,
    pub current_prompt: usize,
}

/// 输入保持（hint_camera_display_condition 消费）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct InputHeld(pub bool);

/// 本地化（宿主装配；lang-lib）。
#[derive(Resource)]
pub struct HintLocalization(pub Localization);

/// hint_orchestrator（0x14227fc30）：请求 → 提示实体（lang-lib lookup 文本、
/// esc 菜单门控、同类型去重、driftsort 排序显示序）。
pub fn hint_orchestrator(
    mut state: ResMut<HintOrchestratorState>,
    esc: Res<EscMenuState>,
    localization: Option<Res<HintLocalization>>,
    mut commands: Commands,
    mut requests: EventReader<DisplayHintRequest>,
    mut linger: EventReader<LingerHintRequest>,
    mut query: Query<(Entity, &HintType)>,
) {
    if esc.is_open() {
        // esc 菜单打开：不发新提示（O：is_open 门控）。
        return;
    }
    let mut pending: Vec<DisplayHintRequest> = requests.read().cloned().collect();
    pending.extend(linger.read().cloned().map(|l| DisplayHintRequest {
        display: l.display,
        hint_type: l.hint_type,
        placement: HintPlacement::Top,
    }));
    if pending.is_empty() {
        return;
    }
    // 同类型去重（O：显示中的同型提示不重复）。
    let live: Vec<HintType> = query.iter().map(|(_, t)| *t).collect();
    pending.retain(|p| !live.contains(&p.hint_type));
    // 显示序（O：driftsort_main 排序；按类型序）。
    pending.sort_by_key(|p| hint_type_ordinal(p.hint_type));
    state.active.clear();
    for request in pending {
        let text = lookup_hint_text(&localization, request.hint_type);
        let mut bundle = HintBundle::new(&request.display, request.hint_type);
        bundle.text = HintText {
            key: crate::hint_label(request.hint_type),
            text,
        };
        bundle.placement = request.placement;
        state.active.push(request.hint_type);
        commands.spawn(bundle);
    }
}

/// lang-lib lookup（O：0x1425e5b70；键缺失回退键名）。
pub fn lookup_hint_text(
    localization: &Option<Res<HintLocalization>>,
    hint_type: HintType,
) -> String {
    let key = crate::hint_label(hint_type);
    match localization {
        Some(loc) => loc.0.lookup(&key).to_string(),
        None => key,
    }
}

/// 提示类型序（driftsort 排序键；I：显示顺序推断）。
pub fn hint_type_ordinal(t: HintType) -> u32 {
    match t {
        HintType::CameraPan => 0,
        HintType::CameraRotate => 1,
        HintType::CameraZoom => 2,
        HintType::CameraProgress => 3,
        HintType::FocusEdit => 4,
        HintType::DeselectTool => 5,
        HintType::DisableSnapping => 6,
        HintType::RoofShape => 7,
        HintType::Stairs => 8,
        HintType::TerrainLayering => 9,
        HintType::TerrainFocus => 10,
        HintType::TerrainSteepness => 11,
        HintType::Undo => 12,
        HintType::UnlinkWindows => 13,
        HintType::Tutorial => 14,
    }
}

/// 绑定树展示载荷（宿主 hint_ui 渲染消费；把组合树展平为键帽/图形序列）。
pub fn flatten_display(display: &BindingHintDisplay, out: &mut Vec<FlatHintPart>) {
    match display {
        BindingHintDisplay::Key(key) => out.push(FlatHintPart::Key(*key)),
        BindingHintDisplay::Graphic(g) => out.push(FlatHintPart::Graphic(g.clone())),
        BindingHintDisplay::Alternate(a, b) => {
            flatten_display(a, out);
            flatten_display(b, out);
        }
        BindingHintDisplay::Adjacent(a, b) => {
            flatten_display(a, out);
            flatten_display(b, out);
        }
        BindingHintDisplay::Together(a, b) => {
            flatten_display(a, out);
            flatten_display(b, out);
        }
    }
}

/// 展平后的提示部件。
#[derive(Clone, Debug, PartialEq)]
pub enum FlatHintPart {
    Key(crate::BindingKey),
    Graphic(crate::HintGraphicParams),
}

/// `cleanup_impl`（0x14227f4a0）：清理过期提示实体（显示完成/解析为空 → despawn）。
/// 语义（O 结构）：逐提示实体，`BindingHintDisplay` 解析为空（None）或显示超时 →
/// 移除。
pub fn cleanup_impl(
    mut commands: Commands,
    mut cooldowns: ResMut<crate::hint_display_system::HintDeletionCooldowns>,
    mut query: Query<(Entity, &crate::HintType, &crate::HintAnimation, Option<&crate::BindingHintDisplay>)>,
) {
    let mut done = Vec::new();
    for (entity, hint_type, anim, display) in &query {
        // 解析为空（is_some 语义）或动画完成 → 清理。
        let empty = display.map_or(true, |d| !d.is_some());
        let ord = crate::hint_display_system::hint_type_ordinal(*hint_type);
        if empty || (anim.age >= anim.duration && cooldowns.remaining[ord] <= 0.0) {
            done.push(entity);
        }
    }
    for entity in done {
        commands.entity(entity).despawn();
    }
}
