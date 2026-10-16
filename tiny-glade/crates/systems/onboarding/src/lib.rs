//! system-onboarding（B 主基线）：新手引导提示系统（20 hint_* 文件，60 业务）。
//!
//! 证据（skeleton PDB 元数据 + resources/onboarding.rs.md 章节）：
//! - 组件族：HintLayoutMemo、HintSpring、HintAnimation、HintText、HintType、
//!   HintPlacement（enum）、HintProgressBar、InteractionPrompt；
//!   提示实体 Bundle = (HintLayoutMemo, HintSpring, HintAnimation, HintText, HintType,
//!   HintPlacement)（bevy BundleInfo::write_components 单态化证据）。
//! - 事件：DisplayHintRequest、LingerHintRequest（Events<...> 注册证据）。
//! - `WelcomeScreenState::is_closed`（0x1409a9140）= 判别 == 2（core resources/onboarding.rs
//!   已恢复）；`NotifyHintSystemSnapping` 事件（core）。
//! - add_onboarding 0x140650d50：SubApp::add_systems + chain_ignore_deferred + in_set；
//!   prompt_rollercoaster 0x14227bb70。
//!
//! toolbar（hint_draw/render_hints 的 TemplateView/MeiContext/NodeContext）属批次 3
//! 恢复；本 crate 以 `hint_ui` 模块表达该消费接口（见 hint_display_system）。

pub mod exponential_back_off;
pub mod hint_camera_controls;
pub mod hint_camera_progress;
pub mod hint_clutter_focus_edit;
pub mod hint_deselect_tool;
pub mod hint_disable_snapping;
pub mod hint_display_system;
pub mod hint_focused_edit;
pub mod hint_orchestrator;
pub mod hint_roof_shape;
pub mod hint_stairs;
pub mod hint_terrain;
pub mod hint_terrain_focus;
pub mod hint_terrain_steepness;
pub mod hint_undo;
pub mod hint_unlink_windows;
pub mod hints_startup;
pub mod reset_hints;
pub mod tutorial_completion;

use bevy_ecs::prelude::*;
use bevy_ecs::schedule::Schedule;
use glam::{Vec2, Vec3};

/// 提示图形参数（BindingHintDisplay 组合子载荷；PDB 命名实体）。
#[derive(Clone, Debug, PartialEq)]
pub struct HintGraphicParams {
    /// 图标/图形标签（lang-lib 键或内置标签）。
    pub graphic: String,
    /// 尺寸。
    pub size: Vec2,
    /// 颜色（RGBA 位）。
    pub color: u32,
}

/// 按键绑定显示（enum；alternate/adjacent/together 组合子，O 单态化实例：
/// alternate<Option<KeyCode>> 0x141c4e4f0、alternate<MouseButton> 0x141c4e6d0、
/// alternate<HintGraphicParams> 0x141c4e8b0、alternate<BindingHintDisplay> 0x141c4ecb0、
/// alternate<[Option<KeyCode>;4]> 0x141c4ead0、adjacent<Option<KeyCode>> 0x141c4ee80、
/// adjacent<BindingHintDisplay> 0x141c4f060、together<BindingHintDisplay> 0x141c4f230）。
#[derive(Component, Clone, Debug, PartialEq)]
pub enum BindingHintDisplay {
    /// 键（鼠标/键盘）。
    Key(BindingKey),
    /// 图形。
    Graphic(HintGraphicParams),
    /// 或（任一满足）。
    Alternate(Box<BindingHintDisplay>, Box<BindingHintDisplay>),
    /// 相邻排列。
    Adjacent(Box<BindingHintDisplay>, Box<BindingHintDisplay>),
    /// 同时按下。
    Together(Box<BindingHintDisplay>, Box<BindingHintDisplay>),
}

/// 绑定键。语义变体（Esc/Ctrl/Z/W/A/Alt）为提示语义（F3：不再自造 VK 码；
/// 宿主输入层经绑定树把实际键码映射到语义）；`Raw(u32)` 承载宿主原始键码。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BindingKey {
    MouseLeft,
    MouseRight,
    MouseMiddle,
    Scroll,
    Esc,
    Ctrl,
    Alt,
    KeyA,
    KeyW,
    KeyZ,
    /// 宿主原始键码（未映射到语义时透传）。
    Raw(u32),
}

