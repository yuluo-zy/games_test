//! `systems/steam/mod.rs` — Steam 集成：客户端资源/覆盖层/截图/输入映射。
//!
//! 原文件（B）：`crates/country-core/src/systems/steam/mod.rs`（23 原函数：业务 16 /
//! 内联 2 / 胶水 5）。
//!
//! 本文件为逐函数 fresh 反编译后的重建（2026-10-17，rebuild-steam）。关键证据：
//! - `add_steam_integration` 0x140945320：bevy 插件 build——`insert_resource<SteamClientResource>`/
//!   `SteamScreenshotRequests`、`HookScreenshots`+`ScreenshotRequested` 回调、`PreStartup`
//!   `startup_steam`、`Update` `update_steam_inputs.in_set(GameSystemSet)`。
//! - `update_steam_inputs` 0x140ae9740（22KB，ReVa fresh 反编译 8158 行）：8 参数
//!   `(Res<SteamClientResource>, Local<PrevSteamInputState>, ResMut<SteamInput>,
//!   ResMut<ActionSetValues>, ResMut<ActiveInputModeTracker>,
//!   EventReader<InputCursorDelta>, Res<GameTime>, Res<EscMenuState>)`；
//!   EscMenuState 门 = `tag==2 || (tag==10 && flag!=1)`（与 translate_input 同）；
//!   逐手柄激活动作集 → read_state → is_any_active（模拟 `max(|x|,|y|)>=0.25`）→
//!   controller_deadline = now + 250ms；模拟 → ActionSetValues.current.analog
//!   （camera_pan += 左摇杆、camera_height += 右摇杆×0.1、camera_zoom.x +=
//!   delta×30×1.5×摇杆）；22 同构数字块沿变化簿记（+0x170 减 / +0x178 增）后压
//!   u16 输入码入 `frame_actions`；尾块消费 `InputCursorDelta` 作右键手势
//!   （阈值 3.0，写 current.digital[1]）。
//!
//! 库外 Steam SDK 绑定（原 `steamworks` 0.11.0 git 依赖）经
//! [`SteamSdk`](crate::resources::steam::SteamSdk) trait 注入；本模块只审调用语义。
//! 22 同构块的 SIMD 上帧状态表（16B/槽位图）在重建中简化为逐动作 `prev_digital` 标志，
//! 可观察行为（压入码/计数/模式截止）与 fresh 反编译一致。

use std::sync::Arc;

use bevy_ecs::prelude::*;
use log::{error, info};

use crate::input::{action_set::ActionSetValues, ActiveInputModeTracker, INPUT_MODE_COOLDOWN};
use crate::resources::gui::EscMenuState;
use crate::resources::steam::{
    SteamActionSetDigitalAction, SteamInput, SteamMenuActionSetState, SteamMainActionSetState,
    SteamSdk,
};
use crate::resources::time::GameTime;
use crate::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta;

pub mod startup_steam;

/// 浮窗文本输入打开标志（原进程全局字节 `0x143634d94`，回调缓存）。
static FLOATING_GAMEPAD_TEXT_INPUT_OPEN: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

/// Steam 客户端资源（`update_steam_inputs`/`startup_steam`/覆盖层方法的客户端面）。
#[derive(Resource, Default)]
pub struct SteamClientResource {
    /// 注入的 Steam 客户端（None = 无 Steam）。
    pub client: Option<Arc<dyn SteamSdk>>,
}

