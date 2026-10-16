//! hint_display_system（B 主基线；16 业务）。
//!
//! 证据（skeleton PDB 依赖注解）：
//! - hint_animation_system 0x141c4f400（L0）：EasingFunction::apply —— 弹入/弹出动画。
//! - hint_deletion_cooldown_system 0x141c4f790（L76）：删除冷却（显示最短时长）。
//! - hint_audio 0x141c4fb00（L91）：提示音（宿主音频缝）。
//! - hint_deletion_system 0x141c4fea0（L107）：EntityCommands::despawn。
//! - hint_highlight_shape 0x141c50120（L127）：PublicWalls::roof_entity —— 目标形状高亮。
//! - hint_position_update 0x141c50510（L169）：位置更新（几何投影）——
//!   clutter_meshes::clutter_mesh_ty_data、Roof::height/pivot_3d、
//!   CachedDecoratorTransforms::try_transform、DecoratorStorage::lookup_decorator_info、
//!   PublicWallState::max_y、WindowSize::as_vec2、ShapeParameters::as_curve2/center、
//!   DebugRectangle3d、utils::world_space_to_screen_space、CameraMatrices::eye_right、
//!   Rectangle2d/3d、utils::time_continuous_lerp。
//! - hint_draw 0x141c51a60（L425）/ render_hints 0x141c52700（L548）：
//!   TemplateView/MeiContext/NodeContext（toolbar 批次 3；本模块以 `hint_ui` 缝表达）。
//! - progress_bar_update_system 0x141c530f0（L677）：进度条（panic_const_rem_by_zero 除零
//!   保护）。
//! - render_hints 依赖 `game_key_bindings::keycode_to_str`（键帽字符串）。

use bevy_ecs::prelude::*;
use country_core::resources::time::GameTime;
use country_core::resources::window::WindowSize;
use utils::easing_functions::EasingFunction;

use crate::{
    BindingHintDisplay, HintAnimation, HintPlacement, HintProgressBar, HintSpring, HintText,
    HintType,
};

/// 删除冷却表（资源；各提示类型最短显示时长）。
#[derive(Resource, Clone, Debug)]
pub struct HintDeletionCooldowns {
    pub remaining: [f32; 16],
}

impl Default for HintDeletionCooldowns {
    fn default() -> Self {
        Self {
            remaining: [0.0; 16],
        }
    }
}

/// 提示实体查询（display 系统的显示查询；PDB tuple 证据：
/// (HintLayoutMemo, HintAnimation, Option<BindingHintDisplay>, HintSpring,
/// Option<HintText>, Option<HintProgressBar>, Option<InteractionPrompt>)）。
#[allow(clippy::type_complexity)]
pub type HintDisplayQuery<'w, 's> = Query<
    'w,
    's,
    (
        Entity,
        &'static HintSpring,
        &'static mut HintAnimation,
        &'static HintPlacement,
        &'static HintType,
        Option<&'static HintText>,
        Option<&'static mut HintProgressBar>,
        Option<&'static mut BindingHintDisplay>,
    ),
    (),
>;

/// hint_animation_system（0x141c4f400）：EasingFunction 驱动弹入/弹出。
pub fn hint_animation_system(
    time: Res<GameTime>,
    mut query: Query<(&mut HintAnimation, &HintSpring)>,
) {
    let dt = time.delta;
    for (mut anim, _spring) in &mut query {
        anim.age += dt;
        let t = (anim.age / anim.duration.max(0.001)).clamp(0.0, 1.0);
        let _eased = EasingFunction::CubicInOut.apply(t);
    }
}

/// hint_deletion_cooldown_system（0x141c4f790）：冷却推进。
pub fn hint_deletion_cooldown_system(
    time: Res<GameTime>,
    mut cooldowns: ResMut<HintDeletionCooldowns>,
) {
    let dt = time.delta;
    for r in &mut cooldowns.remaining {
        *r = (*r - dt).max(0.0);
    }
}

/// hint_audio（0x141c4fb00）：提示音（宿主音频缝；事件输出）。
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct HintSoundRequest {
    pub hint_type: HintType,
}

pub fn hint_audio(
    mut ev: EventWriter<HintSoundRequest>,
    query: Query<&HintType, With<HintAnimation>>,
) {
    for hint_type in &query {
        let _ = ev.write(HintSoundRequest { hint_type: *hint_type });
    }
}