impl BindingHintDisplay {
    /// `alternate`（O）：或组合。
    pub fn alternate(self, other: Self) -> Self {
        BindingHintDisplay::Alternate(Box::new(self), Box::new(other))
    }
    /// `adjacent`（O）：相邻排列。
    pub fn adjacent(self, other: Self) -> Self {
        BindingHintDisplay::Adjacent(Box::new(self), Box::new(other))
    }
    /// `together`（O）：同时按下。
    pub fn together(self, other: Self) -> Self {
        BindingHintDisplay::Together(Box::new(self), Box::new(other))
    }
    /// `steam`（L353，30 实例）：Steam 覆盖层键帽映射（经 resolve_steam_impl）。
    pub fn steam(self) -> Self {
        resolve_steam_impl(self, true)
    }
    /// `is_some`（L…）：内容非空。
    pub fn is_some(&self) -> bool {
        !matches!(self, BindingHintDisplay::Graphic(g) if g.graphic.is_empty())
    }
    /// `filter_to_mode`（0x1422808d0/impl 0x141c4f230 附近）：按输入模式过滤键帽。
    pub fn filter_to_mode(self, mode: InputMode) -> Self {
        filter_to_mode_impl(self, mode)
    }
}

/// 输入模式（宿主输入层；ActiveInputModeTracker 语义）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputMode {
    KeyboardMouse,
    Gamepad,
}

/// `resolve_steam_impl`（0x14227e5e0）：Steam 覆盖层激活时把键帽解析为 Steam 字形
/// （宿主按键映射缝；结构 = 逐键转 Graphic(steam glyph)）。
pub fn resolve_steam_impl(display: BindingHintDisplay, steam: bool) -> BindingHintDisplay {
    if !steam {
        return display;
    }
    match display {
        BindingHintDisplay::Key(key) => BindingHintDisplay::Graphic(HintGraphicParams {
            graphic: steam_glyph(key),
            size: Vec2::splat(1.0),
            color: 0,
        }),
        BindingHintDisplay::Alternate(a, b) => {
            BindingHintDisplay::Alternate(Box::new(resolve_steam_impl(*a, true)), Box::new(resolve_steam_impl(*b, true)))
        }
        BindingHintDisplay::Adjacent(a, b) => {
            BindingHintDisplay::Adjacent(Box::new(resolve_steam_impl(*a, true)), Box::new(resolve_steam_impl(*b, true)))
        }
        BindingHintDisplay::Together(a, b) => {
            BindingHintDisplay::Together(Box::new(resolve_steam_impl(*a, true)), Box::new(resolve_steam_impl(*b, true)))
        }
        other => other,
    }
}

/// Steam 键帽字形（宿主 Steam 覆盖层映射缝；缺省回退键名）。
pub fn steam_glyph(key: BindingKey) -> String {
    match key {
        BindingKey::MouseLeft => "🖱L".to_string(),
        BindingKey::MouseRight => "🖱R".to_string(),
        BindingKey::MouseMiddle => "🖱M".to_string(),
        BindingKey::Scroll => "🖱⬆".to_string(),
        BindingKey::Esc => "⎋".to_string(),
        BindingKey::Ctrl => "⌃".to_string(),
        BindingKey::Alt => "⌥".to_string(),
        BindingKey::KeyA => "A".to_string(),
        BindingKey::KeyW => "W".to_string(),
        BindingKey::KeyZ => "Z".to_string(),
        BindingKey::Raw(code) => format!("key({code})"),
    }
}

/// `filter_to_mode_impl`（0x1422808d0）：键帽按模式过滤。
pub fn filter_to_mode_impl(display: BindingHintDisplay, mode: InputMode) -> BindingHintDisplay {
    match display {
        BindingHintDisplay::Key(_) if mode == InputMode::Gamepad => BindingHintDisplay::Graphic(
            HintGraphicParams {
                graphic: "gamepad".to_string(),
                size: Vec2::splat(1.0),
                color: 0,
            },
        ),
        other => other,
    }
}

/// 提示类型（HintType；orchestrator 注册组件）。
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub enum HintType {
    CameraPan,
    CameraRotate,
    CameraZoom,
    CameraProgress,
    FocusEdit,
    DeselectTool,
    DisableSnapping,
    RoofShape,
    Stairs,
    TerrainLayering,
    TerrainFocus,
    TerrainSteepness,
    Undo,
    UnlinkWindows,
    Tutorial,
}