impl SteamClientResource {
    /// show_floating_gamepad_text_input 0x140ae9390：全局未打开时才弹浮窗
    /// （`ShowFloatingGamepadTextInput(KeyboardMode, 0, 0, 0x500, 400)`），
    /// 注册 Dismissed 回调并把结果缓存到全局标志。
    pub fn show_floating_gamepad_text_input(&self) {
        if FLOATING_GAMEPAD_TEXT_INPUT_OPEN.load(std::sync::atomic::Ordering::Relaxed) {
            return;
        }
        let Some(sdk) = &self.client else {
            return;
        };
        // steamworks `utils::show_floating_gamepad_text_input`：mode 1 = KeyboardMode，
        // 位置 (0,0)，尺寸 (0x500=1280, 400)。回调注册 + Dismissed 缓存为库外 SDK 职责。
        sdk.show_floating_gamepad_text_input(1, 0, 0, 0x500, 400);
        // 原 `SteamAPI_ISteamUtils_ShowFloatingGamepadTextInput` 返回 bool → 缓存。
        FLOATING_GAMEPAD_TEXT_INPUT_OPEN.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    /// is_floating_gamepad_text_input_open 0x140ae9470：读全局字节（RNE：`movzx eax,
    /// byte [0x143634d94]`）。
    pub fn is_floating_gamepad_text_input_open(&self) -> bool {
        FLOATING_GAMEPAD_TEXT_INPUT_OPEN.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// input_show_binding_panel 0x140ae9480：客户端非空 → `ShowBindingPanel(handle)`。
    pub fn input_show_binding_panel(&self, handle: u64) {
        if let Some(sdk) = &self.client {
            sdk.show_binding_panel(handle);
        }
    }
}

/// AppSteamIntegration：steamworks 客户端封装（`client` 字段 @+0）。
pub struct AppSteamIntegration {
    pub client: Arc<dyn SteamSdk>,
}

impl AppSteamIntegration {
    /// try_new 0x140ae94b0：解析 app id 常量 2198150 → info 日志 →
    /// `Client::init_app(app_id)`（库外 SDK 创建，经 `sdk_factory` 注入）。
    pub fn try_new(
        sdk_factory: impl FnOnce(u32) -> Option<Arc<dyn SteamSdk>>,
    ) -> anyhow::Result<Self> {
        let app_id: u32 = "2198150".parse().expect("steam app id is a decimal constant");
        info!("Initializing Steam API with app id {}", app_id);
        let client = sdk_factory(app_id)
            .ok_or_else(|| anyhow::anyhow!("steamworks Client::init_app failed"))?;
        Ok(Self { client })
    }

    /// client 0x14067dd80：`mov rax, rcx`（client 字段 @+0，按引用返回）。
    pub fn client(&self) -> &Arc<dyn SteamSdk> {
        &self.client
    }

    /// user_id 0x140ae96f0：每次现算 `client.user().steam_id()`（Arc 临时 clone+drop）。
    pub fn user_id(&self) -> u64 {
        self.client.user_steam_id()
    }
}

/// `Local<PrevSteamInputState>`（update_steam_inputs 的本地状态）。
#[derive(Default)]
pub struct PrevSteamInputState {
    /// 帧计数（@0x30；每帧 +1）。
    pub frame_count: u64,
    /// 右键手势移动累加器（@0x38）。
    pub mouse_moved: f32,
    /// 右键按住标志（@0x3c）。
    pub right_click_held: bool,
    /// 本帧是否有控制器产生输入（尾块 `local_158`）。
    pub controller_active: bool,
    /// 本帧产生输入的控制器（尾块 `local_178`）。
    pub active_controller: u64,
}

/// Steam 截图请求资源（`ScreenshotRequested` 回调接收）。
#[derive(Resource, Default)]
pub struct SteamScreenshotRequests {
    /// 有未处理截图请求。
    pub pending: bool,
}

/// 原 `country_core::resources::window::OsWindowCmd` 跨域命令事件；本域重建本地承载，
/// 消费端（窗口/UI 系统）接线前须并入 `resources::window`（U）。
#[derive(Event, Clone, Debug)]
pub struct OsWindowCmd {
    /// 命令载荷（U；本域只发浏览器已打开标记）。
    pub _gap: u64,
}

/// 覆盖层资源面（原 `SteamOverlayOpenResources` SystemParam，含
/// `Option<Res<SteamClientResource>>` 客户端 @+0x20 + `Events<OsWindowCmd>`）。
#[derive(Resource, Default)]
pub struct SteamOverlayOpenResources {
    /// 客户端（None = 无 Steam → 直开浏览器）。
    pub client: Option<Arc<dyn SteamSdk>>,
    /// 浏览器打开回调（库外 webbrowser 注入）。
    pub open_browser: Option<Arc<dyn Fn(&str) -> Result<(), String> + Send + Sync>>,
    /// 成功开浏览器后发事件（原 `Events<OsWindowCmd>`）。
    pub browser_opened_events: bevy_ecs::event::Events<OsWindowCmd>,
}

impl SteamOverlayOpenResources {
    /// can_use_overlay 0x140aef1a0：客户端非空 → `IsSteamRunningOnSteamDeck()`；
    /// 否则 false。
    pub fn can_use_overlay(&self) -> bool {
        match &self.client {
            Some(sdk) => sdk.is_steam_running_on_steam_deck(),
            None => false,
        }
    }

    /// open_overlay_or_browser_to_webpage 0x14026d3c0/0x14026d580：客户端非空 →
    /// 覆盖层 `activate_game_overlay_to_web_page(url)`；否则开浏览器。Deck 检查
    /// 在该路径执行但结果不参与分支（按反编译）。
    pub fn open_overlay_or_browser_to_webpage(&mut self, url: &str) {
        if let Some(sdk) = &self.client {
            let _on_deck = sdk.is_steam_running_on_steam_deck();
            info!("Opening overlay to {}", url);
            sdk.activate_game_overlay_to_web_page(url);
        } else {
            self.open_browser_to_webpage(url);
        }
    }

    /// open_browser_to_webpage 0x14026d080：webbrowser::open(url)；成功 → 发
    /// `OsWindowCmd` 事件；失败 → warn 日志。
    pub fn open_browser_to_webpage(&mut self, url: &str) {
        match self.open_browser.as_deref() {
            Some(open) => match open(url) {
                Ok(()) => {
                    self.browser_opened_events.send(OsWindowCmd { _gap: 1 });
                }
                Err(err) => {
                    error!("Failed opening browser window: {}", err);
                }
            },
            None => {
                error!("Failed opening browser window: no opener injected");
            }
        }
    }
}

/// 22 同构数字动作块（O：0x140ae9740，压入序与反编译一致；动作名候选映射见
/// [`SteamActionSetDigitalAction`]）。
const DIGITAL_BLOCKS: [usize; 22] = [
    // (digital 槽位, 码) —— 码与 0x400..0x101 压入序一一对应。
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
];

const DIGITAL_CODES: [SteamActionSetDigitalAction; 22] = [
    SteamActionSetDigitalAction::LeftClick,
    SteamActionSetDigitalAction::RightClick,
    SteamActionSetDigitalAction::Undo,
    SteamActionSetDigitalAction::Redo,
    SteamActionSetDigitalAction::ToolBuilding,
    SteamActionSetDigitalAction::ToolPath,
    SteamActionSetDigitalAction::ToolDecorator,
    SteamActionSetDigitalAction::ToolCustomize,
    SteamActionSetDigitalAction::ToolEraser,
    SteamActionSetDigitalAction::ToolTerrain,
    SteamActionSetDigitalAction::ToolGarden,
    SteamActionSetDigitalAction::ToolTree,
    SteamActionSetDigitalAction::ToolWater,
    SteamActionSetDigitalAction::TimeOfDay,
    SteamActionSetDigitalAction::Photomode,
    SteamActionSetDigitalAction::ToggleSnapping,
    SteamActionSetDigitalAction::ZoomIn,
    SteamActionSetDigitalAction::ZoomOut,
    SteamActionSetDigitalAction::MainMenu,
    SteamActionSetDigitalAction::UiCancel,
    SteamActionSetDigitalAction::UiConfirm,
    SteamActionSetDigitalAction::UiUp,
];

/// 单个数字动作块的沿变化簿记（O：0x140ae9740 块体）：
/// 读数字动作数据 → 与上帧位比较 → 有沿变化则 `released -= prev_bit`、
/// `pressed += 1`，压入码进 `frame_actions`，写回 `prev_digital`。
fn process_digital_block(
    sdk: &dyn SteamSdk,
    steam_input: &mut SteamInput,
    slot: usize,
    action: SteamActionSetDigitalAction,
) {
    let handle = steam_input.main_action_set.digital[slot];
    if handle == 0 {
        return;
    }
    let data = sdk.digital_action_data(handle);
    let prev = steam_input.prev_digital[slot];
    let new_bit = u64::from(data.state);
    if new_bit != prev {
        steam_input.released = steam_input.released.wrapping_sub(prev & 1);
        steam_input.pressed = steam_input.pressed.wrapping_add(1);
        steam_input.frame_actions.push(action);
        steam_input.prev_digital[slot] = new_bit;
    }
}

/// update_steam_inputs 0x140ae9740（22KB，ReVa fresh 反编译 8158 行；O 结构全解）。
///
/// 参数序即 `add_steam_integration` 0x140945320 in_set 签名（PDB）。EscMenuState 门
/// `tag==2 || (tag==10 && flag!=1)` 与 translate_input 相同（0x140ae9740 行 887）。
///
/// 重建注：原 16B/槽位图的上帧状态表（SIMD 位收敛）与逐块槽位 tag 编码简化为
/// [`SteamInput::prev_digital`] 逐动作标志；可观察行为（码压入/计数/截止更新）一致。
pub fn update_steam_inputs(
    client: Res<SteamClientResource>,
    mut prev_state: Local<PrevSteamInputState>,
    mut steam_input: ResMut<SteamInput>,
    mut action_set_values: ResMut<ActionSetValues>,
    mut input_mode: ResMut<ActiveInputModeTracker>,
    mut input_cursor_delta: EventReader<InputCursorDelta>,
    time: Res<GameTime>,
    menu: Res<EscMenuState>,
) {
    // Arc 克隆门：无客户端 → 直接返回（原 `*local_140 == 0` panic 等价为 Option 门）。
    let Some(sdk) = client.client.clone() else {
        return;
    };
    prev_state.frame_count = prev_state.frame_count.wrapping_add(1);

    // 每帧清空动作队列与簿记。
    steam_input.frame_actions.clear();
    steam_input.released = 0;
    steam_input.pressed = 0;
    steam_input.actions_by_type.clear();

    // RunFrame + 手柄连接查询；原手柄不在连接表则清 0（0x140ae9740 行 383..442）。
    sdk.run_frame();
    let controllers = sdk.connected_controllers();
    if !steam_input.has_controller(steam_input.active_controller)
        && !controllers.contains(&steam_input.active_controller)
    {
        steam_input.active_controller = 0;
        steam_input.last_controller = 0;
    }
    if let Some(first) = controllers.first() {
        if steam_input.active_controller == 0 {
            steam_input.active_controller = *first;
            steam_input.last_controller = *first;
        }
    }

    // 动作集首次加载（0x140ae9740 行 710..838；成功后 info 日志）。
    if steam_input.main_action_set.loaded == 0 {
        if steam_input.main_action_set.try_load(sdk.as_ref()) {
            info!("Loaded the main action set from Steam");
        }
    }
    if steam_input.menu_action_set.loaded == 0 {
        if steam_input.menu_action_set.try_load(sdk.as_ref()) {
            info!("Loaded the Menu action set from Steam");
        }
    }

    let view = menu.view();
    let esc_gate = view.tag == 2 || (view.tag == 10 && view.flag != 1);

    let mut controller_active = false;
    let mut active_controller = steam_input.active_controller;

    for &controller in &controllers {
        if esc_gate && steam_input.main_action_set.loaded == 1 {
            // 激活 GameControls 动作集并读状态。
            sdk.activate_action_set(
                steam_input.main_action_set.action_set_handle,
                controller,
            );
            let mut main_state = SteamMainActionSetState::default();
            steam_input.main_action_set.read_state(sdk.as_ref(), &mut main_state);

            if main_state.is_any_active() {
                let now = prev_state.now_hint();
                input_mode.controller_deadline = now + INPUT_MODE_COOLDOWN;
                controller_active = true;
                active_controller = controller;
            }

            // 模拟 → ActionSetValues.current.analog（O：行 907..920）。
            // camera_pan[2] += 左摇杆（CameraPan 模拟 @0）。
            action_set_values.current.analog[2] += main_state.analog[0];
            // camera_height[3] += 右摇杆 × 0.1（camera_height 模拟 @2）。
            action_set_values.current.analog[3] += main_state.analog[2] * 0.1;
            // camera_zoom[4].x += delta×30×1.5（右摇杆 Y 分量 @1，双向）。
            let step = time.delta * 30.0;
            action_set_values.current.analog[4].x +=
                step * 1.5 * main_state.analog[1].y;

            // 22 同构数字动作块。
            for (slot, code) in DIGITAL_BLOCKS.iter().zip(DIGITAL_CODES.iter()) {
                process_digital_block(
                    sdk.as_ref(),
                    &mut steam_input,
                    *slot,
                    *code,
                );
            }
        } else {
            // 菜单打开：清右键手势状态（行 889..891）。
            prev_state.right_click_held = false;
            prev_state.mouse_moved = 0.0;
        }
    }

    // 菜单动作集：激活 Menu 集 → 读状态 → 任一动作触发则刷新控制器截止。
    if esc_gate && steam_input.menu_action_set.loaded == 1 {
        if let Some(controller) = controllers.first() {
            sdk.activate_action_set(steam_input.menu_action_set.action_set_handle, *controller);
            let mut menu_state = SteamMenuActionSetState::default();
            steam_input.menu_action_set.read_state(sdk.as_ref(), &mut menu_state);
            if menu_state.is_any_active() {
                let now = prev_state.now_hint();
                input_mode.controller_deadline = now + INPUT_MODE_COOLDOWN;
                controller_active = true;
                active_controller = *controller;
            }
            // 菜单数字 → current.digital 槽（ui_cancel 等按 Menu 集序映射）。
            // （0x140ae9740 行 5999 `*(lVar16 + 0x9a) |= bVar12` = current.digital[18]。）
            if menu_state.digital[0] {
                action_set_values.current.digital[18] = true; // ui_cancel 候选
            }
        }
    }

    // 尾块：右键手势（消费 InputCursorDelta，阈值 3.0；O：行 8060..8136）。
    // 右键按住时累加最末事件长度；右键松开且累计 < 3.0 → current.digital[1]。
    let last_len = input_cursor_delta
        .read()
        .last()
        .map_or(0.0, |d| d.0.length());
    if prev_state.right_click_held {
        prev_state.mouse_moved += last_len;
    }
    if controller_active && active_controller != 0 {
        let released_click = prev_state.right_click_held;
        if released_click && prev_state.mouse_moved < 3.0 && prev_state.mouse_moved != 3.0 {
            action_set_values.current.digital[1] = true;
        }
    }
    prev_state.right_click_held = controller_active;
    prev_state.controller_active = controller_active;
    prev_state.active_controller = active_controller;
}

impl PrevSteamInputState {
    /// 原 `Instant::now()`（Windows FILETIME）；本 crate 用相对 Duration 模型，
    /// 宿主按真实时间源填充。此处以 0 占位（translate_input 同约定）。
    fn now_hint(&self) -> core::time::Duration {
        core::time::Duration::from_millis(0)
    }
}

/// 注册 Steam 资源与系统（原 `impl$4::add_steam_integration` 0x140945320 插件 build 的
/// 库内部分；`GameSystemSet` 接线由宿主调度）。
pub fn add_steam_integration(world: &mut World) {
    world.init_resource::<SteamClientResource>();
    world.init_resource::<SteamScreenshotRequests>();
    world.init_resource::<SteamOverlayOpenResources>();
    world.init_resource::<bevy_ecs::event::Events<OsWindowCmd>>();
    // SteamInput 由 startup_steam 经 Commands 插入（无 Default/FromWorld），这里不预置。
}

/// 把 22 个输入码的压入序暴露给录制/回放域（O：0x140ae9740）。
pub fn steam_input_codes() -> &'static [SteamActionSetDigitalAction; 22] {
    &DIGITAL_CODES
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digital_blocks_cover_all_22_codes_in_push_order() {
        let codes = [
            0x400, 0x500, 0x600, 0x700, 0x900, 0x800, 0xa00, 0xb00, 0xc00, 0xe00, 0xd00, 0x200,
            0x300, 0x1300, 0x1400, 0x1600, 0x1700, 0x1800, 0x1500, 0x1200, 0xf00, 0x101,
        ];
        assert_eq!(DIGITAL_CODES.len(), 22);
        for (code, action) in codes.iter().zip(DIGITAL_CODES.iter()) {
            assert_eq!(action.code(), *code);
        }
    }

    #[test]
    fn packed_conversion_helpers_are_the_small_from_thunks() {
        // impl$30/31/37::from 的机器码：x<<8 / (x<<8)+1 / a+(b<<8)+1。
        use crate::resources::steam::packed_set;
        assert_eq!(packed_set(0x12), 0x1200);
    }
}
