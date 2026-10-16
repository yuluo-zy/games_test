//! B `ui_systems/ui_place_decorator.rs` — 装饰放置交互（0x1407a9c10，O 结构；
//! 90KB fresh 反编译）。
//!
//! 主要观察（O）：`EvInitPlaceDecoratorInteractionState` 事件（读+写）初始化
//! 交互状态；`TransientDecoratorIdGen::new/generate`（0x1408339c0/0x1408339e0）
//! 分配瞬时 id；`convert_raycast_hit_to_decorator_dst`（0x14127c990）命中转
//! 目的；`calculate_decorator_interaction_intent` + `find_stair_placement_candidate`
//! （0x1407a7e80）楼梯候选；`show_decorator_icon`（0x1407adb20）/`decorator_cursor_icon`
//! （0x1407adcb0）；`apply_decorator_op`（0x141276e30）；dst 跳表 switch
//! （case 3..0x14 观察）；'?' 类型（info+0x69，I：0x70B 包裹记录类型字节）时
//! 遮罩 0x23 射线（排除装饰碰撞体）；事件面：InstaCreateStairPoint/InstaCreateDeco
//! （读）、RequestAppModeChange/PlayStairsAudioCmd/NotifyHintSystemStairsPlaced/
//! FeedbackInputOutOfBorder/DisplayOutOfBorder（写）。
//! `Local<PlayedConnectSoundLastFrame>`：连接音播放节流（I）。
use bevy_ecs::prelude::*;
use country_core::resources::glade_border::{DisplayOutOfBorder, FeedbackInputOutOfBorder};
use country_core::resources::stairs::stairs_state::StairsState;
use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms;
use country_core::resources::walls::decorator::{DecoratorId, TransientDecoratorIdGen};
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::window::WindowSize;
use country_core::systems::audio::PlayStairsAudioCmd;
use country_core::systems::mode_manager::RequestAppModeChange;
use country_core::systems::ui::resources::ui_state::UiState;
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;
use country_core::systems::ui::signifiers::UiAppendSignifier;

/// 初始化放置交互状态事件（lib.rs 符号；O：读+写对）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct EvInitPlaceDecoratorInteractionState;

/// `Local<PlayedConnectSoundLastFrame>`（I：连接音节流）。
#[derive(Default)]
pub struct PlayedConnectSoundLastFrame(pub bool);

/// 放置交互状态（I：当前放置 id/位置）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct PlaceDecoratorInteractionState {
    pub id: Option<DecoratorId>,
}

/// B 0x1407a9c10（O 结构）：放置主系统。
#[allow(clippy::too_many_arguments)]
pub fn ui_place_decorator(
    _ui: Res<UiState>,
    mut stream: ResMut<UiSignifierStream>,
    _dispatcher: Res<super::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,
    _storage: ResMut<DecoratorStorage>,
    _transforms: Res<CachedDecoratorTransforms>,
    _stairs: Res<StairsState>,
    _window: Res<WindowSize>,
    mut _inits: EventReader<EvInitPlaceDecoratorInteractionState>,
    mut _init_w: EventWriter<EvInitPlaceDecoratorInteractionState>,
    mut _app_mode: EventWriter<RequestAppModeChange>,
    mut _stairs_audio: EventWriter<PlayStairsAudioCmd>,
    // NotifyHintSystemStairsPlaced（onboarding 域）core 未落——登记待接线。
    mut _border_feedback: EventWriter<FeedbackInputOutOfBorder>,
    mut _border_display: EventWriter<DisplayOutOfBorder>,
    mut _insta_point: EventReader<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateStairPoint>,
    mut _insta_deco: EventReader<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateDeco>,
    mut _local: Local<PlayedConnectSoundLastFrame>,
) {
    // O 结构：init 事件 → 瞬时 id 生成器 + 交互状态初始化；随后命中转换 +
    // 意图计算 + 楼梯候选 + 指示符 + 释放 → apply。精确流程 I（90KB 反编译
    // 的相位/跳表细节登记于 RESTORE.md；本实现承载结构面）。
    for _ev in _inits.read() {
        _init_w.write(EvInitPlaceDecoratorInteractionState);
    }
    let mut _gen = TransientDecoratorIdGen::new(std::ptr::null_mut(), false);
    let _ = _gen.generate();
    let _ = &mut stream;
}

/// B 0x1407adb20（O 结构）：放置图标指示符。
/// 记录 {1, 1, pos, icon_byte, mesh 名经 SignifierFactory} → stream write。
pub fn show_decorator_icon(
    icon: u8,
    pos: glam::Vec2,
    id: DecoratorId,
    stream: &mut UiSignifierStream,
) {
    let encoded = (u64::from(id.0)).rotate_right(32);
    stream.write(UiAppendSignifier {
        tag: 1,
        kind: 1,
        position: glam::Vec3::new(pos.x, pos.y, 0.0),
        scale: f32::from(icon),
        name: encoded,
    });
}

/// B 0x1407adcb0（O 结构）：取色光标图标（SignifierEntityBuilder 族
/// 0x14089d9b0/0x14089dc90/0x14089dd50 + "link" 名 0x6b6e696c；宿主接线）。
pub fn decorator_cursor_icon(
    _ui: Res<UiState>,
    mut stream: ResMut<UiSignifierStream>,
) {
    stream.write(UiAppendSignifier {
        tag: 1,
        kind: 0,
        position: Default::default(),
        scale: 1.0,
        name: 0x6b6e696c, // 'link'（O：0x14079b860 名称常量）
    });
}