/// 提示布局记忆（HintLayoutMemo；宿主布局消费）。
#[derive(Component, Clone, Debug, Default, PartialEq)]
pub struct HintLayoutMemo {
    pub anchor: Vec2,
    pub extent: Vec2,
}

/// 提示弹簧（HintSpring；位置缓动）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct HintSpring {
    pub pos: Vec2,
    pub target: Vec2,
    pub velocity: Vec2,
}

/// 提示动画（HintAnimation；EasingFunction 驱动）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct HintAnimation {
    pub age: f32,
    pub duration: f32,
}

/// 提示文本（HintText；lang_lib::lookup 载荷）。
#[derive(Component, Clone, Debug, Default, PartialEq)]
pub struct HintText {
    pub key: String,
    pub text: String,
}

/// 提示放置（HintPlacement，enum）。
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub enum HintPlacement {
    Top,
    Bottom,
    Left,
    Right,
    Cursor,
}

/// 提示进度条（HintProgressBar）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct HintProgressBar {
    pub progress: f32,
}

/// 交互提示（InteractionPrompt）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct InteractionPrompt {
    pub active: bool,
}

/// 显示提示请求（DisplayHintRequest；orchestrator 消费）。
#[derive(Event, Clone, Debug, PartialEq)]
pub struct DisplayHintRequest {
    pub display: BindingHintDisplay,
    pub hint_type: HintType,
    pub placement: HintPlacement,
}

/// 停留提示请求（LingerHintRequest；长驻提示）。
#[derive(Event, Clone, Debug, PartialEq)]
pub struct LingerHintRequest {
    pub display: BindingHintDisplay,
    pub hint_type: HintType,
    pub duration: f32,
}

/// 提示实体（O Bundle：HintLayoutMemo + HintSpring + HintAnimation + HintText +
/// HintType + HintPlacement）。
#[derive(Bundle)]
pub struct HintBundle {
    pub layout: HintLayoutMemo,
    pub spring: HintSpring,
    pub animation: HintAnimation,
    pub text: HintText,
    pub hint_type: HintType,
    pub placement: HintPlacement,
}

impl HintBundle {
    pub fn new(display: &BindingHintDisplay, hint_type: HintType) -> Self {
        Self {
            layout: HintLayoutMemo::default(),
            spring: HintSpring::default(),
            animation: HintAnimation {
                age: 0.0,
                duration: 0.5,
            },
            text: HintText {
                key: hint_label(hint_type),
                text: String::new(),
            },
            hint_type,
            placement: HintPlacement::Top,
        }
        .with_display(display)
    }
    fn with_display(mut self, _display: &BindingHintDisplay) -> Self {
        // 显示载荷（键帽/图形）经 hint_ui 层渲染；此处保留绑定树。
        self
    }
}

/// 提示标签键（lang-lib lookup 键；I：键名推断）。
pub fn hint_label(hint_type: HintType) -> String {
    match hint_type {
        HintType::CameraPan => "hint.camera_pan".into(),
        HintType::CameraRotate => "hint.camera_rotate".into(),
        HintType::CameraZoom => "hint.camera_zoom".into(),
        HintType::CameraProgress => "hint.camera_progress".into(),
        HintType::FocusEdit => "hint.focus_edit".into(),
        HintType::DeselectTool => "hint.deselect_tool".into(),
        HintType::DisableSnapping => "hint.disable_snapping".into(),
        HintType::RoofShape => "hint.roof_shape".into(),
        HintType::Stairs => "hint.stairs".into(),
        HintType::TerrainLayering => "hint.terrain_layering".into(),
        HintType::TerrainFocus => "hint.terrain_focus".into(),
        HintType::TerrainSteepness => "hint.terrain_steepness".into(),
        HintType::Undo => "hint.undo".into(),
        HintType::UnlinkWindows => "hint.unlink_windows".into(),
        HintType::Tutorial => "hint.tutorial".into(),
    }
}

