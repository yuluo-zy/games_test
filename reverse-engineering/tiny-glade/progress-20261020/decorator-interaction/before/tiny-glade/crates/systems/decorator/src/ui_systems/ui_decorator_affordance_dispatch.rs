//! B `ui_systems/ui_decorator_affordance_dispatch.rs` — 装饰操作派发（O）。
//!
//! - `DecoratorAffordanceDispatcher::clear` 0x1407af330（O 机器码手解）：16B
//!   `{state u64@0, payload u64@8}` 写 `{6, 0}`。
//! - `ui_dispatch_decorator_affordance` 0x1407af340（O）：门控 state∈{0,3,4} 或
//!   {11,20}（`state-6<0xf` → state-5 ∈ {6,0xf}；否则 `state<5 && bit∈{0,3,4}`）；
//!   `EscMenuState::is_open`（0x14093fbc0）→ 直接返回；构建 widget
//!   （WidgetParams::new 0x1409a8e10 + UiSounds::empty 0x1409a8fb0 + sounds
//!   0x1409a8e80 + occlude_by_world(0) 0x1409a8ee0 + `UiState::add`
//!   0x1409236d0，名字 "ui_decide_decorator_interaction" 0x142a835e6 31 字符）
//!   后按光标网格步长（0x70B 记录）选最近记录写输出；未激活状态写 clear
//!   记录 {6, 0}。
//! - `create_decorator_widget<&str>` 0x1412823c0 / `<DecoratorHandleName>`
//!   0x141282300（O）：volume `{tag=5, layer}`、flags 字节 `bit1 | (name=='?'
//!   时 bit3)`、sounds 写 0xb0B 参数块 → `UiState::add`（&str 单态 0x1409236d0
//!   / DecoratorHandleName 单态 0x14125d180）。
use bevy_ecs::prelude::*;
use country_core::resources::gui::EscMenuState;
use country_core::systems::ui::resources::ui_state::{UiState, WidgetId};
use country_core::systems::ui::resources::widget_params::{UiSounds, WidgetParams, WidgetVolumeShape};

/// 装饰手柄名（I：`UiState::add<DecoratorHandleName>` 0x14125d180 单态键；
/// 按句柄名哈希注册 widget；原布局 U）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct DecoratorHandleName(pub &'static str);

impl DecoratorHandleName {
    /// 手柄名 → WidgetId（I：WidgetId::from_name 0x14092df30 语义）。
    pub fn to_widget_id(self) -> WidgetId {
        WidgetId::from_name(self.0)
    }
}

/// B DecoratorAffordanceDispatcher（O：clear 写 {6, 0}；门控 state 值域）。
///
/// state 值域（O 门控观察）：0=空闲、3=移动/旋转、4=高度调整、5=（I）、
/// 6=清空（clear 写）、11=楼梯继续（payload 低字节 '?'）、20=（I）。
/// 附加字节 @+0x5c（I：ui_change_stair_height `[*gate+0x5c]=='?'` 门控；
/// 原结构体 > 0x60B，字段表 U）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct DecoratorAffordanceDispatcher {
    /// @+0：待处理交互判别。
    pub state: u64,
    /// @+8：载荷（楼梯继续 '?' 标记低字节；I）。
    pub payload: u64,
    /// @+0x5c：装饰类型字节（'?'=0x3f 特殊标记；I）。
    pub ty: u8,
}

impl DecoratorAffordanceDispatcher {
    /// 0x1407af330（O）：清空 → {6, 0}。
    pub fn clear(&mut self) {
        self.state = 6;
        self.payload = 0;
    }

    /// 门控（I：state==n && payload==0；机器码 128 位检查等价）。
    pub fn gated(&self, state: u64) -> bool {
        self.state == state && self.payload == 0
    }

    /// 楼梯继续门控（O：state==4 或 state==11 && payload 低 u32=='?'）。
    pub fn stairs_gated(&self) -> bool {
        self.state == 4 || (self.state == 11 && (self.payload as u32) == 0x3f)
    }
}

/// 派发激活状态集（O：{0,3,4,11,20}）。
pub fn is_dispatch_active(dispatcher: &DecoratorAffordanceDispatcher) -> bool {
    let s = dispatcher.state;
    (s < 5 && (0x19u64 >> (s & 0x1f) & 1) != 0) || s == 11 || s == 20
}

/// 装饰交互 widget 名（O：0x142a835e6，31 字符；"ui_decide_decorator_interaction"）。
pub const DECIDE_DECORATOR_INTERACTION_WIDGET: &str = "ui_decide_decorator_interaction";

/// B 0x1407af340（O 结构）：按当前派发状态构建交互 widget 并写派发输出。
#[allow(clippy::too_many_arguments)]
pub fn ui_dispatch_decorator_affordance(
    ui: Res<UiState>,
    _cursor: Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,
    _blueprints: Res<crate::blueprint::DecoratorBlueprints>,
    esc: Res<EscMenuState>,
    mut dispatcher: ResMut<DecoratorAffordanceDispatcher>,
) {
    if esc.is_open() {
        return;
    }
    if !is_dispatch_active(&dispatcher) {
        // O：非激活 → 写 clear 记录 {6, 0}。
        dispatcher.clear();
        return;
    }
    // O：WidgetParams::new + UiSounds::empty + occlude_by_world(0) + add。
    let params = WidgetParams::new(WidgetVolumeShape::default(), [0; 2])
        .sounds(UiSounds::empty())
        .occlude_by_world(false);
    ui.add(DECIDE_DECORATOR_INTERACTION_WIDGET, params);
    // O 后续：光标网格步长 0x70B 记录选取写输出（I：精确网格 U）。
}

/// `create_decorator_widget<&str>` 0x1412823c0（O）：字符串名 widget 注册。
/// volume {tag=5, layer}；flags = bit1 | (name=='?' → bit3)。
#[allow(clippy::too_many_arguments)]
pub fn create_decorator_widget(
    ui: &UiState,
    name: &str,
    layer: u32,
    is_question_mark: bool,
    sounds: UiSounds,
) {
    let mut flags = 2u8; // O：bit1 恒置。
    if is_question_mark {
        flags |= 1 << 3; // O：`param_5 == '?'` → bit3。
    }
    let params = WidgetParams::new(WidgetVolumeShape::default(), [0; 2])
        .volume(WidgetVolumeShape {
            tag: 5,
            layer: layer as u8,
            size: Default::default(),
            ..Default::default()
        })
        .sounds(sounds)
        .occlude_by_world(false);
    let _ = flags;
    ui.add(name, params);
}

/// `create_decorator_widget<DecoratorHandleName>` 0x141282300（O）：
/// 手柄名注册单态（UiState::add<DecoratorHandleName> 0x14125d180）。
#[allow(clippy::too_many_arguments)]
pub fn create_decorator_widget_handle(
    ui: &UiState,
    name: DecoratorHandleName,
    layer: u32,
    is_question_mark: bool,
    sounds: UiSounds,
) {
    let params = WidgetParams::new(WidgetVolumeShape::default(), [0; 2])
        .volume(WidgetVolumeShape {
            tag: 5,
            layer: layer as u8,
            size: Default::default(),
            ..Default::default()
        })
        .sounds(sounds)
        .occlude_by_world(false);
    let _ = is_question_mark;
    ui.add_with_id(name.to_widget_id(), params);
}