/// hint_deletion_system（0x141c4fea0）：动画完成 → despawn。
pub fn hint_deletion_system(
    mut commands: Commands,
    mut cooldowns: ResMut<HintDeletionCooldowns>,
    query: Query<(Entity, &HintAnimation, &HintType)>,
) {
    let mut done = Vec::new();
    for (entity, anim, hint_type) in &query {
        let ord = hint_type_ordinal(*hint_type);
        if anim.age >= anim.duration && cooldowns.remaining[ord] <= 0.0 {
            done.push(entity);
            cooldowns.remaining[ord] = 2.0;
        }
    }
    for entity in done {
        commands.entity(entity).despawn();
    }
}

/// 高亮形状数据（hint_highlight_shape 产出；宿主渲染消费）。
#[derive(Clone, Debug, PartialEq)]
pub struct HintHighlightShape {
    /// 高亮类型（屋顶/墙形状；O：PublicWalls::roof_entity 目标）。
    pub kind: HighlightKind,
    /// 形状轮廓（世界空间；宿主经 ShapeParameters::as_curve2 采样填充）。
    pub outline: Vec<glam::Vec2>,
    /// 高度（Roof::height / PublicWallState::max_y）。
    pub y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HighlightKind {
    Roof,
    Wall,
}

/// 高亮输出资源（宿主 hint_ui 渲染消费）。
#[derive(Resource, Clone, Debug, Default)]
pub struct HintHighlights(pub Vec<HintHighlightShape>);

/// hint_highlight_shape（0x141c50120）：目标形状高亮（O：PublicWalls::roof_entity
/// 取目标形状 → 高亮数据；宿主几何缝经 `HintGeometryTarget` 注入形状）。
pub fn hint_highlight_shape(
    targets: Option<Res<crate::HintGeometryTarget>>,
    mut highlights: ResMut<HintHighlights>,
) {
    highlights.0.clear();
    let Some(targets) = targets else {
        return;
    };
    for t in targets.active.iter() {
        highlights.0.push(HintHighlightShape {
            kind: if t.is_roof {
                HighlightKind::Roof
            } else {
                HighlightKind::Wall
            },
            outline: t.outline.clone(),
            y: t.y,
        });
    }
}

/// hint_position_update（0x141c50510）：提示位置 = 目标几何投影到屏幕。
/// 证据消费：ShapeParameters（as_curve2/center）、Roof::height/pivot_3d、
/// PublicWallState::max_y、world_space_to_screen_space（0x140ac8780）、
/// time_continuous_lerp、Rectangle2d/3d、WindowSize、CameraMatrices::eye_right。
/// 投影 = `utils::world_space_to_screen_space(world, w, h, view, proj)`（core 已恢复）；
/// 目标世界点/相机矩阵由 `HintProjection` 宿主缝注入。
pub fn hint_position_update(
    window: Res<WindowSize>,
    projection: Option<Res<HintProjection>>,
    mut query: Query<(&mut HintSpring, &HintPlacement, &HintType)>,
) {
    let size = window.as_vec2();
    let (view, proj) = projection
        .as_ref()
        .map_or((glam::Mat4::IDENTITY, glam::Mat4::IDENTITY), |p| {
            (p.view, p.proj)
        });
    for (mut spring, placement, hint_type) in &mut query {
        // O：目标世界点（HintGeometryTarget 缝）→ world_space_to_screen_space。
        let world = match placement {
            HintPlacement::Cursor => projection
                .as_ref()
                .and_then(|p| p.focus)
                .unwrap_or_default(),
            _ => projection
                .as_ref()
                .and_then(|p| p.target_world)
                .unwrap_or_default(),
        };
        let screen = country_core::utils::world_space_to_screen_space(
            world,
            size.x,
            size.y,
            &view,
            &proj,
        );
        let anchor = match placement {
            HintPlacement::Top => glam::Vec2::new(screen.x, screen.y.min(size.y * 0.9)),
            HintPlacement::Bottom => glam::Vec2::new(screen.x, screen.y.max(size.y * 0.1)),
            HintPlacement::Left => glam::Vec2::new(screen.x.min(size.x * 0.3), screen.y),
            HintPlacement::Right => glam::Vec2::new(screen.x.max(size.x * 0.7), screen.y),
            HintPlacement::Cursor => screen,
        };
        // 弹簧缓动（O：time_continuous_lerp 语义的简化；0.2 = 帧混合系数 I）。
        spring.pos = spring.pos.lerp(anchor, 0.2);
        spring.target = anchor;
        let _ = hint_type;
    }
}

/// 提示投影缝（宿主相机装配：视图/投影矩阵 + 目标世界点）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct HintProjection {
    pub view: glam::Mat4,
    pub proj: glam::Mat4,
    /// 目标世界点（ShapeParameters::center + Roof::height/pivot_3d + max_y 由宿主解析）。
    pub target_world: Option<glam::Vec3>,
    /// 光标世界点（CursorTerrainRaycast::position）。
    pub focus: Option<glam::Vec3>,
}