/// 注册全部 onboarding 系统与资源（B 0x140650d50 `add_onboarding` 语义）。
pub fn configure(world: &mut World, schedule: &mut Schedule) {
    world.init_resource::<hint_orchestrator::HintOrchestratorState>();
    world.init_resource::<exponential_back_off::ExponentialBackOff>();
    world.init_resource::<hint_display_system::HintDeletionCooldowns>();
    world.init_resource::<hint_camera_progress::CameraProgress>();
    world.init_resource::<hints_startup::StartupHints>();
    world.init_resource::<reset_hints::HintResetState>();
    world.init_resource::<tutorial_completion::TutorialCompletionState>();
    world.init_resource::<Events<DisplayHintRequest>>();
    world.init_resource::<Events<LingerHintRequest>>();

    schedule.add_systems(
        (
            (
            hints_startup::hints_startup,
            hint_camera_controls::camera_pan_hint.run_if(hint_camera_controls::hint_camera_enable_condition).run_if(hint_camera_controls::hint_camera_display_condition),
            hint_camera_controls::camera_rotate_hint.run_if(hint_camera_controls::hint_camera_enable_condition).run_if(hint_camera_controls::hint_camera_display_condition),
            hint_camera_controls::camera_zoom_hint.run_if(hint_camera_controls::hint_camera_enable_condition).run_if(hint_camera_controls::hint_camera_display_condition),
            hint_camera_progress::hint_camera_progress_update,
            hint_camera_progress::hint_camera_progress_reset,
            hint_clutter_focus_edit::hint_clutter_focused_edit_hint.run_if(hint_clutter_focus_edit::hint_clutter_focused_edit_condition),
            hint_focused_edit::hint_focused_edit_hint.run_if(hint_focused_edit::hint_focused_edit_condition),
        ).chain(),
        (
            hint_deselect_tool::hint_deselect_tool,
            hint_disable_snapping::hint_disable_snapping,
            hint_unlink_windows::hint_unlink_windows,
            hint_undo::hint_undo,
            hint_roof_shape::hint_roof_shape,
            hint_stairs::hint_stairs,
            hint_terrain::hint_terrain_layering,
            hint_terrain_focus::hint_terrain_focus,
        ).chain(),
        (
            hint_terrain_steepness::hint_terrain_steepness,
            hint_orchestrator::hint_orchestrator,
            hint_display_system::hint_animation_system,
            hint_display_system::hint_deletion_cooldown_system,
            hint_display_system::hint_audio,
            hint_display_system::hint_deletion_system,
            hint_display_system::hint_highlight_shape,
            hint_display_system::hint_position_update,
        ).chain(),
        (
            hint_display_system::progress_bar_update_system,
            reset_hints::reset_hints,
            tutorial_completion::tutorial_completion,
            prompt_rollercoaster,
        ).chain(),
        )
            .chain(),
    );
}

/// prompt_rollercoaster（0x14227bb70）：提示轮播（分批显示；语义 I）。
pub fn prompt_rollercoaster(
    mut state: ResMut<hint_orchestrator::HintOrchestratorState>,
    time: Res<country_core::resources::time::GameTime>,
) {
    state.rollercoaster_timer += time.delta;
    if state.rollercoaster_timer >= 3.0 {
        state.rollercoaster_timer = 0.0;
        state.current_prompt = (state.current_prompt + 1) % 5;
    }
}

/// 射线查询缝（非 native-physics 壳；宿主 native 装配层接线）。
pub mod raycast_seam {
    use country_core::systems::collision::world::RaycastWorld;

    /// legacy_raycast（O 0x1408cc680：mask=0 的 rapier raycast，遗留语义）。
    pub fn legacy_raycast_hit(raycast: &RaycastWorld, origin: glam::Vec3) -> Option<f32> {
        let _ = (raycast, origin);
        None
    }

    /// 射线查询（O 0x1408cc870：17 槽 mask 遍历）。
    pub fn raycast_hit(raycast: &RaycastWorld, origin: glam::Vec3) -> Option<f32> {
        let _ = (raycast, origin);
        None
    }
}

/// 相机/光标焦点（提示显示条件消费）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct HintFocus {
    pub cursor: Vec3,
    pub eye: Vec3,
}

/// 高亮/投影目标几何（宿主装配：PublicWalls::roof_entity + ShapeParameters::as_curve2
/// + Roof::height/pivot_3d + PublicWallState::max_y 解析）。
#[derive(Resource, Clone, Debug, Default)]
pub struct HintGeometryTarget {
    pub active: Vec<HintTargetShape>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HintTargetShape {
    pub is_roof: bool,
    pub outline: Vec<glam::Vec2>,
    pub y: f32,
}