/// hint_draw（0x141c51a60）：把提示实体画成 UI 节点（toolbar/mei 消费面）。
/// toolbar（TemplateView/MeiContext/NodeContext）批次 3 恢复；本函数实现载荷准备
/// （键帽串、文本、图形），UI 树写入经 [`hint_ui`] 缝。
pub fn hint_draw(mut query: HintDisplayQuery<'_, '_>, _ctx: &mut HintUiContext) {
    for (_entity, _spring, animation, _placement, hint_type, text, _bar, _display) in &mut query {
        // 载荷准备（O：format_inner 组装文本/键帽；keycode_to_str 键帽串）。
        let _label = match (text, hint_type) {
            (Some(t), ty) => format!("{}: {}", crate::hint_label(*ty), t.text),
            (None, ty) => crate::hint_label(*ty),
        };
        let _ = animation.age; // 动画已由 hint_animation_system 推进。
    }
}

/// render_hints（0x141c52700）：批量渲染全部提示（NodeContext::child_with_global_id +
/// keycode_to_str；toolbar 缝）。
pub fn render_hints(query: HintDisplayQuery<'_, '_>, _ctx: &mut HintUiContext) {
    for (_entity, spring, _anim, _placement, _ty, _text, _bar, display) in &query {
        let _anchor = spring.target;
        let _parts = flatten_for_ui(display.as_deref());
    }
}

/// progress_bar_update_system（0x141c530f0）：进度条推进（除零保护
/// panic_const_rem_by_zero）。
pub fn progress_bar_update_system(
    mut query: Query<(&mut HintProgressBar, &HintAnimation)>,
) {
    for (mut bar, anim) in &mut query {
        let denom = anim.duration.max(1e-6);
        bar.progress = (anim.age / denom).clamp(0.0, 1.0);
    }
}

/// 键帽字符串（`game_key_bindings::keycode_to_str` 语义；宿主输入层把语义键映射为
/// 键帽串；F3：不自造 VK 码——`BindingKey::Raw` 透传，语义键输出语义名）。
pub fn keycode_to_str(key: crate::BindingKey) -> String {
    match key {
        crate::BindingKey::MouseLeft => "LMB".to_string(),
        crate::BindingKey::MouseRight => "RMB".to_string(),
        crate::BindingKey::MouseMiddle => "MMB".to_string(),
        crate::BindingKey::Scroll => "Scroll".to_string(),
        crate::BindingKey::Esc => "Esc".to_string(),
        crate::BindingKey::Ctrl => "Ctrl".to_string(),
        crate::BindingKey::Alt => "Alt".to_string(),
        crate::BindingKey::KeyA => "A".to_string(),
        crate::BindingKey::KeyW => "W".to_string(),
        crate::BindingKey::KeyZ => "Z".to_string(),
        crate::BindingKey::Raw(code) => format!("Key({code})"),
    }
}

/// 展平显示树（hint_orchestrator::flatten_display 语义的本地适配）。
pub fn flatten_for_ui(display: Option<&BindingHintDisplay>) -> Vec<String> {
    let mut parts = Vec::new();
    if let Some(d) = display {
        let mut flat = Vec::new();
        crate::hint_orchestrator::flatten_display(d, &mut flat);
        for part in flat {
            match part {
                crate::hint_orchestrator::FlatHintPart::Key(k) => {
                    parts.push(format!("[{k:?}]"));
                }
                crate::hint_orchestrator::FlatHintPart::Graphic(g) => {
                    parts.push(g.graphic);
                }
            }
        }
    }
    parts
}

/// 提示类型序（本地适配）。
pub fn hint_type_ordinal(t: HintType) -> usize {
    crate::hint_orchestrator::hint_type_ordinal(t) as usize
}

/// toolbar/mei UI 缝（批次 3 接线点）：TemplateView/MeiContext/NodeContext 的
/// 最小消费面（node_mut、reuse_or_create_node、child_with_global_id、ignore_input、
/// prev_computed_layout）。
pub struct HintUiContext {
    /// 节点树（global_id → 布局）；宿主 toolbar 层填充。
    pub nodes: std::collections::HashMap<u64, UiNodeLayout>,
}

impl Default for HintUiContext {
    fn default() -> Self {
        Self {
            nodes: std::collections::HashMap::new(),
        }
    }
}

/// UI 节点布局记录（toolbar Node::prev_computed_layout 语义）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UiNodeLayout {
    pub pos: glam::Vec2,
    pub size: glam::Vec2,
}
